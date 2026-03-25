use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

/// Creates a duration-based progress bar (0-100%)
/// Returns `None` if the user requests "simple" mode
pub fn create_progress_bar(duration: Duration, is_simple: bool) -> Option<ProgressBar> {
    if is_simple {
        return None;
    }
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.green/red}] {percent}%",
        )
        .unwrap()
        .progress_chars("█>-"),
    );
    pb.set_message(format!("{}s test", duration.as_secs()));
    Some(pb)
}

/// Updates progress bar based on elapsed time
pub fn update_progress(pb: &Option<ProgressBar>, elapsed: Duration, total: Duration) {
    if let Some(pb) = pb {
        let percent = (elapsed.as_secs_f64() / total.as_secs_f64() * 100.0).min(100.0) as u64;
        pb.set_position(percent);
    }
}
