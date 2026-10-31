"""Offline regression tests for the generated service-account helper."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest


class HelperTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        source = Path(__file__).with_name("setup-workstation.sh").read_text()
        helper = source.split("<<'HELPER'\n", 1)[1].split("\nHELPER", 1)[0]
        self.helper = self.root / "helper"
        self.helper.write_text(helper)
        self.helper.chmod(0o700)
        self.bin = self.root / "bin"
        self.bin.mkdir()
        curl = self.bin / "curl"
        curl.write_text("""#!/usr/bin/env python3
import datetime,json,os,sys
from pathlib import Path
with Path(os.environ['CALLS']).open('a') as f: f.write(sys.argv[-1]+'\\n')
if os.environ.get('FAIL_CURL'): raise SystemExit(22)
expiry=(datetime.datetime.now(datetime.timezone.utc)+datetime.timedelta(hours=1))
print(json.dumps({'token':'fixture-'+sys.argv[-1],
 'expires_at':expiry.strftime('%Y-%m-%dT%H:%M:%SZ')}))
""")
        curl.chmod(0o700)
        self.key = self.root / "test-only-key.pem"
        subprocess.run(
            ["openssl", "genrsa", "-out", str(self.key), "2048"],
            check=True, capture_output=True,
        )
        # Do not inherit actual service-account credentials.
        self.env = {k: v for k, v in os.environ.items()
                    if not k.startswith(("GH_", "GITHUB_"))}
        self.env.update(
            PATH=str(self.bin) + os.pathsep + os.environ["PATH"],
            XDG_CACHE_HOME=str(self.root / "cache"),
            CALLS=str(self.root / "calls"),
            GH_APP_ID="123", GH_APP_INSTALLATION_ID="456",
            GH_APP_PRIVATE_KEY_FILE=str(self.key),
        )

    def run_helper(self, args=(), data="", **env):
        return subprocess.run(
            [str(self.helper), *args], input=data, text=True,
            capture_output=True, env={**self.env, **env},
        )

    def test_cache_reuse_and_installation_isolation(self):
        first = self.run_helper()
        self.assertEqual(first.returncode, 0, first.stderr)
        self.assertEqual(self.run_helper().stdout, first.stdout)
        other = self.run_helper(GH_APP_INSTALLATION_ID="789")
        self.assertNotEqual(first.stdout, other.stdout)
        self.assertEqual(len((self.root / "calls").read_text().splitlines()), 2)

    def test_app_host_and_api_have_distinct_cache_entries(self):
        for env in ({}, {"GH_APP_ID": "124"},
                    {"GH_SERVICE_HOST": "github.example"},
                    {"GH_SERVICE_API": "http://127.0.0.1:8123"}):
            result = self.run_helper(**env)
            self.assertEqual(result.returncode, 0, result.stderr)
        calls = (self.root / "calls").read_text().splitlines()
        self.assertEqual(len(calls), 4)
        self.assertIn("https://github.example/api/v3/", calls[2])

    def test_failed_exchange_cleans_key_and_header(self):
        result = self.run_helper(FAIL_CURL="1")
        self.assertNotEqual(result.returncode, 0)
        self.assertFalse(list((self.root / "cache").rglob("mint.*")))
        self.assertFalse(list((self.root / "cache").rglob("token.json")))
        self.assertNotIn("PRIVATE KEY", result.stdout + result.stderr)

    def test_untrusted_api_is_rejected_before_signing_or_exchange(self):
        for api in ("https://api.example", "http://localhost.example:8123",
                    "https://api.github.com@example.com"):
            result = self.run_helper(GH_SERVICE_API=api)
            self.assertNotEqual(result.returncode, 0)
            self.assertEqual(result.stdout, "")
        self.assertFalse((self.root / "calls").exists())

    def test_malformed_cached_expiry_cannot_reuse_a_token(self):
        self.assertEqual(self.run_helper().returncode, 0)
        cache = next((self.root / "cache").rglob("token.json"))
        record = json.loads(cache.read_text())
        record["expires_at"] = "not-a-date"
        cache.write_text(json.dumps(record))
        result = self.run_helper()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(len((self.root / "calls").read_text().splitlines()), 2)

    def test_invalid_key_never_reaches_exchange_in_git_mode(self):
        self.key.write_text("invalid fixture key")
        result = self.run_helper(("git", "get"), "protocol=https\nhost=github.com\n\n")
        self.assertEqual(result.stdout, "")
        self.assertFalse((self.root / "calls").exists())
        self.assertFalse(list((self.root / "cache").rglob("mint.*")))

    def test_git_rejects_other_hosts_and_protocols(self):
        for data in ("protocol=https\nhost=example.com\n\n",
                     "protocol=http\nhost=github.com\n\n", ""):
            result = self.run_helper(("git", "get"), data)
            self.assertEqual(result.stdout, "")
        self.assertFalse((self.root / "calls").exists())

    def test_git_returns_only_matching_https_credential(self):
        result = self.run_helper(("git", "get"), "protocol=https\nhost=github.com\n\n")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertTrue(result.stdout.startswith("username=x-access-token\npassword=fixture-"))

    def test_expired_cache_refreshes_and_files_are_private(self):
        self.assertEqual(self.run_helper().returncode, 0)
        cache = next((self.root / "cache").rglob("token.json"))
        record = json.loads(cache.read_text())
        record["expires_at"] = "2000-01-01T00:00:00Z"
        cache.write_text(json.dumps(record))
        self.assertEqual(self.run_helper().returncode, 0)
        self.assertEqual(len((self.root / "calls").read_text().splitlines()), 2)
        self.assertEqual(cache.stat().st_mode & 0o777, 0o600)
        self.assertEqual(cache.parent.stat().st_mode & 0o777, 0o700)

    def test_machine_user_token_needs_no_app_exchange(self):
        result = self.run_helper(GH_APP_ID="", GH_APP_INSTALLATION_ID="",
                                 GH_SERVICE_TOKEN="fixture-pat")
        self.assertEqual(result.stdout, "fixture-pat\n")
        self.assertFalse((self.root / "calls").exists())


if __name__ == "__main__":
    if not all(shutil.which(name) for name in ("openssl", "jq", "bash")):
        raise SystemExit("Tests require openssl, jq and bash")
    unittest.main()
