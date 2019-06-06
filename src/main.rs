use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::thread;
use std::time;
use std::time::SystemTime;
use std::env;

struct DiskIOStats {
    written_sectors: u64,
    read_sectors: u64,
    time: SystemTime
}

// 829
// 837

fn main() {

    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        panic!("Must have device name as first argument")
    }

    let device_name = args[1].clone();

    let mut last_stat = get_io_stats(&device_name);

    loop {
        let current_read = get_io_stats(&device_name);
        let time_difference_in_seconds = last_stat.time.elapsed().expect("error measuring elapsed time").as_secs();

        if time_difference_in_seconds > 0 {
            let w_sectors = current_read.written_sectors - last_stat.written_sectors;
            let r_sectors = current_read.read_sectors - last_stat.read_sectors;

            println!("w: {} kB/s r: {} kB/s",
                (w_sectors*512/1024)/time_difference_in_seconds, 
                (r_sectors*512/1024)/time_difference_in_seconds);
        }

        last_stat = current_read;
        thread::sleep(time::Duration::from_secs(1));
    }
}

fn get_io_stats(target_device: &String) -> DiskIOStats {

    let lines = 
        lines_from_file("/proc/diskstats")
        .expect("unable to read /proc/diskstats")
        .into_iter()
        .filter(|x| x.split_whitespace().collect::<Vec<&str>>()[2] == target_device);
    
    // 5r,9w
    let mut all_read_sectors = 0;
    let mut all_written_sectors = 0;

    for line in lines {
        let line_split = line.split_whitespace().collect::<Vec<&str>>();

        all_read_sectors += line_split[5].parse::<u64>().expect("unexpected format on /proc/diskstats");
        all_written_sectors += line_split[9].parse::<u64>().expect("unexpected format on /proc/diskstats");
    }    

    return DiskIOStats {
        written_sectors: all_written_sectors,
        read_sectors: all_read_sectors,
        time:  SystemTime::now()
    }
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
