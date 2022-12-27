use std::io;
///this function checks if the user input wins
pub fn check_win(winning: String, answer: String) -> bool {
    if answer == winning {
        return true;
    } else {
        return false;
    }
}
///This function handles the user input
pub fn handle_user_input() -> String {
    let mut answer = String::new();
    match io::stdin().read_line(&mut answer) {
        Ok(_n) => return answer,
        Err(_error) => return String::from("Haistappa paska t. sakari {}"),
    }
}

