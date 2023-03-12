use eframe::egui::{self, ColorImage, TextureHandle, Ui};
use image::io::Reader as ImageReader;
use image::ImageError;

use crate::solver_app::bomb_module::Solvable;
use crate::solver_app::edgework::Edgework;

const IMAGES: [&str; 27] = [
    "1.png", "2.png", "3.png", "4.png", "5.png", "6.png", "7.png", "8.png", "9.png", "10.png",
    "11.png", "12.png", "13.png", "14.png", "15.png", "16.png", "17.png", "18.png", "19.png",
    "20.png", "21.png", "22.png", "23.png", "24.png", "25.png", "26.png", "27.png",
];

const COLUMNS: [[usize; 7]; 6] = [
    [24, 11, 25, 10, 6, 8, 20],
    [14, 24, 20, 22, 2, 8, 17],
    [0, 7, 22, 4, 13, 25, 2],
    [9, 18, 26, 6, 4, 17, 3],
    [21, 3, 26, 19, 18, 16, 1],
    [9, 14, 23, 12, 21, 15, 5],
];

#[derive(Default)]
pub struct Keypad {
    textures: Option<Vec<TextureHandle>>,
    symbols: Vec<usize>,
    solved: bool,
}

fn load_image_from_path(path: &str) -> Result<ColorImage, ImageError> {
    let path = format!("assets/Keypad/{path}");

    let image = ImageReader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()))
}

impl Solvable for Keypad {
    fn solve(&mut self, ui: &mut Ui, _: &Edgework) {
        let Self {
            textures,
            symbols,
            solved,
        } = self;

        if textures.is_none() {
            *textures = Some(
                IMAGES
                    .iter()
                    .map(|x| load_image_from_path(x).unwrap())
                    .enumerate()
                    .map(|(index, image)| {
                        ui.ctx().load_texture(
                            format!("Keypad {}", index),
                            image,
                            Default::default(),
                        )
                    })
                    .collect(),
            )
        }

        ui.heading("Symbols");

        let textures = textures.as_ref().unwrap();
        let size = (textures[0].size()[0] as _, textures[0].size()[1] as _);

        ui.horizontal_wrapped(|ui| {
            for (symbol, texture) in textures.iter().enumerate() {
                let contained = symbols.contains(&symbol);

                if ui
                    .add(egui::ImageButton::new(texture, size).selected(contained))
                    .clicked()
                {
                    *solved = false;

                    if contained {
                        symbols.retain(|x| *x != symbol);
                    } else if symbols.len() < 4 {
                        symbols.push(symbol);
                    }
                }
            }
        });

        if ui.button("Solve").clicked() && symbols.len() == 4 {
            if let Some(correct_column) = COLUMNS
                .iter()
                .find(|x| symbols.iter().all(|y| x.contains(y)))
            {
                symbols.sort_by_key(|x| correct_column.iter().position(|y| y == x));
                *solved = true;
            }
        }

        if *solved {
            ui.label("Press the buttons in this order");

            ui.horizontal_wrapped(|ui| {
                for symbol in symbols {
                    ui.image(&textures[*symbol], size);
                }
            });
        }
    }
}
