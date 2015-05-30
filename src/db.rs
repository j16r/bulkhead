use postgres::{Connection, SslMode};

pub fn connect() -> Connection {
    Connection::connect("postgres://bulkhead@localhost",
                        &SslMode::None).unwrap()
}

pub fn migrate(conn: &Connection) {
    conn.execute("CREATE TABLE IF NOT EXISTS users (
                    id         SERIAL PRIMARY KEY,
                    name       VARCHAR NOT NULL,
                    created_at TIMESTAMP NOT NULL
                  )", &[]).unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS sessions (
                    id         SERIAL PRIMARY KEY,
                    user_id    int,
                    created_at TIMESTAMP NOT NULL,

                    CONSTRAINT fk_user FOREIGN KEY (user_id)
                    REFERENCES users (id)
                  )", &[]).unwrap();
}
