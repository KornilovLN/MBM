
//use serde::{Deserialize, Serialize};
//use serde_json;

//use std::io;
//use std::io::prelude::*;

use std::error::Error;

extern crate clap;
use clap::Parser;

extern crate ansi_term;
use ansi_term::Colour;

//extern crate chrono;
//use chrono::{Datelike, Timelike, Utc};

//use serialport::{available_ports, SerialPortType};

mod md_about;
mod md_serial;

use crate::md_about::md_utils;

//=== PATH ===============================================

//const PATH_ABOUT: &str = "../../all/config/about.json";

//=== Секция Argums ======================================

// -c Config_<name>.json   - файл конф | уст флаг default
// -p <RTU | ASCII | TCP>  - ASCII default
// -b <baudrate>           - 230400 default
// -l Log_<name>.log       - name Log_<OS DateTime>
// -d Data_<name>.txt      - name Data_<OS DateTime>
// -mqtt <addr>            - адр брок подписки на публикац
// -topic <topic>          - указ топик для публикации 

/// Name of the protocol
/// baudrate with ttyUSB* or ttyACM*

const MAXCICLES: u32 = 2001;

#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Argums {    
  #[arg(short, long,)]
  pub protocol: String,
    
  #[arg(short, long, default_value_t = 230400)]
  pub baudrate: u32,
}

impl Argums {
  pub fn out(&self) {
    let frm_protocol = format!("{}: {}", 
      Colour::Blue.paint("Protocol: ".to_string()),
      Colour::Yellow.paint(&self.protocol.to_string())); 
    let frm_baudrate = format!("{}: {}",
      Colour::Blue.paint("Baudrate: ".to_string()),
      Colour::Yellow.paint(&self.baudrate.to_string()));                                        
    println!("\t{}", "--- Входные параметры CLI -------");
    println!("\t{}", frm_protocol);
    println!("\t{}", frm_baudrate); 
    println!("\t{}", "---------------------------------");
  }
}

//=== Секция Run =========================================

pub fn run(args: &Argums) -> Result<(), Box<dyn Error>> {  

  //--- Создать структуру для считывания about.json в нее
  let mut about = md_about::
                  StAbout::
                  new("","","","","","","");

  prepare_run(&args); 

  //--- Остальной код ------------------------------------

  start_receive_port();

  //------------------------------------------------------

  edit_about(&mut about);
  save_about(&mut about);

	Ok(())	
}

//========================================================

fn prepare_run(args: &Argums) {
  args.out();  
  //--- Информация о процессоре 
  md_about::md_utils::iron(); 
  //--- Заголовок программы
  md_about::target("mbm",
                   "CLI MODBUS-master [RTU|ASCII|TCP]");
  println!("\t--- Читаем, парсерим выводим about.json -");
  md_about::get_json_from_file();
  println!("\t-----------------------------------------");
  //--- пауза
  md_about::md_utils::waiter(2);
  //--- найти представленные в системе порты
  md_serial::find_ports();
}

//--------------------------------------------------------

const ARR_BAUDRATE: [u32; 16] = [
          4800u32,  7200u32,  9600u32,  14400u32, 
				  19200u32, 28800u32, 38400u32, 57600u32, 
				  76800u32, 115200u32,153600u32,230400u32,
				  357200u32,460800u32,714400u32,921600u32,
      ];
const ARR_PORTNAMES: [&str; 4] = [
          "/dev/ttyUSB0", 
				  "/dev/ttyUSB1", 
				  "/dev/ttyUSB2", 
				  "/dev/ttyUSB3"];

///--- Получить имя последовательного порта
///--- prompt - "\n\tВыбрать порт: "
fn input_port (prompt: &str) -> String {     
  let mut st_port: String;
  let mut port: &str = "";
     
  let mut done = false; 
  while !done {
    st_port = md_utils::read_string(prompt);
    port = st_port.trim();
    if ARR_PORTNAMES.contains(&port) {
      done = true;    
    }    
  }
  port.to_string()    
}

///--- Ввести baudrate
///--- prompt: "\n\tBaudrate: "
fn input_baudrate(prompt: &str) -> u32 {
  let mut done = false; 
  let mut baud:u32 = 0u32;
  while !done {
    baud = md_utils
      ::read_string(prompt)
      .trim()
      .parse::<u32>().unwrap();        
    if ARR_BAUDRATE.contains(&baud) {
      done = true;
    }
  }
  baud
}

///--- Ввести счетчик
///--- prompt: "\n\tCounter cicles: "
fn input_counter(prompt: &str) -> u32 {
  let mut done = false; 
  let mut cnt:u32 = 0u32;
  while !done {
    cnt = md_utils
      ::read_string(prompt)
      .trim()
      .parse::<u32>().unwrap();        
    if cnt > 20 && cnt < MAXCICLES {
      done = true;
    }
  }    
  cnt        
}

fn start_receive_port(){
  //--- ввести имя последовательного порта 
  let port = input_port("\n\tВыбрать порт: ");      

  //--- ввести baudrate
  let baud = input_baudrate("\n\tBaudrate: ");

  //--- и счетчик
  let mut cnt = input_counter("\n\tCounter cicles: "); 

  //--- запустить чтение порта
  md_serial::receive(port, baud, &mut cnt);
}        

//--------------------------------------------------------

fn edit_about(about: &mut md_about::StAbout) {
  println!("\n\t--- Редактируем about.json ------------");
  about.datetime = String::from("28.09.2023 23:46:00");
  about.firstname = String::from("Leon");
  about.secondname = String::from("Nicolaevich");
  println!("\t---------------------------------------"); 
}    

fn save_about(about: &mut md_about::StAbout) {
  println!("\n\t--- Сохр, читаем, парсерим и выводим---");
  md_about::save_json_about(&about);
  md_about::get_json_from_file();
  println!("\t---------------------------------------");    
}

//--------------------------------------------------------


