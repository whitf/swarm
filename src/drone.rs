use std::collections::HashMap; 
use std::sync::mpsc::{Receiver, Sender};
use uuid::Uuid;

use crate::db;
use crate::job;
use crate::models::{DroneCtlType, DroneCtl, Host, LogType, LogMessage};

pub struct Drone {
	pub db:						db::Database,
	pub id:						Uuid,
	pub log_tx:					Sender<LogMessage>,
	pub online:					bool,
	pub swarm:					HashMap<Uuid, Host>,
	pub tags:					Vec<String>,
	pub threads:				usize,
	pub workload:				Vec<job::Job>,
}

impl Drone {
	pub fn new(db: db::Database, log_tx: Sender<LogMessage>) -> Self {
		let id = Uuid::new_v4();
		let online = false;
		let swarm = HashMap::new();
		let tags = Vec::new();
		let threads = 1usize;
		let workload = Vec::new();

		Drone {
			db,
			id,
			log_tx,
			online,
			swarm,
			tags,
			threads,
			workload,
		}
	}

	/** swarm (groups of other hosts) related functions */
	pub fn search(&mut self) {
		// Search local archives (read: sqlite db) for info about messages/host(s)/jobs/etc.
	}

	pub fn sync(&mut self) {
		// Reach out to all known hosts and ask for their host lists, workloads, etc.
	}

	/** swarm (this drone) related functions */
	fn online(&mut self, host: Host) {
		let host_id = host.id.clone();

		self.db.update_host(&host);
		self.swarm.insert(host.id, host);

		self.log_tx.send(LogMessage::new(
			LogType::SystemLog,
			format!("Remote drone id = {} has gone online.", host_id)
		)).unwrap();
	}
	
	fn offline(&mut self, host: Host) {
		let host_id = host.id.clone();

		self.db.update_host(&host);
		self.swarm.insert(host.id, host);

		self.log_tx.send(LogMessage::new(
			LogType::SystemLog,
			format!("Remote drone id = {} has gone offline.", host_id)
		)).unwrap();
	}
	
	pub fn run(&mut self, rx: Receiver<DroneCtl>) {
		self.log_tx.send(LogMessage::new(
			LogType::SystemLog,
			format!("Swarm drone id = {} running.", self.id)
		)).unwrap();

		println!("drone entering work loop...");
		while self.online {
			let msg = rx.recv().unwrap();

			match msg.dronectl_type {
				DroneCtlType::Offline => {
					if let Some(host_data) = msg.host_data {
						self.offline(host_data);
					}
				},
				DroneCtlType::Online => {
					if let Some(host_data) = msg.host_data {
						self.online(host_data);
					}
				}
				DroneCtlType::Stop => {
					self.stop();			
				},
				_ => {},
			}
		}

		// Finish shutdown.
		self.log_tx.send(LogMessage::new(
			LogType::SystemLog,
			format!("Swarm drone id = {} shutdown.", self.id)
		)).unwrap();
		std::thread::sleep(std::time::Duration::from_secs(2));	
		std::process::exit(0x000);
	}

	pub fn report(&mut self) {
		// Send a message to all "online" hosts that we know about.
	}	

	/** Job related functions */
	pub fn archive_job(&mut self, _job_id: Uuid) {}
	
	pub fn finish(&mut self) {}

	pub fn load(&mut self) {
		// Load this worker's state from the local db.
	}

	fn save(&mut self) {
		// Save this worker's state from the local db.
	}
	
	pub fn submit(&mut self) {
		// Add a new job to the queue.
		// This inlcudes passing the job details on to all known hosts.
	}

	pub fn start(&mut self) {
		self.online = true;
	}

	fn stop (&mut self) {
		self.online = false;
	}
	
	pub fn work(&mut self, _job_id: Uuid) {}
}
