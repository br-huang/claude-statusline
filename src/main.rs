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
#[command(version = "1.0.0")]
#[command(about = "Customized and dedicated statusline for Claude Code")]
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
    let args = Args::parse();

    if let Err(e) = run(&args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    // Debug
    if args.debug {
        eprintln!("Rendered in {:?}", start.elapsed());
    }
}

fn run(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::load()?;

    if let Some(theme) = args.theme.as_deref() {
        config.theme_name = Some(theme.to_string());
     }
    if let Some(segments) = args.segments.as_deref() {
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