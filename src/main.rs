mod config_parser;
mod logger;

use config_parser::Config;
use log::error;
use reqwest::Client;
use serde_json::json;
use std::boxed::Box;
use std::error::Error;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use terminal_spinners::{SpinnerBuilder, POINT};

fn main() -> io::Result<()> {
    //call logger
    logger::my_log::create_logging(0, 5 * 1024, "log");

    let file = match fs::read_to_string("config.toml") {
        Ok(f) => f,
        Err(_) => {
            error!("config.toml not found!");
            std::process::exit(0);
        }
    };

    let config: Config = match toml::from_str(&file) {
        Ok(f) => f,
        Err(_) => {
            error!("Cant read config.toml");
            std::process::exit(0);
        }
    };

    create_anime_folder(&config);

    print!("\nPress Enter to exit ");
    io::stdout().flush().unwrap();
    let stdin = io::stdin();
    for _ in stdin.lock().lines() {
        break;
    }

    Ok(())
}

fn get_folder_list(config: &Config) -> io::Result<Vec<std::path::PathBuf>> {
    // get all folder list
    let mut paths: Vec<_> = fs::read_dir(config.path.anime[0].to_string())?
        .filter(|a| a.as_ref().unwrap().path().as_path().is_dir())
        .map(|a| a.unwrap().path())
        .collect();
    //filtering folder
    for exc in &config.path.exclude {
        paths.retain(|a| !a.as_path().ends_with(exc));
    }
    Ok(paths)
}

fn create_anime_folder(config: &Config) {
    let folder_list = match get_folder_list(config) {
        Ok(f) => f,
        Err(_) => {
            error!("Failed to get anime folder list");
            std::process::exit(0);
        }
    };
    println!("");
    println!("Found {} folders\n", folder_list.len());
    for p in folder_list {
        let path_ico = Path::new(p.as_path().to_str().unwrap()).join("a.ico");
        let path_jpg = Path::new(p.as_path().to_str().unwrap()).join("icon.jpg");

        let handle = SpinnerBuilder::new()
            .spinner(&POINT)
            .text(format!(
                " {}",
                p.as_path().file_name().unwrap().to_str().unwrap()
            ))
            .start();

        if !(path_ico.exists() && path_jpg.exists()) {
            // println!("- {}", p.as_path().file_name().unwrap().to_str().unwrap());
            if !path_jpg.exists() {
                match get_img_from_anilist(
                    p.as_path().file_name().unwrap().to_str().unwrap(),
                    p.as_path().to_str().unwrap(),
                    config,
                ) {
                    Ok(f) => f,
                    Err(_) => {
                        error!("Failed get image from server");
                        std::process::exit(0);
                    }
                };
            }
            match process_image(
                path_jpg.to_str().unwrap(),
                path_ico.to_str().unwrap(),
                config,
            ) {
                Ok(f) => f,
                Err(_) => {
                    error!("Failed processing image");
                    std::process::exit(0);
                }
            };
        }
        handle.done();
    }
}

fn process_image(path: &str, out_path: &str, config: &Config) -> Result<(), Box<dyn Error>> {
    //read image from asset
    let top_asset = config.img.top.to_string();
    let bottom_asset = config.img.bottom.to_string();
    let middle_asset = image::open(path)?;
    //load image
    let top_asset = match image::open(top_asset){
        Ok(f) => f,
        Err(_) => {
            error!("Top image not found");
            std::process::exit(0);
        }
    };
    let mut bottom_asset = match image::open(bottom_asset){
        Ok(f) => f,
        Err(_) => {
            error!("Bottom image not found");
            std::process::exit(0);
        }
    };
    //resizing middle image
    let middle_asset = middle_asset.resize_exact(
        config.img.coordinate[0], // W
        config.img.coordinate[1], // H
        image::imageops::FilterType::Lanczos3,
    );
    //stacking bottom & middle img
    image::imageops::overlay(
        &mut bottom_asset,
        &middle_asset,
        config.img.coordinate[2], // X
        config.img.coordinate[3], // Y
    );
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
    config: &Config,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();
    let json = json!({"query": config.api.query.to_string(), "variables": {"search": title}});
    // Make HTTP post request
    let resp = match client
        .post(config.api.url.to_string())
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(json.to_string())
        .send()
        .await {
            Ok(f) => f.text().await,
            Err(_) => {
                error!("Failed get image from server");
                std::process::exit(0);
            }
        };
        
    // Get json
    let result: serde_json::Value = serde_json::from_str(&resp.unwrap()).unwrap();
    let url_img = result["data"]["Media"]["coverImage"]["extraLarge"].to_owned();
    
    //downloading image
    let resp = reqwest::get(url_img.as_str().unwrap()).await?;
    let mut content = resp.bytes().await?;
    let img = image::load_from_memory(&mut content)?;
    img.save_with_format(
        format!("{}/icon.jpg", output_path),
        image::ImageFormat::Jpeg,
    )?;
    Ok(())
}
