mod rest;

use std::env;
use reqwest;

use rest::restart;

fn main() {
    let auth_token = env::var_os("AUTH_TOKEN").unwrap_or_else(|| {
        println!("Environment variable AUTH_TOKEN not set.");
        std::process::exit(1);
    });

    let service_id = env::var_os("SERVICE_ID").unwrap_or_else(|| {
        println!("Environment SERVICE_ID not set.");
        std::process::exit(2);
    });

    let auth_token = auth_token.to_string_lossy();
    let service_id = service_id.to_string_lossy();

    let parameter = restart::Parameter {
        // TODO: make this fields environment variables
        message: "restart from ci/cd due to configuration changes".to_string(),
        restart_message: "server configuration have been changed. server will be restarted".to_string()
    };

    let client = reqwest::blocking::Client::new()
        .post(restart::post_endpoint(&service_id))
        .header("Authorization", format!("Bearer {}", auth_token))
        .json(&parameter);

    let response = client.send().unwrap_or_else(|_| {
        println!("An error occurred while sending the request.");
        std::process::exit(3);
    });

    if !response.status().is_success() {
        let message = if let Ok(text) = response.text() {
            format!("Something went wrong. Response text: {:#?}", text)
        } else {
            "Something went wrong".to_string()
        };
        println!("{}", message);
        std::process::exit(0);
    }

    println!("Server will restart.")
}
