use std::path::Path;

use indicatif::{ProgressBar, ProgressStyle};

pub struct Progress {
    bar: Option<ProgressBar>,
}

impl Progress {
    pub fn new(total: usize, quiet: bool) -> Self {
        if quiet || total == 0 {
            return Self { bar: None };
        }
        let bar = ProgressBar::new(total as u64);
        let style = ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}",
        )
        .unwrap_or_else(|_| ProgressStyle::default_bar())
        .progress_chars("=>-");
        Self {
            bar: Some(bar.with_style(style)),
        }
    }

    pub fn tick(&self, path: &Path) {
        let Some(bar) = &self.bar else {
            return;
        };
        bar.set_message(path.display().to_string());
        bar.inc(1);
    }

    pub fn finish(&self) {
        if let Some(bar) = &self.bar {
            bar.finish_and_clear();
        }
    }
}
