use super::schema::tweets;
use crate::constants::APPLICATION_JSON;
use actix_web::{web, web::Data, HttpResponse, Responder};
use chrono::{NaiveDateTime, Utc};
use diesel::{
    delete, insert_into,
    r2d2::{ConnectionManager, Pool, PooledConnection},
    ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl,
};
use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = tweets)]
struct Tweet {
    id: Uuid,
    created_at: NaiveDateTime,
    message: String,
}

impl Tweet {
    fn new(message: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            message,
        }
    }
}

#[actix_web::get("/tweets")]
async fn get_tweets(pool: Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    println!("GET /tweets");

    let connection_error: &str = "ERROR: The connection has an error.";
    let mut connection: PooledConnection<ConnectionManager<PgConnection>> =
        pool.get().expect(connection_error);

    use crate::schema::tweets::dsl::tweets;
    let table_rows: Result<Vec<Tweet>, diesel::result::Error> =
        tweets.limit(10).load::<Tweet>(&mut connection);
    let response: Vec<Tweet> = match table_rows {
        Ok(rows) => rows,
        Err(_) => vec![],
    };

    let responder: HttpResponse = HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(&response);

    responder
}

#[actix_web::post("/tweets")]
async fn create_tweet(
    request_body: String,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    println!("POST /tweets");
    println!("request_body: {:#?}", request_body);
    let new_tweet: Tweet = Tweet::new(request_body);
    println!(
        "new_tweet: {:#?} | {:#?} | {:#?}",
        new_tweet.id, new_tweet.created_at, new_tweet.message
    );

    let connection_error: &str = "ERROR: The connection has an error.";
    let mut connection: PooledConnection<ConnectionManager<PgConnection>> =
        pool.get().expect(connection_error);

    let insert_error: &str = "ERROR: It can't insert these values in the table.";
    let insert_data = insert_into(tweets::table)
        .values(&new_tweet)
        .execute(&mut connection)
        .expect(insert_error);

    println!("insert_data: {:#?}", insert_data);

    let responder: HttpResponse = HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(&new_tweet);

    println!("responder: {:#?}", responder);

    responder
}

#[actix_web::get("/tweets/{id}")]
async fn get_tweet(
    tweet_id_path: web::Path<String>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    println!("GET /tweets/{{id}}");

    println!("tweet_id_path: {:#?}", tweet_id_path);
    let uuid: Uuid = Uuid::from_str(&tweet_id_path.as_str()).unwrap();
    println!("uuid: {:#?}", uuid);

    let connection_error: &str = "ERROR: The connection has an error.";
    let mut connection: PooledConnection<ConnectionManager<PgConnection>> =
        pool.get().expect(connection_error);

    use crate::schema::tweets::dsl::{id, tweets};
    println!(
        "tweets total: {:#?}",
        tweets.count().first::<i64>(&mut connection).unwrap()
    );
    let table_rows: Result<Vec<Tweet>, _> =
        tweets.filter(id.eq(uuid)).load::<Tweet>(&mut connection);

    let response: Vec<Tweet> = match table_rows {
        Ok(rows) => rows,
        Err(_) => vec![],
    };

    let responder: HttpResponse = HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(response);

    println!("responder: {:#?}", responder);

    responder
}

#[actix_web::delete("/tweets/{id}")]
async fn delete_tweet(
    tweet_id_path: web::Path<String>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    println!("DELETE /tweets/{{id}}");

    // delete_like(tweet_id_path, pool).await;

    println!("tweet_id_path: {:#?}", tweet_id_path);
    let uuid: Uuid = Uuid::from_str(&tweet_id_path.as_str()).unwrap();
    println!("uuid: {:#?}", uuid);

    let connection_error: &str = "ERROR: The connection has an error.";
    let mut connection: PooledConnection<ConnectionManager<PgConnection>> =
        pool.get().expect(connection_error);

    use crate::schema::tweets::dsl::{id, tweets};
    println!(
        "tweets total: {:#?}",
        tweets.count().first::<i64>(&mut connection).unwrap()
    );
    let delete_error: &str = "ERROR: It can't insert these values in the table.";
    let delete_data = delete(tweets.filter(id.eq(uuid)))
        .execute(&mut connection)
        .expect(delete_error);
    println!("delete_data: {:#?}", delete_data);
    println!(
        "tweets total: {:#?}",
        tweets.count().first::<i64>(&mut connection).unwrap()
    );

    let responder: HttpResponse = HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json("{{\"sucess\":true}}");

    println!("responder: {:#?}", responder);

    responder
}
