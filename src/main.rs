use bincode;
use clap::{App, Arg};
use procfs::process::Process;
use std::io::{BufRead, BufReader, Read};
use std::net::{TcpListener};
use std::os::unix::net::{UnixStream, UnixListener};
use std::path::Path;
use std::sync::mpsc;
use std::thread;

use swarm::db;
use swarm::drone;
use swarm::log;
use swarm::models::*;

fn process_command(stream: UnixStream, tx: mpsc::Sender<DroneCtl>) {
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

				tx.send(DroneCtl::new(DroneCtlType::Stop, None, None, None)).unwrap();
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

fn process_message(address: String, port: u32, tx: mpsc::Sender<DroneCtl>) {
	println!("starting external listener on port {}", port);

	loop {
		let tcp_listen_string = format!("{}:{}", address, port);
		let listener = TcpListener::bind(tcp_listen_string).unwrap();

		for stream in listener.incoming() {
			let mut data = [0 as u8; 128];
			match stream.unwrap().read(&mut data) {
				Ok(_) => {
					let msg: Message = bincode::deserialize(&data).unwrap();
					match msg.message_type {
						MessageType::FinishJob => {
							// Notification from a drone that a job has been finished.
							let host: Host = bincode::deserialize(msg.message.as_bytes()).unwrap();
							tx.send(DroneCtl::new(DroneCtlType::FinishJob, Some(host), None, None)).unwrap();
						},
						MessageType::Message => {
							tx.send(DroneCtl::new(DroneCtlType::Message, None, None, Some(msg.message))).unwrap();
						},
						MessageType::Online => {
							// Notification that a drone has come online.
							let host: Host = bincode::deserialize(msg.message.as_bytes()).unwrap();
							tx.send(DroneCtl::new(DroneCtlType::Online, Some(host), None, None)).unwrap();
						},
						MessageType::Offline => {
							// Notification that a drone has gone offline.
							let host: Host = bincode::deserialize(msg.message.as_bytes()).unwrap();
							tx.send(DroneCtl::new(DroneCtlType::Offline, Some(host), None, None)).unwrap();
						},
						MessageType::StartJob => {
							// Notification from a drone that a job has been started.
							let host: Host = bincode::deserialize(msg.message.as_bytes()).unwrap();
							tx.send(DroneCtl::new(DroneCtlType::StartJob, Some(host), None, None)).unwrap();
						},
						MessageType::QueueJob => {
							// Notification of a new job to be queued.
							let job: Job = bincode::deserialize(msg.message.as_bytes()).unwrap();
							tx.send(DroneCtl::new(DroneCtlType::QueueJob, None, Some(job), None)).unwrap();
						},
						_ => {
							// Unknown message from another drone.
						},
					}
				},
				Err(e) => {
					println!("tcp streaming error: {}", e);
				}
			}
		}

		drop(listener);
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
		.arg(Arg::with_name("address")
			.short("A")
			.long("address")
			.takes_value(true)
			.help("Specify the adress to listen on for inter-drone communications (Default: 0.0.0.0)."))
		.arg(Arg::with_name("config")
			.short("c")
			.long("config")
			.takes_value(true)
			.help("Specify a config file (Default: /etc/swarm/drone.cfg.toml)."))
		.arg(Arg::with_name("port")
			.short("p")
			.long("port")
			.takes_value(true)
			.help("Specify the port to listen on for inter-drone communications (Default: 9079)."))
		.get_matches();

	let mut c = Config::load_or_new(matches.value_of("config").unwrap_or(DEFAULT_CONFIG));
	c.address = matches.value_of("address").unwrap_or(&c.address).to_string();
	c.port = matches.value_of("port").unwrap_or(&c.port).to_string();

	// Start logging process.
	let (log_tx, log_rx) = mpsc::channel::<LogMessage>();
	let mut l = log::Log::init(c.id.clone(), c.log_dir.clone(), c.error_log.clone(), c.system_log.clone());
	let log_handle = thread::spawn(move || {
		l.run(log_rx);
	});

	// Database verification (or creation if needed.)
	let db = db::Database::verify_or_init(c.id.clone(), c.db_dir.clone(), c.db_file.clone(), log_tx.clone());

	// Load additional config info from database.
	// @TODO Should this be offloaded to the drone process?

	let me = Process::myself().unwrap();
	let socket_path = format!("/tmp/swarm_drone_{}.sock", me.pid);
	let listener = UnixListener::bind(socket_path).unwrap();

	// Start drone process.
	let (drone_tx, drone_rx) = mpsc::channel::<DroneCtl>();
	let mut d = drone::Drone::new(db.unwrap(), log_tx.clone());
	let drone_handle = thread::spawn(move || {
		d.start();
		d.run(drone_rx);
	});

	// Start external listener (for messages from other drones).
	// Start a thread to listen on the configured port, and pass messages to the drone process via a drone_tx clone.
	// Similar to the listener that works on a local unix socket. Utilize the "process_command" function.

	let listener_address = c.address.clone();
	let listener_port = c.port.parse::<u32>().unwrap();
	let listener_tx = drone_tx.clone();
	let listener_handle = thread::spawn(move || {
		process_message(listener_address, listener_port, listener_tx);
	});

	// I think spawn a "process message" function into another thread?

	println!("Start drone process v. {:?} (pid = {}).", VERSION, me.pid);
	log_tx.send(LogMessage::new(LogType::SystemLog, format!("Drone process v.{}, id = {}, pid = {} is online.", VERSION, c.id, me.pid))).unwrap();

	// Listen for the local "commands" from the dronectl binary.
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

	listener_handle.join().unwrap();
	drone_handle.join().unwrap();
	log_handle.join().unwrap();
}
