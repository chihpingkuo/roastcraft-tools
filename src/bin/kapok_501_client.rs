use tokio_modbus::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {


    let tty_path = "COM7";
    let slave = Slave(1);

    let builder = tokio_serial::new(tty_path, 9600);

    let mut ctx = sync::rtu::connect_slave(&builder, slave)?;
    println!("Reading a sensor value");
    let rsp = ctx.read_holding_registers(18176, 1)?;
    println!("Sensor value is: {rsp:?}");

    Ok(())
}