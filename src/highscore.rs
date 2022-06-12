use std::fs::{self, File};
use std::io::{Write, Error};

pub fn read_highscore() -> u32 {
    let f = fs::read_to_string(".snake-highscore");

    let nothing_works = || {
        match write_highscore(0) {
            Err(_) => 0,
            Ok(_) => 0,
        }
    };

    let content = match f {
        Err(_) => { nothing_works(); return 0; },
        Ok(s) => s,
    };


    let highscore: u32 = match content.parse() {
        Err(_) => { nothing_works(); return 0; },
        Ok(h) => h,
    };

    highscore
}

pub fn write_highscore(score: u32) -> Result<(), Error> {
    let mut file = File::create(".snake-highscore")?;

    write!(file, "{}", score)?;

    Ok(())
}
