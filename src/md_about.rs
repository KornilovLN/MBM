//--------------------------------------------------------
//--- Модуль About 
//--------------------------------------------------------
//--- Author: Kornilov LN (Starmark)
//--- Github: https://github.com/KornilovLN
//--- e-mail: ln.KornilovStar@gmail.com
//--- date:   11.09.2023 14:45:00
//--------------------------------------------------------
//--- 
//--- 
//--------------------------------------------------------

extern crate datetime;
//use std::{thread, time};

extern crate chrono;
//use chrono::{Datelike, Timelike, Utc};

extern crate ansi_term;
use ansi_term::Colour;
//use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Result;
use std::fs;

pub mod md_utils;

const PATH_ABOUT: &str = "../../all/config/about.json";

#[derive(Deserialize, Serialize, Debug)]
pub struct StAbout {
	pub firstname: String,	
	pub secondname: String,	
	pub mainname: String,	
	pub author: String,		
	pub github: String,		 
	pub e_mail: String,		
	pub datetime: String,	
}

//--- Ассоциированные функции
pub fn read_json_about() -> String { 
  // читаем из about.json в строку res
	let res = fs::read_to_string(PATH_ABOUT.to_string())
	  .expect("Can't read file about.json");
	res
}

pub fn get_json_about(data: &str) -> Result<StAbout> {
  // получаем json из прочитанной строки
	let p: StAbout = serde_json::from_str(&data)?;
	Ok(p)
}

pub fn about_out(p: &Result<StAbout>){
  match p {
    Ok(p) => {
      println!("{}",
        Colour::Yellow.paint(struct_to_string(&p)));																												 
    },   
    Err(e) => panic!("parser-> Err! read-pars data {}", e) 
  }
} 

pub fn save_json_about(about: &StAbout) {
  // Запись отредактированного файла в about.json
  fs::write(PATH_ABOUT,
    serde_json::to_string_pretty(&about)
      .unwrap())
      .expect("Can't write to file");
}

pub fn struct_to_string(st: &StAbout) -> String {
	let mut s = String::with_capacity(256);
			
	s.push_str("\tAuthor:      ");
	s.push_str(&st.author);
	s.push_str("\n\tFirst name:  ");
	s.push_str(&st.firstname);
	s.push_str("\n\tSecond name: ");
	s.push_str(&st.secondname);

	s.push_str("\n\tMain name:   ");
	s.push_str(&st.mainname);
	s.push_str("\n\tGithub:      ");
	s.push_str(&st.github);
	s.push_str("\n\te-mail:      ");
	s.push_str(&st.e_mail);
	s.push_str("\n\tDate Time:   ");
	s.push_str(&st.datetime);

	s
}

pub fn get_json_from_file() {
	let contents = read_json_about();
	let prs = get_json_about(&contents);
	about_out(&prs);
}

pub fn target(prj_name: &str, text: &str) {
	let dttm = md_utils::get_date_time();
	println!("\t--- О программе -------------------------");
	println!("\t{}",Colour::Yellow.paint(prj_name));	
	println!("\t{}",Colour::Yellow.paint(text));	
	println!("\t{}",Colour::Yellow.paint(dttm));
	println!("\t-----------------------------------------");
}

//========================================================

impl StAbout {
  pub fn new(
    firstn: &'static str, 
		secondn: &'static str, 
		mainn: &'static str,
		auth: &'static str, 
		gith: &'static str,
		mail: &'static str,
		dttm: &'static str,
		) -> StAbout {
			  Self{ firstname: firstn.to_string(), 
		   		secondname: secondn.to_string(), 
					mainname: mainn.to_string(),
					author: auth.to_string(), 
					github: gith.to_string(),
					e_mail: mail.to_string(),
					datetime: dttm.to_string(),
			  }  
	}

  #[allow(dead_code)]
	pub fn out(&self) {
    println!("\t--- About.out() -----------------------");
    println!("{}",
      Colour::Green.paint(struct_to_string(&self))); 
    println!("\t-------------------------------------\n");
	}
}




