use eframe::egui;
use module::{big_button::BigButton, Solvable};

mod module;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "KTANE solver",
        native_options,
        Box::new(|cc| Box::new(SolverApp::new(cc))),
    );
}

struct SolverApp {
    big_button: BigButton,
}

impl SolverApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl Default for SolverApp {
    fn default() -> Self {
        Self {
            big_button: BigButton::default(),
        }
    }
}

impl eframe::App for SolverApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        let Self { big_button } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            big_button.solve(ui);
        });
    }
}
