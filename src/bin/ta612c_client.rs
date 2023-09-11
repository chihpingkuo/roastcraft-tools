use std::io::{self, Write, Read};
use std::time::Duration;
use std::{thread, time};

fn main() {
    let port_name = "COM4";
    let baud_rate = 9600;

    let port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(100))
        .open();    

    match port {
        Ok(mut port) => {         

            println!("{} connected",port_name);

            let request: [u8; 5] = [0xAA, 0x55, 0x01, 0x03, 0x03];

            loop {

                match port.write(&request) {
                    Ok(_t) => {
                        println!("send instruction");
                        
                    },
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }

                let mut response: [u8; 13] = [0; 13];
                
                
                match port.read(response.as_mut_slice()) {
                    Ok(t) => {
                        println!("{:02X?}", response);
                        println!(""); 
                        let T1 = u16::from_ne_bytes(response[4..6].try_into().unwrap()) as f32 / 10.0;
                        let T2 = u16::from_ne_bytes(response[6..8].try_into().unwrap()) as f32 / 10.0;
                        let T3 = u16::from_ne_bytes(response[8..10].try_into().unwrap()) as f32 / 10.0;
                        let T4 = u16::from_ne_bytes(response[10..12].try_into().unwrap()) as f32 / 10.0;
                        
                        println!("{}", T1);
                        println!("{}", T2);
                        println!("{}", T3);
                        println!("{}", T4);
                    },
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
                
                thread::sleep(time::Duration::from_secs(5));

            }

        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }
}
