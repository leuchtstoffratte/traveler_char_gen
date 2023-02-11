pub struct SkillThrow {
    relevant_skill : str,
    relevant_attribute : Attributes,
    target_number : u8
}

pub struct AttributeThrow {
    relevant_attribute : Attributes,
    target_number : u8
}


pub enum Attributes{
    Strength, 
    Dexterity, 
    Endurance,
    Intelligence,
    Education,
    SocialStatus
}


