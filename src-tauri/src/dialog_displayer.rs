use dialog::DialogBox;

pub fn display_message_box (message: String, action: fn()){
    let choice = dialog::Message::new(message)
                    .title("Vizuara Notification")
                    .show()
                    .expect("Could not display dialog");

    println!("After Dialog");
}