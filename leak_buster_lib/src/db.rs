use rusqlite::{params, Connection, OptionalExtension};
use rusqlite::types::ValueRef;
use std::convert::TryInto;
use std::path::Path;
use std::result;

struct Db {
    connection: Connection
}

impl Db {
    pub fn connect<P: AsRef<Path>>(db_path: P) -> Result<Db> {
        let connection = Connection::open(db_path)?;
        let db = Db { connection };
        db.init()?;
        Ok(db)
    }

    fn init(&self) -> Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS usage (
                app_key     TEXT NOT NULL,
                timestamp   INT,
                duration    INT
            )", [])?;
        Ok(())
    }

    fn record_usage(&self, app_key: &str, timestamp: u64, duration: u32)
        -> Result<()> {
        self.connection.execute(
            "INSERT INTO usage
                (app_key, timestamp, duration) VALUES (?1, ?2, ?3)",
            params![app_key, timestamp, duration]
        )?;
        Ok(())
    }

    fn get_usage(&self, app_key: &str, from: u64, to: u64) -> Result<u64> {
        let usage: i64 = self.connection.query_row(
            "SELECT SUM(duration) FROM USAGE
                WHERE app_key = ?1
                  AND timestamp >= ?2
                  AND timestamp <= ?3",
            params![app_key, from, to],
            |row| Ok(row.get_ref(0)?.as_i64_or_null())
        )??.unwrap_or(0);
        let usage: Result<u64> = usage.try_into().map_err(|err| {
            Error::InvalidDataError(format!(
                    "Total usage must be non-negative ({:?})",
                    err
            ))
        });
        usage
    }

}

#[derive(Debug)]
pub enum Error {
    RusqliteError(rusqlite::Error),
    InvalidDataError(String)
}

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Error {
        Error::RusqliteError(err)
    }
}

impl From<rusqlite::types::FromSqlError> for Error {
    fn from(err: rusqlite::types::FromSqlError) -> Error {
        Error::InvalidDataError(
            format!("Type mismatch while reading from Db: {:?}", err)
        )
    }
}

pub type Result<T, E = Error> = result::Result<T, E>;

mod test {
    use crate::db::Db;

    use tempfile::NamedTempFile;

    #[test]
    fn can_connect_to_and_initialize_new_db() {
        let f = tmpf();
        Db::connect(f.path()).expect("Could not create&init db");
    }

    #[test]
    fn can_connect_to_existing_db() {
        let f = tmpf();
        Db::connect(f.path()).unwrap();
        Db::connect(f.path()).expect("Could not connect to existing db");
    }

    #[test]
    fn usage_is_zero_in_fresh_db() {
        let f = tmpf();
        let db = Db::connect(f.path()).unwrap();
        let usage = db.get_usage("some-app", 0, 1000000000).unwrap();
        assert_eq!(0, usage, "If there are no entries, usage should be 0");
    }

    #[test]
    fn can_record_usage() {
        let f = tmpf();
        let db = Db::connect(f.path()).unwrap();
        db.record_usage("some-app", 1638437768, 60).unwrap();
    }

    #[test]
    fn usage_is_calculated_correctly() {
        let f = tmpf();
        let db = Db::connect(f.path()).unwrap();
        let usages = vec![
            ("a1", 100, 60),
            ("a2", 150, 60),
            ("a1", 200, 60),
            ("a1", 300, 60),
        ];
        for (app_key, timestamp, usage) in usages {
            db.record_usage(app_key, timestamp, usage).unwrap();
        }
        let tests = vec![
            ("a1", 100, 300, 180),
            ("a1", 150, 300, 120),
            ("a1", 100, 250, 120),
            ("a1", 150, 250, 60),
            ("a1", 150, 170, 0),
            ("a2", 100, 300, 60),
        ];
        for (app_key, from, to, expected_usage) in tests {
            let param_str = format!(
                "app_key={}, from={}, to={}, expected_usage={}",
                app_key, from, to, expected_usage
            );
            let u = db.get_usage(app_key, from, to)
                .expect(&format!("Could not get usage for params: {}",
                                param_str));
            assert_eq!(expected_usage, u, "{}", param_str);
        }

    }

    fn tmpf() -> NamedTempFile {
        NamedTempFile::new().unwrap()
    }
}
