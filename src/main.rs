use std::collections::HashMap;

use bomb_module::{big_button::BigButton, wrong::Wrong, Module, Solvable};
use edgework::{Edgework, Indicator, Label, Ports};
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

            ui.collapsing("Ports", |ui| {
                for (plate_index, plate) in edgework.ports.iter_mut().enumerate() {
                    ui.collapsing(format!("Plate #{}", plate_index + 1), |ui| {
                        for (port_index, port) in plate.iter_mut().enumerate() {
                            ui.horizontal(|ui| {
                                egui::ComboBox::from_id_source(format!(
                                    "Port {}-{}",
                                    plate_index, port_index
                                ))
                                .selected_text(format!("{:?}", port))
                                .show_ui(ui, |ui| {
                                    ui.style_mut().wrap = Some(false);
                                    ui.selectable_value(port, Ports::DVI, "DVI");
                                    ui.selectable_value(port, Ports::Parallel, "Parallel");
                                    ui.selectable_value(port, Ports::PS2, "PS2");
                                    ui.selectable_value(port, Ports::RCA, "RCA");
                                    ui.selectable_value(port, Ports::RJ45, "RJ45");
                                    ui.selectable_value(port, Ports::Serial, "Serial");
                                });
                            });
                        }

                        ui.horizontal(|ui| {
                            if ui.button("+").clicked() {
                                plate.push(Ports::DVI);
                            }

                            if !plate.is_empty() && ui.button("-").clicked() {
                                plate.pop();
                            }
                        });
                    });
                }

                ui.horizontal(|ui| {
                    if ui.button("+").clicked() {
                        edgework.ports.push(Vec::new());
                    }

                    if !edgework.ports.is_empty() && ui.button("-").clicked() {
                        edgework.ports.pop();
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
