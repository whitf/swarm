use uuid::Uuid;

pub enum HostStatus {
	Online,
	Offline,
	Idle,
	Working,
}

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
