import os

CHECK_QUEUE = [
    ("Modified files present", "git diff-index --quiet HEAD"),
    ("Cargo test", "cargo test"),
]

COMMAND_QUEUE = [
    "rustdoc-include --root ./",
    "git commit -a --message \"Pre-publish commit\"",
    "cargo publish --dry-run"
]

def execute(command: str) -> int:
    if os.name == "nt":
        return os.system(command)
    else:
        raise Exception("OS Not supported")

def main() -> None:
    for (name, command) in CHECK_QUEUE:
        print(f"=====> Testing \"{name}\" ({command})")
        if execute(command) != 0:
            print(f"`{name}` check failed.")
            exit(2)

    for command in COMMAND_QUEUE:
        print(f"=====> Executing {command}")
        if execute(command) != 0:
            exit(1)

try:
    main()
except KeyboardInterrupt:
    exit(1)