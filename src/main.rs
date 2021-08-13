#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

extern crate glob;
extern crate image;
extern crate rust_embed;
mod constant;

//rust-embed
use rust_embed::RustEmbed;
#[derive(RustEmbed)]
#[folder = "src/img/"]
struct Asset;

use constant::MIDDLE_IMG as MIDDLE;
use glob::glob;
use hyper::Client;
use image::io::Reader as ImageReader;
use image::{GenericImage, GenericImageView, ImageBuffer, ImageDecoder, RgbImage};
use std::boxed::Box;
use std::error::Error;
use std::path::Path;
use std::vec::Vec;

fn main() -> Result<(), Box<dyn Error>> {
    create_anime_folder("")?;
    // process_image("src/img/test.jpg", "test2.ico")?;
    Ok(())
}

fn create_anime_folder(folder: &str) -> Result<(), Box<dyn Error>> {
    let folder = "/mnt/d/KOLEKSI/NEWANIME/*/";
    for entry in glob(folder).unwrap() {
        match entry {
            Ok(p) => {
                let path_ico = Path::new(p.as_path().to_str().unwrap()).join("a.ico");
                let path_jpg = Path::new(p.as_path().to_str().unwrap()).join("icon.jpg");
                if path_ico.exists() && !path_jpg.exists() && !p.as_path().ends_with("1. new") {
                    println!("{}", p.as_path().file_name().unwrap().to_str().unwrap());
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
    Ok(())
}

fn process_image(path: &str, out_path: &str) -> Result<(), Box<dyn Error>> {
    //read image from asset
    let top_asset = Asset::get("top.png").unwrap();
    let bottom_asset = Asset::get("bottom.png").unwrap();
    let middle_asset = image::open(path)?;
    //load image
    let top_asset = image::load_from_memory(top_asset.data.as_ref())?;
    let mut bottom_asset = image::load_from_memory(bottom_asset.data.as_ref())?;
    //resizing middle image
    let middle_asset =
        middle_asset.resize_exact(MIDDLE::W, MIDDLE::H, image::imageops::FilterType::Lanczos3);
    //stacking bottom & middle img
    image::imageops::overlay(&mut bottom_asset, &middle_asset, MIDDLE::X, MIDDLE::Y);
    //stacking bottom & top img
    image::imageops::overlay(&mut bottom_asset, &top_asset, 0, 0);
    //save image
    bottom_asset.save_with_format(out_path, image::ImageFormat::Ico)?;
    Ok(())
}

#[tokio::main]
async fn get_img_from_anilist() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // This is where we will setup our HTTP client requests.

    Ok(())
}
