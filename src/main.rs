use dll_syringe::{process::OwnedProcess, Syringe};
// use std::process::Command;
use std::{
    env,
    io::{Read, Write},
    net::TcpListener,
    path::PathBuf,
    vec,
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

    // Causes a threading issue.
    // info!("Launching WoW.exe");
    // let _wow_process = Command::new(r"O:\Warcraft Development\Vanilla Client\WoW.exe")
    //     .spawn()
    //     .expect("Failed to launch WoW.exe");

    info!("Searching for WoW.exe...");
    let target_process: OwnedProcess = OwnedProcess::find_first_by_name("WoW").unwrap();
    info!("Found a WoW.exe process!");

    info!("Creating Syringe");
    let syringe = Syringe::for_process(target_process);

    // Causes bug that crashes WoW and causes access violation errors if a tracing_subscriber is enabled.
    // info!("Creating TcpListener");
    // let listener: TcpListener = TcpListener::bind("127.0.0.1:7331")?;

    info!("Grabbing main.dll");
    let mut current_directory: String = get_current_working_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    current_directory.push_str(r"\target\i686-pc-windows-msvc\debug\main.dll");

    info!("Injecting main.dll");
    // let injected_payload = syringe.inject(current_directory).unwrap();
    match syringe.inject(current_directory) {
        Ok(_) => info!("Successful Injection"),
        Err(ERROR) => error!(%ERROR),
    }

    // let mut buf: Vec<u8> = vec![0u8; 1024];
    // let mut stdout = std::io::stdout();

    // // info!("Incoming Connection");
    // let (mut stream, addr) = listener.accept()?;
    // info!(%addr, "Connection from ElwynnBot: ");
    // while let Ok(n) = stream.read(&mut buf[..]) {
    //     stdout.write_all(&buf[..n])?;
    // }

    info!("All Done!");
    // let _ = syringe.eject(injected_payload);
    Ok(())
}

fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}
