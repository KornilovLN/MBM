//-----------------------------------------------------------------------------
//--- Модуль Utils
//-----------------------------------------------------------------------------
//--- Author: Kornilov LN (Starmark)
//--- Github: https://github.com/KornilovLN
//--- e-mail: ln.KornilovStar@gmail.com
//--- date:   28.09.2023 01:45:00
//-----------------------------------------------------------------------------
//--- 
//--- 
//-----------------------------------------------------------------------------
use std::fs::File;
//use std::fs;

use std::io;
//use std::io::prelude::*;
use std::io::Write;

extern crate num_cpus;

extern crate datetime;
use std::{thread, time};

extern crate chrono;
use chrono::{Datelike, Timelike, Utc};

//use std::fmt;
extern crate ansi_term;
use ansi_term::Colour;

//-----------------------------------------------------------------------------

/// --- Получить информацию о железе
pub fn get_num_cpus() -> usize {
	num_cpus::get()
}

pub fn iron() {
    let frm_num_cpus = format!("{}: {}", 
        Colour::Green.paint("Количество ядер процессора: ".to_string()),
        Colour::Yellow.paint(&get_num_cpus().to_string()));                                              
    println!("\t{}", "--- Информация о железе --------------------------------------------------");
    println!("\t{}", frm_num_cpus);
    println!("\t{}", "-------------------------------------------------------------------------\n");
}

/// --- Получить текущий timestamp
pub fn get_timestamp() -> i64 {
	let dt = Utc::now();
	let timestamp: i64 = dt.timestamp_micros();	
	timestamp		
}

/// --- Взять текущее значение Utc и возвратить дату и время
/// --- Применять примерно так:
/// --- println!("\t{}",Colour::Yellow.paint(dttm));
pub fn get_date_time() -> String {
	let now = Utc::now();
	let (is_common_era, year) = now.year_ce();	
	let (is_pm, hour) = now.hour12();

	format!(
        "{}-{:02}-{:02} {:?} ({})  {:02}:{:02}:{:02} {}",
		year,
        now.month(),
        now.day(),
        now.weekday(),
        if is_common_era { "CE" } else { "BCE" },
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" }
    )
}

/// --- Задержка
pub fn waiter(pause: u64) {
	thread::sleep(time::Duration::from_secs(pause));
}

/// --- Запись в файл
pub fn write_out(f: String, st: &str) -> io::Result<()> {
	let mut out = File::create(f)?;
	write!(out,"{}",st)?;
	Ok(())
} 

/// --- Ввод с терминала с выводом подсказки commentn
pub fn read_string(comment:&str) -> String {
    print!("{}", comment);
    let _ = io::stdout().flush();

    let mut string: String = String::new();

    io::stdin().read_line(&mut string)
        .ok()
        .expect("Error read line!");

    return string;
}








