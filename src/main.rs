mod import_game_info;
mod character_generation;
mod character_sheet;
mod mechanix;

use import_game_info::import_careers;
use character_sheet::CharacterSheet;


fn main() {
    import_careers();

    println!("now roll up ne chara");


    let new_char : CharacterSheet = CharacterSheet::roll_up_new_basic_char();


    println!("Got a \n {}", new_char);


}
