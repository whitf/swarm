use procfs::process::Process;
use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixStream, UnixListener};
use std::path::Path;
use std::sync::mpsc;
use std::thread;

use swarm::db;
use swarm::log;
use swarm::models;

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
	println!();
	const VERSION: &'static str = env!("CARGO_PKG_VERSION");
	const DEFAULT_CONFIG: &'static str = "data/etc/swarm/drone.cfg.toml";

	// Check args for non-standard config file.

	let c = models::Config::load_or_new(DEFAULT_CONFIG);

	// Start logging process.
	let (log_tx, log_rx) = mpsc::channel::<models::LogMessage>();
	let mut l = log::Log::init(c.id.clone(), c.log_dir.clone(), c.error_log.clone(), c.system_log.clone());
	let log_handle = thread::spawn(move || {
		l.run(log_rx);
	});

	// Database verification (or creation if needed.)
	let _db = db::Database::verify_or_init(c.id.clone(), c.db_dir.clone(), c.db_file.clone());


	// Load additional config info from database.






	let me = Process::myself().unwrap();
	let socket_path = format!("/tmp/swarm_drone_{}.sock", me.pid);
	let listener = UnixListener::bind(socket_path).unwrap();

	println!("Start drone process v. {:?} (pid = {}).", VERSION, me.pid);
	log_tx.send(models::LogMessage::new(models::LogType::SystemLog, format!("Drone process v.{:?}, id = {}, pid = {} is online.", VERSION, c.id, me.pid))).unwrap();

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

	log_handle.join().unwrap();
}
