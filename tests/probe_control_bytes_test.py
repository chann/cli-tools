import os
import pty
import termios
import unittest

from scripts import probe_control_bytes as module


def effective_termios(attributes):
    effective = list(attributes)
    # macOS reports PENDIN as kernel state after raw input, not configuration.
    effective[3] &= ~termios.PENDIN
    return effective


class ProbeTests(unittest.TestCase):
    def test_reads_control_c_then_control_g_as_bytes(self):
        master, slave = pty.openpty()
        try:
            before = termios.tcgetattr(slave)
            with module.raw_terminal(slave):
                os.write(master, b"\x03\x07")
                self.assertEqual(
                    module.read_expected(slave, (0x03, 0x07)),
                    [0x03, 0x07],
                )
            self.assertEqual(
                effective_termios(termios.tcgetattr(slave)),
                effective_termios(before),
            )
        finally:
            os.close(master)
            os.close(slave)

    def test_wrong_byte_fails_and_still_restores_termios(self):
        master, slave = pty.openpty()
        before = termios.tcgetattr(slave)
        try:
            with self.assertRaises(module.ProbeError):
                with module.raw_terminal(slave):
                    os.write(master, b"x")
                    module.read_expected(slave, (0x03, 0x07))
        finally:
            self.assertEqual(
                effective_termios(termios.tcgetattr(slave)),
                effective_termios(before),
            )
            os.close(master)
            os.close(slave)


if __name__ == "__main__":
    unittest.main()
