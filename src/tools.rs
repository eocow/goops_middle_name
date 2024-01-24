use rand::Rng;
use std::fs::File;
use std::fs::{read_to_string, OpenOptions};
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::process::Command;

// read in from the names text file
pub fn read_in_names(filepath: &String) -> Result<Vec<String>, io::Error> {
    let mut names: Vec<String> = Vec::new();

    for line in read_to_string(filepath).unwrap().lines() {
        names.push(line.to_string())
    }

    Ok(names)
}

// adding a name to the text file
pub fn add_name(name: &String, filepath: &String) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).open(filepath)?;

    writeln!(file, "{}", name)?;

    Ok(())
}

// get a random number based off length of the vector
pub fn random_num(names: &Vec<String>) -> usize {
    let mut rng = rand::thread_rng();

    let len_names = names.len() - 1;
    rng.gen_range(0..=len_names)
}

pub fn auth_user(ip_addr: &String) -> bool {
    // TODO move this to the new toml config
    let ip_addr: &str = "10.4.44.3";

    let output = Command::new("ping")
        .arg("-c")
        .arg("1")
        .arg(ip_addr)
        .output()
        .expect("Failed to execute ping command");

    output.status.success()
}

pub fn push_names(filepath: &String, ip_addr: &String) -> io::Result<()> {
    // Connect to the server
    let mut stream = TcpStream::connect(ip_addr)?;

    // Open the file to send
    let file_path = filepath; // get toml set up for this
    let mut file = File::open(file_path)?;

    // Read the file content into a buffer
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Send the file content to the server
    stream.write_all(&buffer)?;

    Ok(())
}

// pull down func
