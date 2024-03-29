use std::{fmt::{self, Display}};

use rand::distributions::{Distribution, Uniform};

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
            Err(_) => return Err(MechanixParseError::new("Could not parse target number for {}")),
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
        write!(f, "{} {}+", self.relevant_attribute , self.target_number)
    }

}


#[derive(Debug)]
pub enum Attributes{ //TODO: this is probably a java anchronism
    Strength, 
    Dexterity, 
    Endurance,
    Intelligence,
    Education,
    SocialStatus,
    Nope
}

impl Display for Attributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let name = match self {
            Attributes::Strength => "STR",
            Attributes::Dexterity => "DEX",
            Attributes::Endurance => "END",
            Attributes::Intelligence => "INT",
            Attributes::Education => "EDU",
            Attributes::SocialStatus => "SOC",
            Attributes::Nope => "--"};

        write!(f, "{}", name)

    }

}


#[derive(Debug)]
pub enum Dice {
    D4,
    D6, 
    D8,
    D10,
    D12,
    D20
}

impl Dice{
    pub fn roll(&self) -> u8{
       self.get_distribution().sample(&mut rand::thread_rng())
    }


    pub fn max_of_n(&self, amount : u8)->u8{

        let max_of_two = |a : u8, b:u8| -> u8 { a.max( b) };

        self.something_of_n(amount, max_of_two)

    }

    pub fn sum_of_n(&self, amount : u8)->u8{
        self.something_of_n(amount, |a, b| -> u8 {a+b})
    }


    fn something_of_n<F: Fn(u8,u8)->u8>(&self, amount:u8, aggregator : F ) ->u8{
        let mut result :u8 = 0;
        let mut rng = rand::thread_rng();

        let distrbution : Uniform<u8> = self.get_distribution();

        for _ in 0..amount {
            result = aggregator(result, distrbution.sample(&mut rng));
        }

        result

    }


    fn get_distribution (&self) -> Uniform<u8>{
        //from yields range excludung upper bound
        match self {
            Dice::D4 => Uniform::from(1..5),
            Dice::D6 => Uniform::from(1..7),
            Dice::D8 => Uniform::from(1..9),
            Dice::D10 => Uniform::from(1..11),
            Dice::D12 => Uniform::from(1..13),
            Dice::D20 => Uniform::from(1..21)

        }

    }
}





