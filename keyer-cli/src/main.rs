use std::{io::Write, thread, time::Duration};

use log::info;
use serialport;

use keyer_protocol::winkeyer::{AdminCommand, Command, KeyInput};

fn main() -> Result<(), Error> {
    env_logger::init();

    info!("starting up");

    for available_port in serialport::available_ports().unwrap_or_else(|_| Vec::new()) {
        info!(
            "\tavailable port: {} (type: {:?})",
            available_port.port_name, available_port.port_type
        );
    }

    let port_path = std::env::var("KEYER_SERIAL_PORT")?;

    let builder = serialport::new(port_path, 1200);
    dbg!(&builder);

    info!("connecting...");

    let mut serial_port = builder.open_native()?;
    dbg!(&serial_port);

    info!("connected to serial device");
    info!("starting session");

    let open_command = Command::Admin(AdminCommand::OpenHostConnection);
    let cmd: Vec<u8> = open_command.try_into().expect("open command bytes");
    serial_port.write(&cmd).expect("write");

    for _ in 1..=5 {

        let key_command = Command::DoKey(KeyInput::Dah);
        let cmd: Vec<u8> = key_command.try_into().expect("key command bytes");
        serial_port.write(&cmd).expect("write");


        thread::sleep(Duration::from_secs(3));

        let key_command = Command::DoKey(KeyInput::Release);
        let cmd: Vec<u8> = key_command.try_into().expect("key command bytes");
        serial_port.write(&cmd).expect("write");

        thread::sleep(Duration::from_secs(1));

    }

    let close_command = Command::Admin(AdminCommand::CloseHostConnection);
    let cmd: Vec<u8> = close_command.try_into().expect("close command bytes");
    serial_port.write(&cmd).expect("write close");

    info!("closed session");

    Ok(())
}

type Error = Box<dyn std::error::Error>;
