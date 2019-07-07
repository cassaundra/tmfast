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