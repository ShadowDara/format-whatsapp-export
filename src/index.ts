// Format Whatsapp Export

// Const for the Current Develeping Version
const VERSION = "V2";

// Function to select the export dir
function selectdir(): String {
    return ""
}

// Function which searches for the message txt file in the export Folder
// and returns the content of it as a String
function search_main_txt(selected_dir: String): String {
    // Search in the selected dir for all txt files

    // Get of the name of the top folder from the path

    // Try to contruct a filename with the foldername and .txt as a
    // File Extension

    // If this is not possible, print the Foldername and all / similliar
    // .txt Files in the Terminal and let the User decide which file is
    // the main entry file

    return ""
}

// Main function for the Code
function main() {
    // Title Message
    console.log("Format Whatsapp Export " + VERSION);

    // Select the dir where you export is located!
    const export_dir = selectdir();
    console.log("Selected Dir: "+ export_dir);

    // Ask if the User want to continue with this path

    // Content contains the content of the main .txt file
    const content = search_main_txt(export_dir);

    // Parse the content of the Message via a Regex or other Parser
    // into this struct
    //
    // {
    //      "date": "",
    //      "datetime": "",
    //      "message_sender": "",
    //      "message_content": "",
    // }
    //
    // Difficultys here:
    // - Messages which last via multiple lines
    // - Messages which contain Pictures or other files embedded

    // now make an array full of these structs

    // convert the struct to JSON and paste it in the HTML Template

    // Start a static http Server which displays the output
    // and open the link automaticly in the standard browser

    // although add the option to compile the content and bundle it
    // into 1 native HTML File
}

// Entrypoint of the prorgamm
main();
