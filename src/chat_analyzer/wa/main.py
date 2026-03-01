# Main for WA Data Analyzer

import re
from datetime import datetime
from collections import Counter

import chat_analyzer.wa.data as data

pattern = re.compile(
    r'^(\d{1,2}\.\d{1,2}\.\d{2,4}),\s(\d{1,2}:\d{2})\s-\s([^:]+?):\s(.*)$'
)

# Main function
def main():
    # Load Input
    lines = ""

    with open("input.txt", "r", encoding="utf-8") as f:
        lines = f.readlines()

    messages: list[data.Message] = []

    for line in lines:
        m = pattern.match(line)
        if m:
            date, time, author, message = m.groups()

            timestamp = datetime.strptime(
                f"{date} {time}",
                "%d.%m.%y %H:%M"
            )

            messagedata: data.Message = data.Message(timestamp, author, message)
            messages.append(messagedata)
    
    print(f"Parsed {len(lines)} Messaged!")
    
    for me in messages:
        # print(me.timestamp, me.author, me.text)
    #     # print(me.timestamp)
        pass

    # Analyse the Messages here
    per_month = Counter()

    for m in messages:
        key = m.timestamp.strftime("%Y-%m")
        per_month[key] += 1
    
    for month, count in sorted(per_month.items()):
        print(month, count)
    
    # Analyse by User
    users = {}
    for m in messages:
        if m.author not in users:
            users[m.author] = 0
        users[m.author] += 1
    
    for key, value in users.items():
        print(f"User {key}: Messages {value}")
