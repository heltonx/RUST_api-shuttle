use axum::{routing::get, Router};
use reqwest;
use tokio; // Required for asynchronous runtime

// This function will now asynchronously fetch data from the API
async fn fetch_from_api() -> String {
    let api_url = "https://api.jsonbin.io/v3/b/658c718cdc7465401889955f";
    
    match reqwest::get(api_url).await {
        Ok(response) => match response.text().await {
            Ok(text) => text,
            Err(_) => "Failed to read response".to_string(),
        },
        Err(_) => "Failed to fetch data".to_string(),
    }
}

// This will replace the hello_world function and call fetch_from_api instead
async fn fetch_api_response() -> String {
    fetch_from_api().await
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(fetch_api_response)); // Modified to use fetch_api_response

    Ok(router.into())
}
