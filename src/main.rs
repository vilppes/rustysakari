use std::io; 
fn check_win(winning: String, answer: String) {
    if answer == winning {
        println!("Hihhihhii kutittaa");
    } else {
        println!("Hävisit pelin");
    }
}
fn handle_user_input() -> String {
    let mut answer = String::new();
    match io::stdin().read_line(&mut answer) {
        Ok(_n) => return answer,
        Err(error) => return String::from("Haistappa paska t. sakari {}"),
    }
}
fn main() {
    let winning = String::from("joo\n");
    println!("Pue sakarille villapaita");
    let mut answer = String::new();
    match io::stdin().read_line(&mut answer) {
        Ok(_n) => {
            check_win(winning, answer);
        },
        Err(error) => println!("Haistappa paska t. sakari {}", error),
    }
}
