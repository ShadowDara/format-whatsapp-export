// Virtual Dom for the Messages

let count = 0;

for (const entry of window.json_data) {
    if (count >= 30) break;

    document.getElementById("msg-store").innerHTML += `
    <div class="main-box">
        <div class="author">${entry.sender}</div>
        <div class="message">${entry.msg}</div>
        <div class="time">${entry.date} ${entry.time}</div>
    </div>`;

    count++;
}
