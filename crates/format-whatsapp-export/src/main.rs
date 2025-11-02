use msg_parser;
use msg_to_json;

use dirs_next::download_dir;
use rfd::FileDialog;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use win_utf8_rs;

// Function to select the Folder where the Export is located
fn selectfolder() -> Option<PathBuf> {
    let mut eingabe = String::new();

    println!("Do you want to use an File Dialog Window to select the txt Chat File? [y/n]:");

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

fn write_json_to_file(filename: &str, json_str: &str) -> std::io::Result<()> {
    let mut file = fs::File::create(filename)?; // Datei erstellen / überschreiben
    file.write_all(json_str.as_bytes())?; // JSON-String reinschreiben
    Ok(())
}

fn download_path(filename: &str) -> PathBuf {
    let downloads = download_dir().expect("Download-Ordner konnte nicht ermittelt werden");
    downloads.join(filename)
}

fn filename_without_extension(path: &PathBuf) -> Option<String> {
    path.file_stem() // gibt Option<&OsStr>
        .and_then(|os_str| os_str.to_str()) // Option<&str>
        .map(|s| s.to_string()) // Option<String>
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
    let path2 = path.clone();

    // Main txt File
    let content = fs::read_to_string(path)?;
    let messages = msg_parser::parsetxt(content);

    // let mut msg_cpunter = 0;

    // for _ in messages {
    //     msg_cpunter += 1;
    //     // println!("{}", message.message);
    // }

    // println!("Messages: {}", msg_cpunter);

    // Ask what todo with the messages
    println!("\nWhat do you want todo with the Messages?");
    println!("  [1] Export as JSON to the Terminal");
    println!("  [2] Export as JSON to a File");

    let mut eingabe = String::new();

    io::stdin()
        .read_line(&mut eingabe)
        .expect("Error while reading the Input!");

    // Print Messages to the Terminal as JSON
    if eingabe.trim() == "1" {
        let jsondata = msg_to_json::messages_to_json(&messages);
        println!("{}", jsondata);
    }
    // Save the Messages in a File as JSON
    else if eingabe.trim() == "2" {
        let jsondata = msg_to_json::messages_to_json(&messages);
        if let Some(name) = filename_without_extension(&path2) {
            
        } else {
            return Err("Could not slice the filename".into());
        }
        let path = download_path("aktuelles date hier mit zeit" + name +".json");
        let _ = write_json_to_file(&path.display().to_string(), &jsondata);
    }
    // Except Condition
    else {
        println!("Wrong Input!");
    }
    println!("Action Finished!");

    // Returning
    Ok(())
}

const VERSION: &str = "3.0.0";

fn main() {
    let _ = win_utf8_rs::enable_utf8();

    println!("Format WhatsApp Export V{} in Rust", VERSION);
    println!("\n=== Menu ===");

    // Run start
    start().expect("REASON");

    let mut eingabe = String::new();

    io::stdin()
        .read_line(&mut eingabe)
        .expect("Error while reading the Input!");
}
