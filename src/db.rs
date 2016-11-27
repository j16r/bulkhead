use iron::BeforeMiddleware;
use iron::prelude::*;
use iron::typemap::Key;
use r2d2::{Config, Pool, LoggingErrorHandler, PooledConnection, InitializationError};
use r2d2_postgres::{TlsMode, PostgresConnectionManager};

pub struct DbPool {
    pool: Pool<PostgresConnectionManager>
}

impl DbPool {
    pub fn new() -> Result<DbPool, InitializationError> {
        let config = Config::builder()
            .error_handler(Box::new(LoggingErrorHandler))
            .build();
        let manager = PostgresConnectionManager::new("postgres://bulkhead@localhost",
                                                     TlsMode::None)
            .unwrap();
        let pool = try!(Pool::new(config, manager));
        let db_pool = DbPool {pool: pool};
        db_pool.migrate();
        Ok(db_pool)
    }

    fn migrate(&self) {
        let pool = self.pool.clone();
        let connection = pool.get().unwrap();
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

pub type DbConnection = PooledConnection<PostgresConnectionManager>;

impl Key for DbPool {
    type Value = DbConnection;
}

impl BeforeMiddleware for DbPool {
    fn before(&self, request: &mut Request) -> IronResult<()> {
        let pool = self.pool.clone();
        request.extensions.insert::<DbPool>(pool.get().unwrap());
        Ok(())
    }
}

pub fn connection<'req>(request: &'req mut Request) -> &'req DbConnection {
    request.extensions.get::<DbPool>().unwrap()
}
