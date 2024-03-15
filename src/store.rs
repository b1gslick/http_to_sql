pub mod db_store {

    use crate::error;
    use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};

    #[derive(Clone, Debug)]
    pub struct Store {
        pub connection: PgPool,
    }

    impl Store {
        pub async fn new(db_url: &str) -> Self {
            let db_pool = match PgPoolOptions::new()
                .max_connections(5)
                .connect(db_url)
                .await
            {
                Ok(pool) => pool,
                Err(e) => panic!("Couldn't establish DB connection:{}", e),
            };
            Store {
                connection: db_pool,
            }
        }
        pub async fn get_query(
            self,
            query: String,
        ) -> std::result::Result<std::vec::Vec<PgRow>, error::Error> {
            println!("query {:?}", &query);
            match sqlx::query(&query).fetch_all(&self.connection).await {
                Ok(activities) => Ok(activities),
                Err(_) => Err(error::Error::DatabaseQueryError),
            }
        }
    }
}
