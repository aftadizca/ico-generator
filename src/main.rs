mod config_parser;
mod logger;

use config_parser::Config;
use reqwest::Client;
use serde_json::json;
use std::boxed::Box;
use std::error::Error;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use log::{error};
// use log4rs::encode::pattern::PatternEncoder;
// use log4rs::config::{Appender, Config as LogConfig, Root};
// use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
// use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
// use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
// use log4rs::filter::threshold::ThresholdFilter;
// use log4rs::append::rolling_file::RollingFileAppender;


use terminal_spinners::{SpinnerBuilder, POINT};

fn main() -> io::Result<()> {

    // let window_size = 0; // log0, log1, log2
    // let fixed_window_roller = 
    // FixedWindowRoller::builder().build("log.{}",window_size).unwrap();
    // let size_limit = 5 * 1024; // 5KB as max log file size to roll
    // let size_trigger = SizeTrigger::new(size_limit);
    // let compound_policy = CompoundPolicy::new(Box::new(size_trigger),Box::new(fixed_window_roller));

    // let config = LogConfig::builder()
    // .appender(
    //     Appender::builder()
    //         .filter(Box::new(ThresholdFilter::new(LevelFilter::Error)))
    //         .build(
    //             "log",
    //             Box::new(
    //                 RollingFileAppender::builder()
    //                     .encoder(Box::new(PatternEncoder::new("{d} {l}::{m}{n}")))
    //                     .build("log", Box::new(compound_policy))?,
    //             ),
    //         ),
    // )
    // .build(
    //     Root::builder()
    //         .appender("log")
    //         .build(LevelFilter::Error),
    // ).unwrap();

    // let _handle = log4rs::init_config(config).unwrap();
    //call logger
    logger::my_log::create_logging(0,5*1024,"log");

    for _ in 1..1000{
        error!("test");
    }

    let file = fs::read_to_string("config.toml").expect("config.toml not found");
    let config: Config = toml::from_str(&file).expect("failed to read config.toml");

    create_anime_folder(&config)?;

    print!("\nPress Enter to exit ");
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
    let folder_list = get_folder_list(config)?;
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
                get_img_from_anilist(
                    p.as_path().file_name().unwrap().to_str().unwrap(),
                    p.as_path().to_str().unwrap(),
                    config,
                )
                .unwrap();
            }
            process_image(
                path_jpg.to_str().unwrap(),
                path_ico.to_str().unwrap(),
                config,
            )
            .expect("Error processing image");
        }
        handle.done();
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
