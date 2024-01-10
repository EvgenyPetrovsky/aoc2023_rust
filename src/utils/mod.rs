use chrono::{TimeZone, Utc};
use curl::easy::Easy;
use std::fs::{self, File};
use std::io::{stdout, Write};
use std::path::Path;

pub fn download_input(year: i32, day: u32) -> Result<(), String> {
    // implementation of https://www.reddit.com/r/adventofcode/comments/a2vonl/how_to_download_inputs_with_a_script/
    let requested_date = Utc.with_ymd_and_hms(year, 12, day, 5, 0, 0).unwrap();
    let this_date = Utc::now();
    let file = format!("./input/day_{:0>2}.txt", day);
    if year < 2015 {
        Err(format!("Year is too low! Advent of Code starts from 2015. Year you provided is {year}. Change year and try again"))
    } else if requested_date > this_date {
        Err(format!(
            "Date is too high! Requested date has not come yet. Date you requested is {}",
            requested_date.format("%Y-%b-%d")
        ))
    } else if Path::new(&file).exists() {
        println!("File has already been downloaded earlier. Content:");
        stdout().write_all(&fs::read(&file).unwrap()).unwrap();
        // add one empty line
        stdout().write(b"\n").unwrap();
        Ok(())
    } else {
        let mut f = File::create(&file).unwrap();
        //let mut f = File::open(&file).unwrap();

        let key = fs::read_to_string("./secrets/SESSION")
            .expect("Secret file 'SESSION' with cookie session id is missing");
        println!("download file using cookie session={key}");
        let url = format!("https://adventofcode.com/{year}/day/{day}/input");
        let mut handle = Easy::new();
        handle.url(&url).unwrap();
        handle.cookie(format!("session={key}").as_str()).unwrap();
        handle
            .write_function(move |data| {
                f.write_all(data).unwrap();
                //std::fs::write(&file, data).unwrap();
                Ok(data.len())
            })
            .unwrap();

        handle
            .perform()
            .map(|_| {
                println!("File has been downloaded. Content:");
                stdout().write_all(&fs::read(&file).unwrap()).unwrap();
            })
            .map_err(|e| String::from(e.description()))
    }
}
