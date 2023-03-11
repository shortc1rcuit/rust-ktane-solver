mod big_button;
mod wires;
pub mod wrong;

use super::bomb_module::Solvable;

pub fn string_to_solver(name: &str) -> Option<Box<dyn Solvable>> {
    match name {
        "The Button" => Some(Box::<big_button::BigButton>::default()),
        "Wires" => Some(Box::<wires::Wires>::default()),
        _ => None,
    }
}
