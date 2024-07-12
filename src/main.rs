use exitfailure::ExitFailure;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};
use serde_json::{from_value, json, Value};
use std::{fs::File, io::Read};
use structopt::StructOpt;

#[derive(Serialize, Deserialize, Debug)]
struct W {
    coord: Coord,
    weather: Weather,
    base: String,
    main: Main,
}

async fn mock_get(city: &String) -> Result<W, ExitFailure> {
    let mut file = File::open("./data/weather.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mock_data: Value = serde_json::from_str(&contents)?;
    let result: W = from_value(mock_data)?;
    Ok(result)
}

impl W {
    async fn get(city: &String) -> Result<Self, ExitFailure> {
        // let url = format!("www.baidu.com{}", city);
        // let url = Url::parse(&url)?;
        // let resp = reqwest::get(url).await?.json::<W>().await?;
        let resp = mock_get(city).await?;
        Ok(resp)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    details: Details,
}

#[derive(Serialize, Deserialize, Debug)]
struct Details {
    id: i32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Main {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: i32,
    humidity: i32,
    sea_level: i32,
    grnd_level: i32,
}

#[derive(StructOpt)]
struct Input {
    city: String,
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let input = Input::from_args();
    println!("{}", input.city);
    let resp = W::get(&input.city).await?;
    println!(
        "{} \n 当前温度 {} \n 最高温度 {} \n 最低温度 {} \n 湿度 {} \n",
        input.city, resp.main.temp, resp.main.temp_max, resp.main.temp_min, resp.main.humidity
    );
    Ok(())
}
