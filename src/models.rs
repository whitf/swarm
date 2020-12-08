use serde::Serialize;
use std::fs;
use toml::Value;
use uuid::Uuid;

use crate::host::Host;

pub enum MessageType {
	FinishJob,
	Online,
	Offline,
	StartJob,
	QueueJob,
}

pub struct Message {
	pub carbon_copy:				Vec<Host>,
	pub id:							Uuid,
	pub message:					String,
}

impl Message {
	pub fn new(carbon_copy: Vec<Host>, message: String) -> Self {
		let id = Uuid::new_v4();

		Message {
			carbon_copy,
			id,
			message,
		}
	}
}

#[derive(Debug, Serialize)]
pub struct Config {
	pub db_path:						String,
	pub error_log:						String,
	pub file:							String,
	pub id:								Uuid,
	pub system_log:						String,
}

impl Config {
	pub fn load_or_new(file: &str) -> Self {
		// Create default values, which will be overwritten if values are found in a config file.
		let mut db_path = String::from("/usr/local/swarm/drone.db");
		let mut error_log = String::from("/var/log/swarm/error.log");
		let mut id = Uuid::new_v4();
		let mut system_log = String::from("/var/log/swarm/system.log");

		let toml_content = fs::read_to_string(file);
		match toml_content {
			Ok(content) => {
				let config_value: Value = toml::from_str(&content).expect("Failed to parse config values after file was found.\nFix or delete (for automatic recreation) config file.");
				let config: &toml::map::Map<String, Value> = config_value["swarm"].as_table().unwrap();

				for (k, v) in config.iter() {
				let v_str = v.as_str().unwrap().to_string();
					match k.as_str() {
						"db_path" => {
							db_path = v_str;
						},
						"error_log" => {
							error_log = v_str;
						},
						"id" => {
							id = Uuid::parse_str(&v_str).unwrap();
						},
						"system_log" => {
							system_log = v_str;
						},
						_ => {
							// Unrecognized items are irgnore and removed on "writeback".
						},
					}
				}
			},
			Err(content_err) => {
				if std::io::ErrorKind::NotFound == content_err.kind() {
					// Log file not found issue, file will be created later by the config.save() call.
					println!("Config file {} not found, creating a new one.", file);
				}

			}
		}

		let mut config = Config {
			db_path,
			error_log,
			file: file.to_string(),
			id,
			system_log,
		};

		config.save();

		return config;
	}

	pub fn save(&mut self) -> bool {
		let mut config_toml = String::from("[swarm]");
		config_toml.push('\n');
		config_toml = config_toml + &toml::to_string(&self).unwrap();

		fs::write(self.file.to_string(), config_toml).expect("Failed to write config file.");

		true
	}
}
