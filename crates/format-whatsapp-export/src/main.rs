use msg_parser;
use rfd::FileDialog;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use win_utf8_rs;

// Function to select the Folder where the Export is located
fn selectfolder() -> Option<PathBuf> {
    println!("Select the Export Folder");

    let mut eingabe = String::new();

    println!("Do you want to use an Folder Dialog Window to select the Folder? [y/n]");

    // Lesen der Terminal Eingabe
    io::stdin()
        .read_line(&mut eingabe)
        .expect("Fehler beim Lesen der Eingabe");

    if eingabe == "y" {
        if let Some(folder_path) = FileDialog::new().pick_folder() {
            println!("Selected folder: {:?}", folder_path);
            return Some(folder_path);
        } else {
            return None;
        }
    } else {
        println!("TODO 1");
        return None;
    }
}

// Funktion to search the txt File
fn searchtxt(path: PathBuf) -> io::Result<PathBuf> {
    // Alle .txt-Dateien sammeln
    let mut txt_files = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("txt") {
            txt_files.push(path);
        }
    }

    // Wenn keine Dateien gefunden wurden
    if txt_files.is_empty() {
        println!("Keine .txt-Dateien gefunden!");
        std::process::exit(0);
    }

    // Alle Dateien mit Nummer anzeigen
    println!("Gefundene .txt-Dateien:");
    for (i, file) in txt_files.iter().enumerate() {
        println!("{}: {}", i + 1, file.display());
    }

    // Benutzereingabe lesen
    print!("\nWelche Datei möchtest du auswählen? (Zahl eingeben): ");
    io::stdout().flush()?; // wichtig, damit prompt sofort erscheint

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let index: usize = input.trim().parse().unwrap_or(0);

    // Index prüfen und Pfad zurückgeben
    if index == 0 || index > txt_files.len() {
        println!("Ungültige Auswahl!");
        std::process::exit(1);
    }

    Ok(txt_files[index - 1].clone())
}

// Funktion to start the Analysing
fn start() -> Result<(), Box<dyn std::error::Error>> {
    // Get the Path
    let maybepath = selectfolder();

    // Return if no folder was selected
    if maybepath == None {
        return Err("Irgendetwas ist schiefgelaufen".into());
    }
    let path = maybepath.unwrap();

    // Main txt File
    let selected = searchtxt(path)?;
    let content = fs::read_to_string(selected)?;
    let messages = msg_parser::parsetxt(content);
    for message in messages {
        println!("{}", message.message);
    }
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
