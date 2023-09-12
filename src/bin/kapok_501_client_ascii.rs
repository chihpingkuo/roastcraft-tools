
use std::io::{self, Write, Read};
use std::time::Duration;
use std::{thread, time};

fn calc_lrc(frame: &[u8]) -> u8 {
    let mut lrc: i32 = 0;
    for i in 0..frame.len() {
        lrc -= i32::from(frame[i as usize]);
    }
    lrc as u8
}

fn main() {
    let port_name = "COM7";
    let baud_rate = 9600;

    let port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(100))
        .open();    

    match port {
        Ok(mut port) => {         

            println!("{} connected",port_name);

            let pdu: String = format!("{:02X}", 1) +     // slave address
                             &format!("{:02X}", 3) +     // function (read holding register)
                             &format!("{:02X}", 18176) + // holding register address
                             &format!("{:04X}", 1);      // quantity

            let lrc: u8 = calc_lrc(hex::decode(&pdu).unwrap().as_slice());

            let mut request: Vec<u8> = Vec::new();
            request.push(0x3A); // modbus ascii start ":"
            request.extend_from_slice(pdu.as_bytes());
            request.extend_from_slice(format!("{:02X}", lrc).as_bytes());
            request.push(0x0D); // modbus ascii end   "CR"
            request.push(0x0A); // modbus ascii end   "LF"           

            // let request: [u8; 17] = [0x3A, 0x30, 0x31, 0x30, 0x33, 0x34, 0x37, 0x30, 0x30, 0x30, 
            //                          0x30, 0x30, 0x31, 0x42, 0x34, 0x0D, 0x0A];

            println!("request {:02X?}", request);

            // loop {

                match port.write(&request) {
                    Ok(_t) => {
                        println!("send instruction");
                        
                    },
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }

                let mut response: [u8; 15] = [0; 15];
                
                
                match port.read(response.as_mut_slice()) {
                    Ok(t) => {
                        println!("response {:02X?}", response);
                        println!(""); 
                        
                    },
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
                
                // thread::sleep(time::Duration::from_secs(5));

            // }

        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }
}
