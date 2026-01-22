use serde_json::Value;


// println!("indecidefunc {}",payload);
pub fn decide(action: &str, payload: Value) {
    match action {

        "sendMessagePrivate" => {
            println!("Logic for private message here...");
        },
        "login" => {
            println!("Logic for login here...");
        },
        _ => {
            println!("Unknown action: {}", action);
        }
    }
}