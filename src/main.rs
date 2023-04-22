use dll_syringe::{process::OwnedProcess, Syringe};
use std::{
    env, path::PathBuf
};
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default tracing subscriber failed");

    info!("Searching for WoW.exe...");
    let target_process: OwnedProcess = OwnedProcess::find_first_by_name("WoW").unwrap();
    info!("Found a WoW.exe process!");

    info!("Creating Syringe");
    let syringe = Syringe::for_process(target_process);

    info!("Grabbing main.dll");
    let mut current_directory: String = get_current_working_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    current_directory.push_str(r"\target\i686-pc-windows-msvc\debug\main.dll");

    info!("Injecting main.dll");

    match syringe.inject(current_directory) {
        Ok(_) => info!("Successful Injection"),
        Err(ERROR) => error!(%ERROR),
    }
    info!("All Done!");
    Ok(())
}

fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}
