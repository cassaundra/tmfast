use std::fs::{DirEntry, File};
use std::path::{Path, PathBuf};
use std::time::Instant;

use lazy_static::lazy_static;

lazy_static! {
	static ref DATA_DIR: PathBuf = std::env::temp_dir().join("tmfast");
	static ref APP_ID: String = std::env::var("TRIMET_APPID").unwrap_or_default();
}

pub enum Error {
	IO(std::io::Error),
	Network(reqwest::Error),
}

// a 32-bit float is close enough for ~2.4m at the equator (:
// and to save space we'll just be using 32-bit integers too
mod raw {
	use std::time::Instant;

	use serde::Deserialize;
	use chrono::{Utc, DateTime};

	pub struct GTFSData<'a> {
		trips: &'a [Trip<'a>],
		routes: &'a [Route<'a>],
		route_directions: &'a [RouteDirection<'a>],
		stop_times: &'a [StopTime],
		stops: &'a [Stop<'a>],
		transfers: &'a [Transfer],
		shapes: &'a [Shape],
		calendar_dates: &'a [CalendarDate<'a>],
	}

	#[derive(Deserialize)]
	pub struct Trip<'a> {
		route_id: u32,
		service_id: &'a str,
		trip_id: u32,
		direction_id: u8, // might want to use bool or an enum or something
		shape_id: u32,
	}

	#[derive(Deserialize)]
	pub struct Route<'a> {
		route_id: u32,
		route_short_name: Option<u32>,
		route_long_name: &'a str,
	}

	#[derive(Deserialize)]
	pub struct RouteDirection<'a> {
		route_id: u32,
		direction_id: u8, // see earlier comment
		direction_name: &'a str,
	}

	#[derive(Deserialize)]
	pub struct StopTime {
		trip_id: u32,
		arrival_time: DateTime<Utc>,
		departure_time: DateTime<Utc>,
		stop_id: u32,
		stop_sequence: u32,
//		stop_headsign: &'a str,
		shape_dist_traveled: f32,
	}

	#[derive(Deserialize)]
	pub struct Stop<'a> {
		stop_id: u32,
		stop_code: u32,
		stop_name: &'a str,
		stop_desc: &'a str,
		stop_lat: f32,
		stop_lon: f32,
		direction: &'a str,
		position: &'a str,
	}

	#[derive(Deserialize)]
	pub struct Transfer {
		from_stop_id: u32,
		to_stop_id: u32,
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
	pub struct CalendarDate<'a> {
		service_id: &'a str,
		date: u64, // TODO might need a custom parser
	}
}

mod structured {

}

pub fn load_gtfs_data(tmp_dir: &Path) -> std::result::Result<DirEntry, Error> {
	unimplemented!()
}

pub fn load_vehicle_positions(tmp_dir: &Path, since: Instant) -> std::result::Result<DirEntry, Error> {
	unimplemented!()
}