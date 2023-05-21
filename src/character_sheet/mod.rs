use std::{ fmt::{Display}};

use crate::mechanix::{Attributes, Dice};




#[derive(Debug)]
pub struct CharacterSheet {
    name : String,
    description : String,
    age : u16,
    
    strength : AttributeStat,
    dexterity : AttributeStat,
    endurace : AttributeStat,
    intelligence : AttributeStat,
    education : AttributeStat,
    social_status : AttributeStat,



    skills : Vec<String>, //TODO: replace with vector of skill
}

impl CharacterSheet {

    pub fn roll_up_new_basic_char () -> Self{



        CharacterSheet { name: String::from("Test Testor"), description: String::from(""), age: 18, strength: AttributeStat::roll_once(Attributes::Strength) , dexterity: AttributeStat::roll_once(Attributes::Dexterity), endurace: AttributeStat::roll_once(Attributes::Endurance), intelligence: AttributeStat::roll_once(Attributes::Intelligence), education: AttributeStat::roll_once(Attributes::Education), social_status: AttributeStat::roll_once(Attributes::SocialStatus), skills: vec![String::from("0")] }



    }

}

impl Display for CharacterSheet{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name: {}\n Age: {}\n Description: {}\n \t {}\n \t {}\n \t {}\n \t {}\n \t {}\n \t {}\n", self.name, self.age, self.description, self.strength, self.dexterity, self.endurace, self.intelligence, self.education, self.social_status)

    }    
}



///
/// 
/// 
/// 
/// 
/// 
#[derive(Debug)]
struct AttributeStat {
    attribute : Attributes, // the type of the attribute
    value : u8, // regular value 
    current_value : u8, //including temporary effects like damage
    bonus : i8, // bonus associated with the value
    current_bouns : i8
}


impl AttributeStat {
    fn roll_once (attr : Attributes) -> Self{

        let rand : u8  = Dice::D6.sum_of_n(2);

        let bonus  = map_to_bonus(&rand);

        AttributeStat { attribute: attr, value: rand, current_value: rand, bonus: bonus, current_bouns: bonus }

    }

}

impl Display for AttributeStat{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} ({})", self.attribute, self.value, self.bonus)
    }    
}


fn map_to_bonus (attr_val : &u8) -> i8{
    match *attr_val{
        15 => 3,
        i if i >= 12  => 2,
        i if i >= 9  => 1,
        i if i >= 6 => 0,
        i if i >= 2 => -1,
        1 => -2,
        0 => -3,
        _ => 0 //Probably bad idea
    }
} 