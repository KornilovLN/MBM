extern crate ansi_term;
//use ansi_term::Colour;
//use std::fmt;

use std::io::{self, Write};
use std::time::Duration;

use serialport::{available_ports, SerialPortType};


pub fn find_ports() {
  println!("\n\t-------------------------------------");

  match available_ports() {          
    Ok(ports) => {
      match ports.len() {
        0 => println!("\tNo ports found."),
        1 => println!("\tFound 1 port:"),                
        n => println!("\tFound {} ports:", n),
      };

      for p in ports {
        println!("\t{}", p.port_name);
        match p.port_type {
        
          SerialPortType::UsbPort(info) => {
            println!("\t    Type: USB");
            println!("\t    VID:{:04x} PID:{:04x}",
              info.vid, info.pid);
            println!("\t    Serial Number: {}",
              info.serial_number.as_ref()
                .map_or("", String::as_str)
            );
            println!("\t    Manufacturer:  {}",
              info.manufacturer
              .as_ref()
              .map_or("", String::as_str)
            );
            println!("\t    Product:       {}",
              info.product
              .as_ref()
              .map_or("", String::as_str)
            );
            
            #[cfg(feature = "usbportinfo-interface")]
            println!("\t    Interface:     {}",
              info.interface
              .as_ref()
              .map_or("".to_string(),
                      |x| format!("{:02x}", *x))
            );
          },
          
          SerialPortType::BluetoothPort => {
            println!("\t    Type: Bluetooth");
          },
          
          SerialPortType::PciPort => {
            println!("\t    Type: PCI");
          },
          
          SerialPortType::Unknown => {
            println!("\t    Type: Unknown");
          },
          
        } // match
      } // for

    println!("\t-------------------------------------\n");
    } // Ok
  
    Err(e) => {
      eprintln!("\t    {:?}", e);
      eprintln!("\t    Error listing serial ports");
    } // Err
  } // match
  
} // fn



pub fn receive(port_name:String,
  baud_rate:u32,
  cnt:&mut u32) {

  let port = serialport::new(&port_name, baud_rate)
    .timeout(Duration::from_millis(55))
    .open();

  const MAX_BUF_RCV: usize = 256;	
	
  match port {
    Ok(mut port) => {        	
      let mut serial_buf: Vec<u8> = vec![0; MAX_BUF_RCV];
      println!("Receiving data on {} at {} baud:",
        port_name, &baud_rate);
      loop {
        match port.read(serial_buf.as_mut_slice()) {
          Ok(t) => {
            io::stdout()
              .write_all(&serial_buf[..t]).unwrap();
            *cnt -= 1;
            if *cnt < 1 {
              break;
            }
          },
          Err(ref e) if e.kind() == 
            io::ErrorKind::TimedOut => (),
          Err(e) => eprintln!("{:?}", e),
        }
      }
    },
    Err(e) => {
      eprintln!("Failed to open \"{}\". Error: {}",
        port_name, e);
      ::std::process::exit(1);
    }
  }
}

