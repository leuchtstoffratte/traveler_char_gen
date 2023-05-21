mod from_text_file;

use std::fmt;

use crate::character_generation::Career;


#[derive(Debug, Clone)]
pub struct ImportGameInfoError;

impl fmt::Display for ImportGameInfoError{
    fn fmt(&self, f :&mut fmt::Formatter) -> fmt::Result{
        write!(f, "Something terribe happend while parsing game data.")

    }
}



///wildly export half assed import from text file
pub fn import_careers() -> Vec<Career>{
    let raw_data :String = from_text_file::import_my_dummy_text();

    let mut careers : Vec<Career> = Vec::new();    

    for line in raw_data.clone().split("\n"){
        if !line.starts_with("#") && !line.trim().is_empty() { //TODO this empty-line-guard does not work
            match from_text_file::parse_career_line(line){
                Err(why) => println!("something went wrong: {}", why),
                Ok(parsed_career) => {println!("{}", &parsed_career); careers.push(parsed_career)}
            };
    
        }

    }

    careers
}

