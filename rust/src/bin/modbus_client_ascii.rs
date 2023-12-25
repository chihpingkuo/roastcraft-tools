use std::io::{Read, Write};
use std::time::Duration;

use rmodbus::{client::ModbusRequest, guess_response_frame_len, ModbusProto};
use rmodbus::{generate_ascii_frame, parse_ascii_frame};

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
    let mut mreq = ModbusRequest::new(1, ModbusProto::Ascii);
    let mut request = Vec::new();

    // get holding registers
    mreq.generate_get_holdings(18176, 1, &mut request).unwrap();

    let mut request_ascii = Vec::new();
    generate_ascii_frame(&request, &mut request_ascii).unwrap();

    stream.write(&request_ascii).unwrap();

    let mut buf = [0u8; 7];
    stream.read_exact(&mut buf).unwrap();
    let mut response_ascii = Vec::new();
    response_ascii.extend_from_slice(&buf);
    let len = guess_response_frame_len(&buf, ModbusProto::Ascii).unwrap();
    if len > 7 {
        let mut rest = vec![0u8; (len - 7) as usize];
        stream.read_exact(&mut rest).unwrap();
        response_ascii.extend(rest);
    }
    println!("  len {:?}", len);
    println!("ascii {:02X?}", response_ascii);

    let mut response = vec![0; (len as usize - 3) / 2];
    parse_ascii_frame(&response_ascii, len as usize, &mut response, 0).unwrap();
    println!("  hex {:02X?}", response);
    let mut data = Vec::new();
    // check if frame has no Modbus error inside and parse response bools into data vec
    mreq.parse_u16(&response, &mut data).unwrap();
    for i in 0..data.len() {
        println!("{} {}", i, data[i]);
    }
}
