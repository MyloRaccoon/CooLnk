use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "coolnk", about = "CooLnk - easily create shorcut")]
pub struct Cli {
	#[command(subcommand)]
	pub command: Commands,
}


#[derive(Debug, Subcommand)]
pub enum Commands {

	#[command(about="Create a shortcut")]
	Create {
		#[arg(help="Name of the shortcut")]
		name: String,
		
		#[arg(help="Target of the shortcut")]
		target: PathBuf,

		#[arg(help="Dir where to create the shortcut")]
		destination: Option<PathBuf>,

		#[arg(help="Path of the shortcut's icon")]
		icon: Option<PathBuf>,
	},

	#[command(about="Set a configuration value")]
	SetConf {
		#[arg(help="default destination directory for shorcuts")]
		directory: PathBuf
	},

	SeeConf,

	ResetConf,
}