use clap::{App, Arg};
use procfs::process::Process;
use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixStream, UnixListener};
use std::path::Path;
use std::sync::mpsc;
use std::thread;

use swarm::db;
use swarm::drone;
use swarm::log;
use swarm::models;

fn process_command(stream: UnixStream, tx: mpsc::Sender<models::DroneCtl>) {
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

				tx.send(models::DroneCtl::new(models::DroneCtlType::Offline, "".to_string())).unwrap();
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

	let title = format!("swarm-drone v.{}", VERSION);
	let matches = App::new(title)
		.version(VERSION)
		.about("A simple framework to create a server-less swarm of worker drones.")
		.arg(Arg::with_name("config")
			.short("c")
			.long("config")
			.takes_value(true)
			.help("Specify a config file (Default: /etc/swarm/drone.cfg.toml"))
		.arg(Arg::with_name("port")
			.short("p")
			.long("port")
			.takes_value(true)
			.help("Specify the port to listen on for inter-drone communications."))
		.get_matches();

	let mut c = models::Config::load_or_new(matches.value_of("config").unwrap_or(DEFAULT_CONFIG));
	c.port = matches.value_of("port").unwrap_or(&c.port).to_string();

	// Start logging process.
	let (log_tx, log_rx) = mpsc::channel::<models::LogMessage>();
	let mut l = log::Log::init(c.id.clone(), c.log_dir.clone(), c.error_log.clone(), c.system_log.clone());
	let log_handle = thread::spawn(move || {
		l.run(log_rx);
	});

	// Database verification (or creation if needed.)
	let db = db::Database::verify_or_init(c.id.clone(), c.db_dir.clone(), c.db_file.clone(), log_tx.clone());

	// Load additional config info from database.
	// @TODO


	let me = Process::myself().unwrap();
	let socket_path = format!("/tmp/swarm_drone_{}.sock", me.pid);
	let listener = UnixListener::bind(socket_path).unwrap();

	let (drone_tx, drone_rx) = mpsc::channel::<models::DroneCtl>();
	let mut d = drone::Drone::new(db.unwrap(), log_tx.clone());
	let drone_handle = thread::spawn(move || {
		d.online();
		d.run(drone_rx);
	});

	println!("Start drone process v. {:?} (pid = {}).", VERSION, me.pid);
	log_tx.send(models::LogMessage::new(models::LogType::SystemLog, format!("Drone process v.{:?}, id = {}, pid = {} is online.", VERSION, c.id, me.pid))).unwrap();

	for stream in listener.incoming() {
		match stream {
			Ok(stream) => {
				println!("doing some stream stuff in another thread ....");
				let dtx = drone_tx.clone();
				thread::spawn(|| process_command(stream, dtx));
			},
			Err(err) => {
				println!("Error: {}", err);
				break;
			},
		}
	}

	drone_handle.join().unwrap();
	log_handle.join().unwrap();
}
