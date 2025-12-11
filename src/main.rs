use serde::Deserialize;
use reqwest::blocking;
use std::error::Error;


#[derive(Debug, Deserialize)]
struct Fact {
    text: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Fetching a random fact from the web...");


    let url = "https://uselessfacts.jsph.pl/random.json?language=en";


    let response = blocking::get(url)?;


    let fact_data: Fact = response.json()?;


    println!("\n------------------------------------------------");
    println!("RUST API DATA FETCHER:");
    println!("{}", fact_data.text);
    println!("------------------------------------------------\n");

    Ok(())
}

