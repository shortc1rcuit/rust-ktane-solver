use std::collections::HashMap;

use bomb_module::{big_button::BigButton, wrong::Wrong, Module, Solvable};
use edgework::{Edgework, Indicator, Label};
use eframe::egui;

mod bomb_module;
mod edgework;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "KTANE solver",
        native_options,
        Box::new(|cc| Box::new(SolverApp::new(cc))),
    )
    .unwrap()
}

#[derive(Default)]
struct SolverApp<'a> {
    edgework: Edgework<'a>,
    modules: HashMap<Module, Box<dyn Solvable>>,
    selected_module: Option<Module>,
}

impl SolverApp<'_> {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut bomb = Self::default();

        bomb.modules.insert(
            Module {
                id: "The Button".to_string(),
                index: 0,
            },
            Box::<BigButton>::default(),
        );

        bomb.modules.insert(
            Module {
                id: "The Button".to_string(),
                index: 1,
            },
            Box::<BigButton>::default(),
        );

        bomb.edgework.indicators.push(Indicator {
            label: Label::BOB,
            lit: true,
        });

        bomb
    }
}

impl eframe::App for SolverApp<'_> {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            edgework,
            modules,
            selected_module,
        } = self;

        let module_name = match selected_module.clone() {
            None => "".to_string(),
            Some(x) => format!("{}", x),
        };

        egui::SidePanel::left("Bomb info").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Batteries");
                ui.add(egui::DragValue::new(&mut edgework.batteries).speed(0.1));
            });

            ui.horizontal(|ui| {
                ui.label("Holders");
                ui.add(egui::DragValue::new(&mut edgework.holders).speed(0.1));
            });

            ui.collapsing("Indicators", |ui| {
                for (index, indicator) in edgework.indicators.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        egui::ComboBox::from_id_source(format!("Label {}", index))
                            .selected_text(format!("{:?}", indicator.label))
                            .show_ui(ui, |ui| {
                                ui.style_mut().wrap = Some(false);
                                ui.selectable_value(&mut indicator.label, Label::BOB, "BOB");
                                ui.selectable_value(&mut indicator.label, Label::CAR, "CAR");
                                ui.selectable_value(&mut indicator.label, Label::CLR, "CLR");
                                ui.selectable_value(&mut indicator.label, Label::FRK, "FRK");
                                ui.selectable_value(&mut indicator.label, Label::FRQ, "FRQ");
                                ui.selectable_value(&mut indicator.label, Label::IND, "IND");
                                ui.selectable_value(&mut indicator.label, Label::MSA, "MSA");
                                ui.selectable_value(&mut indicator.label, Label::NSA, "NSA");
                                ui.selectable_value(&mut indicator.label, Label::SIG, "SIG");
                                ui.selectable_value(&mut indicator.label, Label::SND, "SND");
                                ui.selectable_value(&mut indicator.label, Label::TRN, "TRN");
                            });

                        ui.checkbox(&mut indicator.lit, "Lit");
                    });
                }

                ui.horizontal(|ui| {
                    if ui.button("+").clicked() {
                        edgework.indicators.push(Indicator::default());
                    }
                    
                    if !edgework.indicators.is_empty() && ui.button("-").clicked() {
                        edgework.indicators.pop();
                    }
                });
            });

            ui.horizontal(|ui| {
                ui.label("Module");
                egui::ComboBox::from_id_source("Current Module")
                    .selected_text(module_name)
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap = Some(false);
                        for module in modules.keys() {
                            ui.selectable_value(
                                selected_module,
                                Some(module.clone()),
                                format!("{}", module),
                            );
                        }
                    });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(module) = selected_module {
                modules
                    .entry(module.clone())
                    .or_insert_with(|| Box::<Wrong>::default())
                    .solve(ui);
            }
        });
    }
}
