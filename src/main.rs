#[macro_use] extern crate log;

use actix_web::{get, middleware, post, web, App, Error, HttpResponse, HttpRequest, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/")]
async fn get_home(
    pool: web::Data<DbPool>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Failed to get DB pool connection");
    debug!("Created DB connection");
    let res = HttpResponse::Ok().body("<h2>Home Page</h2>");
    Ok(res)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    dotenv::dotenv().ok();
    debug!("Entered main thread");

    let conn_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(conn_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB pool.");
    debug!("Created DB Pool");

    let bind = "127.0.0.1:8080";
    info!("Starting server at {}", &bind);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(get_home)
    })
        .bind(&bind)?
        .run()
        .await
}
