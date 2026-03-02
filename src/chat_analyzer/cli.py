# Chat Analyzer

import chat_analyzer.whatsapp.main as whatsapp
import chat_analyzer.discord.main as discord
import chat_analyzer.instagram.main as instagram
import chat_analyzer.signal.main as signal

# Main function
def main():
    print("Chat Analyser")

    # for now just start wa analyser
    whatsapp.main()
    # instagram.main()
