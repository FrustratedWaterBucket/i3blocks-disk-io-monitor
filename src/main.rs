use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = lines_from_file("/proc/diskstats").expect("oitat");
    let target_disk = "sdb";

    let filtered_lines =  lines
        .into_iter()
        .filter(|x| x.contains(target_disk))
        .collect::<Vec<String>>();

    let target_line;

    if filtered_lines.len() >= 1 {
        target_line = filtered_lines.first().expect("should exist");
    }
    else {
        panic!("Non existant disk");
    }

    println!("{}", target_line);
}

fn lines_from_file<P>(filename: P) -> Result<Vec<String>, io::Error>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;

    io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<String>, io::Error>>()
}