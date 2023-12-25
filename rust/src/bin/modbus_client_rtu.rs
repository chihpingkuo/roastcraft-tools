use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use rmodbus::{client::ModbusRequest, guess_response_frame_len, ModbusProto};
use serialport::SerialPort;

fn main() {
    let timeout = Duration::from_secs(1);

    let port_name = "COM4";
    let baud_rate = 9600;

    let mut stream = serialport::new(port_name, baud_rate)
        .timeout(timeout)
        .open()
        .expect("Failed to open port");

    println!("{} connected", port_name);

    // create request object
    let mut mreq = ModbusRequest::new(1, ModbusProto::Rtu);
    let mut request = Vec::new();

    // get holding registers
    mreq.generate_get_holdings(18176, 1, &mut request).unwrap();
    println!("{:02X?}", request);
    stream.write(&request).unwrap();
    let mut buf = [0u8; 7];
    stream.read_exact(&mut buf).unwrap();
    let mut response = Vec::new();
    response.extend_from_slice(&buf);
    let len = guess_response_frame_len(&buf, ModbusProto::Rtu).unwrap();
    if len > 7 {
        let mut rest = vec![0u8; (len - 7) as usize];
        stream.read_exact(&mut rest).unwrap();
        response.extend(rest);
    }
    println!("{:02X?}", response);
    let mut data = Vec::new();
    // check if frame has no Modbus error inside and parse response bools into data vec
    mreq.parse_u16(&response, &mut data).unwrap();
    for i in 0..data.len() {
        println!("{} {}", i, data[i]);
    }
}
