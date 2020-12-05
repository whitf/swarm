use uuid::Uuid;

pub struct Job {
	id:							Uuid,
	tags:						Vec<String>,
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
