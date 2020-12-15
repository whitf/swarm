use clap::{App, AppSettings, Arg};
use regex::Regex;
use std::env;
use std::io::prelude::*;
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::process::Command;
use std::str;

fn get_pid() -> Option<String> {
	let mut command = Command::new("ps");
	command.arg("-C")
		.arg("swarm")
		.arg("-o")
		.arg("pid=");

	let output = command.output().expect("some error");
	let mut output_str = str::from_utf8(&output.stdout).unwrap().to_string();

	if 0 == output_str.len() {
		return None;
	}

	output_str.pop();	// Gets rid of a newline char at the end of the string.

	Some(output_str)
}

fn get_socket(pid: String) -> Option<String> {
	let socket = format!("/tmp/swarm_drone_{}.sock", pid);

	if Path::new(&socket).exists() {
		println!("socket file {} is there", socket);
	} else {
		println!("socket file {} not found", socket);
		return None;
	}

	Some(socket)
}

fn get_sockets() -> Vec<String> {

	let mut command = Command::new("ls");
	command.arg("/tmp/");

	let output = command.output().expect("some error");
	let output = str::from_utf8(&output.stdout).unwrap().to_string();

	let mut sockets: Vec<String> = Vec::new();

	let srx = Regex::new(r"swarm_drone_[0-9].*.sock").unwrap();
	for cap in srx.captures_iter(&output) {
		sockets.push(cap[0].to_string());
	}

	return sockets;
}

pub fn main() {
	const VERSION: &'static str = env!("CARGO_PKG_VERSION");

	let matches = App::new("swarm dronectl utility")
		.version(VERSION)
		.version_short("v")
		.setting(AppSettings::ArgRequiredElseHelp)
		.setting(AppSettings::NextLineHelp)
		.about("The command line control utility for a swarm drone.")
		.arg(Arg::with_name("details")
			.long("details")
			.takes_value(false)
			.help("Report extended drone process details."))
		.arg(Arg::with_name("kill")
			.long("kill")
			.takes_value(false)
			.conflicts_with_all(&["configure", "details", "restart", "status", "start", "stop"])
			.help("Perform an immediate \"hard\" shutdown of the drone process.\nNOTE!: This command will preempt any other commands."))
		.arg(Arg::with_name("restart")
			.long("restart")
			.takes_value(false)
			.help("Restart the drone process."))
		.arg(Arg::with_name("start")
			.long("start")
			.takes_value(false)
			.conflicts_with_all(&["restart", "stop"])
			.help("Start the drone process."))
		.arg(Arg::with_name("status")
			.long("status")
			.takes_value(false)
			.help("Report the basic drone status information."))
		.arg(Arg::with_name("stop")
			.long("stop")
			.takes_value(false)
			.conflicts_with_all(&["restart", "start"])
			.help("Perform a \"clean\" shutdown of the drone process."))
		.get_matches();

	println!();

	if matches.is_present("kill") {
		println!("Killing the drone process...");

		if let Some(pid) = get_pid() {
			Command::new("kill")
				.arg("-9")
				.arg(pid)
				.spawn()
				.expect("Failed to kill process");
		} else {
			println!("No active swarm drones found. Checking for abandoned drone sockets...");
		}

		let sockets = get_sockets();

		for socket in sockets {
			println!("Removing abandoned socket: {}", socket);
			let file_path = format!("/tmp/{}", socket);
			std::fs::remove_file(Path::new(&file_path)).unwrap();
		}

		println!("Done.");
	}

	if matches.is_present("status") {
		println!("Reporting the basic drone status...");

		 if let Some(pid) = get_pid() {
		 	println!("pid is {}, checking for socket file...", pid);
		 	if let Some(socket) = get_socket(pid) {
		 		println!("found socket file at {}", socket);
		 	} else {
		 		println!("missing socket file");
		 	}
		 } else {
		 	println!("Swarm drone is not running.");

		 	let sockets = get_sockets();
		 	if sockets.len() > 0 {
			 	for socket in sockets {
			 		println!("Found abandoned socket file: {}", socket);
			 	}

			 	println!("\nConsider dronectl --kill to remove abandoned socket files (or manually clear them).");
			 }
		 }

	}

	if matches.is_present("details") {
		println!("Reporting more detailed drone information...");

		if let Some(pid) = get_pid() {
			println!("pid = {} found...", pid);
		} else {
			println!("Swarm drone is not running.");
		}
	}

	if matches.is_present("restart") {
		println!("Restarting the drone process...");
	}

	if matches.is_present("start") {
		println!("Starting the drone process...");

		// Check to see if the process is already running...
		if let Some(pid) = get_pid() {
			println!("Swarm drone process is already running, pid = {}\n", pid);
			std::process::exit(0x000);
		}

		// Clean up any "orphaned" socket files.
		let sockets = get_sockets();
		for socket in sockets {
			let file_path = format!("/tmp/{}", socket);
			std::fs::remove_file(Path::new(&file_path)).unwrap();
		}

		let mut command = Command::new("target/debug/swarm");
		if let Ok(child) = command.spawn() {
			println!("swarm drone pid = {} is running", child.id());
		} else {
			println!("could not launch swarm drone");
		}
	}

	if matches.is_present("stop") {
		println!("Stopping the drone process...");

		if let Some(pid) = get_pid() {
			if let Some(socket) = get_socket(pid) {
				println!("found running socket file ({})...", socket);
				let mut stream = UnixStream::connect(socket).expect("error opening socket...");
				stream.write_all(b"SHUTDOWN").expect("error writing to stream...");
			}
		}
	}

	println!();

}
