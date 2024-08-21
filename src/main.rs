use std::env;
use std::process::exit;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let healthcheck_uri = match env::var("HEALTHCHECK_URI") {
        Ok(val) => val,
        Err(_) => "http://localhost".to_string(),
    };
    let port = match env::var("PORT") {
        Ok(val) => val,
        Err(_) => "9000".to_string(),
    };
    let path = match env::var("API_PATH") {
        Ok(val) => val,
        Err(_) => "".to_string(),
    };

    println!("Checking http://{}:{}/{}", healthcheck_uri, port, path);
    let url = format!("http://{}:{}/{}", healthcheck_uri, port, path);
    let client = reqwest::Client::new();
    let res = client.get(url).send().await;
    match res {
        Ok(res) => {
            if !res.status().is_success() {
                println!("Failed: {}; {}", res.status(), res.text().await.unwrap_or("".to_string()));
                exit(1)
            }
            println!("Success");
            exit(0)
        }
        Err(_) => {

            println!("Failed: Connection error");
            exit(1)
        }
    }
}
