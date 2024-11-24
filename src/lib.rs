use std::time::SystemTime;

use chrono::{DateTime, Local};
use colored::Colorize;

pub fn log(msg: impl std::fmt::Display) {
    let time = DateTime::<Local>::from(SystemTime::now())
        .format("%m-%d-%Y %H:%M:%S")
        .to_string();
    
    println!("[{}] {} {}", 
        time.white(), 
        "[LOG]:".white(), 
        msg.to_string().white()
    );
}

pub fn warn(msg: impl std::fmt::Display) {
    let time = DateTime::<Local>::from(SystemTime::now())
        .format("%m-%d-%Y %H:%M:%S")
        .to_string();

    println!("[{}] {} {}", 
        time.white(),
        "[WARN]:".truecolor(240, 164, 93),
        msg.to_string().white()
    );
}

pub fn error(msg: impl std::fmt::Display) {
    let time = DateTime::<Local>::from(SystemTime::now())
        .format("%m-%d-%Y %H:%M:%S")
        .to_string();

    println!("[{}] {} {}", 
        time.white(),
        "[ERROR]:".red(),
        msg.to_string().white()
    );
}

pub fn success(msg: impl std::fmt::Display) {
    let time = DateTime::<Local>::from(SystemTime::now())
        .format("%m-%d-%Y %H:%M:%S")
        .to_string();

    println!("[{}] {} {}", 
        time.white(),
        "[SUCCESS]:".green(),
        msg.to_string().white()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logging() {
        let number= 100;

        log("Test logging");
        warn(format!("Test warning: {}", number));
        error("Test error");
        success("Test success");
    }
}
