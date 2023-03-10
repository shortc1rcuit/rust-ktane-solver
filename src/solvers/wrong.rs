use crate::{bomb_module::Solvable, edgework::Edgework};
use eframe::egui::Ui;

//Used to tell me if something has gone wrong

#[derive(Default)]
pub struct Wrong;

impl Solvable for Wrong {
    fn solve(&mut self, ui: &mut Ui, _: &Edgework) {
        ui.label("SOMETHING HAS GONE WRONG");
    }
}
