use std::collections::HashMap;

use bomb_module::{big_button::BigButton, wrong::Wrong, Module, ModuleIDs, Solvable};
use eframe::egui;

mod bomb_module;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "KTANE solver",
        native_options,
        Box::new(|cc| Box::new(SolverApp::new(cc))),
    );
}

#[derive(Default)]
struct SolverApp {
    modules: HashMap<Module, Box<dyn Solvable>>,
    selected_module: Option<Module>,
}

impl SolverApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut bomb = Self::default();

        bomb.modules.insert(
            Module {
                id: ModuleIDs::BigButton,
                index: 0,
            },
            Box::new(BigButton::default()),
        );

        bomb.modules.insert(
            Module {
                id: ModuleIDs::BigButton,
                index: 1,
            },
            Box::new(BigButton::default()),
        );
        bomb
    }
}

impl eframe::App for SolverApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            modules,
            selected_module,
        } = self;

        let module_name = match selected_module.clone() {
            None => "".to_string(),
            Some(x) => format!("{}", x),
        };

        egui::SidePanel::left("Bomb info").show(ctx, |ui| {
            egui::ComboBox::from_id_source("Current Module")
                .selected_text(module_name)
                .show_ui(ui, |ui| {
                    for module in modules.iter().map(|(key, _)| key) {
                        ui.selectable_value(
                            selected_module,
                            Some(module.clone()),
                            format!("{}", module),
                        );
                    }
                });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(module) = selected_module {
                modules
                    .entry(module.clone())
                    .or_insert_with(|| Box::new(Wrong::default()))
                    .solve(ui);
            }
        });
    }
}
