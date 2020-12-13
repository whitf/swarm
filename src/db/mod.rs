//use fallible_iterator::FallibleIterator;
use rusqlite::{Connection, NO_PARAMS, params, Result};
use std::fs;
use std::iter::Iterator;
use std::sync::mpsc::Sender;
use uuid::Uuid;

use crate::models::{LogType, LogMessage};

pub mod sql;

pub struct Database {
	pub db_dir:						String,
	pub db_file:					String,
	pub db_path:					String,
	pub id:							Uuid,
	pub log_tx:						Sender<LogMessage>,
}

impl Database {
	pub fn verify_or_init(id: Uuid, db_dir: String, db_file: String, log_tx: Sender<LogMessage>) -> Self {

		match fs::create_dir_all(&db_dir) {
			Err(err) => {
				println!("failed to create log dir because {:?}", err);
			},
			_ => {},
		}

		let mut db_path: String = db_dir.to_owned();
		db_path.push('/');
		db_path.push_str(&db_file);

		let conn = Connection::open(&db_path).expect("[Database::verify_or_init] Failed to open database file.");

		let mut stmt = conn.prepare(sql::SELECT_TABLE_COUNT).expect("Sql prepare statement failed.");
		let count: i32 = stmt.query_row(NO_PARAMS, |row| row.get(0)).unwrap();
		let count = count as usize;

		if count != sql::TABLE_COUNT && count != 0 {
			log_tx.send(LogMessage::new(
				LogType::ErrorLog,
				format!("Database validation error: TABLE_COUNT should be 0 (empty database) or {} (fully initialized database).", sql::TABLE_COUNT)
			)).unwrap();

			log_tx.send(LogMessage::new(
				LogType::SystemLog,
				"Database validation failed. See error log.".to_string()
			)).unwrap();
			
			std::process::exit(0x0100);
		} else if count == 0 {
			 log_tx.send(LogMessage::new(
			 	LogType::SystemLog,
			 	"Initializing database from empty state...".to_string()
			 )).unwrap();

			 for create_table_stmt in sql::CREATE_TABLES.iter() {
			 	conn.execute(create_table_stmt, NO_PARAMS).unwrap();
			 }

			 log_tx.send(LogMessage::new(
			 	LogType::SystemLog,
			 	format!("Database initialization complete: {} tables.", sql::TABLE_COUNT)
			 )).unwrap();
		} else {
			log_tx.send(LogMessage::new(
				LogType::SystemLog,
				format!("Database validated: {} of {} tables.", count, sql::TABLE_COUNT)
			)).unwrap();
		}

		Database {
			db_dir,
			db_file,
			db_path,
			id,
			log_tx,
		}
	}
}
