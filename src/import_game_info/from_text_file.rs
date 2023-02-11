use std::fs::File as File;
use std::io::prelude::*;
use std::path::Path as Path;
use std::str::FromStr;


use crate::character_generation::Career;
use super::ImportGameInfoError;



pub fn import_my_dummy_text()->String{
    let path  = Path::new("../input/Careers.txt");

    let display  = path.display();

    let mut file  = match File::open(&path){
        Err(why) => panic!("Could not open {}:{}", display, why),
        Ok(file) => file
    };

    let mut s = String::new();

    match file.read_to_string(&mut s){
        Err(why) => panic!("Count not read {} : {}", display, why),
        Ok(_) => s

    }

    

}



pub fn parse_career_line(line : &str ) -> Result<Career, ImportGameInfoError>{

    let tokens : Vec<&str> = line.split(";").collect::<Vec<_>>();

    let mut career : Career = blank_career();

    career.main_career = unwarp_string_data( tokens.get(0));
    career.sub_career =unwarp_string_data( tokens.get(1));
    career.succeed =unwarp_string_data( tokens.get(2));
    career.ascend = unwarp_string_data(tokens.get(3));
    career.draft=unwarp_boolean_data( tokens.get(4));
    career.officer_rank_available =unwarp_boolean_data( tokens.get(5));
    career.starting_skills =unwarp_string_data( tokens.get(6));
    career.career_steps=unwarp_string_data( tokens.get(7));
    career.career_skills =unwarp_string_data( tokens.get(8));
    

    return Ok(career);

}


fn blank_career () -> Career{
    return Career { main_career: String::from(""), sub_career: String::from(""), succeed: String::from(""), ascend: String::from(""), draft:false, officer_rank_available: false, starting_skills: String::from(""), career_steps: String::from(""), career_skills: String::from("") }
}



fn unwarp_string_data (token : Option<&&str>) -> String{
    match token {
        None => String::from(""),
        Some(inner) => String::from(*inner)
    }

}

fn unwarp_boolean_data (token : Option<&&str>) -> bool{
    match token {
        None => false,
        Some(inner) => match (*inner).trim() {
            "No" => false,
            "Yes" => true,
            i @ _ => {
                println!("try to parse {} to bool", i);
                bool::from_str(i).unwrap()
            }

        }
        
        

    }

}

