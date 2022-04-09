use libusb;

mod dev;
use dev::Dev;

use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

/// Constant path to our database file
const DATABASE_PATH: &str = "/tmp/db_ex"; // CHANGE ME!!!
const THREAT_WAIT: u64 = 2;


fn main() {
    // initialize the internal vector
    let mut database: Vec<Dev> = Vec::new();
    let context = libusb::Context::new().unwrap();
    
    // see if our original history file exists
    let b = Path::new(DATABASE_PATH).exists();
    if b {
        println!("[+] Database found. Loading...");
        // load the stuff from the file
        let f = File::open(DATABASE_PATH).unwrap();
        let rdr = BufReader::new(f);
        let mut tmp: Vec<Dev> = serde_json::from_reader(rdr).unwrap();
        database.append(&mut tmp);
    } else {
        println!("[-] No database found. Generating new profile...");
        // print device info
        for device in context.devices().unwrap().iter() {
            let tmp_dev = Dev::new(device);
            database.push(tmp_dev);
        }
        println!("[+] Profile generated. Saving...");
        let mut f = File::create(DATABASE_PATH).unwrap();
        let out = serde_json::to_string(&database).unwrap();
        write!(f, "{}", out).unwrap();
    }

    let database = database;
    
    println!("[+] Database initialized. Monitoring...");




    // monitor the database
    let mut work_database: Vec<Dev> = Vec::new();
    loop {
        // sleep a bit
        std::thread::sleep(std::time::Duration::from_secs(THREAT_WAIT));

        for device in context.devices().unwrap().iter() {
            let tmp_dev = Dev::new(device);
            work_database.push(tmp_dev);
        }

        println!("Checking threats... {}, {}", database.len(), work_database.len());
        // see if any of the potential threats aren't in the database
        for threat in work_database.iter() {
            let mut found = false;
            for target in database.iter() {
                if threat == target {
                    found = true;
                    break;
                }
            }

            if !found {
                println!("[-] New device detected!");
                println!("{}", threat);
            }

        }

        // see if any of the good devices are missing
        for target in database.iter() {
            let mut found = false;
            for t in work_database.iter() {
                if target == t {
                    found = true;
                    break;
                }
            }

            if !found {
                println!("[-] Device missing!");
                println!("{}", target);
            }
        }
        println!("No threats found...");


        // clear the test database
        work_database.clear();
    }

    
}
