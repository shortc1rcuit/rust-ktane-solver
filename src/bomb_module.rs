use std::fmt::Display;

use eframe::egui::Ui;

use crate::edgework::Edgework;

pub mod big_button;
pub mod wrong;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Module {
    pub id: String,
    //There may be multiple modules of the same type so we need some way of differentiating them
    pub index: u32,
}

impl Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} #{}", self.id, self.index + 1)
    }
}

pub trait Solvable {
    fn solve(&mut self, ui: &mut Ui, edgework: &Edgework);
}
