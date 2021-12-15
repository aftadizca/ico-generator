use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Config {
    pub api: Api,
    pub img: Img,
    pub path: MyPath
}

#[derive(Deserialize)]
pub struct MyPath {
    pub anime: Vec<String>,
    pub exclude : Vec<String>
}

#[derive(Deserialize)]
pub struct Api {
    pub query: String,
    pub url: String
}

#[derive(Deserialize)]
pub struct Img {
    pub top: String,
    pub bottom: String
}