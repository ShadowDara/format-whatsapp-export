// Virtual Dom for the Messages

let count = 0;
let count_end = 30;
let page_nr = 1;
let msg_count = 1;
// Message Variable for the Search Function
let sr_messages = []

// Settings Values
const show_pictures = document.getElementById("show_pictures").checked;

// Switch to the Previous Messages Page
document.getElementById("next_page").onclick = () => {
    count += 30;
    count_end += 30;
    page_nr += 1;
    do_count(count, count_end);
};

// Switch to the Next Messages Page
document.getElementById("previous_page").onclick = () => {
    if (count >= 30) {
        count -= 30;
        count_end - 30;
        page_nr -= 1;
        do_count(count, count_end);
    } else {
        alert("There is no previous Page available!");
    }
};

// Settings POPUP Window
const openBtn = document.getElementById("settings");
const closeBtn = document.getElementById("closeModalBtn");
const overlay = document.getElementById("modalOverlay");

openBtn.addEventListener("click", () => {
    overlay.style.display = "flex";
});

closeBtn.addEventListener("click", () => {
    overlay.style.display = "none";
});

overlay.addEventListener("click", (event) => {
    if (event.target === overlay) {
        overlay.style.display = "none";
    }
});

// Regex Search for Messages
function regexsearch(eingabe) {
    const escaped = eingabe
        .trim()
        .split(/\s+/)
        .map(wort => wort.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'))
        .join(".*");

    return new RegExp(escaped, 'i');
}

// to add a Message on the HTML Site
function do_count(c1, c2, data) {
    document.getElementById("msg-store").innerHTML = ""
    
    for (const entry of data.slice(c1, c2)) {
        document.getElementById("msg-store").innerHTML += `
        <div class="main-box">
            <div class="author">${entry.sender}</div>
            <div class="message">${entry.msg}</div>
            <div class="time">${entry.date} ${entry.time}</div>
        </div>`;
    }
    
    document.getElementById("page_number").innerText = page_nr;
}


// EXECUTING CODE

// When no data
document.getElementById("msg-store").innerHTML = "<pre>No Data Available!<pre>";

// Inserting the Data
for (const entry of window.json_data) {
    msg_count += 1;
}
document.getElementById("message_count").innerText = msg_count;
do_count(count, count_end, window.json_data);

// Search Bar
document.getElementById("search").addEventListener("keypress", function (event) {
    if (event.key === "Enter") {
        event.preventDefault();
        const search_value = this.value;

        const search_regex = regexsearch(search_value);

        msg_count = 0;

        for (const entry of window.json_data) {
            if (search_regex.test(entry.msg)) {
                sr_messages.push(entry);
                msg_count += 1;
            }
        }

        document.getElementById("message_count").innerText = msg_count;
        do_count(count, count_end, sr_messages);

        // Search Works
        alert(`${msg_count} Results`);
    }
});
