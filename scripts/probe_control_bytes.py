from __future__ import annotations

import contextlib
import os
import sys
import termios
import tty
from collections.abc import Iterator


class ProbeError(RuntimeError):
    pass


@contextlib.contextmanager
def raw_terminal(fd: int) -> Iterator[None]:
    original = termios.tcgetattr(fd)
    try:
        tty.setraw(fd, termios.TCSANOW)
        yield
    finally:
        termios.tcsetattr(fd, termios.TCSANOW, original)


def read_expected(fd: int, expected: tuple[int, ...]) -> list[int]:
    seen: list[int] = []
    for wanted in expected:
        value = os.read(fd, 1)
        if not value or value[0] != wanted:
            actual = "EOF" if not value else f"0x{value[0]:02x}"
            raise ProbeError(f"expected 0x{wanted:02x}, got {actual}")
        seen.append(value[0])
    return seen


def main() -> int:
    if not sys.stdin.isatty():
        print("ERROR: stdin is not a terminal", file=sys.stderr)
        return 1

    fd = sys.stdin.fileno()
    print("Press physical Control-C, then physical Control-G", flush=True)
    try:
        with raw_terminal(fd):
            seen = read_expected(fd, (0x03, 0x07))
    except ProbeError as error:
        print(f"ERROR: {error}", file=sys.stderr)
        return 1
    print("PASS: " + " ".join(f"{value:02x}" for value in seen))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
