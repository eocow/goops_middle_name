use dirs::home_dir;
use std::env;

mod tools;
use tools::*;

mod config;
use config::AppConfig;

fn main() {
    let mut ip_addr: String = String::new();
    let mut path_to_names: String = String::new();

    let mut filepath_raw_temp = home_dir().unwrap_or_default();
    filepath_raw_temp.push(".config/goop/config.toml");
    let filepath_to_config = filepath_raw_temp.to_str().unwrap();

    if let Ok(config) = AppConfig::load_from_file(filepath_to_config) {
        // Now you can use the config throughout your program
        ip_addr = config.ip_addr;
        path_to_names = config.path_to_names;
    } else {
        eprintln!("Error loading configuration");
    }

    // GLOBAL VARS START
    let mut filepath_raw = home_dir().unwrap_or_default();
    filepath_raw.push(path_to_names);

    let filepath = filepath_raw.to_string_lossy().to_string();
    //TODO make a toml or something file in new config dir
    // GLOBAL VARS END

    let args: Vec<String> = env::args().collect();

    let mut vec_of_names: Vec<String> = Vec::new();

    if args.len() == 1 {
        // user ran with no args
        // read in the text file
        match read_in_names(&filepath) {
            Ok(names) => {
                vec_of_names = names;
            }
            Err(e) => eprintln!("Error reading file: {}", e),
        }
        let number = random_num(&vec_of_names); // get the random number based off the len of the vector of names
        println!(
            "> Today his name is :3\n> goop \"{}\" Richard Stallman",
            vec_of_names[number]
        ) // print out
    } else if args[1] == "add" {
        // user ran with arg "add"
        match add_name(&args[2], &filepath) {
            Ok(()) => println!("> name added to the list :3"),
            Err(e) => eprintln!("Error appending to file: {}", e),
        }
    } else if args[1] == "push" {
        match push_names(&filepath, &ip_addr) {
            Ok(()) => println!("> names pushed to the server :3"),
            Err(e) => eprintln!("Error pushing to server: {}", e),
        }
    } else if args[1] == "pull" {
        // code here
    }
}

// ip 10.4.44.3
