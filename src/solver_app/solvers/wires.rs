use crate::solver_app::{bomb_module::Solvable, edgework::*};
use eframe::egui::{self, Ui};

#[derive(Clone, Debug, PartialEq)]
enum Colour {
    Black,
    Blue,
    Red,
    White,
    Yellow,
}

pub struct Wires {
    wires: Vec<Colour>,
    result: Option<u8>,
}

impl Default for Wires {
    fn default() -> Self {
        Self {
            wires: vec![Colour::Black; 3],
            result: None,
        }
    }
}

impl Solvable for Wires {
    fn solve(&mut self, ui: &mut Ui, edgework: &Edgework) {
        let Self { wires, result } = self;

        for (index, wire) in wires.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                egui::ComboBox::from_label(format!("Wire #{}", index + 1))
                    .selected_text(format!("{:?}", wire))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(wire, Colour::Black, "Black");
                        ui.selectable_value(wire, Colour::Blue, "Blue");
                        ui.selectable_value(wire, Colour::Red, "Red");
                        ui.selectable_value(wire, Colour::White, "White");
                        ui.selectable_value(wire, Colour::Yellow, "Yellow");
                    });
            });
        }

        ui.horizontal(|ui| {
            if wires.len() < 6 && ui.button("+").clicked() {
                wires.push(Colour::Black);
            }

            if wires.len() > 3 && ui.button("-").clicked() {
                wires.pop();
            }
        });

        if ui.button("Solve").clicked() {
            *result = match wires.len() {
                3 => {
                    if !wires.contains(&Colour::Red) {
                        Some(2)
                    } else if wires[2] == Colour::White {
                        Some(3)
                    } else {
                        let blue_wires = wires
                            .iter()
                            .enumerate()
                            .filter(|(_, wire)| **wire == Colour::Blue);

                        if blue_wires.clone().count() > 1 {
                            Some(blue_wires.last().unwrap().0 as u8 + 1)
                        } else {
                            Some(3)
                        }
                    }
                }
                4 => {
                    let red_wires = wires
                        .iter()
                        .enumerate()
                        .filter(|(_, wire)| **wire == Colour::Red);

                    if (edgework.last_digit_serial() % 2 == 1) && (red_wires.clone().count() > 1) {
                        Some(red_wires.last().unwrap().0 as u8 + 1)
                    } else if (red_wires.count() == 0)
                        || (wires.iter().filter(|wire| **wire == Colour::Blue).count() == 1)
                    {
                        Some(1)
                    } else if wires.iter().filter(|wire| **wire == Colour::Yellow).count() > 1 {
                        Some(4)
                    } else {
                        Some(2)
                    }
                }
                5 => {
                    if (wires[4] == Colour::Black) && (edgework.last_digit_serial() % 2 == 1) {
                        Some(4)
                    } else if (wires.iter().filter(|wire| **wire == Colour::Red).count() == 1)
                        && (wires.iter().filter(|wire| **wire == Colour::Yellow).count() > 1)
                    {
                        Some(1)
                    } else if wires.iter().filter(|wire| **wire == Colour::Black).count() == 0 {
                        Some(2)
                    } else {
                        Some(1)
                    }
                }
                6 => {
                    if wires.iter().filter(|wire| **wire == Colour::Yellow).count() == 0
                        && (edgework.last_digit_serial() % 2 == 1)
                    {
                        Some(3)
                    } else if (wires.iter().filter(|wire| **wire == Colour::Yellow).count() == 1)
                        && (wires.iter().filter(|wire| **wire == Colour::White).count() > 1)
                    {
                        Some(4)
                    } else if wires.iter().filter(|wire| **wire == Colour::Red).count() == 0 {
                        Some(6)
                    } else {
                        Some(4)
                    }
                }
                _ => None,
            }
        }

        if let Some(result) = result {
            ui.label(format!("Cut wire {}", result));
        }
    }
}
