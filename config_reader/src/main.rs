
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


fn recite(title: &String, play: &Play){

    let mut speaker = String::new();

    println!("{}", title);

    for l in play{
        match l{
            (line_num,player,text)=>{

                if *player != speaker{
                    speaker = player.clone():
                    println!()
                    println!("{}.",player)
                }
                println!("{}", text);
            }
        }
    }
}

fn main() -> Result<(), u8> {
    
    let mut config = String::new();
    if let Err(err) = parse_args(&mut config_file){
        
        eprintln!("Error: {err}");

        process::exit(BAD_COMMAND_LINE_ARGS);
    }

}
