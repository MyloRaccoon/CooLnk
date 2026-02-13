mod cli;
mod conf;

use cli::{Cli, Commands};
use conf::{Conf, load_conf, save_conf, remove_conf};

use mslnk::{ShellLink, MSLinkError};
use clap::Parser;
use std::path::PathBuf;

fn create(conf: Conf, name: String, target: PathBuf, destination: Option<PathBuf>, icon: Option<PathBuf>) -> Result<(), MSLinkError> {
    let mut destination = match destination {
        Some(dir) => dir,
        None => {
            if conf.is_default {
                println!("/!\\ WARNING: you did not set a default directory. You will find your shortcut in {}", conf.default_dir.display());
            }
            conf.default_dir
        },
    };
    destination.push(format!("{}.lnk", name));
    let mut sl = ShellLink::new(&target)?;

    if let Some(icon) = icon {
        sl.set_icon_location(Some(icon.into_os_string().into_string().unwrap()));
    }

    sl.create_lnk(&destination)?;

    println!("Created shortcut {} => {}", target.display(), destination.display());
    Ok(())
}

fn set_conf(directory: PathBuf) {
    let new_conf = Conf {
        default_dir: directory,
        is_default: false,
    };
    save_conf(new_conf);
}

fn main() -> Result<(), MSLinkError> {

    let conf: Conf = load_conf();

    let cli = Cli::parse();

    match cli.command {
        Commands::Create {name, target, destination, icon} => {
            create(conf, name, target, destination, icon)?;
        },
        Commands::SetConf { directory } => { 
            set_conf(directory);
        },
        Commands::SeeConf => println!("{conf}"),
        Commands::ResetConf => remove_conf(),
    }

    Ok(())
}
