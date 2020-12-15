use clap::{App, AppSettings, Arg};
use std::collections::HashMap;
use std::env;
use std::path::Path;

fn _get_sockets() -> Vec<String> {

	let pid = 509000i32;

	let mut sockets: Vec<String> = Vec::new();

	let socket_path = format!("/tmp/swarm_drone_{}.sock", pid);
	let pid_path = Path::new(&socket_path);

	sockets.push(pid_path.to_str().unwrap().to_string());

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
		println!("Killing the drone process.");
	}

	if matches.is_present("status") {
		println!("Reporting the basic drone status...");
	}

	if matches.is_present("details") {
		println!("Reporting more detailed drone information...");
	}

	if matches.is_present("restart") {
		println!("Restarting the drone process...");
	}

	if matches.is_present("start") {
		println!("Starting the drone process...");
	}

	if matches.is_present("stop") {
		println!("Stopping the drone process...");
	}

	println!();

}
