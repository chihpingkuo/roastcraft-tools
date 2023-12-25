use tokio_modbus::prelude::*;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tty_path = "COM4";
    let slave = Slave(1);

    let builder = tokio_serial::new(tty_path, 9600);
    let port = tokio_serial::SerialStream::open(&builder).unwrap();

    let mut ctx = rtu::attach_slave(port, slave);
    println!("Reading a sensor value");
    let rsp = ctx.read_holding_registers(18176, 1).await?;
    println!("Sensor value is: {rsp:?}");

    Ok(())
}
