/* JS Code for format-whatsapp-export */
/* Licensed under Appache 2.0 by Shadowdara */
/* https://github.com/ShadowDara/format-whatsapp-export */

/* Languages */
const en = []
const de = {
    "0001": "Seite: ",
    "0002": "Nachrichten Anzahl",
    "statistics": "Statistiken",
    "settings": "Einstellungen",
    "previous_page": "",
    "subcredit": "",
    "next_page": "Nächste Seite"
}

/* Virtual Dom for the Messages */
const message_dom = document.getElementById("msg-store");

let count = 0;
let count_end = 30;
let page_nr = 1;
let msg_count = 1;
let page_msg_count = 0;
let Lang = [];

/* Message DB Variable for the Search Function */
let sr_messages = [];

/* Settings Values */
const show_pictures = document.getElementById("show_pictures").checked;

/* Switch to the Next Messages Page */
document.getElementById("next_page").onclick = () => {
    count += 30;
    count_end += 30;
    page_nr += 1;
    do_count(count, count_end, window.json_data);

    console.log("Goto Next Page");
};

/* Switch to the Previous Messages Page */
document.getElementById("previous_page").onclick = () => {
    goto_previous_page();
};

function goto_previous_page() {
    if (count >= 30) {
        count -= 30;
        count_end - 30;
        page_nr -= 1;
        do_count(count, count_end, window.json_data);
    } else {
        alert("There is no previous Page available!");
    }

    console.log("Goto Previous Page");
}

/* Statistics POPUP Window */
const statistics_openBtn = document.getElementById("statistics");
const statistics_closeBtn = document.getElementById("statistics_closeModalBtn");
const statistics_overlay = document.getElementById("statistics_modalOverlay");

statistics_openBtn.addEventListener("click", () => {
    statistics_overlay.style.display = "flex";
});

statistics_closeBtn.addEventListener("click", () => {
    statistics_overlay.style.display = "none";
});

statistics_overlay.addEventListener("click", (event) => {
    if (event.target === statistics_overlay) {
        statistics_overlay.style.display = "none";
    }
});

/* Settings POPUP Window */
const settings_openBtn = document.getElementById("settings");
const settings_closeBtn = document.getElementById("settings_closeModalBtn");
const settings_overlay = document.getElementById("settings_modalOverlay");

settings_openBtn.addEventListener("click", () => {
    settings_overlay.style.display = "flex";
});

settings_closeBtn.addEventListener("click", () => {
    settings_overlay.style.display = "none";
});

settings_overlay.addEventListener("click", (event) => {
    if (event.target === settings_overlay) {
        settings_overlay.style.display = "none";
    }
});

/* Search Bar */
document.getElementById("search").addEventListener("keypress", function (event) {
    if (event.key === "Enter") {
        event.preventDefault();
        const search_value = this.value;

        const search_regex = regexsearch(search_value);

        msg_count = 0;
        page_msg_count = 0;
        sr_messages = [];

        for (const entry of window.json_data) {
            if (search_regex.test(entry.msg)) {
                sr_messages.push(entry);
                msg_count += 1;
            }
        }

        /* Switch to the frist Page */
        count = 0;
        count_end = 30;
        page_nr = 1;

        document.getElementById("message_count").innerText = msg_count;

        do_count(count, count_end, sr_messages);

        /* Search Works */
        alert(`${msg_count} Results`);
    }
});

/* Regex Search for Messages */
function regexsearch(eingabe) {
    const escaped = eingabe
        .trim()
        .split(/\s+/)
        .map(wort => wort.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'))
        .join(".*");

    return new RegExp(escaped, 'i');
}

// Adding hyperlinks to a string
function linkify(text) {
    const urlRegex = /(https?:\/\/[^\s]+)/g;
    return text.replace(urlRegex, (url) => {
        return `<i><a href="${url}" target="_blank" rel="noopener noreferrer">${url}</a></i>`;
    });
}

/* to add a Message on the HTML Site */
function do_count(c1, c2, data) {
    page_msg_count = 0;
    message_dom.innerHTML = "";

    for (const entry of data.slice(c1, c2)) {
        // Calling the Hyperlink Regex
        const output = linkify(entry.msg);

        message_dom.innerHTML += `<div class="main-box"><div class="author">${entry.sender}</div><div class="message"><p>${output}</p></div><div class="time">${entry.date} ${entry.time}</div></div>`;
        page_msg_count += 1;
    }

    document.getElementById("page_number").innerText = page_nr;

    console.log("Messages: " + page_msg_count + " , Page Number: " + page_nr);
}

/* =============================================================== */

/* EXECUTING CODE */

/* When no data */
message_dom.innerHTML = "<pre>No Data Available!<pre>";

/* Selecting the Language */
if (window.selected_lang == "en") {
    Lang = en;
}
else if (window.selected_lang == "de") {
    Lang = de;
}
else {
    Lang = en;
}

/* Inserting the Data */
for (const entry of window.json_data) {
    msg_count += 1;
}
document.getElementById("message_count").innerText = msg_count;

do_count(count, count_end, window.json_data);
