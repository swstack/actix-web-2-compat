use crate::fut1_client;
use crate::fut3_client;
use actix_web2::http::StatusCode;
use actix_web2::{web, HttpRequest, HttpResponse};
use futures3::compat::Future01CompatExt;

pub struct AppState {
    pub app_name: String,
}

pub async fn index(data: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    println!("Hello {}!", &data.app_name);
    println!("path: {:?}", req.path());
    println!(
        "header: {:?}",
        req.headers().get("X-Platform-Api-Token").unwrap()
    );
    println!("query: {:?}", req.query_string());

    let fut3_http_req = fut3_client::make_http_request().await;
    println!("{:?}", fut3_http_req.unwrap());

    let fut3_sleep = fut3_client::sleep().await;
    println!("{:?}", fut3_sleep.unwrap());
    //
    // let fut1_http_req = fut1_client::make_http_request().compat().await;
    // println!("{:?}", fut1_http_req.unwrap());

    let fut1_sleep = fut1_client::sleep().compat().await;
    println!("{:?}", fut1_sleep.unwrap());

    HttpResponse::new(StatusCode::OK)
}
