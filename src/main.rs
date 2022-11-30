use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "KTANE solver",
        native_options,
        Box::new(|cc| Box::new(SolverApp::new(cc))),
    );
}

#[derive(Debug, PartialEq)]
enum Colour {
    Blue,
    Red,
    White,
    Yellow,
}

#[derive(Debug, PartialEq)]
enum Label {
    Abort,
    Detonate,
    Hold,
    Press,
}

#[derive(PartialEq)]
enum Result {
    Hold,
    Tap,
}

struct SolverApp {
    colour: Colour,
    label: Label,
    batteries: u32,
    car: bool,
    lit_frk: bool,
    result1: Option<Result>,
    led: Colour,
    result2: Option<u8>,
}

impl SolverApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl Default for SolverApp {
    fn default() -> Self {
        Self {
            colour: Colour::Blue,
            label: Label::Abort,
            batteries: 0,
            car: false,
            lit_frk: false,
            result1: None,
            led: Colour::Blue,
            result2: None,
        }
    }
}

impl eframe::App for SolverApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        let Self {
            colour,
            label,
            batteries,
            car,
            lit_frk,
            result1,
            led,
            result2,
        } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Button Colour");

                egui::ComboBox::from_id_source("Button Colour")
                    .selected_text(format!("{:?}", colour))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(colour, Colour::Blue, "Blue");
                        ui.selectable_value(colour, Colour::Red, "Red");
                        ui.selectable_value(colour, Colour::White, "White");
                        ui.selectable_value(colour, Colour::Yellow, "Yellow");
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Button Label");

                egui::ComboBox::from_id_source("Button Label")
                    .selected_text(format!("{:?}", label))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(label, Label::Abort, "Abort");
                        ui.selectable_value(label, Label::Detonate, "Detonate");
                        ui.selectable_value(label, Label::Hold, "Hold");
                        ui.selectable_value(label, Label::Press, "Press");
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Batteries");
                ui.add(egui::DragValue::new(batteries).speed(0.1));
            });

            ui.horizontal(|ui| {
                ui.label("CAR");
                ui.checkbox(car, "");
            });

            ui.horizontal(|ui| {
                ui.label("Lit FRK");
                ui.checkbox(lit_frk, "");
            });

            if ui.button("Solve").clicked() {
                if (*colour == Colour::Blue) && (*label == Label::Abort) {
                    *result1 = Some(Result::Hold);
                } else if (*batteries > 1) && (*label == Label::Detonate) {
                    *result1 = Some(Result::Tap);
                } else if (*colour == Colour::White) && (*car) {
                    *result1 = Some(Result::Hold);
                } else if (*batteries > 2) && (*lit_frk) {
                    *result1 = Some(Result::Tap);
                } else if *colour == Colour::Yellow {
                    *result1 = Some(Result::Hold);
                } else if (*colour == Colour::Red) && (*label == Label::Hold) {
                    *result1 = Some(Result::Tap);
                } else {
                    *result1 = Some(Result::Hold);
                }
            }

            ui.add_space(20.0);

            if let Some(x) = result1 {
                match x {
                    Result::Hold => {
                        ui.label("Hold the button");

                        ui.add_space(20.0);

                        ui.horizontal(|ui| {
                            ui.label("Light Colour");

                            egui::ComboBox::from_id_source("Light Colour")
                                .selected_text(format!("{:?}", led))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(led, Colour::Blue, "Blue");
                                    ui.selectable_value(led, Colour::Red, "Red");
                                    ui.selectable_value(led, Colour::White, "White");
                                    ui.selectable_value(led, Colour::Yellow, "Yellow");
                                });
                        });

                        if ui.button("Solve").clicked() {
                            *result2 = Some(match *led {
                                Colour::Blue => 4,
                                Colour::Yellow => 5,
                                _ => 1,
                            });
                        }

                        if let Some(y) = result2 {
                            ui.label(format!(
                                "Release the button when the timer has a {} in it",
                                y
                            ));
                        }
                    }
                    Result::Tap => {
                        ui.label("Tap the button");
                    }
                };
            }
        });
    }
}
