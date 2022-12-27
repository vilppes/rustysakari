mod extractable_functions;
fn game() {
    extractable_functions::states_handler(
        extractable_functions::check_win(String::from("joo\n"), 
        extractable_functions::handle_user_input(String::from("Pue sakarille villapaita (joo/ei)"))), 
        String::from("Hihhihhii kutittaa"), 
        String::from("Hävisit pelin")
    )
}
fn main() {
    game();
}
