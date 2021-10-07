use reqwest;
use reqwest::{Body, Client, Error, Response};
use std::process;
use crate::data::domain::{Continent, Country};

// fn create_client() -> Client {
//     let client: Client =  reqwest::Client::new();
//     return client
// }

pub async fn fetch_data_one_contry(country:&str) -> Country {
    let url = format!("https://corona.lmao.ninja/v2/countries/{}?yesterday=true&strict=true&query =", country);
    let country :Country= reqwest::get(url).await.unwrap().json().await.unwrap();
    country
}

pub async fn fetch_data_continent() -> Vec<Continent> {
    let url = "https://corona.lmao.ninja/v2/continents?yesterday=true&sort=";
    let continents: Vec<Continent> = reqwest::get(url).await.unwrap().json().await.unwrap();
    continents
}

pub async fn fetch_data_one_continent(continent:&str) -> Continent {
    let url: String = format!("https://corona.lmao.ninja/v2/continents/{}?yesterday=&strict=", continent);

    let continent:Continent = reqwest::get(url).await.unwrap().json().await.unwrap();
    continent

}




