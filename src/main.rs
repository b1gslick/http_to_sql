use std::collections::HashMap;

use crate::store::db_store::Store;
use sqlx::postgres::PgRow;
use sqlx::{Column, Row, ValueRef};
use warp::http::StatusCode;
use warp::{http::Method, Filter};
mod error;
mod store;

#[tokio::main]
async fn main() {
    let db_type = std::env::var("DB_TYPE").unwrap_or("postgres".to_string());
    let db_user = std::env::var("DB_USER").expect("no db user provide");
    let db_password = std::env::var("DB_PASSWORD").expect("no password provide");
    let db_host = std::env::var("DB_HOST").expect("no db host");
    let db_port = std::env::var("DB_PORT").expect("no port provide");
    let db_name = std::env::var("DB_NAME").expect("no db name");

    let url = format!(
        "{}://{}:{}@{}:{}/{}",
        db_type, db_user, db_password, db_host, db_port, db_name
    );
    let store = Store::new(&url).await;
    let store_filter = warp::any().map(move || store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let make_query = warp::post()
        .and(warp::path("query"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(make_query);

    let routes = make_query
        .with(cors)
        .with(warp::trace::request())
        .recover(error::return_error);
    warp::serve(routes).run(([0, 0, 0, 0], 4545)).await;
}
async fn make_query(store: Store, query: String) -> Result<impl warp::Reply, warp::Rejection> {
    match store.get_query(query.clone()).await {
        Ok(query_result) => {
            let mut result = vec![];
            for row in query_result {
                result.push(row_to_json(row))
            }
            println!("{:?}", result);
            Ok(warp::reply::with_status(
                format!("{:?}", result),
                StatusCode::OK,
            ))
        }
        Err(e) => Err(warp::reject::custom(e)),
    }
}

fn row_to_json(row: PgRow) -> HashMap<String, String> {
    let mut result = HashMap::new();
    for col in row.columns() {
        let value = row.try_get_raw(col.ordinal()).unwrap();
        let value = match value.is_null() {
            true => "NULL".to_string(),
            false => match value.as_str() {
                Ok(val) => val.to_string(),
                Err(_) => "date created".to_string(),
            },
        };
        result.insert(col.name().to_string(), value);
    }

    result
}
