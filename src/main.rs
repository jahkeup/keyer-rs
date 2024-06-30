use std::io::Write;

use log::info;
use serialport;

use winkeyer::winkeyer::{AdminCommand, Command, KeyInput};

fn main() -> Result<(), Error> {
    env_logger::init();

    info!("starting up");

    for available_port in serialport::available_ports().unwrap_or_else(|_| Vec::new()) {
        info!(
            "\tavailable port: {} (type: {:?})",
            available_port.port_name, available_port.port_type
        );
    }

    let port_path = std::env::var("WINKEYER_SERIAL_PORT")?;

    let builder = serialport::new(port_path, 1200);
    dbg!(&builder);

    let mut serial_port = builder.open_native()?;
    dbg!(&serial_port);

    let open_command = Command::Admin(AdminCommand::OpenHostConnection);
    let cmd: Vec<u8> = open_command.try_into().expect("open command bytes");
    serial_port.write(&cmd).expect("write");

    let key_command = Command::DoKey(KeyInput::Dah);
    let cmd: Vec<u8> = key_command.try_into().expect("key command bytes");
    serial_port.write(&cmd).expect("write");

    Ok(())
}

type Error = Box<dyn std::error::Error>;
