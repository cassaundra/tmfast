use std::fs::{DirEntry, File};
use std::path::{Path, PathBuf};
use std::time::Instant;

use log::*;

use lazy_static::lazy_static;
use serde::Deserialize;

mod raw;
mod structured;

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

pub fn load_transit_data(data_dir: &Path) -> std::result::Result<structured::TransitData, Error> {
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