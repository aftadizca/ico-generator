// #![allow(unused_imports)]
// #![allow(unused_variables)]
// #![allow(dead_code)]

mod config_parser;
mod constant;

use config_parser::Config;
use constant::middle_img::{H, W, X, Y};
use reqwest::Client;
use serde_json::json;
use std::boxed::Box;
use std::error::Error;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn main() -> io::Result<()> {
    let file = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&file)?;

    create_anime_folder(&config)?;
    print!("\nPress Enter to Exit ");
    io::stdout().flush().unwrap();
    let stdin = io::stdin();
    for _ in stdin.lock().lines() {
        break;
    }

    Ok(())
}

fn get_folder_list(config: &Config) -> io::Result<Vec<std::path::PathBuf>> {
    let mut paths: Vec<_> = fs::read_dir(config.path.anime[0].to_string())?
        .filter(|a| a.as_ref().unwrap().path().as_path().is_dir())
        .map(|a| a.unwrap().path())
        .collect();
    //filtering path
    for exc in &config.path.exclude {
        paths.retain(|a| !a.as_path().ends_with(exc));
    }
    Ok(paths)
}

fn create_anime_folder(config: &Config) -> io::Result<()> {
    let mut found = false;
    let folder_list = get_folder_list(config)?;
    println!("");
    for p in folder_list {
        let path_ico = Path::new(p.as_path().to_str().unwrap()).join("a.ico");
        let path_jpg = Path::new(p.as_path().to_str().unwrap()).join("icon.jpg");
        if !(path_ico.exists() && path_jpg.exists()) {
            println!("- {}", p.as_path().file_name().unwrap().to_str().unwrap());
            if !path_jpg.exists() {
                get_img_from_anilist(
                    p.as_path().file_name().unwrap().to_str().unwrap(),
                    p.as_path().to_str().unwrap(),
                    config
                ).unwrap();
            }
            found = true;
            process_image(
                path_jpg.to_str().unwrap(),
                path_ico.to_str().unwrap(),
                config,
            ).expect("Error processing image");
        }
    }
    if !found {
        println!("All folder already have icon")
    }

    Ok(())
}

fn process_image(path: &str, out_path: &str, config: &Config) -> Result<(), Box<dyn Error>> {
    //read image from asset
    let top_asset = config.img.top.to_string();
    let bottom_asset = config.img.bottom.to_string();
    let middle_asset = image::open(path)?;
    //load image
    let top_asset = image::open(top_asset)?;
    let mut bottom_asset = image::open(bottom_asset)?;
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
    config:&Config
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();
    let json = json!({"query": config.api.query.to_string(), "variables": {"search": title}});
    // Make HTTP post request
    let resp = client
        .post(config.api.url.to_string())
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
