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

struct TempPlotApp {
    data: Arc<Mutex<Vec<[f64; 2]>>>,
    start: Instant,
}

impl Default for TempPlotApp {
    fn default() -> Self {
        let start = Instant::now();
        let thread_start = start.clone();
        let data = Arc::new(Mutex::new(Vec::new()));
        let thread_data = Arc::clone(&data);

        thread::spawn(move || {
            let port_name = "COM4"; // adjust this to your port
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

            let reader = std::io::BufReader::new(port);

            for line in reader.lines() {
                if let Ok(line) = line {
                    if let Ok(temp) = line.trim().parse::<f64>() {
                        let timestamp = thread_start.elapsed().as_secs_f64();
                        let mut vec = thread_data.lock().unwrap();
                        vec.push([timestamp, temp]);

                        if vec.len() > 1000 {
                            vec.remove(0);
                        }
                    }
                }
            }
        });

        Self {
            data,
            start: Instant::now(),
        }
    }
}

impl eframe::App for TempPlotApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Live Temperature Plot");

            let data = self.data.lock().unwrap();
            let line = Line::new(PlotPoints::from(data.clone()));

            Plot::new("temp_plot")
                .view_aspect(2.0)
                .show(ui, |plot_ui| {
                    plot_ui.line(line);
                });
        });

        ctx.request_repaint();
    }
}
