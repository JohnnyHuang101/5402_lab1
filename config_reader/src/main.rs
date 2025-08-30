
use std::env;
use std::process::ExitCode;
// add atomic stuff



fn main() -> Result<(), u8> {
    let args: Vec<String> = env::args().collect();
    
    //helpful message for wrong usage
    if args.len()<2{
        println!("Usage: {} <config_file> || Only takes 1 ")
        return return Err(BAD_ARGS);
    }

    let config = &args[1]
    println!("Processing config file: {}", config_file);
    // got the config.txt next going to parse it thorugh another module func
    let name = 



    println!("Hello, world!");
    return Ok(())
}
