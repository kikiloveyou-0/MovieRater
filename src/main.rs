use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;





#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct Movie {
    pub title: String,
    pub year: String,
    pub rating: String,
}

fn main() {
    println!("Welcome to MovieRater!");
    println!("Please enter your movie title:");
    let mut movieTitle = String::new();
    io::stdin().read_line(&mut movieTitle).expect("Failed to read line");


    println!("Please enter your movie year:");
    let mut movieYear = String::new();
    io::stdin().read_line(&mut movieYear).expect("Failed to read line");

    println!("Please enter your movie rating:");
    let mut movieRating = String::new();
    io::stdin().read_line(&mut movieRating).expect("Failed to read line");

    let movie = Movie {
        title: movieTitle.trim().to_string(),
        year: movieYear.trim().to_string(),
        rating: movieRating.trim().to_string(),
    };
    let json = serde_json::to_string(&movie).unwrap();

    if Path::new("movies.json").exists(){
        println!("File exists!");

        let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("movies.json")
        .unwrap();
            
        file.write(json.as_bytes()).expect("Failed to write to file");


    } else {
        println!("File does not exist!");
        let mut file = std::fs::File::create("movies.json").expect("Failed to create file");
        file.write(json.as_bytes()).expect("Failed to write to file");
    }
    println!("Serialisze: {}", json);

}