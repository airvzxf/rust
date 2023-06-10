use actix_web::{web::Data, App, HttpServer};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenv::dotenv;
use std::env;
mod constants;
mod likes;
mod schema;
mod tweets;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url_error: &str = "ERROR: DATABASE_URL not found in environment variables.";
    let database_url: String = env::var("DATABASE_URL").expect(database_url_error);
    let manager: ConnectionManager<PgConnection> =
        ConnectionManager::<PgConnection>::new(database_url);
    let pool_error: &str = "ERROR: It can't build the pool.";
    let pool: Pool<ConnectionManager<PgConnection>> =
        Pool::builder().build(manager).expect(pool_error);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(tweets::get_tweets)
            .service(tweets::create_tweet)
            .service(tweets::get_tweet)
            .service(tweets::delete_tweet)
            .service(likes::get_like)
            .service(likes::create_like)
            .service(likes::delete_like)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
