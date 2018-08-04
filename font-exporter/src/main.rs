extern crate image;
extern crate rusttype;

const I8_BLOCK_WIDTH: usize = 8;
const I8_BLOCK_HEIGHT: usize = 4;

use image::imageops::overlay;
use image::GrayImage;
use rusttype::gpu_cache::CacheBuilder;
use rusttype::Point;
use rusttype::{Font, Scale};

use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

fn main() {
    let width = 256;
    let height = width;
    let scale = Scale::uniform(50.0);

    let font_data = include_bytes!("../FiraSans-Regular.ttf");
    let font = Font::from_bytes(font_data as &[u8]).expect("Error constructing Font");
    let mut atlas = GrayImage::new(width, height);

    let mut cache = CacheBuilder {
        width,
        height,
        ..CacheBuilder::default()
    }.build();

    let glyphs = font
        .glyphs_for((0x21..=0x7E).map(|i: u8| i as char))
        .map(|g| g.scaled(scale).positioned(Point { x: 0.0, y: 0.0 }))
        .collect::<Vec<_>>();

    for glyph in &glyphs {
        cache.queue_glyph(0, glyph.clone());
    }

    cache
        .cache_queued(|rect, data| {
            let glyph = GrayImage::from_raw(rect.width(), rect.height(), data.to_vec())
                .expect("Bad GrayImage");
            overlay(&mut atlas, &glyph, rect.min.x, rect.min.y);
        })
        .expect("cache queue");

    atlas.save("image_example.png").unwrap();

    let rects = glyphs
        .iter()
        .map(|glyph| {
            (
                glyph.pixel_bounding_box().unwrap().max.y as f32,
                cache
                .rect_for(0, glyph)
                .unwrap()//expect("Failed to get rect.")
                .unwrap()//expect("Failed to unwrap TextureCoords")
                .0,
            )
        })
        .collect::<Vec<_>>();

    let o_file = File::create("out_file").unwrap();
    let mut buffer = BufWriter::new(o_file);

    {
        for row in 0..(height as usize / I8_BLOCK_HEIGHT) {
            let row_y = row * I8_BLOCK_HEIGHT;
            for column in 0..(width as usize / I8_BLOCK_WIDTH) {
                let column_x = column * I8_BLOCK_WIDTH;
                for y in 0..I8_BLOCK_HEIGHT {
                    let y = row_y + y;
                    let x = column_x;
                    let pixel_index = y * width as usize + x;
                    let src = &(*atlas)[pixel_index..][..I8_BLOCK_WIDTH];
                    buffer.write_all(src).unwrap();
                }
            }
        }
    }
    println!("{:#?}", rects);
}
