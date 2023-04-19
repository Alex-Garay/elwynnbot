use dll_syringe::{process::OwnedProcess, Syringe};
use std::process::Command;
use std::{
    env,
    io::{ self, Read, Write },
    net::{ TcpListener },
    path::PathBuf,
    vec,
};
use tracing::{ info, Level };
use tracing_subscriber::FmtSubscriber;

// BUILD: cargo build --target=i686-pc-windows-msvc
// RUN: .\target\i686-pc-windows-msvc\debug\elenarun.exe
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default tracing subscriber failed");

    info!("Launching WoW.exe");
    let _wow_process = Command::new(r"O:\Warcraft Development\Vanilla Client\WoW.exe")
        .spawn()
        .expect("Failed to launch WoW.exe");

    info!("Searching for WoW.exe...");
    let target_process: OwnedProcess = OwnedProcess::find_first_by_name("WoW").unwrap();
    info!("Found a WoW.exe process!");

    info!("Creating Syringe");
    let syringe = Syringe::for_process(target_process);

    info!("Creating TcpListener");
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7331")?;

    info!("Grabbing main.dll");
    let mut current_directory: String = String::from(
        get_current_working_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap(),
    );

    current_directory.push_str(r"\target\i686-pc-windows-msvc\debug\main.dll");

    info!("Injecting main.dll");
    let injected_payload = syringe.inject(current_directory).unwrap();

    let mut buf: Vec<u8> = vec![0u8; 1024];
    let mut stdout = std::io::stdout();

    let (mut stream, addr) = listener.accept()?;
    info!(%addr, "Connection from ElwynnBot: ");

    while let Ok(n) = stream.read(&mut buf[..]) {
        stdout.write_all(&buf[..n])?;
    }

    info!("All Done!");
    let _ = syringe.eject(injected_payload);
    Ok(())
}

fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}
