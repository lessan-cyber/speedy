use crate::ui;
use reqwest::blocking::Client;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Debug)]
#[allow(dead_code)]
pub struct UploadStats {
    pub bytes_uploaded: u64,
    pub duration: Duration,
    pub mbps: f64,
}

pub fn measure_upload_speed(
    url: &str,
    size_megabytes: u64,
    is_simple: bool,
) -> Result<UploadStats, String> {
    let max_duration = Duration::from_secs(30);
    let connect_timeout = Duration::from_secs(10);

    let client = Client::builder()
        .connect_timeout(connect_timeout)
        .timeout(max_duration)
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let payload_size = (size_megabytes * 1_048_576) as usize;
    let payload = vec![0u8; payload_size];
    let progress_bar = ui::create_progress_bar(is_simple);

    let start = Instant::now();
    let done = Arc::new(AtomicBool::new(false));

    // Progress thread - exits early when upload completes
    let progress_handle = {
        let pb = progress_bar.clone();
        let done_flag = done.clone();
        std::thread::spawn(move || {
            while !done_flag.load(Ordering::Relaxed) && start.elapsed() < max_duration {
                ui::update_progress(&pb, start.elapsed(), max_duration);
                std::thread::sleep(Duration::from_millis(50));
            }
        })
    };

    // Perform upload
    let response = client
        .post(url)
        .header("Content-Type", "application/octet-stream")
        .body(payload)
        .send()
        .map_err(|e| format!("Failed to connect to upload server: {}", e))?;

    // Signal progress thread to stop
    done.store(true, Ordering::Relaxed);
    progress_handle.join().ok();

    // Check HTTP status - consume response to complete request
    response
        .error_for_status()
        .map_err(|e| format!("Upload server error: {}", e))?
        .bytes()
        .ok();

    let elapsed = start.elapsed();

    if let Some(pb) = &progress_bar {
        pb.set_position(100);
        pb.finish_with_message("Upload test complete");
    }

    let megabits = (payload_size as f64 * 8.0) / 1_000_000.0;
    let mbps = megabits / elapsed.as_secs_f64();

    Ok(UploadStats {
        bytes_uploaded: payload_size as u64,
        duration: elapsed,
        mbps,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_measure_upload_speed() {
        let result = measure_upload_speed("https://speed.cloudflare.com/__up", 1, true);
        assert!(result.is_ok());
        let stats = result.unwrap();
        assert!(stats.mbps > 0.0);
        assert!(stats.bytes_uploaded > 0);
        assert!(stats.duration < Duration::from_secs(30));
    }
}
