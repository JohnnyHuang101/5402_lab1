
use std::env;
use std::process::ExitCode;
// add atomic stuff


fn usage(program_name: &String) {
    println!("usage: {} <configuration_file_name> [whinge]", program_name);
}


fn parse_args(config: &mut String) -> Result<(), u8> {

    let mut args: Vec<String> = Vec::new();

    // push each argument string into the vector
    for arg in env::args() {
        args.push(arg);
    }

    if !(args.len() == 2 || (args.len() == 3 && args[2] == "whinge")) {
        usage(&args[0]);
        return Err(BAD_COMMAND_LINE_ARGS);
    }

    config = &args[1].clone();
    
    if args.len() == 3 && args[2] == "whinge" {
        WHINGE.store(true, Ordering::SeqCst);
    }

     Ok(())
}



fn main() -> Result<(), u8> {
    
      let config = &args[1]
    println!("Processing config file: {}", config_file);
    // got the config.txt next going to parse it thorugh another module func
    let name = 



    println!("Hello, world!");
}
