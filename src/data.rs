use std::fs::{DirEntry, File};
use std::path::{Path, PathBuf};
use std::time::Instant;

use log::*;

use lazy_static::lazy_static;
use serde::Deserialize;

lazy_static! {
	static ref DATA_DIR: PathBuf = std::env::temp_dir().join("tmfast");
	static ref APP_ID: String = std::env::var("TRIMET_APPID").unwrap_or_default();
}

pub enum Error {
	Io(std::io::Error),
	Network(reqwest::Error),
}

impl From<reqwest::Error> for Error {
	fn from(error: reqwest::Error) -> Self {
		Error::Network(error)
	}
}

impl From<std::io::Error> for Error {
	fn from(error: std::io::Error) -> Self {
		Error::Io(error)
	}
}

// a 32-bit float is close enough for ~2.4m at the equator (:
// and to save space we'll just be using 32-bit integers too
mod raw {
	use serde::Deserialize;

	#[derive(Default)]
	pub struct GTFSData {
		pub trips: Vec<Trip>,
		pub routes: Vec<Route>,
		pub route_directions: Vec<RouteDirection>,
		pub stop_times: Vec<StopTime>,
		pub stops: Vec<Stop>,
		pub transfers: Vec<Transfer>,
		pub shapes: Vec<Shape>,
		pub calendar_dates: Vec<CalendarDate>,
	}

	#[derive(Deserialize)]
	pub struct Trip {
		route_id: u32,
		service_id: String,
		trip_id: u32,
		direction_id: u8, // enum parse would be cool here
		shape_id: u32,
	}

	#[derive(Deserialize)]
	pub struct Route {
		route_id: u32,
		route_short_name: Option<u32>,
		route_long_name: String,
	}

	#[derive(Deserialize)]
	pub struct RouteDirection {
		route_id: u32,
		direction_id: u8, // see earlier comment
		direction_name: String,
	}

	#[derive(Deserialize)]
	pub struct StopTime {
		trip_id: u32,
//		arrival_time: DateTime<Utc>,
//		departure_time: DateTime<Utc>,
		stop_id: u32,
		stop_sequence: u32,
//		stop_headsign: String,
		shape_dist_traveled: f32,
	}

	#[derive(Deserialize)]
	pub struct Stop {
		stop_id: u32,
		stop_code: u32,
		stop_name: String,
		stop_desc: String,
		stop_lat: f32,
		stop_lon: f32,
		direction: String,
		position: String,
	}

	#[derive(Deserialize)]
	pub struct Transfer {
		pub from_stop_id: u32,
		pub to_stop_id: u32,
	}

	#[derive(Deserialize)]
	pub struct Shape {
		shape_id: u32,
		shape_pt_lat: f32,
		shape_pt_lon: f32,
		shape_pt_sequence: u32,
		shape_dist_traveled: u32,
	}

	#[derive(Deserialize)]
	pub struct CalendarDate {
		service_id: String,
		date: u64, // TODO might need a custom parser
	}
}

pub fn load_gtfs_data(data_dir: &Path) -> std::result::Result<raw::GTFSData, Error> {
	let archive_path = data_dir.join("gtfs.zip");
	let dir_path = data_dir.join("gtfs");

	info!("Downloading GTFS archive from TriMet...");
	{
		let mut dest = File::create(&archive_path)?;
		let url = "https://developer.trimet.org/schedule/gtfs.zip";
		let mut response = reqwest::get(url)?;

		std::io::copy(&mut response, &mut dest)?;
	}

	info!("Extracting files from GTFS archive...");
	{
		let archive = File::open(&archive_path)?;
		let mut archive = zip::ZipArchive::new(archive).unwrap();

		for i in 0..archive.len() {
			let mut file = archive.by_index(i).unwrap();

			debug!("Extracting {}", file.name());

			let mut outfile = File::create(dir_path.join(file.sanitized_name()))?;
			std::io::copy(&mut file, &mut outfile)?;
		}
	}

	info!("Parsing files...");
	/*
	trips: &'a [Trip<'a>],
	routes: &'a [Route<'a>],
	route_directions: &'a [RouteDirection<'a>],
	stop_times: &'a [StopTime],
	stops: &'a [Stop<'a>],
	transfers: &'a [Transfer],
	shapes: &'a [Shape],
	calendar_dates: &'a [CalendarDate<'a>],
	*/

//	gtfs_data.trips = parse_gtfs(&dir_path.join("trips.txt")).as_ref();
//	gtfs_data.trips = parse_gtfs(&dir_path.join("trips.txt")).as_ref();

	let gtfs_data = raw::GTFSData {
		trips: parse_gtfs(&dir_path,"trips.txt"),
		routes: parse_gtfs(&dir_path, "routes.txt"),
		route_directions: parse_gtfs(&dir_path, "route_directions.txt"),
		stop_times: parse_gtfs(&dir_path, "stop_times.txt"),
		stops: parse_gtfs(&dir_path, "stops.txt"),
		transfers: parse_gtfs(&dir_path, "transfers.txt"),
		shapes: parse_gtfs(&dir_path, "shapes.txt"),
		calendar_dates: parse_gtfs(&dir_path, "calendar_dates.txt"),
	};

	Ok(gtfs_data)
}

fn parse_gtfs<T>(path: &Path, filename: &str) -> Vec<T>
	where T: serde::de::DeserializeOwned {
	info!("Parsing {}", filename);

	let mut reader = csv::Reader::from_reader(File::open(path.join(filename)).unwrap());

	let mut values: Vec<T> = Vec::new();

	for entry in reader.deserialize() {
		if let Ok(value) = entry {
			values.push(value);
		}
	}

	values
}

pub fn load_vehicle_positions(tmp_dir: &Path, since: Instant) -> std::result::Result<DirEntry, Error> {
	unimplemented!()
}