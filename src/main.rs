use clap::Parser;
use statusline:: {
    config::Config,
    input::StatusInput,
    output::OutputRenderer,
    segment::RenderContext,
    segments::SegmentRegistry,
    theme::Theme,
};
use std::time::Instant;


#[derive(Parser, Debug)]
#[command(name = "claude-statusline")]
#[command(author = "Brian Huang")]
#[command(version = "0.1.0")]
#[command(about = "Costomized and dedicated statuslien for Cluade Code")]
struct Args {
    #[arg(short, long)]
    theme: Option<String>,

    #[arg(short, long)]
    segments: Option<String>,

    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let start = Instant::now();

    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    // Debug
    let args = Args::parse();
    if args.debug {
        eprintln!("Rendered in {:?}", start.elapsed());
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut config = Config::load()?;

    if let Some(theme) = args.theme {
        config.theme_name = Some(theme);
    }
    if let Some(segments) = args.segments {
        config.enabled_segments = Some(
            segments
                .split(',')
                .map(|s| s.trim().to_string())
                .collect()
        );
    }

    let mut theme = if let Some(theme_path) = &config.theme_path {
        Theme::from_file(theme_path)?
    } else {
        Theme::default()
    };

    if let Some(enabled) = config.enabled_segments {
        theme.enabled_segments = enabled;
    }

    let input = StatusInput::from_stdin()?;
    let ctx = RenderContext{
        input: &input,
        theme: &theme,
    };
    let registry = SegmentRegistry::new();
    let segments = registry.segments();
    let renderer = OutputRenderer::new(&theme);
    let output = renderer.render(&segments, &ctx)?;

    println!("{}", output);

    Ok(())
}