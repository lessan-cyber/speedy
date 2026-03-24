mod cli;
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
}
