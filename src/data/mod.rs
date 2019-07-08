use std::fs::{DirEntry, File};
use std::path::{Path, PathBuf};
use std::time::Instant;

use log::*;

use lazy_static::lazy_static;
use serde::Deserialize;
use futures::executor::ThreadPool;
use futures::future::Future;
use gtfs_structures::Gtfs;

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

pub fn load_transit_data(root_dir: &Path, use_cached: bool) -> std::result::Result<Gtfs, Error> {
	let archive_path = root_dir.join("gtfs.zip");

	// TODO check if there is a cached version

	if !use_cached {
		info!("Downloading GTFS archive from TriMet...");
		{
			let mut dest = File::create(&archive_path)?;
			let url = "https://developer.trimet.org/schedule/gtfs.zip";
			let mut response = reqwest::get(url)?;

			std::io::copy(&mut response, &mut dest)?;
		}

//		info!("Extracting files from GTFS archive...");
//		{
//			let archive = File::open(&archive_path)?;
//			let mut archive = zip::ZipArchive::new(archive).unwrap();
//
//			for i in 0..archive.len() {
//				let mut file = archive.by_index(i).unwrap();
//
//				debug!("Extracting {}", file.name());
//
//				let mut outfile = File::create(dir_path.join(file.sanitized_name()))?;
//				std::io::copy(&mut file, &mut outfile)?;
//			}
//		}
	}

//	let trips = futures::executor::block_on(parse_gtfs_async(&dir_path.join("trips.txt")));

	info!("Parsing files...");
	let gtfs = Gtfs::from_zip(archive_path.to_str().unwrap()).expect("Could not read GTFS");

	Ok(gtfs)
}

pub fn load_vehicle_positions(tmp_dir: &Path, since: Instant) -> std::result::Result<DirEntry, Error> {
	unimplemented!()
}