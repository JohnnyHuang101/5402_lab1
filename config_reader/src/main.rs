
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

    if !(args.len() == MIN_ARGS || (args.len() == MAX_ARGS && args[WHINGE_STATUS] == "whinge")) {
        usage(&args[PROGRAM_NAME]);
        return Err(BAD_COMMAND_LINE_ARGS);
    }

    config = &args[CONFIG_FILE].clone();
    
    if args.len() == MAX_ARGS && args[WHINGE_STATUS] == "whinge" {
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

    let mut playName = String::new();

    let mut play = Play::new();

    if let Err(err) = script_gen(&config, &mut playName, &mut play){
        
        eprintln!("Error: {err}");

        process::exit(GENERATION_FAILURE);
    }


    sort(&mut play);
    recite(&playName, &play);

    Ok(())
}
