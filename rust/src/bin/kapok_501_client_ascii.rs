use std::io::{self, Read, Write};
use std::time::Duration;
use std::{thread, time};

use serialport::SerialPort;

fn calc_lrc(frame: &[u8]) -> u8 {
    let mut lrc: i32 = 0;
    for i in 0..frame.len() {
        lrc -= i32::from(frame[i as usize]);
    }
    lrc as u8
}

fn modbus_ascii(mut port: Box<dyn SerialPort>, address: u8) -> () {
    let pdu: String = format!("{:02X}", address) + // slave address
                     &format!("{:02X}", 3) +       // function (read holding register)
                     &format!("{:02X}", 18176) +   // holding register address
                     &format!("{:04X}", 1); // quantity

    let lrc: u8 = calc_lrc(hex::decode(&pdu).unwrap().as_slice());

    let mut request: Vec<u8> = Vec::new();
    request.push(0x3A); // modbus ascii start ":"
    request.extend_from_slice(pdu.as_bytes());
    request.extend_from_slice(format!("{:02X}", lrc).as_bytes());
    request.push(0x0D); // modbus ascii end   "CR"
    request.push(0x0A); // modbus ascii end   "LF"

    // let request: [u8; 17] = [0x3A, 0x30, 0x31, 0x30, 0x33, 0x34, 0x37, 0x30, 0x30, 0x30,
    //                          0x30, 0x30, 0x31, 0x42, 0x34, 0x0D, 0x0A];

    println!("request  {:02X?}", request);

    match port.write(&request) {
        Ok(_t) => {
            println!("send request");
        }
        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
        Err(e) => eprintln!("{:?}", e),
    }

    let mut response: [u8; 15] = [0; 15];

    match port.read(response.as_mut_slice()) {
        Ok(t) => {
            println!("response {:02X?}", response);
            println!("");

            // TODO: add address check and LRC check

            let temp_hex_str =
                std::str::from_utf8(&response[7..11]).expect("invalid utf-8 sequence");
            let Temp: f32 = u16::from_str_radix(temp_hex_str, 16).unwrap() as f32 / 10.0;

            println!("Temp @ {}, {}", address, Temp);
        }
        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
        Err(e) => eprintln!("{:?}", e),
    }
}

fn main() {
    let port_name = "COM4";
    let baud_rate = 9600;

    let port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(100))
        .open()
        .expect("Failed to open port");

    println!("{} connected", port_name);

    loop {
        modbus_ascii(port.try_clone().unwrap(), 1);
        // modbus_ascii(port.try_clone().unwrap(),2);
        // modbus_ascii(port.try_clone().unwrap(),3);
        thread::sleep(time::Duration::from_secs(5));
    }
}
