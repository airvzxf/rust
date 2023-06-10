use super::schema::likes;
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
#[diesel(table_name = likes)]
struct Like {
    id: Uuid,
    created_at: NaiveDateTime,
    tweet_id: Uuid,
}

impl Like {
    fn new(tweet_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            tweet_id,
        }
    }
}

#[actix_web::get("/tweets/{id}/likes")]
async fn get_like(
    tweet_id_path: web::Path<String>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    println!("GET /tweets/{{id}}/likes");

    println!("tweet_id_path: {:#?}", tweet_id_path);
    let uuid: Uuid = Uuid::from_str(&tweet_id_path.as_str()).unwrap();
    println!("uuid: {:#?}", uuid);

    let connection_error: &str = "ERROR: The connection has an error.";
    let mut connection: PooledConnection<ConnectionManager<PgConnection>> =
        pool.get().expect(connection_error);

    use crate::schema::likes::dsl::{likes, tweet_id};
    let table_rows: Result<Vec<Like>, _> = likes
        .filter(tweet_id.eq(uuid))
        .load::<Like>(&mut connection);

    let response: Vec<Like> = match table_rows {
        Ok(rows) => rows,
        Err(_) => vec![],
    };

    let responder: HttpResponse = HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(&response);

    responder
}

#[actix_web::post("/tweets/{id}/likes")]
async fn create_like(
    tweet_id_path: web::Path<String>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    println!("POST /tweets/{{id}}/likes");

    println!("tweet_id_path: {:#?}", tweet_id_path);
    let uuid: Uuid = Uuid::from_str(&tweet_id_path.as_str()).unwrap();
    println!("uuid: {:#?}", uuid);
    let new_like: Like = Like::new(uuid);
    println!(
        "new_like: {:#?} | {:#?} | {:#?}",
        new_like.id, new_like.created_at, new_like.tweet_id
    );

    let connection_error: &str = "ERROR: The connection has an error.";
    let mut connection: PooledConnection<ConnectionManager<PgConnection>> =
        pool.get().expect(connection_error);

    use crate::schema::likes::dsl::likes;
    let insert_error: &str = "ERROR: It can't insert these values in the table.";
    let insert_data = insert_into(likes)
        .values(&new_like)
        .execute(&mut connection)
        .expect(insert_error);

    println!("insert data: {:#?}", insert_data);

    let responder: HttpResponse = HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(&new_like);

    println!("responder: {:#?}", responder);

    responder
}

#[actix_web::delete("/tweets/{id}/likes")]
async fn delete_like(
    tweet_id_path: web::Path<String>,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    println!("DELETE /tweets/{{id}}/likes");

    println!("tweet_id_path: {:#?}", tweet_id_path);
    let uuid: Uuid = Uuid::from_str(&tweet_id_path.as_str()).unwrap();
    println!("uuid: {:#?}", uuid);

    let connection_error: &str = "ERROR: The connection has an error.";
    let mut connection: PooledConnection<ConnectionManager<PgConnection>> =
        pool.get().expect(connection_error);

    use crate::schema::likes::dsl::{likes, tweet_id};
    println!(
        "likes total: {:#?}",
        likes.count().first::<i64>(&mut connection).unwrap()
    );
    let delete_error: &str = "ERROR: It can't insert these values in the table.";
    let delete_data = delete(likes.filter(tweet_id.eq(uuid)))
        .execute(&mut connection)
        .expect(delete_error);
    println!("delete_data: {:#?}", delete_data);
    println!(
        "likes total: {:#?}",
        likes.count().first::<i64>(&mut connection).unwrap()
    );

    let responder: HttpResponse = HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json("{{\"sucess\":true}}");

    println!("responder: {:#?}", responder);

    responder
}
