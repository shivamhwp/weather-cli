use reqwest::Url;
use structopt::StructOpt;
use exitfailure::ExitFailure;
use serde_derive::{Deserialize, Serialize};

#[derive(StructOpt)]
struct Cli {
    city: String,
    country_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Forcast {
    coord : Coord,
    weather: Weather,
    base: String,
    main: Temps,
    visibility: i32,
    wind:Wind,
    clouds: Clouds,
    dt: i32,
    timezone: i32,
    id : i32,
    name: String,
    cod : i32
}

#[derive(Serialize, Deserialize, Debug)]

struct Coord{
    lon : f64,
    lat: f64
}
#[derive(Serialize, Deserialize, Debug)]
struct  Weather {
    details :Details
}
#[derive(Serialize, Deserialize, Debug)]
struct  Details {
    id :i32,
    main: String,
    description : String,
    icon :String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Temps{
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max : f64,
    pressure: i32,
    humidity: i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed : f64,
    deg:i32
}

#[derive(Serialize, Deserialize, Debug)]
struct Clouds {
    all :i32
}

impl Forcast {
async fn get(city: &String, country_code: &String) -> Result<Self, ExitFailure>{
    // enter you api key here in appid
    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={},{}&appid=", city, country_code);
    let url  = Url::parse(&*url)?;

    let repsonse = reqwest::get(url)
    .await?
    .json::<Forcast>()
    .await?;
    Ok(repsonse)
}
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure>{
    let args = Cli::from_args();
    let response = Forcast::get(&args.city, &args.country_code).await?;


    println!(" city: {}\n country code :{}\n humidity : {}%\n windspeed : {}km/h\n description: {}\n  ", args.city, args.country_code , response.main.humidity, response.wind.speed, response.weather.details.main  );
    Ok(())
}
