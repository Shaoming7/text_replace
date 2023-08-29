use crate::translate::Translate;
use config::{Config, ConfigError, File};
use serde::Deserialize;
use mysql::*;
use mysql::prelude::*;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct DatabaseConfig {
    url: String,
}

impl DatabaseConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let mut settings = Config::default();
        // Merge in configurations from the config file
        settings.merge(File::with_name("src/db/db_config"))?;   // 改为相对路径
        // Deserialize the configuration into DatabaseConfig struct
        settings.try_into()
    }
}

fn establish_connection() -> Result<Pool, Box<dyn Error>> {
    let database_config = DatabaseConfig::new()?;
    let pool = Pool::new(database_config.url.as_str())?;
    Ok(pool)
}

pub fn get_translations() -> Result<Vec<Translate>, Box<dyn Error>> {
    let pool = establish_connection()?;

    let query = "SELECT id, chinese, english, row_num FROM translate";

    let translations = pool
        .get_conn()?
        .query_map(query, |(id, chinese, english, row_num)| {
            Translate { id, chinese, english, row_num }
        })?;

    Ok(translations)
}

pub fn insert_translations(translations: &[Translate]) -> Result<(), Box<dyn Error>> {
    let pool = establish_connection()?;

    let insert_query = r"INSERT INTO translate (chinese, english, row_num)
                        VALUES (:chinese, :english, :row_num)";

    let params = translations.iter().map(|t| params! {
        "chinese" => &t.chinese,
        "english" => &t.english,
        "row_num" => t.row_num,
    });

    pool.get_conn()?.exec_batch(insert_query, params)?;

    Ok(())
}