#![feature(async_await)]

mod data;

fn main() {
	env_logger::init();
	let data = data::load_transit_data(std::env::temp_dir().as_path(), true).unwrap_or_default();
}
