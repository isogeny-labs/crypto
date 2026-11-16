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
            "CRYPTO_AR_SPARSE_PROFILE": "full",
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

    def stub_repo(self, name, tool, command):
        """A git repository whose sparse tool appends its arguments to a log."""
        repo = self.root / "workspace" / name
        (repo / tool).parent.mkdir(parents=True)
        log = self.root / f"{name}.log"
        (repo / tool).write_text(f"{command}\n" if command.startswith("#!") else command)
        subprocess.run(["git", "init", "-q", repo], check=True)
        return repo, log

    def sparse_repos(self):
        crypto, crypto_log = self.stub_repo(
            "crypto", "scripts/sparse-checkout.sh",
            '#!/bin/sh\nprintf "%s\\n" "$*" >> "$SPARSE_LOG_DIR/crypto.log"')
        ar, ar_log = self.stub_repo(
            "crypto-autoresearcher", "tools/sparse_checkout.py",
            "import os, sys\n"
            "open(os.path.join(os.environ['SPARSE_LOG_DIR'], 'crypto-autoresearcher.log'), 'a')"
            ".write(' '.join(sys.argv[1:]) + '\\n')\n")
        return (crypto, crypto_log), (ar, ar_log)

    def test_both_harness_profiles_apply_by_default(self):
        (_, crypto_log), (_, ar_log) = self.sparse_repos()
        env = {k: v for k, v in self.env.items() if not k.endswith("SPARSE_PROFILE")}
        result = subprocess.run(["bash", SCRIPT], env=dict(env, CLOUD_RUNTIME="claude",
                                                           SPARSE_LOG_DIR=str(self.root)),
                                capture_output=True, text=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(crypto_log.read_text(), "apply --profile harness\n")
        self.assertEqual(ar_log.read_text(), "apply harness\n")

    def test_full_profile_and_existing_sparse_checkout_are_left_alone(self):
        (crypto, crypto_log), (_, ar_log) = self.sparse_repos()
        subprocess.run(["git", "-C", crypto, "config", "core.sparseCheckout", "true"], check=True)
        env = dict(self.env, CLOUD_RUNTIME="claude", SPARSE_LOG_DIR=str(self.root))
        del env["CRYPTO_SPARSE_PROFILE"]
        result = subprocess.run(["bash", SCRIPT], env=env, capture_output=True, text=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("crypto: sparse checkout already applied", result.stdout)
        self.assertFalse(crypto_log.exists())
        self.assertFalse(ar_log.exists())    # CRYPTO_AR_SPARSE_PROFILE=full

    def test_dry_run_writes_nothing(self):
        result = subprocess.run(["bash", SCRIPT, "--dry-run"], env=dict(self.env, CLOUD_RUNTIME="codex"),
                                capture_output=True, text=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertFalse(self.envfile().exists())
        self.assertFalse((self.home / ".bashrc").exists())


if __name__ == "__main__":
    unittest.main()
