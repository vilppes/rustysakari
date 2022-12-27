use std::io; 
fn check_win(winning: String, answer: String) -> bool {
    if answer == winning {
        return true;
    } else {
        return false;
    }
}
fn handle_user_input() -> String {
    let mut answer = String::new();
    match io::stdin().read_line(&mut answer) {
        Ok(_n) => return answer,
        Err(_error) => return String::from("Haistappa paska t. sakari {}"),
    }
}
fn game() {
    let winning = String::from("joo\n");
    println!("Pue sakarille villapaita (Joo/En)");
    if check_win(winning, handle_user_input()) {
        println!("Hihhihhii kutittaa");
    } else {
        println!("Hävisit pelin");
    }
}
fn main() {
    game();
}
