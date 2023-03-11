use crate::solver_app::{bomb_module::Solvable, edgework::*};
use eframe::egui::{self, Ui};

#[derive(Debug, PartialEq, Eq)]
enum Colour {
    Blue,
    Red,
    White,
    Yellow,
}

#[derive(Debug, PartialEq)]
enum ButtonLabel {
    Abort,
    Detonate,
    Hold,
    Press,
}

#[derive(PartialEq)]
enum Instruction {
    Hold,
    Tap,
}

pub struct BigButton {
    colour: Colour,
    label: ButtonLabel,
    result1: Option<Instruction>,
    led: Colour,
    result2: Option<u8>,
}

impl Default for BigButton {
    fn default() -> Self {
        Self {
            colour: Colour::Blue,
            label: ButtonLabel::Abort,
            result1: None,
            led: Colour::Blue,
            result2: None,
        }
    }
}

impl Solvable for BigButton {
    fn solve(&mut self, ui: &mut Ui, edgework: &Edgework) {
        ui.horizontal(|ui| {
            ui.label("Button Colour");

            egui::ComboBox::from_id_source("Button Colour")
                .selected_text(format!("{:?}", self.colour))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.colour, Colour::Blue, "Blue");
                    ui.selectable_value(&mut self.colour, Colour::Red, "Red");
                    ui.selectable_value(&mut self.colour, Colour::White, "White");
                    ui.selectable_value(&mut self.colour, Colour::Yellow, "Yellow");
                });
        });

        ui.horizontal(|ui| {
            ui.label("Button Label");

            egui::ComboBox::from_id_source("Button Label")
                .selected_text(format!("{:?}", self.label))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.label, ButtonLabel::Abort, "Abort");
                    ui.selectable_value(&mut self.label, ButtonLabel::Detonate, "Detonate");
                    ui.selectable_value(&mut self.label, ButtonLabel::Hold, "Hold");
                    ui.selectable_value(&mut self.label, ButtonLabel::Press, "Press");
                });
        });

        if ui.button("Solve").clicked() {
            if (self.colour == Colour::Blue) && (self.label == ButtonLabel::Abort) {
                self.result1 = Some(Instruction::Hold);
            } else if (edgework.batteries > 1) && (self.label == ButtonLabel::Detonate) {
                self.result1 = Some(Instruction::Tap);
            } else if (self.colour == Colour::White) && (edgework.label_exists(Label::CAR)) {
                self.result1 = Some(Instruction::Hold);
            } else if (edgework.batteries > 2) && (edgework.indicator_exists(Label::FRK, true)) {
                self.result1 = Some(Instruction::Tap);
            } else if self.colour == Colour::Yellow {
                self.result1 = Some(Instruction::Hold);
            } else if (self.colour == Colour::Red) && (self.label == ButtonLabel::Hold) {
                self.result1 = Some(Instruction::Tap);
            } else {
                self.result1 = Some(Instruction::Hold);
            }
        }

        ui.add_space(20.0);

        if let Some(x) = &self.result1 {
            match x {
                Instruction::Hold => {
                    ui.label("Hold the button");

                    ui.add_space(20.0);

                    ui.horizontal(|ui| {
                        ui.label("Light Colour");

                        egui::ComboBox::from_id_source("Light Colour")
                            .selected_text(format!("{:?}", self.led))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.led, Colour::Blue, "Blue");
                                ui.selectable_value(&mut self.led, Colour::Red, "Red");
                                ui.selectable_value(&mut self.led, Colour::White, "White");
                                ui.selectable_value(&mut self.led, Colour::Yellow, "Yellow");
                            });
                    });

                    if ui.button("Solve").clicked() {
                        self.result2 = Some(match self.led {
                            Colour::Blue => 4,
                            Colour::Yellow => 5,
                            _ => 1,
                        });
                    }

                    if let Some(y) = self.result2 {
                        ui.label(format!(
                            "Release the button when the timer has a {} in it",
                            y
                        ));
                    }
                }
                Instruction::Tap => {
                    ui.label("Tap the button");
                }
            };
        }
    }
}
