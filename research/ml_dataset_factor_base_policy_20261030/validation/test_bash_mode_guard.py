import json
from pathlib import Path
import subprocess
import tempfile
import unittest


class BashModeGuard(unittest.TestCase):
    """Execute the exact final loop extracted from the proposed runner script."""

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name)
        self.path = self.root / "fixture.sh"
        self.path.write_bytes(b"#!/bin/sh\nexit 0\n")
        self.inventory = self.root / "inventory.json"
        source = Path(__file__).with_name("verify-source.sh").read_text()
        start = source.index("while IFS=$'\\t' read -r path bytes git_mode; do\n")
        end = source.index('\n', source.index('done < <(jq -r', start))
        self.fragment = "set -euo pipefail\ninventory=$1\n" + source[start:end] + "\n"

    def tearDown(self):
        self.tmp.cleanup()

    def check_guard(self, expected, expected_mode="100755", size=17):
        self.inventory.write_text(json.dumps({"entries": [{"path": "fixture.sh", "bytes": size,
                                                        "git_mode": expected_mode}]}))
        result = subprocess.run(["bash", "-c", self.fragment, "mode-guard-fixture", str(self.inventory)],
                                cwd=self.root, text=True, capture_output=True)
        self.assertEqual(result.returncode, expected, result.stdout + result.stderr)
        return result

    def test_owner_executable_modes_accepted_without_chmod(self):
        for mode in (0o700, 0o755, 0o4755):
            with self.subTest(mode=oct(mode)):
                self.path.chmod(mode)
                self.check_guard(0)
                self.assertEqual(self.path.stat().st_mode & 0o7777, mode)

    def test_missing_owner_execute_rejected_even_with_group_execute(self):
        for mode in (0o600, 0o644, 0o610):
            with self.subTest(mode=oct(mode)):
                self.path.chmod(mode)
                result = self.check_guard(1)
                self.assertIn("expected 100755 observed 100644", result.stderr)

    def test_regular_nonexecutables_accepted_and_unexpected_execute_rejected(self):
        for mode in (0o600, 0o644, 0o610):
            self.path.chmod(mode)
            self.check_guard(0, "100644")
        self.path.chmod(0o700)
        self.assertIn("expected 100644 observed 100755", self.check_guard(1, "100644").stderr)

    def test_size_symlink_directory_and_missing_rejected(self):
        self.path.chmod(0o700)
        self.check_guard(1, size=18)
        self.path.unlink()
        self.path.symlink_to(self.inventory)
        self.check_guard(1)
        self.path.unlink()
        self.path.mkdir()
        self.check_guard(1)
        self.path.rmdir()
        self.check_guard(1)

    def test_skip_worktree_does_not_hide_mode_loss_from_bash_guard(self):
        self.path.chmod(0o700)
        subprocess.run(["git", "init", "-q"], cwd=self.root, check=True)
        subprocess.run(["git", "add", "fixture.sh"], cwd=self.root, check=True)
        subprocess.run(["git", "-c", "core.hooksPath=/dev/null", "-c", "user.name=Fixture", "-c",
                        "user.email=fixture@invalid", "commit", "-qm", "mode fixture"], cwd=self.root, check=True)
        subprocess.run(["git", "update-index", "--skip-worktree", "fixture.sh"], cwd=self.root, check=True)
        self.path.chmod(0o600)
        subprocess.run(["git", "diff", "--exit-code", "HEAD"], cwd=self.root, check=True)
        self.assertIn("Materialized mode mismatch", self.check_guard(1).stderr)


if __name__ == "__main__":
    unittest.main()
