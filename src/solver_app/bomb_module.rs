use std::fmt::Display;

use eframe::egui::Ui;

use super::edgework::Edgework;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Module {
    pub id: String,
    //There may be multiple modules of the same type so we need some way of differentiating them
    pub index: usize,
}

impl Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} #{}", self.id, self.index + 1)
    }
}

impl PartialOrd for Module {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.id.partial_cmp(&other.id) {
            Some(core::cmp::Ordering::Equal) => self.index.partial_cmp(&other.index),
            ord => ord,
        }
    }
}

impl Ord for Module {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.id.cmp(&other.id) {
            std::cmp::Ordering::Equal => self.index.cmp(&other.index),
            ord => ord,
        }
    }
}

pub trait Solvable {
    fn solve(&mut self, ui: &mut Ui, edgework: &Edgework);
}
