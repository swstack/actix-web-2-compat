mod endpoint;
mod fut1_client;
mod fut3_client;
use actix_rt;
use actix_web2::{web, App, HttpServer};

fn main() {
    let mut runtime = tokio_compat::runtime::Builder::new().build().unwrap();
    let local_tasks = tokio2::task::LocalSet::new();
    let sys = actix_rt::System::run_in_tokio("main", &local_tasks);
    local_tasks.spawn_local(sys);
    runtime
        .block_on_std(local_tasks.run_until(run_server()))
        .unwrap();
}

async fn run_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(endpoint::AppState {
                app_name: String::from("Actix-web"),
            })
            .route("/", web::post().to(endpoint::index))
    })
    .bind("127.0.0.1:8242")?
    .run()
    .await
}
