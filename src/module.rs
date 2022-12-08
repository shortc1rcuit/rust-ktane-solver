use eframe::egui::Ui;

pub mod big_button;

enum ModuleIDs {
    BigButton,
}

struct Module {
    id: ModuleIDs,
    //There may be multiple modules of the same type so we need some way of differentiating them
    index: u32,
}

pub trait Solvable {
    fn solve(&mut self, ui: &mut Ui);
}
