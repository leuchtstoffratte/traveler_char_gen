use std::{str::FromStr, num::ParseIntError, fmt};

pub mod house_keeping;

use house_keeping::MechanixParseError;



pub struct SkillThrow {
    relevant_skill : String,
    relevant_attribute : Attributes,
    target_number : u8
}

impl SkillThrow{
    fn new ()->Self{
        SkillThrow { relevant_skill: String::new(), relevant_attribute: Attributes::Nope, target_number: 0 }
    }
}



pub struct AttributeThrow {
    relevant_attribute : Attributes,
    target_number : u8
}




impl AttributeThrow {
    pub fn from_str(s : &str) -> Result<AttributeThrow, MechanixParseError>{
        if s.len() < 4 ||s.len() > 5{
            return Err(MechanixParseError::new("Wrong format."));
        }

        let relevant_attribute = match &s[0..2] {
            "str" | "Str" | "STR" => Attributes::Strength,
            "dex" | "Dex" | "DEX" => Attributes::Dexterity,
            "end" | "End" | "END" => Attributes::Endurance,
            "int" | "Int" | "INT" => Attributes::Intelligence,
            "edu" | "Edu" | "EDU" => Attributes::Education,
            "soc" | "Soc" | "SOC" => Attributes::SocialStatus,
            _ => return Err(MechanixParseError::new("Could not parse corresponding attribute for {}."))
        };

        let target_number :u8 = match s[3..].parse::<u8>() {
            Err(ParseIntError) => return Err(MechanixParseError::new("Could not parse target number for {}")),
            Ok(result) => result
        };

        return Ok(AttributeThrow { relevant_attribute, target_number });
    }

    ///remove later if I find a better solution
    pub fn dummy_val () -> AttributeThrow{
        AttributeThrow { relevant_attribute: Attributes::Nope, target_number: 0 }
    }

   
}

impl fmt::Display for AttributeThrow{

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{} {}+", match self.relevant_attribute {
            Attributes::Strength => "STR",
            Attributes::Dexterity => "DEX",
            Attributes::Endurance => "END",
            Attributes::Intelligence => "INT",
            Attributes::Education => "EDU",
            Attributes::SocialStatus => "SOC",
            Attributes::Nope => "--"

        }, self.target_number)

    }

}



pub enum Attributes{ //TODO: this is probably a java anchronism
    Strength, 
    Dexterity, 
    Endurance,
    Intelligence,
    Education,
    SocialStatus,
    Nope
}

