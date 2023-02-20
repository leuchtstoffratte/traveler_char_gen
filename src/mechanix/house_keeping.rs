use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct MechanixParseError{
    message : String
}


impl Display for MechanixParseError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.message.as_str());
        Ok(())
    }



}

impl MechanixParseError{

    pub fn new (new_message : &str)->Self{
       
        MechanixParseError {message : String::from( new_message)} 
    }

}
