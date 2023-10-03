
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

//=== PATH ====================================================================

//const PATH_ABOUT: &str = "../../all/config/about.json";

//=== Секция Argums ===========================================================

// -c Config_<name>.json   - загрузит файл конфигурации или установит флажек, чтобы работать по default
// -p <RTU | ASCII | TCP>  - выбор протокола работы программы                (ASCII default)
// -b <baudrate>           - имеет смысл для работы с ком-портом             (19200 default)
// -l Log_<name>.log       - будет писать в лог файл текущие события         (name Log_<OS DateTime>)
// -d Data_<name>.txt      - будет писать данные в файл Data_<name>.txt      (name Data_<OS DateTime>)
// -mqtt <addr>            - указать адрес брокера для подписки на публикации
// -topic <topic>          - указать топик для публикации 

/// Name of the protocol
/// baudrate with ttyUSB* or ttyACM*
#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Argums {    
    #[arg(short, long,)]
    pub protocol: String,
    
    #[arg(short, long, default_value_t = 19200)]
    pub baudrate: u32,
}

impl Argums {
    pub fn out(&self) {//----------------------------------------------------------------------------------------
        let frm_protocol = format!("{}: {}", Colour::Blue.paint("Protocol: ".to_string()),
                                                     Colour::Yellow.paint(&self.protocol.to_string())); 
        let frm_baudrate = format!("{}: {}", Colour::Blue.paint("Baudrate: ".to_string()),
                                                     Colour::Yellow.paint(&self.baudrate.to_string()));                                        
        println!("\t{}", "--- Входные параметры CLI -----------------------------------------------");
        println!("\t{}", frm_protocol);
        println!("\t{}", frm_baudrate); 
        println!("\t{}", "-------------------------------------------------------------------------");
    }
}

//=== Секция Run ==============================================================

pub fn run(args: &Argums) -> Result<(), Box<dyn Error>> {  

    //--- Создать структуру для считывания about.json в нее
    let mut about = md_about::StAbout::new("","","","","","","");

    prepare_run(&args); 

    //--- Остальной код ------------------------------------------------------------------

    start_receive_port();

    //------------------------------------------------------------------------------------

    edit_about(&mut about);
    save_about(&mut about);

	Ok(())	
}

//========================================================================================

fn prepare_run(args: &Argums) {
    args.out();  
    //--- Информация о процессоре 
    md_about::md_utils::iron(); 
    //--- Заголовок программы
    md_about::target("mbm", "CLI приложение MODBUS-master [RTU | ASCII | TCP]");
    println!("\t--- Читаем, парсерим и выводим about.json -------------------------------");
    md_about::get_json_from_file();
    println!("\t-------------------------------------------------------------------------");
    //--- пауза
    md_about::md_utils::waiter(2);
    //--- найти представленные в системе порты
    md_serial::find_ports();
}

//----------------------------------------------------------------------------------------

const ARR_BAUDRATE: [u32; 10] = [9600u32, 14400u32, 19200u32, 28800u32, 38400u32, 57600u32, 76800u32, 115200u32, 153600u32, 230400u32];
const ARR_PORTNAMES: [&str; 4] = ["/dev/ttyUSB0", "/dev/ttyUSB1", "/dev/ttyUSB2", "/dev/ttyUSB3"];
fn start_receive_port(){
    //--- ввести имя последовательного порта, 
    let mut st_port: String = std::string::String::from(""); 
    let mut port: &str = st_port.trim();
     
    let mut done = false; 
    while !done {
        st_port = md_utils::read_string("\n\tВыбрать порт: ");
        port = st_port.trim().clone();
        if ARR_PORTNAMES.contains(&port) {
            done = true;    
        }    
    }    

    //--- ввести baudrate
    done = false; 
    let mut baud:u32 = 0u32;
    while !done {
        baud = md_utils::read_string("\n\tBaudrate: ").trim().parse::<u32>().unwrap();        
        if ARR_BAUDRATE.contains(&baud) {
            done = true;
        }
    }

    //--- и счетчик
    let mut cnt = 0;
    println!();

    //--- запустить чтение порта
    md_serial::receive(port, baud, &mut cnt);

}        

//----------------------------------------------------------------------------------------

fn edit_about(about: &mut md_about::StAbout) {
    println!("\n\t--- Редактируем about.json ----------------------------------------------");
    about.datetime = String::from("28.09.2023 23:46:00");
    about.firstname = String::from("Leon");
    about.secondname = String::from("Nicolaevich");
    println!("\t-------------------------------------------------------------------------"); 
}    

fn save_about(about: &mut md_about::StAbout) {
    println!("\n\t--- Сохраняем, для проверки читаем, парсерим и выводим about.json -----");
    md_about::save_json_about(&about);
    md_about::get_json_from_file();
    println!("\t-------------------------------------------------------------------------");    
}




//----------------------------------------------------------------------------------------


