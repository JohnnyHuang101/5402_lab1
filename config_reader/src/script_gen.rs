use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

type PlayConfig = Vec<(String, String)>; // name of char, name of text file containing the lines that the character will deliver

const TITLE_IND: usize = 0;             // Index of the line giving the title of the play
const FIRST_CHAR_IND: usize = 1; // Index of the first line containing character info

const CHAR_NAME_IND: usize = 0; // Index of the character's name in a line
const FILE_NAME_TOKEN_IND: usize = 1;      // Index of the file containing the character's lines
const EXPECTED_TOKENS: usize = 2;       // Expected number of tokens in a character line



fn add_script_line(play: &mut Play, line: &String, part_name: &String) {
    if line.len() > 0 {
        // Attempt to split the line into a first token and the rest
        if let Some((first_token, rest_of_line)) = line.split_once(char::is_whitespace) {
            let rest_trimmed = rest_of_line.trim(); // split into first and rest

            // Try parsing the first token as a usize
            match first_token.parse::<usize>() {

                Ok(line_number) => {
                    play.push((line_number, part_name.clone(), rest_trimmed.to_string()));
                }
                Err(e) => {
                    // If WARNINGS_ENABLED is true, print a warning
                    if WHINGE.load(Ordering::SeqCst) {
                        println!(
                            "Warning: token '{}' does not represent a valid usize value.",
                            first_token
                        );
                    }
                }
            }
        }
    }
}


fn grab_trimmed_file_lines(file_name: &String, lines: &mut Vec<String>) -> Result<(), u8> {
    // Try to open the file
    let file = match File::open(file_name) {
        Ok(f) => f,
        Err(e) => {
            println!("Error: could not open file '{}': {}", file_name, e);
            return Err(GENERATION_FAILURE);
        }
    };

    let mut reader = BufReader::new(file);
    let mut buffer = String::new();

    loop {
        buffer.clear(); // reuse the same buffer each time

        // Try to read a line
        let bytes_read = match reader.read_line(&mut buffer) {
            Ok(n) => n,
            Err(e) => {
                println!("Error: failed to read from '{}': {}", file_name, e);
                return Err(GENERATION_FAILURE);
            }
        };

        if bytes_read == 0 {
            // End of file has been reached so we are done and return
            return Ok(());
        }

        //ok theres more stuff so we puish to lines
        let trimmed: &str = buffer.trim();
        lines.push(trimmed.to_string());
    }
}


fn process_config(play: &mut Play, config: &PlayConfig) -> Result<(), u8> {
    
    for tuple in config { // name of char, name of text file containing the lines that the character will deliver

        match tuple {
            (part_name, part_file) => {
                let mut lines: Vec<String> = Vec::new(); // all the lines for that partname

                if let Err(e) = grab_trimmed_file_lines(part_file, &mut lines) {
                    println!("Error: failed to read part name '{}'", part_name);
                    return Err(GENERATION_FAILURE); 
                }

                // Add each line to the Play
                for line in &lines {
                    add_script_line(play, line, part_name);
                }
            }
        }
    }

    Ok(())
}




// In your script generation code file write an add_config function that takes a shared (immutable) reference to a String for a line from a configuration file, 
// and a mutable reference to a PlayConfig variable. The function should declare a vector of shared references to text strings (of type Vec<&str>) 
// that is initialized by calling the collect method on the result of calling the split_whitespace method on the reference to the string that was passed 
// into the function (after which the vector will contain references to all the whitespace delimited tokens that were in the string).
// If fewer or more than two tokens were obtained, then if calling the load method with Ordering::SeqCst on the static AtomicBool variable returns true,
// the function should print out an appropriate warning message to complain about that. If at least two tokens were obtained the function should then push a tuple consisting of strings for the part name and the part file name (at positions 0 and 1 respectively, in the vector) into the PlayConfig variable.

fn add_config(a_line: &String, a_play_config: &mut PlayConfig) {
    //TODO
}
