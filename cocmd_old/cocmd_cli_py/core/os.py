from enum import Enum

OS_OVERRIDES = {"darwin": "osx"}


class OS(Enum):
    ANY = "any"
    OSX = "osx"
    DEBIAN = "debian"

    def __str__(self):
        return self.value

    @staticmethod
    def from_str(text: str) -> "OS":
        text = text.lower()
        for os in OS:
            if str(os).lower() == text:
                return os

        if text in OS_OVERRIDES:
            return OS.from_str(OS_OVERRIDES[text])

        raise ValueError(f"unable to find os {text}")
