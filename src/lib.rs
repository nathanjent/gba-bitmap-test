#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]
#![feature(slice_pattern)]

extern crate alloc;

use agb::include_palette;
use agb::mgba::DebugLevel;
use agb::mgba::Mgba;
use agb::display::WIDTH;
use agb::interrupt::VBlank;
use agb::display::bitmap4::Bitmap4;

mod resources;

const PALETTE_SHINY: [u16; 16] = include_palette!("palettes/shiny-16-1x.png");

pub fn entry(mut gba: agb::Gba) -> ! {
    let vblank = VBlank::get();

    let mut logger = Mgba::new();
    logger.as_mut().and_then(|l| {
        l.print(
            format_args!(
                "Starting game {}",
                "Whitch"
            ),
            DebugLevel::Info,
        )
        .ok()
    });

    let mut bitmap = gba.display.video.bitmap4();

    for (i, c) in PALETTE_SHINY.iter().enumerate() {
        bitmap.set_palette_entry(i as u32, *c);
    }

    draw_page(
        &mut bitmap,
        &resources::frames::OPENING_FRAME_IMG,
        0,
        0,
        resources::frames::OPENING_IMG_WIDTH as i32,
        resources::frames::OPENING_IMG_HEIGHT as i32,
    );

    bitmap.flip_page();

    loop {
        vblank.wait_for_vblank();
    }
}

fn draw_page(bitmap: &mut Bitmap4 , image: &[u8], x: i32, y: i32, w: i32, h: i32) {
    for row in x..h {
        for col in y..w {
            let frame_index = (row * WIDTH + col) as usize;
            let palette_index = image[frame_index];

            bitmap.draw_point(col, row, palette_index.clone());
        }
    }
}
