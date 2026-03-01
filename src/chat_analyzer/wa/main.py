# Main for WA Data Analyzer

import re

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
            messagedata: data.Message = data.Message(date, time, author, message)
            messages.append(messagedata)
    
    print(f"Parsed {len(lines)} Messaged!")
    
    # for me in messages:
    #     print(me.date, me.time, me.author, me.text)
    #     # print(me.timestamp)
    #     pass

    # Analyse the Messages here
    