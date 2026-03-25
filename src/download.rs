use reqwest::blocking::Client;
use std::io::Read;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct DownloadStats {
    pub mbps: f64,
    pub bytes_downloaded: u64,
    pub duration: Duration,
}

/// Measures download speed by pulling a file in a chunks for a maximum of 10 seconds.
pub fn measure_download_speed(url: &str) -> Result<DownloadStats, String> {
    let client = Client::new();
    let mut response = client
        .get(url)
        .send()
        .map_err(|e| format!("Failed to connect to download server: {}", e))?;
    if !response.status().is_success() {
        return Err(format!("Failed to download file: {}", response.status()));
    }
    let mut buffer = [0; 32 * 1024];
    let max_duration = Duration::from_secs(10);
    let start = Instant::now();
    let mut bytes_downloaded = 0;
    loop {
        match response.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                bytes_downloaded += n as u64;
                if start.elapsed() >= max_duration {
                    break;
                }
            }
            Err(e) => return Err(format!("Error reading data stream: {}", e)),
        }
    }
    let elapsed = start.elapsed();
    let megabits = (bytes_downloaded as f64 * 8.0) / 1_000_000.0;
    let mbps = megabits / elapsed.as_secs_f64();
    Ok(DownloadStats {
        mbps,
        bytes_downloaded,
        duration: elapsed,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measure_download_speed() {
        let result = measure_download_speed("http://httpbin.org/bytes/1024");
        assert!(result.is_ok());
        let stats = result.unwrap();
        assert!(stats.mbps > 0.0);
        assert!(stats.bytes_downloaded > 0);
        assert!(stats.duration < Duration::from_secs(10));
    }
}
