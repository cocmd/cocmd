import platform
from cocmd.core.os import OS

def get_os()->OS:
    return OS.from_str(platform.system().lower())