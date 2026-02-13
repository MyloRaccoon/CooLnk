use std::path::PathBuf;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "coolnk", about = "CooLnk - easily create shorcut")]
pub struct Cli {
	#[arg(help="Name of the shortcut")]
	pub name: String,
	
	#[arg(help="Target of the shortcut")]
	pub target: PathBuf,

	#[arg(help="Dir where to create the shortcut")]
	pub destination: Option<PathBuf>,

	#[arg(help="Path of the shortcut's icon")]
	pub icon: Option<PathBuf>,
}