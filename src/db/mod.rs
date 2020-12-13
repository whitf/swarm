//use fallible_iterator::FallibleIterator;
use rusqlite::{Connection, NO_PARAMS, params, Result};
use std::fs;
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
	pub fn verify_or_init(id: Uuid, db_dir: String, db_file: String, log_tx: Sender<LogMessage>) -> Result<Self, rusqlite::Error> {
		const DATABASE_VERSION: &'static str = env!("CARGO_PKG_VERSION");

		match fs::create_dir_all(&db_dir) {
			Err(err) => {
				log_tx.send(LogMessage::new(
					LogType::ErrorLog,
					format!("Failed to create database dir. Exit from fatal error: {:?}", err)
				)).unwrap();

				log_tx.send(LogMessage::new(
					LogType::SystemLog,
					"Database creation failed. See error log.".to_string()
				)).unwrap();

				println!("Database validation error: could not run fs::create_dir_all(db_dir). Exit from fata error: {:?}", err);
				std::thread::sleep(std::time::Duration::from_secs(2));
				std::process::exit(0x0102);
			},
			_ => {},
		}

		let mut db_path: String = db_dir.to_owned();
		db_path.push('/');
		db_path.push_str(&db_file);

		let conn = Connection::open(&db_path)?;

		let mut stmt = conn.prepare(sql::SELECT_TABLE_COUNT)?;
		let count: i32 = stmt.query_row(NO_PARAMS, |row| row.get(0))?;
		let count = count as usize;

		if count != sql::TABLE_COUNT && count != 0 {
			log_tx.send(LogMessage::new(
				LogType::ErrorLog,
				format!("Database validation error: TABLE_COUNT should be 0 (empty database) or {} (fully initialized database). Found {}", sql::TABLE_COUNT, count)
			)).unwrap();

			log_tx.send(LogMessage::new(
				LogType::SystemLog,
				"Database validation failed. See error log.".to_string()
			)).unwrap();

			println!("Database validation error: TABLE_COUNT should be 0 (empty database) or {} (fully initialized database). Found {}. Exit from fatal error.", sql::TABLE_COUNT, count);
			std::thread::sleep(std::time::Duration::from_secs(2));
			std::process::exit(0x0100);
		} else if count == 0 {
			 log_tx.send(LogMessage::new(
			 	LogType::SystemLog,
			 	"Initializing database from empty state...".to_string()
			 )).unwrap();

			 println!("Inilizing {} database tables...", sql::CREATE_TABLES.len());
			 for create_table_stmt in sql::CREATE_TABLES.iter() {
			 	conn.execute(create_table_stmt, NO_PARAMS)?;
			 }

			 for job_status in sql::JOB_STATUS_VALUES.iter() {
			 	conn.execute(sql::INSERT_JOB_STATUS_VALUES, &[job_status])?;
			 }

			 conn.execute(sql::INSERT_DATABASE_VERSION, &[DATABASE_VERSION])?;

			 log_tx.send(LogMessage::new(
			 	LogType::SystemLog,
			 	format!("Database initialization complete: {} tables.", sql::TABLE_COUNT)
			 )).unwrap();
			 
			 println!(" finished.");
		}

		let mut stmt = conn.prepare(sql::SELECT_DATABASE_VERSION)?;
		let database_version: String = stmt.query_row(NO_PARAMS, |row| row.get(0))?;

		if database_version != DATABASE_VERSION {
			log_tx.send(LogMessage::new(
				LogType::ErrorLog,
				format!("Database validation error: DATABASE_VERSION ({}) expected to be {}. Exit from fatal error.", database_version, DATABASE_VERSION)
			)).unwrap();

			log_tx.send(LogMessage::new(
				LogType::SystemLog,
				"Database validation failed. See error log.".to_string()
			)).unwrap();

			println!("Database validation error: DATABASE_VERSION ({}) expected to be {}. Exit from fatal error.", database_version, DATABASE_VERSION);
			std::thread::sleep(std::time::Duration::from_secs(2));
			std::process::exit(0x0101);
		}

		log_tx.send(LogMessage::new(
			LogType::SystemLog,
			format!("Database v.{} validated: {} of {} tables.", DATABASE_VERSION, count, sql::TABLE_COUNT)
		)).unwrap();

		Ok(Database {
			db_dir,
			db_file,
			db_path,
			id,
			log_tx,
		})
	}
}
