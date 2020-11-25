mod utils;

use std::io::{Cursor, Read, Seek, SeekFrom};

use image::{self, ImageFormat};
use image::{
    DynamicImage,
    GenericImage,
    GenericImageView
};

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn process_image(image1: Vec<u8>, image2: Vec<u8>) -> Vec<u8> {
    console_error_panic_hook::set_once();
    let dynamic_img_1 = image::load_from_memory_with_format(&image1, image::ImageFormat::Png).unwrap();
    let dynamic_img_2 = image::load_from_memory_with_format(&image2, image::ImageFormat::Png).unwrap();
    
    let mut canvas = dynamic_img_1.clone();
    
    let (width, height) = canvas.dimensions();

    for y in 0..height {
        for x in 0..width {
            let canvas_x = x;
            let canvas_y = y;
            let color1 = dynamic_img_1.get_pixel(canvas_x, canvas_y);
            let a1 = color1[3] as f32 / 255.0; // convert to 0.0 - 1.0
            let r1 = color1[0] as f32 * a1;
            let g1 = color1[1] as f32 * a1;
            let b1 = color1[2] as f32 * a1;
            
            let color2 = dynamic_img_2.get_pixel(x, y);
            let a2 = color2[3] as f32 / 255.0 * 1.0; // convert to 0.0 - 1.0
            let r2 = color2[0] as f32;
            let g2 = color2[1] as f32;
            let b2 = color2[2] as f32;

            let r3 = ((a2 * r2) + ((1.0 - a2) * r1)) as u8;
            let g3 = ((a2 * g2) + ((1.0 - a2) * g1)) as u8;
            let b3 = ((a2 * b2) + ((1.0 - a2) * b1)) as u8;
            let a3 = 255;

            let rgba = image::Rgba([
                r3,
                g3,
                b3,
                a3
            ]);

            canvas.put_pixel(canvas_x, canvas_y, rgba);
        }
    }

    get_image_as_array(canvas)
}

fn get_image_as_array(_img: DynamicImage) -> Vec<u8> {
    // Create fake "file"
    let mut c = Cursor::new(Vec::new());

    match _img.write_to(&mut c, ImageFormat::Png) {
        Ok(c) => c,
        Err(error) => {
            panic!(
                "There was a problem writing the resulting buffer: {:?}",
                error
            )
        }
    };
    c.seek(SeekFrom::Start(0)).unwrap();

    // Read the "file's" contents into a vector
    let mut out = Vec::new();
    c.read_to_end(&mut out).unwrap();

    out
}