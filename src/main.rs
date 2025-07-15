use eframe::egui;
use rand::Rng;

pub struct AlgoEngineApp {
    price: f64,
    last_signal: String,
}

impl Default for AlgoEngineApp {
    fn default() -> Self {
        Self {
            price: 0.0,
            last_signal: "Nothing yet".to_string(),
        }
    }
}

impl eframe::App for AlgoEngineApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust Algo Engine GUI");

            if ui.button("Next Tick").clicked() {
                self.price = generate_market_price();

                if self.price < 100.0 {
                    self.last_signal = format!("BUY at ${:.2}", self.price);
                } else {
                    self.last_signal = "No trade.".to_string();
                }
            }

            ui.label(format!("Market Price: ${:.2}", self.price));
            ui.label(format!("Strategy Signal: {}", self.last_signal));
        });
    }
}

fn generate_market_price() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(95.0..105.0)
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust Algo Engine",
        options,
        Box::new(|_cc| Box::new(AlgoEngineApp::default())),
    )
}

