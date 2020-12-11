use fallible_iterator::FallibleIterator;
use rusqlite::{Connection, NO_PARAMS, params, Result};
use std::fs;
use std::iter::Iterator;
use uuid::Uuid;

pub mod sql;

pub struct Database {
	pub db_dir:						String,
	pub db_file:					String,
	pub db_path:					String,
	pub id:							Uuid,
}

impl Database {
	pub fn verify_or_init(id: Uuid, db_dir: String, db_file: String) -> Self {

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
		let count = stmt.query_map(NO_PARAMS, |row| {
			let count_str: String = row.get(0).unwrap();
			Ok(count_str)
		}).unwrap();

		println!("count = {:?}", count.size_hint().0);

		let count = count.size_hint().0 as usize;

		if count != sql::TABLE_COUNT && count != 0 {
			println!();
			println!("Database validation error: TABLE_COUNT should be 0 (empty database) or {} (fully initialized database).", sql::TABLE_COUNT);
			println!();

			std::process::exit(0x0100);
		} else {
			println!();
			println!("database verifed, found {} tables.", count);
		}

		/*
		while let Some(count) = tbl_count.next().unwrap() {
			let n: i32 = count.get(0).unwrap();
			println!("table count = {:?}", n);
		}
		*/

		Database {
			db_dir,
			db_file,
			db_path,
			id,
		}
	}
}
