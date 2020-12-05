use std::env;

pub fn main() {
	const VERSION: &'static str = env!("CARGO_PKG_VERSION");

	let title = format!("swarn drone control v.{}", VERSION);

	let args: Vec<String> = env::args().collect();

	println!();

	let cmd  = &args[1];

	match &cmd[..] {
		"restart" => {
			println!("restart the daemon...");
		},
		"start" => {
			println!("start the daemon...");
		},
		"status" => {
			println!("display the status");
		},
		"stop" => {
			println!("stop the daemon...");
		},
		_ => {
			println!("help info goes here");
		},
	}

	println!();

}