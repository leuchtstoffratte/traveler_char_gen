mod Career_selection;

use std::{fmt};

use crate::{mechanix::AttributeThrow, character_sheet::CharacterSheet, import_game_info::import_careers};


pub struct Career {
    pub main_career : String,
    pub sub_career : String, 
    pub succeed : AttributeThrow,
    pub ascend : AttributeThrow,
    pub draft : bool,
    pub officer_rank_available : bool,
    pub starting_skills : String, 
    pub career_steps : String, 
    pub career_skills : String

}


impl fmt::Display for Career {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{} ({}) \n  Qualify: missing_Data\n  Survive period: {}\n  Ascend a rank: {}\n  Can be draftet to this career: {}\n  Can be promoted to officer: {}\n  CareerSteps: {}\n  Starting skills: {}\n  Career skills: {}", self.sub_career, self.main_career,self.succeed, self.ascend, self.draft, self.officer_rank_available, self.career_steps, self.starting_skills, self.career_skills)

    }

}



pub struct Enhancement{
    enhancement_type : EnhancementType,
    enhanced_property : String,
    bonus : u8
}



enum EnhancementType{
    Attribute,
    Skill,
    BoundedSkill,
    Specialization,
    BoundedSpecialization
}


#[derive(Debug, Default)]
enum NextCarreerStep{
    ContinueCurrent,
    BranchToNew,
    GetDrafted,
    FinalizeChar,
    #[default]
    Noop,
}

// impl Default for NextCarreerStep {
    
// }



pub fn generate_char() -> CharacterSheet{

    let mut my_new_character : CharacterSheet = CharacterSheet::roll_up_new_basic_char();

    let mut careers : Vec<Career> = import_careers();

    let mut current_career : Option<Career> = None;

    loop {
        match read_next_step_from_cli() {
                NextCarreerStep::Noop => println!("printing help"),
                NextCarreerStep::BranchToNew => current_career = Career_selection::select_new_carreer(&mut my_new_character, &mut careers),
                NextCarreerStep::GetDrafted => println!("joining up"),
                NextCarreerStep::ContinueCurrent => println!("Continuing"),
                NextCarreerStep::FinalizeChar => {println!("finalizing"); break},

        }

        println!("in da loop \n char looks like {}", &my_new_character);


    }

    my_new_character
}


///Parse user enties
fn read_next_step_from_cli() -> NextCarreerStep{

    let mut line = String::new();

    let mut next_step : Option<NextCarreerStep> = None;

    while next_step.is_none() {

        line = String::new(); //clear buffer

        std::io::stdin().read_line(&mut line).expect("Should have been a valid next step in your carreer.");

        next_step =  match line.trim() {
            "C" => Some(NextCarreerStep::ContinueCurrent),
            "c" => Some(NextCarreerStep::ContinueCurrent),
            "N" => Some(NextCarreerStep::BranchToNew),
            "n" => Some(NextCarreerStep::BranchToNew),
            "F" => Some(NextCarreerStep::FinalizeChar),
            "f" => Some(NextCarreerStep::FinalizeChar),
            "D" => Some(NextCarreerStep::GetDrafted),
            "d" => Some(NextCarreerStep::GetDrafted),
            _ => Some(NextCarreerStep::Noop)
        }

    }

    next_step.unwrap_or_default()

}

