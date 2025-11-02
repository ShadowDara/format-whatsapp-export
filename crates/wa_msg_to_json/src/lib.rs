use wa_msg_parser;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
struct MessageJson<'a> {
    date: &'a str,
    datetime: &'a str,
    sender: &'a str,
    message: &'a str,
}

// Convert an Array of Messages to JSON
pub fn messages_to_json(messages: &[wa_msg_parser::Message]) -> String {
    let serializable: Vec<_> = messages
        .iter()
        .map(|m| MessageJson {
            date: &m.date,
            datetime: &m.datetime,
            sender: &m.sender,
            message: &m.message,
        })
        .collect();

    serde_json::to_string(&serializable).unwrap()
}

// Convert Message JSON Struct to Messages
pub fn json_to_messages(json_str: &str) -> Vec<wa_msg_parser::Message> {
    // Zuerst in die Zwischendarstellung deserialisieren
    let temp: Vec<MessageJson> = serde_json::from_str(json_str).unwrap();

    // Dann in den originalen Typ umwandeln
    temp.into_iter()
        .map(|m| wa_msg_parser::Message {
            date: m.date.to_string(),
            datetime: m.datetime.to_string(),
            sender: m.sender.to_string(),
            message: m.message.to_string(),
        })
        .collect()
}
