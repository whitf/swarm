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

#[derive(Serialize)]
pub struct Config {
	pub db_path:						String,
	pub error_log:						String,
	pub file:							String,
	pub id:								Uuid,
	pub system_log:						String,
}

impl Config {
	pub fn new(file: &str) -> Self {
		let toml_content = fs::read_to_string(file).expect("Failed to read toml config file.");
		let config_value: Value = toml::from_str(&toml_content).expect("Failed to parse config values.");

		let mut db_path = String::from("/usr/local/swarm/drone.db");
		let mut error_log = String::from("/var/log/swarm/error.log");
		let mut id = Uuid::new_v4();
		let mut system_log = String::from("/var/log/swarm/system.log");

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

		Config {
			db_path,
			error_log,
			file: file.to_string(),
			id,
			system_log,
		}
	}

	pub fn save(&mut self) -> bool {
		let mut config_toml = String::from("[swarm]");
		config_toml.push('\n');
		config_toml = config_toml + &toml::to_string(&self).unwrap();

		fs::write(self.file.to_string(), config_toml).expect("Failed to write config file.");

		true
	}
}
