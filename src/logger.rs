use std::sync::RwLock;
use std::fs::{OpenOptions, File};
use std::io::Write;
use lazy_static::*;

extern crate chrono;
use chrono::Local;

lazy_static! {
    static ref DEBUG: RwLock<bool> = RwLock::new(false);
    static ref LOG_FILE: RwLock<File> = RwLock::new(
        OpenOptions::new().write(true).truncate(true).create(true).open("debug.log").unwrap()
    );
}

fn timestamp() -> String {
    Local::now().format("[%Y-%m-%d %H:%M:%S] ").to_string()
}

pub fn init(debug_mode: bool) {
    let mut debug = DEBUG.write().unwrap();
    *debug = debug_mode;
}

pub fn log(message: String) {
    //println!("{}", timestamp() + &message);
    debug(message);
}

pub fn debug(message: String) {
    if *DEBUG.read().unwrap() {
        let log_message = timestamp() + &message + "\n";
        let mut file = LOG_FILE.write().unwrap();
        file.write_all(log_message.as_bytes()).unwrap();
    }
}