use rand::Rng;
use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::time::{Duration, Instant};


#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    address: String,
    email: String,
    role: String,
}

#[derive(Serialize, Deserialize)]
struct MyJson {
    user: User,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();

    // Set up the HTTP client
    let client = Client::new();
    let url = "http://localhost:4000/api/users";
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    // Open the output file
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("request_times.txt")?;

    let mut c:u64 = 1_000;

    while c > 0 {
        // Generate random values for the JSON fields
        let name = format!("User {}", rng.gen_range(1..=10));
        let address = format!(
            "{} {}, {}",
            rng.gen_range(1..=1000),
            ["Main St", "Oak St", "Elm St", "Maple Ave"][rng.gen_range(0..=3)],
            ["New York", "Los Angeles", "Chicago", "Houston"][rng.gen_range(0..=3)]
        );
        let email = format!(
            "{}@{}",
            name.to_lowercase(),
            ["gmail.com", "yahoo.com", "hotmail.com"][rng.gen_range(0..=2)]
        );
        let role = ["Admin", "Manager", "Employee"][rng.gen_range(0..=2)].to_string();

        let user = User {
            name: name,
            address: address,
            email: email,
            role: role,
        };

        // Create the JSON object
        let my_json = MyJson { user };

        // Serialize the JSON object to a string
        let json_str = serde_json::to_string(&my_json)?;

        // Set up the request body
        let body = json_str;

        // Start the timer
        let start_time = Instant::now();

        // Send the POST request
        let _response = client
            .post(url)
            .headers(headers.clone())
            .body(body)
            .send()?;

        // End the timer and calculate the elapsed time
        let elapsed_time = start_time.elapsed().as_secs_f64();

        // Print the response status code and body
        // println!("Status: {}", response.status());
        // println!("Body: {}", response.text()?);
        
        // Write the elapsed time to the output file
        writeln!(file, "{}", elapsed_time)?;

        // Wait for some time before sending the next request
        std::thread::sleep(Duration::from_secs(1));
        c=c+1;
    }
    Ok(())
}
