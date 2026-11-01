"""Exercise scripts/cloud-setup.sh's own steps (runtime detection, the env file,
opt-in secret persistence) with every setup-workstation component skipped:
temporary HOME and workspace, no installs, no network."""

import os
from pathlib import Path
import subprocess
import tempfile
import unittest


SCRIPT = Path(__file__).with_name("cloud-setup.sh")
COMPONENTS = subprocess.run(["bash", SCRIPT.with_name("setup-workstation.sh"), "--list"],
                            capture_output=True, text=True, check=True).stdout.split()


class CloudSetupTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="cloud-setup-test-")
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.home = self.root / "home"
        self.home.mkdir()
        self.env = {
            "HOME": str(self.home),
            "PATH": os.environ["PATH"],
            "WORKSPACE": str(self.root / "workspace"),
            "SETUP_SKIP": ",".join(COMPONENTS),
            "SETUP_CLEANUP": "0",
            "CRYPTO_SPARSE_PROFILE": "full",
        }

    def run_setup(self, **extra):
        return subprocess.run(["bash", SCRIPT], env=dict(self.env, **extra),
                              capture_output=True, text=True)

    def envfile(self):
        return self.home / ".config/cloud-setup/env"

    def test_env_file_is_private_and_sourced_by_login_shells(self):
        result = self.run_setup(CLOUD_RUNTIME="claude")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("runtime claude", result.stdout)
        self.assertEqual(self.envfile().stat().st_mode & 0o777, 0o600)
        self.assertIn("export PATH=", self.envfile().read_text())
        for rc in (".bashrc", ".profile"):
            self.assertIn(str(self.envfile()), (self.home / rc).read_text())
        # Re-running adds no second source line.
        self.run_setup(CLOUD_RUNTIME="claude")
        lines = (self.home / ".bashrc").read_text().splitlines()
        self.assertEqual(sum(str(self.envfile()) in line for line in lines), 1)

    def test_secrets_are_listed_by_name_and_never_printed(self):
        result = self.run_setup(CLOUD_RUNTIME="claude", AWS_SECRET_ACCESS_KEY="s3cr3t-value")
        self.assertIn("AWS_SECRET_ACCESS_KEY", result.stdout)
        self.assertNotIn("s3cr3t-value", result.stdout + result.stderr)
        self.assertNotIn("s3cr3t-value", self.envfile().read_text())

    def test_persist_secrets_only_on_codex(self):
        result = self.run_setup(CLOUD_RUNTIME="claude", PERSIST_SECRETS="1", HF_TOKEN="hf_x")
        self.assertIn("applies to Codex only", result.stderr)
        self.assertNotIn("hf_x", self.envfile().read_text())
        result = self.run_setup(CLOUD_RUNTIME="codex", PERSIST_SECRETS="1",
                                HF_TOKEN="hf_x y'z", UNRELATED_SECRET="nope")
        self.assertEqual(result.returncode, 0, result.stderr)
        text = self.envfile().read_text()
        self.assertNotIn("UNRELATED_SECRET", text)
        self.assertEqual(self.envfile().stat().st_mode & 0o777, 0o600)
        value = subprocess.run(["bash", "-c", f". {self.envfile()} && printf %s \"$HF_TOKEN\""],
                               capture_output=True, text=True).stdout
        self.assertEqual(value, "hf_x y'z")

    def test_dry_run_writes_nothing(self):
        result = subprocess.run(["bash", SCRIPT, "--dry-run"], env=dict(self.env, CLOUD_RUNTIME="codex"),
                                capture_output=True, text=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertFalse(self.envfile().exists())
        self.assertFalse((self.home / ".bashrc").exists())


if __name__ == "__main__":
    unittest.main()
