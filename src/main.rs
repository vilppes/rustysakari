mod extractable_functions;
fn game() {
    let winning = String::from("joo\n");
    println!("Pue sakarille villapaita (Joo/En)");
    if extractable_functions::check_win(winning, extractable_functions::handle_user_input()) {
        println!("Hihhihhii kutittaa");
    } else {
        println!("Hävisit pelin");
    }
}
fn main() {
    game();
}
