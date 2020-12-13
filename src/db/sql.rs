
pub const TABLE_COUNT: usize = 5;

pub const CREATE_TABLES: [&str; 5] = [
	CREATE_TABLE_DRONE,
	CREATE_TABLE_JOB,
	CREATE_TABLE_JOB_STATUS,
	CREATE_TABLE_DRONE_OWNERSHIP,
	CREATE_TABLE_DATABASE_VERSION,
];

/* Create tables. */
pub const CREATE_TABLE_DATABASE_VERSION: &str = "
	CREATE TABLE database_version (
		version VARCHAR(16)
	);
";

pub const CREATE_TABLE_DRONE: &str = "
	CREATE TABLE drone (
		online bool NOT NULL DEFAULT true,
		address VARCHAR(20) NOT NULL,
		id Uuid PRIMARY KEY NOT NULL,
		port INTEGER NOT NULL DEFAULT 9079
	);
";

pub const CREATE_TABLE_DRONE_OWNERSHIP: &str = "
	CREATE TABLE drone_ownership (
		drone_id Uuid NOT NULL,
		job_id Uuid NOT NULL,
		PRIMARY KEY (drone_id, job_id)
		FOREIGN KEY(drone_id) REFERENCES drone(id),
		FOREIGN KEY(job_id) REFERENCES job(id)
	);
";

pub const CREATE_TABLE_JOB: &str = "
	CREATE TABLE job (
		active bool NOT NULL DEFAULT true,
		created TIMESTAMP DEFAULT CURRENT_TIMMESTAMP,
		finished TIMESTAMP DEFAULT NULL,
		id Uuid PRIMARY KEY,
		status VARCHAR(32)
	);
";

pub const CREATE_TABLE_JOB_STATUS: &str = "
	CREATE TABLE job_status_enum (
		id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
		job_status VARCHAR(32)
	);
";

pub const JOB_STATUS_VALUES: [&str; 5] = [
	"Canceled",
	"Error",
	"Finished",
	"New",
	"Working"
];

/* INSERT sql statements */
pub const INSERT_JOB_STATUS_VALUES: &str = "INSERT INTO job_status_enum (job_status) VALUES(?1);";

pub const INSERT_DATABASE_VERSION: &str = "INSERT INTO database_version (version) VALUES(?1);";

pub const INSERT_OR_UPDATE_DRONE: &str = "
	INSERT INTO drone (address, id, online, port)
	VALUES(?1, ?2, ?3, ?4)
	ON CONFLICT (id)
	DO
		UPDATE
			address = ?1, online = ?3, port = ?4;
";



/* SELECT sql statements */
pub const SELECT_TABLE_COUNT: &str = "SELECT count(*) FROM sqlite_master WHERE type = 'table' AND name != 'sqlite_sequence';";

pub const SELECT_DATABASE_VERSION: &str = "SELECT version FROM database_version LIMIT 1;";

/* Tests */
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn table_count_test() {
		assert!(TABLE_COUNT == CREATE_TABLES.len());
	}

}