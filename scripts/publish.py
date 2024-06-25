import os

COMMAND_QUEUE = [
    "cargo test",
    "rustdoc-include --root ./",
    "cargo publish --dry-run"
]

def execute(command: str) -> int:
    if os.name == "nt":
        return os.system(command)

for command in COMMAND_QUEUE:
    if execute(command) != 0:
        exit(1)