use uuid::Uuid;

pub struct Job {
	_id:							Uuid,
	_tags:						Vec<String>,
}

impl Job {
	pub fn new() -> Self {
		let id = Uuid::new_v4();
		let tags = Vec::new();

		Job {
			_id: id,
			_tags: tags,
		}
	}


}
