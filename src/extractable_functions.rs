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
pub fn handle_user_input(prompt: String) -> String {
    println!("{}",prompt);
    let mut answer = String::new();
    match io::stdin().read_line(&mut answer) {
        Ok(_n) => return answer,
        Err(_error) => return String::from("Haistappa paska t. sakari {}"),
    }
}
///prompts for true and false
pub fn states_handler(sakaristate: bool, true_prompt: String, false_prompt: String) {
    if sakaristate {
        println!("{}",true_prompt);
    } else {
        println!("{}",false_prompt);
    }
}
