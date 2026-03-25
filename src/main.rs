mod cli;
mod download;
mod ping;

use cli::Args;
use colored::Colorize;
//use ping::PingStats;

fn main() {
    let args = Args::new();
    println!("{}", args.simple);

    match ping::measure_ping_and_jitter("8.8.8.8:53", 5) {
        Ok(stats) => {
            println!("Average Ping: {:.2} ms", stats.average_ping);
            println!("Average Jitter: {:.2} ms", stats.average_jitter);
        }
        Err(_) => {
            // Print to standard error instead of standard out
            eprintln!(
                "{}",
                "You are not even connected to the internet, anyways it's not that deep go touch some grass".red().bold()
            );
            // Exit gracefully with an error code (1)
            std::process::exit(1);
        }
    };

    // -- DOWNLOAD TEST --
    println!("Measuring download speed(this may take up to 10 seconds)...");
    let download_url = "https://proof.ovh.net/files/100Mb.dat";
    match download::measure_download_speed(download_url) {
        Ok(stats) => {
            let downloaded_mb = stats.bytes_downloaded as f64 / 1_048_576.0;
            println!(
                "Download: {:.2} Mbps (Downloaded {:.2} MB in {:.2} seconds)",
                stats.mbps,
                downloaded_mb,
                stats.duration.as_secs_f64()
            )
        }
        Err(e) => {
            eprintln!("{} {}", "Download Error".red().bold(), e.red().bold());
            std::process::exit(1);
        }
    }
}
