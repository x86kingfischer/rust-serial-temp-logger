use std::time::Duration;
use std::io::{BufRead, BufReader, Write};
use std::fs::OpenOptions;
use chrono::Local;
use serialport::SerialPort;


fn main() {
    let port_name = "COM4"; // port the arduino is using
    let baud_rate = 9600;

    let port = serialport::new(port_name, baud_rate)
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

    println!("Reading temperature data from {}...", port_name);

    let log_file_path = "temperature_log.csv";
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_path)
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
