mod cli;

use std::path::Path;
use mslnk::{ShellLink, MSLinkError};
use cli::Cli;
use clap::Parser;

fn main() -> Result<(), MSLinkError> {

    const DIR: &str = "C:\\Users\\Mylo\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\";

    let cli = Cli::parse();
    let target = cli.target;
    let path_str = format!("{}{}.lnk", DIR, cli.name);
    let path = Path::new(&path_str);

    let mut sl = ShellLink::new(target)?;

    if let Some(icon) = cli.icon {
        sl.set_icon_location(Some(icon.into_os_string().into_string().unwrap()));
    }

    sl.create_lnk(&path)?;

    Ok(())
}
