use crate::ui;
use reqwest::blocking::Client;
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
    let client = Client::new();
    let payload_size = (size_megabytes * 1_048_576) as usize;
    let max_duration = Duration::from_secs(10);
    let progress_bar = ui::create_progress_bar(is_simple);

    let start = Instant::now();
    let payload = vec![0u8; payload_size];

    // Update progress in background while upload happens
    let progress_handle = {
        let pb = progress_bar.clone();
        let max_dur = max_duration;
        std::thread::spawn(move || {
            while start.elapsed() < max_dur {
                ui::update_progress(&pb, start.elapsed(), max_dur);
                std::thread::sleep(Duration::from_millis(50));
            }
        })
    };

    let response = client.post(url).body(payload).send();

    progress_handle.join().ok();

    let response = response.map_err(|e| format!("Failed to connect to upload server: {}", e))?;

    // Read response to completion
    let _ = response.text();

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
    fn test_measure_upload_speed() {
        let result = measure_upload_speed("http://httpbin.org/post", 1, true);
        assert!(result.is_ok());
        let stats = result.unwrap();
        assert!(stats.mbps > 0.0);
        assert!(stats.bytes_uploaded > 0);
        assert!(stats.duration < Duration::from_secs(10));
    }
}
