mod input;
mod warrior;
use input::Input;
use std::env;
use warrior::Warrior;
fn main() {
    let args: Vec<String> = env::args().collect();
    Input::new(&args);
    let arthur = Warrior::new(String::from("Arthur"),10, 5, 5);
    Warrior::warrior_print(&arthur);
    println!(
        "Name: {},HP: {}, Armor: {}, Attack: {}",
        Warrior::name(&arthur),
        Warrior::hp(&arthur),
        Warrior::armor(&arthur),
        Warrior::attack(&arthur)
    );
}
// https://codezup.com/creating-restful-api-with-rust-guide/