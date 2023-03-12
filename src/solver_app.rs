use std::collections::HashMap;

use bomb_module::Module;
use edgework::{Edgework, Indicator, Label, Port};
use eframe::egui::{self, Ui};
use solvers::{string_to_solver, wrong::Wrong};

use self::bomb_module::ModuleInfo;

mod bomb_module;
mod edgework;
mod solvers;

#[derive(Default)]
pub struct SolverApp {
    edgework: Edgework,
    modules: HashMap<Module, ModuleInfo>,
    added_module: String,
    selected_module: Option<Module>,
}

impl SolverApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self::default()
    }

    fn get_edgework(&mut self, ui: &mut Ui) {
        let edgework = &mut self.edgework;

        ui.heading("Edgework");

        get_batteries(&mut edgework.batteries, ui);
        get_holders(&mut edgework.holders, ui);

        get_indicators(&mut edgework.indicators, ui);

        get_ports(&mut edgework.ports, ui);

        get_serial_num(&mut edgework.serial_num, ui);
    }

    fn add_module(&mut self, ui: &mut Ui) {
        let selected_module = &mut self.selected_module;
        let added_module = &mut self.added_module;
        let modules = &mut self.modules;

        ui.heading("Modules");

        let module_name = match selected_module.clone() {
            None => "".to_string(),
            Some(x) => format!("{}", x),
        };

        ui.horizontal(|ui| {
            ui.label("Add Module");
            ui.add(egui::TextEdit::singleline(added_module).desired_width(120.0));

            if ui.button("Add").clicked() {
                if let Some(solver) = string_to_solver(added_module) {
                    let same_type = modules
                        .keys()
                        .filter(|module| module.id == *added_module)
                        .count();

                    let module = Module {
                        id: added_module.to_string(),
                        index: same_type,
                    };

                    let info = ModuleInfo {
                        solver,
                        solved: false,
                    };

                    modules.insert(module, info);
                }
            }
        });

        ui.horizontal(|ui| {
            ui.label("Modules");
            egui::ComboBox::from_id_source("Current Module")
                .selected_text(module_name)
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);

                    let mut unsolved = modules
                        .iter()
                        .filter(|(_, x)| !x.solved)
                        .map(|(x, _)| x)
                        .collect::<Vec<_>>();
                    unsolved.sort();

                    for module in unsolved {
                        ui.selectable_value(
                            selected_module,
                            Some(module.clone()),
                            format!("{}", module),
                        );
                    }
                });
        });
    }

    fn show_module(&mut self, ui: &mut Ui) {
        let selected_module = &mut self.selected_module;
        let modules = &mut self.modules;
        let edgework = &mut self.edgework;

        if let Some(module) = selected_module {
            modules
                .entry(module.clone())
                .or_insert_with(|| ModuleInfo {
                    solver: Box::<Wrong>::default(),
                    solved: false,
                })
                .solver
                .solve(ui, edgework);
        }
    }

    fn module_result(&mut self, ui: &mut Ui) {
        let selected_module = &mut self.selected_module;
        let modules = &mut self.modules;

        if let Some(module) = selected_module {
            if ui.button("Solved").clicked() {
                modules
                    .entry(module.clone())
                    .or_insert_with(|| ModuleInfo {
                        solver: Box::<Wrong>::default(),
                        solved: false,
                    })
                    .solved = true;

                *selected_module = None;
            }
        }
    }
}

fn get_batteries(batteries: &mut u32, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.label("Batteries");
        ui.add(egui::DragValue::new(batteries).speed(0.1));
    });
}

fn get_holders(holders: &mut u32, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.label("Holders");
        ui.add(egui::DragValue::new(holders).speed(0.1));
    });
}

fn get_indicators(indicators: &mut Vec<Indicator>, ui: &mut Ui) {
    ui.collapsing("Indicators", |ui| {
        for (index, indicator) in indicators.iter_mut().enumerate() {
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
                indicators.push(Indicator::default());
            }

            if !indicators.is_empty() && ui.button("-").clicked() {
                indicators.pop();
            }
        });
    });
}

fn get_ports(ports: &mut Vec<Vec<Port>>, ui: &mut Ui) {
    ui.collapsing("Ports", |ui| {
        for (plate_index, plate) in ports.iter_mut().enumerate() {
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
                            ui.selectable_value(port, Port::DVI, "DVI");
                            ui.selectable_value(port, Port::Parallel, "Parallel");
                            ui.selectable_value(port, Port::PS2, "PS2");
                            ui.selectable_value(port, Port::RCA, "RCA");
                            ui.selectable_value(port, Port::RJ45, "RJ45");
                            ui.selectable_value(port, Port::Serial, "Serial");
                        });
                    });
                }

                ui.horizontal(|ui| {
                    if ui.button("+").clicked() {
                        plate.push(Port::DVI);
                    }

                    if !plate.is_empty() && ui.button("-").clicked() {
                        plate.pop();
                    }
                });
            });
        }

        ui.horizontal(|ui| {
            if ui.button("+").clicked() {
                ports.push(Vec::new());
            }

            if !ports.is_empty() && ui.button("-").clicked() {
                ports.pop();
            }
        });
    });
}

fn get_serial_num(serial_num: &mut String, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.label("Serial Number");
        ui.add(egui::TextEdit::singleline(serial_num).desired_width(60.0));
    });
}

impl eframe::App for SolverApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("Bomb info").show(ctx, |ui| {
            self.get_edgework(ui);

            ui.add_space(10.0);

            self.add_module(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_module(ui);

            ui.add_space(20.0);

            self.module_result(ui);
        });
    }
}
