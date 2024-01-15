use std::env;

mod tools;
use tools::*;

fn main() {
    // GLOBAL VARS START
    let filepath = "names";
    // GLOBAL VARS END

    let args: Vec<String> = env::args().collect();

    let mut vec_of_names: Vec<String> = Vec::new();

    if args.len() == 1 {
        // user ran with no args
        // read in the text file
        match read_in_names(filepath) {
            Ok(names) => {
                vec_of_names = names;
            }
            Err(e) => eprintln!("Error reading file: {}", e),
        }
        let number = random_num(&vec_of_names); // get the random number based off the len of the vector of names
        println!(
            "Today his name is:\ngoop \"{}\" Richard Stallman",
            vec_of_names[number]
        ) // print out
    } else if args[1] == "add" {
        // user ran with arg "add"
        match add_name(&args[2], filepath) {
            Ok(()) => println!("> name added to the list :3"),
            Err(e) => eprintln!("Error appending to file: {}", e),
        }
    }
}
