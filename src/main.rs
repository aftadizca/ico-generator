// #![allow(unused_imports)]
// #![allow(unused_variables)]
// #![allow(dead_code)]

mod constant;

//rust-embed
use rust_embed::RustEmbed;
#[derive(RustEmbed)]
#[folder = "src/img/"]
struct Asset;

use constant::anilist::{QUERY, URL};
use constant::middle_img::{H, W, X, Y};
use glob::glob;
// use image::{GenericImage, GenericImageView, ImageBuffer, ImageDecoder, RgbImage};
use reqwest::Client;
use serde_json::json;
use std::boxed::Box;
use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // get_img_from_anilist("AnoHana")?;
    create_anime_folder(std::env::current_dir().unwrap().to_str().unwrap());
    // create_anime_folder("/mnt/d/KOLEKSI/NEWANIME");
    print!("\nPress Enter to Exit ");
    io::stdout().flush().unwrap();
    let stdin = io::stdin();
    for _ in stdin.lock().lines() {
        break;
    }
}

fn create_anime_folder(folder: &str) {
    let mut found = false;
    let folder = format!("{}/*/", folder);
    println!("");
    for entry in glob(folder.as_ref()).unwrap() {
        match entry {
            Ok(p) => {
                let path_ico = Path::new(p.as_path().to_str().unwrap()).join("a.ico");
                let path_jpg = Path::new(p.as_path().to_str().unwrap()).join("icon.jpg");
                if !(path_ico.exists() && path_jpg.exists())
                    && !p.as_path().ends_with("1. new")
                    && !p.as_path().ends_with("$RECYCLE.BIN")
                {
                    println!("- {}", p.as_path().file_name().unwrap().to_str().unwrap());
                    if !path_jpg.exists() {
                        get_img_from_anilist(
                            p.as_path().file_name().unwrap().to_str().unwrap(),
                            p.as_path().to_str().unwrap(),
                        )
                        .unwrap();
                    }
                    found = true;
                    process_image(path_jpg.to_str().unwrap(), path_ico.to_str().unwrap()).unwrap()
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
    if !found {
        println!("All folder already have icon")
    }
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
    let middle_asset = middle_asset.resize_exact(W, H, image::imageops::FilterType::Lanczos3);
    //stacking bottom & middle img
    image::imageops::overlay(&mut bottom_asset, &middle_asset, X, Y);
    //stacking bottom & top img
    image::imageops::overlay(&mut bottom_asset, &top_asset, 0, 0);
    //save image
    bottom_asset.save_with_format(out_path, image::ImageFormat::Ico)?;
    Ok(())
}

#[tokio::main]
async fn get_img_from_anilist(
    title: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();
    let json = json!({"query": QUERY, "variables": {"search": title}});
    // Make HTTP post request
    let resp = client
        .post(URL)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(json.to_string())
        .send()
        .await
        .unwrap()
        .text()
        .await;
    // Get json
    let result: serde_json::Value = serde_json::from_str(&resp.unwrap()).unwrap();
    let url_img = result["data"]["Media"]["coverImage"]["extraLarge"].to_owned();
    // println!("{}", url_img.as_str().unwrap());
    let resp = reqwest::get(url_img.as_str().unwrap()).await?;
    let mut content = resp.bytes().await?;
    let img = image::load_from_memory(&mut content)?;
    img.save_with_format(
        format!("{}/icon.jpg", output_path),
        image::ImageFormat::Jpeg,
    )?;
    Ok(())
}
