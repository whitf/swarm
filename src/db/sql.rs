//use swarm::sql;

pub const TABLE_COUNT: usize = 2;
pub const FUNCTION_COUNT: usize = 0;

pub const CREATE_TABLES: [&str; 2] = [
	CREATE_TABLE_JOB,
	CREATE_TABLE_JOB_STATUS,
];

pub const SELECT_TABLE_COUNT: &str = "
	SELECT count(*) FROM sqlite_master WHERE type = 'table' AND name != 'sqlite_sequence';
";

/* Create tables. */
pub const CREATE_TABLE_JOB: &str = "
	CREATE TABLE job (
		active bool NOT NULL DEFAULT true,
		created TIMESTAMP DEFAULT CURRENT_TIMMESTAMP,
		finished TIMESTAMP DEFAULT NULL,
		id Uuid PRIMARY KEY,
		status job_status
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn table_count_test() {
		assert!(TABLE_COUNT == CREATE_TABLES.len());
	}

}