mod cli;
mod download;
mod ping;
mod ui;
mod upload;
use cli::Args;
use colored::Colorize;
use serde::Serialize;
use tabled::settings::Style;
use tabled::{Table, Tabled};

fn display_option(o: &Option<f64>) -> String {
    match o {
        Some(v) => format!("{:.2}", v),
        None => "N/A".to_string(),
    }
}

#[derive(Serialize, Default, Tabled)]
#[tabled(display(Option<f64>, "display_option"))]
struct SpeedTestResult {
    #[tabled(rename = "Ping (ms)")]
    ping_ms: Option<f64>,
    #[tabled(rename = "Jitter (ms)")]
    jitter_ms: Option<f64>,
    #[tabled(rename = "Download (Mbps)")]
    download_mbps: Option<f64>,
    #[tabled(rename = "Upload (Mbps)")]
    upload_mbps: Option<f64>,
}

fn main() {
    let args = Args::new();
    let mut final_results = SpeedTestResult::default();
    let run_download = !args.upload_only;
    let run_upload = !args.download_only;

    match ping::measure_ping_and_jitter("8.8.8.8:53", 5) {
        Ok(stats) => {
            final_results.ping_ms = Some(stats.average_ping);
            final_results.jitter_ms = Some(stats.average_jitter);
        }
        Err(_) => {
            eprintln!(
                "{}",
                "You are not even connected to the internet, anyways it's not that deep go touch some grass"
                    .red()
                    .bold()
            );
            std::process::exit(1);
        }
    };

    // -- DOWNLOAD TEST --
    if run_download {
        println!("Measuring download speed (this may take up to 10 seconds)...");
        let download_url = "https://proof.ovh.net/files/100Mb.dat";
        match download::measure_download_speed(download_url, args.simple) {
            Ok(stats) => {
                final_results.download_mbps = Some(stats.mbps);
            }
            Err(e) => {
                eprintln!("{} {}", "Download Error".red().bold(), e.red().bold());
                std::process::exit(1);
            }
        }
    }

    // -- UPLOAD TEST --
    if run_upload {
        println!("Measuring upload speed (this may take up to 10 seconds)...");
        let upload_url = "http://speedtest.tele2.net/upload.php";
        match upload::measure_upload_speed(upload_url, 10, args.simple) {
            Ok(stats) => {
                final_results.upload_mbps = Some(stats.mbps);
            }
            Err(e) => {
                eprintln!("{} {}", "Upload Error".red().bold(), e.red().bold());
                std::process::exit(1);
            }
        }
    }
    // --- 4. FINAL OUTPUT ---
    if args.json {
        match serde_json::to_string_pretty(&final_results) {
            Ok(json_output) => println!("{}", json_output),
            Err(e) => eprintln!("❌ Failed to generate JSON: {}", e),
        }
    } else {
        let mut table = Table::new([&final_results]);
        table.with(Style::ascii());
        println!("{}", table);
    }
}
