use std::env;
use std::path::Path;
use std::process::Command;

fn get_sockets() -> Vec<String> {

	let pid = 509000i32;

	let mut sockets: Vec<String> = Vec::new();

	let socket_path = format!("/tmp/swarm_drone_{}.sock", pid);
	let pid_path = Path::new(&socket_path);

	sockets.push(pid_path.to_str().unwrap().to_string());

	return sockets;
}


pub fn main() {
	//const VERSION: &'static str = env!("CARGO_PKG_VERSION");

	let mut args: Vec<String> = env::args().collect();

	println!();
	println!("length of args = {}", args.len());

	if 1 <= args.len() {
		args.push(String::from("help"));
	}

	let cmd = &args[1];

	match &cmd[..] {
		"configure" => {

		},
		"restart" => {
			println!("restart the daemon...");
		},
		"start" => {
			println!("start the daemon...");

			let mut command = Command::new("/usr/local/bin/looper");

			if let Ok(child) = command.spawn() {
				println!("child (pid = {}) is running", child.id());
			} else {
				println!("ls command didn't start...");
			}
		},
		"status" => {
			println!("display the status");
		},
		"stop" => {
			println!("Shutting down the process...");
			let paths = get_sockets();

			println!("found {} socket files", paths.len());

			for path in paths {
				println!("path = {:?}", path);
			}

			/*
			if let Some(path) = get_sockets() {
				println!("socket path = {:?}", path);
			} else {
				println!("failed to find pid for process");
			}
			*/
		},
		"help" | _ => {
			println!("help info goes here");
		},
	}

	println!();

}
