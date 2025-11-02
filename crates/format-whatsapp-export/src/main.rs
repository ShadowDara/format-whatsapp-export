use msg_parser;
use rfd::FileDialog;
use std::fs;
use std::io;
use std::path::PathBuf;
use win_utf8_rs;

// Function to select the Folder where the Export is located
fn selectfolder() -> Option<PathBuf> {
    println!("txt file");

    let mut eingabe = String::new();

    println!("Do you want to use an File Dialog Window to select the File? [y/n]");

    // Lesen der Terminal Eingabe
    io::stdin()
        .read_line(&mut eingabe)
        .expect("Error while reading the Input!");

    if eingabe.trim() == "y" {
        if let Some(folder_path) = FileDialog::new().pick_file() {
            println!("Selected file: {:?}", folder_path);
            return Some(folder_path);
        } else {
            return None;
        }
    } else {
        println!("TODO 1");
        return None;
    }
}

// Funktion to start the Analysing
fn start() -> Result<(), Box<dyn std::error::Error>> {
    // Get the Path
    let maybepath = selectfolder();

    // Return if no folder was selected
    if maybepath == None {
        return Err("No Path Selected!".into());
    }
    let path = maybepath.unwrap();

    // Main txt File
    let content = fs::read_to_string(path)?;
    let messages = msg_parser::parsetxt(content);

    let mut msg_cpunter = 0;

    for _ in messages {
        msg_cpunter += 1;
        // println!("{}", message.message);
    }

    println!("Messages: {}", msg_cpunter);

    let mut eingabe = String::new();

    io::stdin()
        .read_line(&mut eingabe)
        .expect("Error while reading the Input!");

    Ok(())
}

const VERSION: &str = "3.0.0";

fn main() {
    let _ = win_utf8_rs::enable_utf8();

    println!("Format WhatsApp Export V{}", VERSION);
    println!("Menu");

    // Run start
    start().expect("REASON");
}
