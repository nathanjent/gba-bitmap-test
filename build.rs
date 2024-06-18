use std::env;
use quote::quote;
use std::fs::File;
use std::io::{BufWriter, Write};

use image::open;

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR environment variable must be specified");

    if let Ok(opening_frame) = open("gfx/full-frame.png") {
        let frame = opening_frame.into_rgb8();

        let img_data: Vec<u8> = frame.pixels().map(|p| {
            let [r, g, b] = p.0;
            match (r, g, b) {
                (0, 0, 0) => 0x0,
                (60, 47, 82) => 0x1,
                (140, 127, 144) => 0x2,
                (188, 176, 179) => 0x3,
                (226, 226, 226) => 0x4,
                (216, 104, 48) => 0x5,
                (240, 149, 72) => 0x6,
                (239, 208, 129) => 0x7,
                (178, 39, 65) => 0x8,
                (245, 70, 76) => 0x9,
                (247, 156, 136) => 0xa,
                (70, 86, 165) => 0xb,
                (73, 149, 243) => 0xc,
                (114, 222, 235) => 0xd,
                (62, 197, 75) => 0xe,
                (180, 230, 86) => 0xf,
                _ => 0,
            }
        })
        .collect();

        let img_width = frame.width();
        let img_height = frame.height();
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
