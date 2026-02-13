mod cli;
mod conf;

use cli::Cli;
use conf::{Conf, load_conf};

use mslnk::{ShellLink, MSLinkError};
use clap::Parser;

fn main() -> Result<(), MSLinkError> {

    let conf: Conf = load_conf();

    let cli = Cli::parse();

    let target = cli.target;

    let mut destination = match cli.destination {
        Some(dir) => dir,
        None => {
            if conf.is_default {
                println!("/!\\ WARNING: you did not set a default directory. You will find your shortcut in {}", conf.default_dir.display());
            }
            conf.default_dir
        },
    };

    destination.push(format!("{}.lnk", cli.name));

    let mut sl = ShellLink::new(target)?;

    if let Some(icon) = cli.icon {
        sl.set_icon_location(Some(icon.into_os_string().into_string().unwrap()));
    }

    sl.create_lnk(&destination)?;

    Ok(())
}
