use actix_web1::client::Client;
use futures1::Future;
use std::time::{Duration, Instant};
use tokio1::timer;

pub fn make_http_request() -> Box<dyn Future<Item = String, Error = ()>> {
    let client = Client::default();

    Box::new(
        client
            .get("http://www.rust-lang.org")
            .header("User-Agent", "Actix-web")
            .send()
            .map_err(|_| ())
            .and_then(|response| {
                println!("Response: {:?}", response);
                Ok("fut1 http req done".to_string())
            }),
    )
}

pub fn sleep() -> Box<dyn Future<Item = String, Error = ()>> {
    let when = Instant::now() + Duration::from_millis(1000);
    Box::new(
        timer::Delay::new(when)
            .map_err(|e| {
                println!("{:?}", e);
                ()
            })
            .and_then(|_| Ok("fut1 sleep done".to_string())),
    )
}
