mod import_game_info;
mod character_generation;
mod character_sheet;
mod mechanix;

use character_sheet::CharacterSheet;


fn main() {
 
    println!("now roll up ne chara");

    let chara : CharacterSheet  = character_generation::generate_char();

    println!("got a new chara: \n {}", chara);

}
