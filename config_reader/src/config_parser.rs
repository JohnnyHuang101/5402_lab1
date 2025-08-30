use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;


pub fn config_parser(config){

    //open file to parse the title and players
    match File::open(config){
        Ok(file_handle)=>{

            let read = BufReader::new(file_handle);

            let mut title: <String> = "";
            let mut parts: Vec<(S)>
        }
    }
}