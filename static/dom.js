// Virtual Dom for the Messages

let count = 0;
let count_end = 30;
let page_nr = 1;
let msg_count = 1;

// Buttons
document.getElementById("next_page").onclick = () => {
    count += 30;
    count_end += 30;
    page_nr += 1;
    do_count(count, count_end);
};

document.getElementById("previous_page").onclick = () => {
    if (count >= 30) {
        count -= 30;
        count_end -30;
        page_nr -= 1;
        do_count(count, count_end);
    } else {
        alert("There is no previous Page available!");
    }
};

function do_count(c1, c2) {
    document.getElementById("msg-store").innerHTML = ""
    for (const entry of window.json_data.slice(c1, c2)) {
        document.getElementById("msg-store").innerHTML += `
    <div class="main-box">
        <div class="author">${entry.sender}</div>
        <div class="message">${entry.msg}</div>
        <div class="time">${entry.date} ${entry.time}</div>
    </div>`;
    }
    document.getElementById("page_number").innerText = page_nr;
}

// execute
document.getElementById("msg-store").innerHTML = "<pre>No Data Available!<pre>";
for (const entry of window.json_data) {
    msg_count += 1;
}
document.getElementById("message_count").innerText = msg_count;
do_count(count, count_end);
