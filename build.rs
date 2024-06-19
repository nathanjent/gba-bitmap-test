use quote::quote;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};

use image::open;

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR environment variable must be specified");

    if let Ok(opening_frame) = open("gfx/full-frame.png") {
        if let Ok(palette_img) = open("palettes/shiny-16-1x.png") {
            let palette_img = palette_img.into_rgb8();
            let opening_frame = opening_frame.into_rgb8();

            let palette_map: HashMap<(u8, u8, u8), u8> = palette_img
                .pixels()
                .enumerate()
                .map(|(i, p)| {
                    assert!(i < 256);
                    let [r, g, b] = p.0;
                    ((r, g, b), i as u8)
                })
                .collect();

            let img_data: Vec<u8> = opening_frame
                .pixels()
                .map(|p| {
                    let [r, g, b] = p.0;
                    *palette_map.get(&(r, g, b)).unwrap_or(&0u8)
                })
                .collect();

            let img_width = opening_frame.width();
            let img_height = opening_frame.height();
            let frame_output = quote! {
                pub const OPENING_IMG_WIDTH: u32 = #img_width;
                pub const OPENING_IMG_HEIGHT: u32 = #img_height;
                pub const OPENING_FRAME_IMG: &[u8] = &[#(#img_data),*];
            };

            let frame_output_file = File::create(format!("{out_dir}/frame.rs"))
                .expect("Failed to open frame.rs for writing");
            let mut frame_writer = BufWriter::new(frame_output_file);

            write!(&mut frame_writer, "{frame_output}").unwrap();
        }
    }
}
