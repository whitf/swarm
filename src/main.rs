use procfs::process::Process;
use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixStream, UnixListener};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

fn process_command(stream: UnixStream) {
	let stream = BufReader::new(stream);
	for line in stream.lines() {
		println!("received command...");

		match &line.unwrap()[..] {
			"HALT" => {
				//shutdown signal
				let me = Process::myself().unwrap();
				println!("Shutting down swarm drone (pid = {}).", me.pid);
				

				// clear pid file
				let socket_path = format!("/tmp/swarm_drone_{}.sock", me.pid);
				let pid_path = Path::new(&socket_path);
				let _ = std::fs::remove_file(&pid_path).unwrap();

				println!();

				std::process::exit(0);
			},
			"RESTART" => {},
			"SYNC" => {},
			_ => {
				// unrecognised command
				// log and ignore
			}
		}
	}
}

fn main() {

	let me = Process::myself().unwrap();
	let socket_path = format!("/tmp/swarm_drone_{}.sock", me.pid);
	let listener = UnixListener::bind(socket_path).unwrap();

	println!("start process pid = {}", me.pid);

	for stream in listener.incoming() {
		match stream {
			Ok(stream) => {
				println!("doing some stream stuff in another thread ....");
				thread::spawn(|| process_command(stream));
			},
			Err(err) => {
				println!("Error: {}", err);
				break;
			},
		}
	}
}
