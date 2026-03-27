use std::net::TcpStream;
use std::time::Instant;

pub struct PingStats {
    pub average_ping: f64,
    pub average_jitter: f64,
}

/// Pings a host and returns PingStats.
/// Returns a Result with a custom error message if it fails.
pub fn measure_ping_and_jitter(host: &str, num_pings: u32) -> Result<PingStats, String> {
    let mut ping_times = Vec::new();

    for _ in 0..num_pings {
        let start = Instant::now();

        // We use .map_err to convert the standard I/O error into our custom String message
        match TcpStream::connect(host) {
            Ok(_) => {
                ping_times.push(start.elapsed().as_millis() as f64);
            }
            Err(e) => {
                // If even one ping completely fails, we can abort and return the error
                return Err(format!("Failed to connect to {}. Error: {}", host, e));
            }
        }
    }

    if ping_times.is_empty() {
        return Err("Internet connection appears to be down. No pings succeeded.".to_string());
    }

    // 1. Calculate Average Ping
    let total_ping: f64 = ping_times.iter().sum();
    let average_ping = total_ping / ping_times.len() as f64;

    // 2. Calculate Jitter
    let mut total_jitter = 0.0;
    let mut jitter_count = 0;

    for i in 1..ping_times.len() {
        let difference = (ping_times[i] - ping_times[i - 1]).abs();
        total_jitter += difference;
        jitter_count += 1;
    }

    let average_jitter = if jitter_count > 0 {
        total_jitter / jitter_count as f64
    } else {
        0.0
    };

    // Return our shiny new struct wrapped in an Ok()
    Ok(PingStats {
        average_ping,
        average_jitter,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measure_ping_and_jitter_success() {
        let result = measure_ping_and_jitter("8.8.8.8:53", 5);
        assert!(result.is_ok());
        let stats = result.unwrap();
        assert!(stats.average_ping > 0.0);
        assert!(stats.average_jitter >= 0.0);
    }

    #[test]
    fn test_measure_ping_and_jitter_failure() {
        let result = measure_ping_and_jitter("invalid.host:53", 5);
        assert!(result.is_err());
    }
}
