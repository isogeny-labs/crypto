"""Exercise the `github` step and its gh-app-token helper against a local API
stand-in: no real GitHub credential, no network, temporary HOME and git config."""

import base64
import http.server
import json
import os
from pathlib import Path
import subprocess
import tempfile
import threading
import time
import unittest


SCRIPT = Path(__file__).with_name("setup-workstation.sh")


def unb64(text):
    return base64.urlsafe_b64decode(text + "=" * (-len(text) % 4))


class AppApi(http.server.ThreadingHTTPServer):
    """POST /app/installations/<id>/access_tokens with an RS256 JWT whose
    signature, `iss` and expiry are checked against the registered Apps.
    Each (App, installation, server) gets its own token, and requests are
    counted so cache reuse is observable."""

    def __init__(self, apps, label):
        self.apps = apps            # {app_id: public key path}
        self.label = label
        self.requests = 0
        super().__init__(("127.0.0.1", 0), AppApiHandler)


class AppApiHandler(http.server.BaseHTTPRequestHandler):
    def log_message(self, *args):
        pass

    def do_POST(self):
        server = self.server
        server.requests += 1
        try:
            header_b64, body_b64, sig_b64 = self.headers["Authorization"].split()[1].split(".")
            claims = json.loads(unb64(body_b64))
            app_id = claims["iss"]
            public_key = server.apps[app_id]
            with tempfile.TemporaryDirectory() as tmp:
                signed = Path(tmp, "signed")
                signed.write_bytes(f"{header_b64}.{body_b64}".encode())
                signature = Path(tmp, "signature")
                signature.write_bytes(unb64(sig_b64))
                verified = subprocess.run(
                    ["openssl", "dgst", "-sha256", "-verify", public_key,
                     "-signature", signature, signed],
                    capture_output=True).returncode == 0
            parts = self.path.strip("/").split("/")
            ok = (verified and json.loads(unb64(header_b64))["alg"] == "RS256"
                  and claims["exp"] > time.time()
                  and parts[:2] == ["app", "installations"] and parts[3:] == ["access_tokens"])
            installation = parts[2]
        except Exception:
            ok = False
        if not ok:
            self.send_response(401)
            self.end_headers()
            return
        expires = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() + 3600))
        token = f"ghs_{server.label}_app{app_id}_inst{installation}"
        self.send_response(201)
        self.end_headers()
        self.wfile.write(json.dumps({"token": token, "expires_at": expires}).encode())


class GithubStepTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="workstation-github-test-")
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.home = self.root / "home"
        self.home.mkdir()
        # gh validates tokens online; a stub keeps the test offline.
        stubs = self.root / "stubs"
        stubs.mkdir()
        (stubs / "gh").write_text("#!/bin/sh\ncat >/dev/null\nexit 0\n")
        (stubs / "gh").chmod(0o755)
        self.keys = {}
        for app in ("111", "222"):
            private = self.root / f"app{app}.pem"
            subprocess.run(["openssl", "genrsa", "-out", private, "2048"],
                           check=True, capture_output=True)
            public = self.root / f"app{app}.pub"
            subprocess.run(["openssl", "rsa", "-in", private, "-pubout", "-out", public],
                           check=True, capture_output=True)
            self.keys[app] = (private, public)
        self.api_a = self.serve("A")
        self.api_b = self.serve("B")
        self.env = {
            "HOME": str(self.home),
            "PATH": f"{stubs}:{os.environ['PATH']}",
            "GIT_CONFIG_GLOBAL": str(self.home / ".gitconfig"),
            "NO_PROXY": "127.0.0.1,localhost",
            "no_proxy": "127.0.0.1,localhost",
            "GIT_TERMINAL_PROMPT": "0",
        }

    def serve(self, label):
        server = AppApi({app: str(pub) for app, (_, pub) in self.keys.items()}, label)
        threading.Thread(target=server.serve_forever, daemon=True).start()
        self.addCleanup(server.server_close)
        self.addCleanup(server.shutdown)
        return server

    def api(self, server):
        return f"http://127.0.0.1:{server.server_address[1]}"

    def app_env(self, app, installation, server, key_as_text=False):
        private, _ = self.keys[app]
        env = dict(self.env, GH_APP_ID=app, GH_APP_INSTALLATION_ID=installation,
                   GH_SERVICE_API=self.api(server))
        if key_as_text:
            env["GH_APP_PRIVATE_KEY"] = private.read_text().replace("\n", "\\n")
        else:
            env["GH_APP_PRIVATE_KEY_FILE"] = str(private)
        return env

    def setup(self, env, *extra):
        return subprocess.run(["bash", SCRIPT, "--only", "github", *extra], env=env,
                              capture_output=True, text=True)

    def helper(self, env, *args):
        return subprocess.run([self.home / ".local/bin/gh-app-token", *args], env=env,
                              capture_output=True, text=True)

    def credential(self, env, host="github.com"):
        out = subprocess.run(["git", "credential", "fill"], env=env, text=True,
                             input=f"protocol=https\nhost={host}\n\n", capture_output=True)
        return dict(line.split("=", 1) for line in out.stdout.splitlines() if "=" in line)

    def test_app_token_reaches_git_and_cache_is_private(self):
        env = self.app_env("111", "10", self.api_a, key_as_text=True)
        result = self.setup(env)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("use GitHub App 111 (installation 10)", result.stdout)
        self.assertEqual(self.credential(env).get("password"), "ghs_A_app111_inst10")
        cached = list((self.home / ".cache/gh-app-token").rglob("*"))
        files = [p for p in cached if p.is_file()]
        self.assertTrue(files)
        for path in files:
            self.assertEqual(path.stat().st_mode & 0o777, 0o600, path)

    def test_same_identity_reuses_the_cached_token(self):
        env = self.app_env("111", "10", self.api_a)
        self.assertEqual(self.setup(env).returncode, 0)
        before = self.api_a.requests
        for _ in range(3):
            self.assertEqual(self.helper(env).stdout.strip(), "ghs_A_app111_inst10")
        self.assertEqual(self.api_a.requests, before)

    def test_switching_identity_never_returns_another_identitys_token(self):
        first = self.app_env("111", "10", self.api_a)
        self.assertEqual(self.setup(first).returncode, 0)
        self.assertEqual(self.helper(first).stdout.strip(), "ghs_A_app111_inst10")
        # Each of these differs from the cached identity in one setting only,
        # while the first token is still valid.
        cases = {
            "installation": (self.app_env("111", "20", self.api_a), "ghs_A_app111_inst20"),
            "app": (self.app_env("222", "10", self.api_a), "ghs_A_app222_inst10"),
            "api host": (self.app_env("111", "10", self.api_b), "ghs_B_app111_inst10"),
        }
        for changed, (env, expected) in cases.items():
            with self.subTest(changed=changed):
                self.assertEqual(self.helper(env).stdout.strip(), expected)
                self.assertEqual(self.credential(env).get("password"), expected)
        # And the original identity still gets its own token back.
        self.assertEqual(self.helper(first).stdout.strip(), "ghs_A_app111_inst10")

    def test_wrong_app_is_rejected_and_git_left_unconfigured(self):
        env = self.app_env("111", "10", self.api_a)
        env["GH_APP_ID"] = "999"
        result = self.setup(env)
        self.assertIn("GitHub credential check failed", result.stdout + result.stderr)
        config = subprocess.run(["git", "config", "--global", "--get-all",
                                 "credential.https://github.com.helper"],
                                env=env, capture_output=True, text=True)
        self.assertEqual(config.stdout, "")

    def test_untrusted_api_endpoint_never_receives_the_jwt(self):
        for url in ("https://attacker.example", "http://127.0.0.1.attacker.example",
                    "https://api.github.com.attacker.example", "http://localhost:80/x"):
            with self.subTest(url=url):
                env = dict(self.app_env("111", "10", self.api_a), GH_SERVICE_API=url)
                result = self.setup(env)
                self.assertIn("GH_SERVICE_API must be https://api.github.com",
                              result.stdout + result.stderr)
                self.assertNotEqual(self.helper(env).returncode, 0)
        self.assertEqual(self.api_a.requests, 0)

    def test_service_token_is_scoped_to_its_host(self):
        env = dict(self.env, GH_SERVICE_TOKEN="ghp_TESTSERVICETOKEN")
        self.assertEqual(self.setup(env).returncode, 0)
        self.assertEqual(self.credential(env).get("password"), "ghp_TESTSERVICETOKEN")
        self.assertNotIn("password", self.credential(env, host="example.com"))

    def test_without_a_credential_the_step_is_skipped(self):
        result = self.setup(dict(self.env))
        self.assertIn("no GitHub service credential set", result.stdout + result.stderr)
        self.assertFalse((self.home / ".local/bin/gh-app-token").exists())

    def test_dry_run_changes_nothing(self):
        env = self.app_env("111", "10", self.api_a)
        result = self.setup(env, "--dry-run")
        self.assertIn("+ write", result.stdout)
        self.assertFalse((self.home / ".local/bin/gh-app-token").exists())
        self.assertEqual(self.api_a.requests, 0)


if __name__ == "__main__":
    unittest.main()
