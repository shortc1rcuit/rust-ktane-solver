pub mod big_button;
pub mod wrong;

use crate::bomb_module::Solvable;

pub fn string_to_solver(name: &str) -> Option<Box<dyn Solvable>> {
    match name {
        "The Button" => Some(Box::<big_button::BigButton>::default()),
        _ => None,
    }
}
