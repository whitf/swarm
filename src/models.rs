use serde::{Deserialize, Serialize};
use std::fs;
use toml::Value;
use uuid::Uuid;


#[derive(Clone, Debug, Serialize)]
pub struct Config {
	pub address:						String,
	pub db_dir:							String,
	pub db_file:						String,
	pub error_log:						String,
	pub file:							String,
	pub id:								Uuid,
	pub log_dir:						String,
	pub port:							String,
	pub system_log:						String,
}

impl Config {
	pub fn load_or_new(file: &str) -> Self {
		// Create default values, which will be overwritten if values are found in a config file.
		let mut address = String::from("0.0.0.0");
		let mut db_dir = String::from("data/usr/local/swarm");
		let mut db_file = String::from("drone.db");
		let mut error_log = String::from("error.log");
		let mut id = Uuid::new_v4();
		let mut log_dir = String::from("data/var/log/swarm");
		let mut port = String::from("9079");
		let mut system_log = String::from("system.log");

		let toml_content = fs::read_to_string(file);
		match toml_content {
			Ok(content) => {
				let config_value: Value = toml::from_str(&content).expect("Failed to parse config values after file was found.\nFix or delete (for automatic recreation) config file.");
				let config: &toml::map::Map<String, Value> = config_value["swarm"].as_table().unwrap();

				for (k, v) in config.iter() {
				let v_str = v.as_str().unwrap().to_string();
					match k.as_str() {
						"address" => {
							address = v_str;
						},
						"db_dir" => {
							db_dir = v_str;
						},
						"db_file" => {
							db_file = v_str;
						},
						"error_log" => {
							error_log = v_str;
						},
						"id" => {
							id = Uuid::parse_str(&v_str).unwrap();
						},
						"log_dir" => {
							log_dir = v_str;
						},
						"port" => {
							port = v_str;
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
			address,
			db_dir,
			db_file,
			error_log,
			file: file.to_string(),
			id,
			log_dir,
			port,
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

#[derive(Deserialize, Debug, Serialize)]
pub struct DroneCtl {
	pub dronectl_type:						DroneCtlType,
	pub host_data:							Option<Host>,
	pub job_data:							Option<Job>,
	pub msg:								Option<String>,
}

impl DroneCtl {
	pub fn new(dronectl_type: DroneCtlType, host_data: Option<Host>, job_data: Option<Job>, msg: Option<String>) -> Self {
		DroneCtl {
			dronectl_type,
			host_data,
			job_data,
			msg,
		}
	}
}

#[derive(Deserialize, Debug, Serialize)]
pub enum DroneCtlType {
	FinishJob,
	Message,
	Online,
	Offline,
	QueueJob,
	Stop,
	StartJob,
}

#[derive(Deserialize, Debug, Serialize)]
pub enum HostStatus {
	Online,
	Offline,
	Idle,
	Working,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Host {
	pub address:					String,
	pub id:							Uuid,
	pub port:						String,
	pub online:						bool,
	pub status:						HostStatus,
}

impl Host {
	pub fn new(id: Uuid, address: String, port: String) -> Self {
		let online = false;
		let status = HostStatus::Offline;

		Host {
			address,
			id,
			port,
			online,
			status,
		}
	}

	pub fn ping(&mut self) -> bool {
		// Check to see if a remote host is responding.
		true
	}

	// Mark a remote host online.
	pub fn online(&mut self) {
		self.online = true;
	}

	// Mark a remote host offline.
	pub fn offline(&mut self) {
		self.online = false;
	}
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Job {
	id:								Uuid,
	tags:							Vec<String>,
}

impl Job {
	pub fn new() -> Self {
		let id = Uuid::new_v4();
		let tags = Vec::new();

		Job {
			id,
			tags,
		}
	}
}

pub struct LogMessage {
	pub log_type:					LogType,
	pub message:					String,
	pub message_type:				MessageType,
}

impl LogMessage {
	pub fn new(log_type: LogType, message: String) -> Self {
		LogMessage {
			log_type,
			message,
			message_type: MessageType::Message,
		}
	}
}

pub enum LogType {
	ErrorLog,
	SystemLog,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Message {
	pub carbon_copy:				Vec<Host>,
	pub id:							Uuid,
	pub message:					String,
	pub message_type:				MessageType,
}

impl Message {
	pub fn new(carbon_copy: Vec<Host>, message: String, message_type: MessageType) -> Self {
		let id = Uuid::new_v4();

		Message {
			carbon_copy,
			id,
			message,
			message_type,
		}
	}
}

#[derive(Deserialize, Debug, Serialize)]
pub enum MessageType {
	FinishJob,
	Message,
	Online,
	Offline,
	StartJob,
	Unknown,
	QueueJob,
}
