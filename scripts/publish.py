import os


class CommandException(Exception):
    def __init__(self, command: str, code: int):
        self.command = command
        self.code = code

    def __str__(self):
        return f"Command \"{self.command}\" failed with exit code {self.code}"


class TestFailException(Exception):
    def __init__(self, test_name, command: str, code: int):
        self.command = command
        self.code = code
        self.name = test_name

    def __str__(self):
        return f"Test \"{self.name}\" failed"


def execute(command: str) -> int:
    if os.name == "nt":
        return os.system(command)
    else:
        raise Exception("OS Not supported")


def try_execute(command: str) -> None:
    print(f"=====> Executing {command}")
    code = execute(command)
    if code != 0:
        raise CommandException(command, code)


def test(name: str, command: str) -> None:
    print(f"=====> Testing \"{name}\" ({command})")
    code = execute(command)
    if code != 0:
        raise TestFailException(name, command, code)


def main() -> None:
    test("No modified files since last commit", "git diff-index --quiet HEAD")
    test("Cargo test", "cargo test")

    try_execute("rustdoc-include --root ./")
    if execute("git diff-index --quiet HEAD") != 0:
        try_execute("git commit -a --message \"Pre-publish commit\"")
    try_execute("cargo publish")


try:
    main()
except KeyboardInterrupt:
    exit(1)
except TestFailException as e:
    print(e)
    exit(1)
except CommandException as e:
    print(e)
    exit(1)
