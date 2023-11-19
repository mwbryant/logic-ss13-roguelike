use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use image::*;
use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead};
use std::num::NonZeroU32;
use wfc_image::*;

use crate::grid::{GRID_SIZE_X, GRID_SIZE_Y};
use crate::log::AddToLog;

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct WfcSettings {
    #[inspector(min = 1)]
    pattern_size: u32,
}

impl Default for WfcSettings {
    fn default() -> Self {
        Self { pattern_size: 3 }
    }
}

pub fn text_to_image(path: &str) -> io::Result<DynamicImage> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let lines: Vec<String> = reader.lines().map_while(Result::ok).collect();
    let width = lines.iter().map(|line| line.len()).max().unwrap_or(0) as u32;
    let height = lines.len() as u32;

    let mut image = GrayImage::new(width, height);

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let value = ch as u8;
            image.put_pixel(x as u32, y as u32, Luma([value]));
        }
    }

    Ok(DynamicImage::ImageLuma8(image))
}

pub fn image_to_text(image: DynamicImage, output_path: &str) -> io::Result<()> {
    let gray_image = image.into_luma8();
    let mut file = File::create(output_path)?;

    for y in 0..gray_image.height() {
        for x in 0..gray_image.width() {
            let pixel = gray_image.get_pixel(x, y);
            let value = pixel[0] as char;
            write!(file, "{}", value)?;
        }
        writeln!(file)?;
    }

    Ok(())
}

pub fn wfc(mut commands: Commands, input: Res<Input<KeyCode>>, settings: Res<WfcSettings>) {
    if input.just_pressed(KeyCode::Space) {
        commands.add(AddToLog("Generated".to_string(), None));
        let orientation = &orientation::ALL;
        let input_image = text_to_image("assets/input.txt").unwrap();

        let output_size = Size::new(GRID_SIZE_X as u32, GRID_SIZE_Y as u32);
        let start_time = ::std::time::Instant::now();
        let pattern_size =
            NonZeroU32::new(settings.pattern_size).expect("pattern size may not be zero");
        /*
        let result = generate_image(
            &input_image,
            pattern_size,
            output_size,
            orientation,
            WrapXY,
            ForbidNothing,
            retry::NumTimes(10),
        );
        match result {
            Err(_) => {
                eprintln!("Too many contradictions");
            }
            Ok(output_image) => {
                let end_time = ::std::time::Instant::now();
                println!("{:?}", end_time - start_time);
                image_to_text(output_image, "result.txt").unwrap();
            }
        }
        */
    }
}
