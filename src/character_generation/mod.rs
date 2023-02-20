use std::fmt;

use crate::mechanix::AttributeThrow;


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

