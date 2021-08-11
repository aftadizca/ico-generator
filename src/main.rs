#![allow(unused_imports)]
#![allow(dead_code)]

extern crate image;
extern crate rust_embed;

use rust_embed::RustEmbed;
#[derive(RustEmbed)]
#[folder = "src/img/"]
// #[prefix = "/"]
struct Asset;

use image::io::Reader as ImageReader;
use image::{GenericImage, GenericImageView, ImageBuffer, ImageDecoder, RgbImage};
use std::boxed::Box;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let img1_asset = Asset::get("test2.jpg").unwrap();
    let img2_asset = Asset::get("test.png").unwrap();
    // println!("{:?}", img_path.data.as_ref());
    // for file in Asset::iter() {
    //     println!("{}", file.as_ref());
    // }

    // let img = image::open("src/img/test2.jpg")?;
    // let img2 = image::open("src/img/test.png")?;
    let img = image::load_from_memory(img1_asset.data.as_ref())?;
    let img2 = image::load_from_memory(img2_asset.data.as_ref())?;

    let mut img = img.resize_exact(512, 512, image::imageops::FilterType::Lanczos3);
    let img2 = img2.resize_exact(512, 512, image::imageops::FilterType::Lanczos3);

    image::imageops::overlay(&mut img, &img2, 0, 0);

    img.save_with_format("test2.png", image::ImageFormat::Png)?;
    println!("w:{} h:{}", img.width(), img.height());
    Ok(())
}
