use std::process;

extern crate clap;
use clap::Parser;

extern crate ansi_term;
use ansi_term::Colour;

use mbm::Argums;

//-----------------------------------------------------------------------------
fn main() {
    let args = Argums::parse();

    if let Err(e) = mbm::run(&args) {
        let frm_err = format!("{}", Colour::Red.paint("Main: Ошибка в приложении".to_string()));
		eprintln!("{}: {}", frm_err, e);		
		process::exit(1);
	}    
}  


