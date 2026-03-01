from dataclasses import dataclass
from datetime import datetime

@dataclass
class Message:
    date: str
    time: str
    author: str | None
    text: str
