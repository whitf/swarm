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
