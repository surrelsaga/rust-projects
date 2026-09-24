use std::{thread, time::Duration}; // to delay time

fn clear_terminal() {
    // \x1B[2J clears the screen
    // \x1B[1;1H moves the cursor to the top-left corner
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);
    
    // Alternatively, using standard escape syntax:
    // print!("\x1B[2J\x1B[1;1H");
}

pub fn pause_and_clear() {
    thread::sleep(Duration::from_secs(2)); // delay 2 seconds before clearing
    clear_terminal();
}
