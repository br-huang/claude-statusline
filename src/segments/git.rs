use crate::segment::{Segment, RenderContext, SegmentOutput};
use std::error::Error;
use std::process::Command;
use std::sync::Mutex;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
struct GitStatus {
    branch: String,
    ahead: usize,
    behind: usize,
    modified: usize
}

pub struct GitSegment {
    cache: Mutex<Option<(Instant, GitStatus)>>,
    cache_ttl: Duration, // Cache Timeout
}

impl GitSegment {
    pub fn new() -> Self {
        Self {
            cache: Mutex::new(None),
            cache_ttl: Duration::from_secs(5)
        }
    }

    fn get_git_status(&self, workspace_dir: &str) -> Result<GitStatus, Box<dyn Error>> {
        {
            let cache = self.cache.lock().unwrap_or_else(|e| e.into_inner());
            if let Some((timestamp, status)) = cache.as_ref() {
                if timestamp.elapsed() < self.cache_ttl {
                    return Ok(status.clone());
                }
            }
        }
        let status = self.fetch_git_status(workspace_dir)?;

        {
            let mut cache = self.cache.lock().unwrap_or_else(|e| e.into_inner());
            *cache = Some((Instant::now(), status.clone()));
        }

        Ok(status)
    }

    fn fetch_git_status(&self, workspace_dir: &str) -> Result<GitStatus, Box<dyn Error>> {
        let path = std::path::Path::new(workspace_dir);
        if !path.exists() || !path.is_dir() {
            return Err("Invalid workspace directory".into());
        }
        
        let branch_output = Command::new("git")
            .args(["branch", "--show-current"])
            .current_dir(workspace_dir)
            .output()?;

        if !branch_output.status.success() {
            return Err("Not a git repository".into());
        }

        let branch = String::from_utf8_lossy(&branch_output.stdout)
            .trim()
            .to_string();
        
        let status_output = Command::new("git")
            .args(["status", "--porcelain=v2", "--branch"])
            .current_dir(workspace_dir)
            .output()?;

        if !status_output.status.success() {
            return Err("Failed to read git status".into());
        }

        let status_str = String::from_utf8_lossy(&status_output.stdout);

        let mut ahead = 0;
        let mut behind = 0;
        let mut modified = 0;

        for line in status_str.lines() {
            // ab: ahead (+2), behind (-1)
            if line.starts_with("# branch.ab") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    ahead = parts[2]
                        .trim_start_matches('+')
                        .parse()
                        .unwrap_or(0);
                    behind = parts[3]
                        .trim_start_matches('-')
                        .parse()
                        .unwrap_or(0); 
                } 
            } else if line.starts_with("1 ") || line.starts_with("2 ") {
                modified += 1;
            }
        }

        Ok(GitStatus { 
            branch, 
            ahead, 
            behind, 
            modified
         })
            
    }
}

impl Segment for GitSegment {
    fn id(&self) -> &str {
        "git"
    }

    fn priority(&self) -> i32 {
        50
    }

    fn render(&self, ctx: &RenderContext) -> Result<SegmentOutput, Box<dyn Error>> {
        if !self.is_enabled(ctx) {
            return Ok(SegmentOutput::hidden());
        }

        let workspace = &ctx.input.workspace.current_dir;
        let status = match self.get_git_status(workspace) {
            Ok(status) => status,
            Err(_) => return Ok(SegmentOutput::hidden()),
        };

        let colors = &ctx.theme.colors.git;
        let mut parts = Vec::new();

        parts.push(format!(
            "{}{}{}",
            colors.branch,
            status.branch,
            colors.reset,
        ));

        if status.ahead > 0 {
            parts.push(format!(
                "{}↑{} {}",
                colors.ahead,
                status.ahead,
                colors.reset,
            ));
        }

        if status.behind > 0 {
            parts.push(format!(
                "{}↓{} {}",
                colors.behind,
                status.behind,
                colors.reset,
            ));
        }

        if status.modified > 0 {
            parts.push(format!(
                "{}✏{} {}",
                colors.modified,
                status.modified,
                colors.reset,
            ));
        }

        let text = parts.join(" ");

        Ok(SegmentOutput::visible(text))
    }
}

impl Default for GitSegment {
    fn default() -> Self {
        Self::new()
    }
}