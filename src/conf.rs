use std::path::PathBuf;
 use serde::{Serialize, Deserialize};
use std::io::{Write, Read};
use std::fs::{File};
use home::home_dir;
use std::fs;

const CONF_FILE: &str = ".coolnk.conf";
const DEFAULT_DIR: &str = "coolnk_shortcuts";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conf {
	pub default_dir: PathBuf,
	pub is_default: bool,
}

impl Default for Conf {
	fn default() -> Self {
		let mut path = home_dir().unwrap();
		path.push(DEFAULT_DIR);
		Self {
			default_dir: path,
			is_default: true,
		}
	}
}

fn get_conf_path() -> PathBuf {
	let mut path: PathBuf = home_dir().unwrap();
	path.push(CONF_FILE);
	path.set_extension("json");
	path
}

fn get_default_dir() -> PathBuf {
	let mut path: PathBuf = home_dir().unwrap();
	path.push(DEFAULT_DIR);
	path
}

pub fn save_conf(conf: Conf) {
	let mut file = File::create(get_conf_path()).unwrap();
	let data = serde_json::to_string(&conf).unwrap();
	file.write_all(data.as_bytes()).unwrap();
}

pub fn load_conf() -> Conf {
	match File::open(get_conf_path()) {
		Ok(mut file) => {
			let mut data = vec![];
			let _ = file.read_to_end(&mut data);
			let conf: Conf = serde_json::from_slice(&data).unwrap();
			conf
		},
		Err(_) => {
			let _ = fs::create_dir_all(get_default_dir());
			Conf::default()
		}
	}
}