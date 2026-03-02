# Chat Analyser for Insta Export

import os
import json

# Main function
def main():
    folder = "instagram-firesammyyy-2026-03-01-J3Tjionx"

    messages = folder + "/" + "your_instagram_activity" + "/" + "messages" + "/" + "inbox"

    # Print every Message Folder
    for eintrag in os.listdir(messages):
        voller_pfad = os.path.join(messages, eintrag)
        if os.path.isdir(voller_pfad):
            print("Ordner:", voller_pfad)
            # Open first Chatfile
