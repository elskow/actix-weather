use std::path::Path;
use std::fs::File;
use serde::{Deserialize};
use serde_json::from_str;
use std::io::Read;

#[derive(Deserialize)]
pub struct CityList {
    pub cities: Vec<String>
}

pub async fn load_cities() -> Result<CityList, Box<dyn std::error::Error>> {
    let path = Path::new("supported_city.json");
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();

    let _ = file.read_to_string(&mut contents);

    Ok(from_str(&contents)?)
}