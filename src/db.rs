use iron::BeforeMiddleware;
use iron::prelude::*;
use iron::typemap::Key;
use postgres::SslMode;
use r2d2::{Pool, NoopErrorHandler, PooledConnection};
use r2d2_postgres::{PostgresConnectionManager};
use std::default::Default;
use std::sync::Arc;

pub struct DbPool {
    pool: Arc<Pool<PostgresConnectionManager>>
}

impl DbPool {
    pub fn new() -> DbPool {
        let config = Default::default();
        let manager = PostgresConnectionManager::new("postgres://bulkhead@localhost",
                                                     SslMode::None)
            .unwrap();
        let error_handler = Box::new(NoopErrorHandler);
        let pool = Arc::new(
            Pool::new(config, manager, error_handler).unwrap());

        DbPool {pool: pool}
    }

    fn migrate(&self) {
        let connection = Pool::get_arc(self.pool.clone()).unwrap();
        connection.execute("CREATE TABLE IF NOT EXISTS users (
                                id         SERIAL PRIMARY KEY,
                                name       VARCHAR NOT NULL,
                                created_at TIMESTAMP NOT NULL
                            )", &[]).unwrap();
        connection.execute("CREATE TABLE IF NOT EXISTS sessions (
                                id         SERIAL PRIMARY KEY,
                                user_id    int,
                                created_at TIMESTAMP NOT NULL,

                                CONSTRAINT fk_user FOREIGN KEY (user_id)
                                REFERENCES users (id)
                            )", &[]).unwrap();
    }
}

pub type DbConnection = PooledConnection<'static, PostgresConnectionManager>;

impl Key for DbPool {
    type Value = DbConnection;
}

impl BeforeMiddleware for DbPool {
    fn before(&self, request: &mut Request) -> IronResult<()> {
        request.extensions.insert::<DbPool>(Pool::get_arc(self.pool.clone()).unwrap());
        Ok(())
    }
}

pub fn connection<'req>(request: &'req mut Request) -> &'req DbConnection {
    request.extensions.get::<DbPool>().unwrap()
}
