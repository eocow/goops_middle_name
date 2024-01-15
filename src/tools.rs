use rand::Rng;
use std::fs::{read_to_string, OpenOptions};
use std::io::{self, Write};

// read in from the names text file
pub fn read_in_names(filepath: &str) -> Result<Vec<String>, io::Error> {
    let mut names: Vec<String> = Vec::new();

    for line in read_to_string(filepath).unwrap().lines() {
        names.push(line.to_string())
    }

    Ok(names)
}

// adding a name to the text file
pub fn add_name(name: &String, filepath: &str) -> io::Result<()> {
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
