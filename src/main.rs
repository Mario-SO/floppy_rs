fn floppy_say(message: &str) {
    // Define the floppy disk ASCII art parts
    let floppy_top = r#"
 ____________________________________.
|;;|                              |;;||
|[]|------------------------------|[]||
|;;|                              |;;||
"#;

    let floppy_bot = r#"
|;;|                              |;;||
|;;|______________________________|;;||
|;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;||
|;;;;;;________________________ ;;;;;||
|;;;;;|  ___                   |;;;;;||
|;;;;;| |;;;| Made by          |;;;;;||
|;;;;;| |___| @mariodev        |;;;;;||
\_____|________________________|_____||
 ~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^~~~~~~ 
"#;

    // Define the width of the empty space in the floppy disk
    let screen_width = 28;
    let screen_height = 6;

    // Split the message into lines that fit the screen width
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in message.split_whitespace() {
        if current_line.len() + word.len() + 1 <= screen_width {
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        } else {
            lines.push(current_line);
            current_line = String::from(word);
        }
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }

    // Add empty lines if necessary to fit the floppy disk space
    while lines.len() < screen_height {
        lines.push(String::from(""));
    }

    // Define the content inside the empty space of the floppy disk
    let mut screen_content = String::new();
    for line in lines.iter().take(screen_height) {
        screen_content.push_str(&format!("|;;| {:<28} |;;||\n", line));
    }

    // Print the floppy disk with the message inside the empty space
    println!(
        "{}{}{}",
        floppy_top,
        screen_content,
        floppy_bot.trim_start()
    );
}

fn main() {
    // Ensure your message fits within the empty space or handle wrapping
    let message = "Floppy disks are back. Send messages to friends in a retro way.";
    floppy_say(message);
}
