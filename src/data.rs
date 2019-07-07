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
	use chrono::{NaiveTime, NaiveDate};

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
		pub route_id: u32,
		pub service_id: String,
		pub trip_id: u32,
		pub direction_id: u8, // enum parse would be cool here
		pub shape_id: u32,
	}

	#[derive(Deserialize)]
	pub struct Route {
		pub route_id: u32,
		pub route_short_name: Option<u32>,
		pub route_long_name: String,
	}

	#[derive(Deserialize)]
	pub struct RouteDirection {
		pub route_id: u32,
		pub direction_id: u8, // see earlier comment
		pub direction_name: String,
	}

	#[derive(Deserialize)]
	pub struct StopTime {
		pub trip_id: u32,
		#[serde(with = "time_format")]
		pub arrival_time: NaiveTime,
		#[serde(with = "time_format")]
		pub departure_time: NaiveTime,
		pub stop_id: u32,
		pub stop_sequence: u32,
//		stop_headsign: String,
		pub shape_dist_traveled: f32,
	}

	#[derive(Deserialize)]
	pub struct Stop {
		pub stop_id: u32,
		pub stop_code: u32,
		pub stop_name: String,
		pub stop_desc: String,
		pub stop_lat: f32,
		pub stop_lon: f32,
		pub direction: String,
		pub position: String,
	}

	#[derive(Deserialize)]
	pub struct Transfer {
		pub from_stop_id: u32,
		pub to_stop_id: u32,
	}

	#[derive(Deserialize)]
	pub struct Shape {
		pub shape_id: u32,
		pub shape_pt_lat: f32,
		pub shape_pt_lon: f32,
		pub shape_pt_sequence: u32,
		pub shape_dist_traveled: u32,
	}

	#[derive(Deserialize)]
	pub struct CalendarDate {
		pub service_id: String,
		#[serde(with = "date_format")]
		pub date: NaiveDate,
	}

	mod time_format {
		use serde::{Deserialize, Deserializer};
		use chrono::NaiveTime;

		const FORMAT: &'static str = "%H:%M:%S";

		pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveTime, D::Error>
			where D: Deserializer<'de>,
		{
			let s = String::deserialize(deserializer)?;
			NaiveTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
		}
	}

	mod date_format {
		use serde::{Deserialize, Deserializer};
		use chrono::NaiveDate;

		const FORMAT: &'static str = "%y%m%d";

		pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
			where D: Deserializer<'de>,
		{
			let s = String::deserialize(deserializer)?;
			NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
		}
	}
}

mod structured {
	use serde::{Serialize, Deserialize};

	#[derive(Serialize, Deserialize)]
	pub struct TransitData {
        date: Naive
	}
}

pub fn load_gtfs_data(data_dir: &Path) -> std::result::Result<raw::GTFSData, Error> {
	let archive_path = data_dir.join("gtfs.zip");
	let dir_path = data_dir.join("gtfs");

	// TODO check if downloaded

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
	let gtfs_data = raw::GTFSData {
		trips: parse_gtfs(&dir_path.join("trips.txt")),
		routes: parse_gtfs(&dir_path.join("routes.txt")),
		route_directions: parse_gtfs(&dir_path.join("route_directions.txt")),
		stop_times: parse_gtfs(&dir_path.join("stop_times.txt")),
		stops: parse_gtfs(&dir_path.join("stops.txt")),
		transfers: parse_gtfs(&dir_path.join("transfers.txt")),
		shapes: parse_gtfs(&dir_path.join("shapes.txt")),
		calendar_dates: parse_gtfs(&dir_path.join("calendar_dates.txt")),
	};

	Ok(gtfs_data)
}

fn parse_gtfs<T>(path: &Path) -> Vec<T>
	where T: serde::de::DeserializeOwned {
	info!("Parsing {}", path.display());

	let mut reader = csv::Reader::from_reader(File::open(path).unwrap());

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