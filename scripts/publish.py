import os

CHECK_QUEUE = [
    ("Modified files present", "git diff-index --quiet HEAD")
]

COMMAND_QUEUE = [
    "cargo test",
    "rustdoc-include --root ./",
    "git commit -a --message \"Pre-publish commit\""
    "cargo publish --dry-run"
]

def execute(command: str) -> int:
    if os.name == "nt":
        return os.system(command)

for (name, command) in CHECK_QUEUE:
    if execute(command) != 0:
        print(f"`{name}` check failed.")
        exit(2)

for command in COMMAND_QUEUE:
    if execute(command) != 0:
        exit(1)