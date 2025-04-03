use eframe::egui::{self, CentralPanel};
use egui_plot::{Line, Plot, PlotPoints};
use std::io::BufRead;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust Serial Temp Logger - GUI",
        options,
        Box::new(|_cc| Box::new(TempPlotApp::default())),
    )
}

// Shared application state, including graph data and session timer
struct TempPlotApp {
    data: Arc<Mutex<Vec<[f64; 2]>>>,
    start: Instant,
}

impl Default for TempPlotApp {
    fn default() -> Self {
        let start = Instant::now(); // Timestamp at app launch
        let data = Arc::new(Mutex::new(Vec::new()));
        let data_clone = Arc::clone(&data);
        let thread_start = start.clone();

        // Spawn a thread to read from the serial port
        thread::spawn(move || {
            let port_name = "COM4"; // Adjust this as needed
            let baud_rate = 9600;

            let port = serialport::new(port_name, baud_rate)
                .timeout(Duration::from_secs(2))
                .open();

            let mut port = match port {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Failed to open port: {}", e);
                    return;
                }
            };

            let reader = std::io::BufReader::new(port);

            // Read each line, parse the float, and store with timestamp
            for line in reader.lines() {
                if let Ok(line) = line {
                    if let Ok(temp) = line.trim().parse::<f64>() {
                        println!("Received: {}", temp); // Debug print

                        let timestamp = thread_start.elapsed().as_secs_f64();

                        let mut vec = data_clone.lock().unwrap();
                        vec.push([timestamp, temp]);

                        if vec.len() > 1000 {
                            vec.remove(0); // Keep buffer size under control
                        }
                    }
                }
            }
        });

        Self {
            data,
            start,
        }
    }
}

impl eframe::App for TempPlotApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Live Temperature Plot");

            let data = self.data.lock().unwrap();

            // Draw the most recent temperature value
            let current_temp = data
                .last()
                .map(|v| format!("Current Temp: {:.2} °C", v[1]))
                .unwrap_or("No data yet.".to_string());
            ui.label(current_temp);

            // Draw the graph using the collected data
            let line = Line::new(PlotPoints::from(data.clone()));
            Plot::new("temp_plot")
                .view_aspect(2.0)
                .show(ui, |plot_ui| {
                    plot_ui.line(line);
                });
        });

        // Keep the plot updating in real-time
        ctx.request_repaint();
    }
}
