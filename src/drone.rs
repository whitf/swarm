use uuid::Uuid;

use crate::host::Host;
use crate::job;

pub struct Drone {
	pub id:						Uuid,
	pub swarm:					Vec<Host>,
	pub tags:					Vec<String>,
	pub threads:				usize,
	pub workload:				Vec<job::Job>,
}

impl Drone {
	pub fn new() -> Self {
		let id = Uuid::new_v4();
		let swarm = Vec::new();
		let tags = Vec::new();
		let threads = 1usize;
		let workload = Vec::new();

		Drone {
			id,
			swarm,
			tags,
			threads,
			workload,
		}
	}

	/** swarm (groups of other hosts) related functions */
	pub fn remember(&mut self, host: Host) {
		// Add a known host the the "swarm" list.
		self.swarm.push(host);
	}

	pub fn forget(&mut self, _host: Host) {
		// Remove a known host from the swarm list (and archive it.);
	}

	pub fn search(&mut self) {
		// Search local archives (read: sqlite db) for info about messages/host(s)/jobs/etc.
	}

	pub fn sync(&mut self) {
		// Reach out to all known hosts and ask for their host lists, workloads, etc.
	}

	/** swarm (this drone) related functions */
	pub fn online(&mut self) {}
	
	pub fn offline(&mut self) {}
	
	pub fn run(&mut self) {}

	pub fn report(&mut self) {
		// Send a message to all "online" hosts that we know about.
	}	

	/** Job related functions */
	pub fn archive_job(&mut self, _job_id: Uuid) {}
	
	pub fn finish(&mut self) {}

	pub fn load(&mut self) {
		// Load this worker's state from the local db.
	}

	pub fn save(&mut self) {
		// Save this worker's state from the local db.
	}
	
	pub fn submit(&mut self) {
		// Add a new job to the queue.
		// This inlcudes passing the job details on to all known hosts.
	}

	pub fn start(&mut self) {}
	
	pub fn work(&mut self, _job_id: Uuid) {}
}
