use std::time::Duration;
use std::io::{BufRead, BufReader, Write};
use std::fs::OpenOptions;
use chrono::Local;
use serialport::SerialPort;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Rust Serial Temp Logger")]
#[command(about = "Logs temperature data from a serial device", long_about = None)]
struct Args{
    /// Serial port to connect to (e.g., COM4)
    #[arg(short, long, default_value = "COM4")]
    port: String,

    /// Baud rate (e.g., 9600)
    #[arg(short, long, default_value_t = 9600)]
    baud: u32,

    /// Output CSV file
    #[arg(short, long, default_value = "temperature_log.csv")]
    out: String,
}

fn main() {
    let args = Args::parse();

    let port = serialport::new(&args.port, args.baud)
        .timeout(Duration::from_secs(10))
        .open();

    let port = match port {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to open port: {}", e);
            return;
        }
    };

    let reader = BufReader::new(port.try_clone().expect("Failed to clone port"));

    println!(
        "Reading temperature data from {} at {} baud...",
        args.port, args.baud
    );

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&args.out)
        .expect("Failed to open log file");

    for line in reader.lines() {
        match line {
            Ok(data) => {
                let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
                let line = format!("{},{}\n", timestamp, data);
                print!("{}", line);
                if let Err(e) = file.write_all(line.as_bytes()) {
                    eprintln!("Failed to write to file: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error reading from serial port: {}", e);
                break;
            }
        }
    }
}
