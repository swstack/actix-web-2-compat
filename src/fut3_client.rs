use actix_web2::client::Client;
use std::time::Duration;
use tokio2::time;

pub async fn make_http_request() -> Result<String, ()> {
    let client = Client::default();
    let response = client
        .get("http://www.rust-lang.org")
        .header("User-Agent", "Actix-web")
        .send()
        .await
        .unwrap();

    println!("Response: {:?}", response);
    Ok("fut3 http req done".to_string())
}

pub async fn sleep() -> Result<String, ()> {
    time::delay_for(Duration::from_millis(1000)).await;
    Ok("fut3 sleep done".to_string())
}
