use chrono::Local;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use uuid::Uuid;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// @TODO - log levels Warn, Fatal, Info, etc

pub enum LogType {
	ErrorLog,
	SystemLog,
}

pub struct Log {
	pub error_log:					String,
	pub system_log:					String,
}

impl Log {

	pub fn error(&self, message: String) {
		self.write(LogType::ErrorLog, message);
	}

	pub fn system(&self, message: String) {
		self.write(LogType::SystemLog, message);
	}
	
	fn format_msg(message: String) -> String {
		let now = Local::now();
		// YEAR-MM-DD HH-mm-ss
		let timestamp = now.format("[%Y-%m-%d %H:%M:%S]");

		return format!("{} - {}\n", timestamp, message);
	}
	
	pub fn init(id: Uuid, log_dir: String, error_log_file: String, system_log_file: String) -> Self {
		let startup_msg = Log::format_msg(format!("Starting swarm drone v.{}. id = {}", VERSION, id));

		match fs::create_dir_all(&log_dir) {
			Err(err) => {
				println!("failed to create log dir because {:?}", err);
			},
			_ => {},
		}

		let mut error_log: String = log_dir.to_owned();
		error_log.push('/');
		error_log.push_str(&error_log_file);

		let mut system_log: String = log_dir.to_owned();
		system_log.push('/');
		system_log.push_str(&system_log_file);

		let mut errlog = OpenOptions::new().create(true).append(true).open(&error_log).unwrap();
		errlog.write(startup_msg.as_bytes()).expect("Failed to write startup message to error_log.");

		let mut syslog = OpenOptions::new().create(true).append(true).open(&system_log).unwrap();
		syslog.write(startup_msg.as_bytes()).expect("Failed to write startup message to system_log.");

		Log {
			error_log,
			system_log,
		}
	}

	fn write(&self, log_type: LogType, message: String) {
		let msg = Log::format_msg(message);

		match log_type {
			LogType::ErrorLog => {
				let mut errlog = OpenOptions::new().create(true).append(true).open(&self.error_log).unwrap();
				errlog.write(msg.as_bytes()).expect("Failed to write message to error_log.");
			},
			LogType::SystemLog => {
				let mut syslog = OpenOptions::new().create(true).append(true).open(&self.system_log).unwrap();
				syslog.write(msg.as_bytes()).expect("Failed to write message to system_log");
			},
		}

	}
}
