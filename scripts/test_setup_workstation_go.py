"""Exercise the Go installer against temporary directories, without downloads/root."""

import hashlib
import io
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile
import tempfile
import unittest


SCRIPT = Path(__file__).with_name("setup-workstation.sh")


class GoInstallTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="workstation-go-test-")
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.destination = self.root / "local"
        (self.destination / "go").mkdir(parents=True)
        (self.destination / "go" / "original").write_text("retained toolchain\n")
        self.bin = self.root / "bin"
        self.bin.mkdir()
        self.archive = self.root / "download.tar.gz"
        self.make_archive("1.26.9")
        self.metadata = self.root / "releases.json"
        self.write_metadata()
        self.stub("curl", """#!/usr/bin/env python3
import os, shutil, sys
args=sys.argv[1:]; url=next(x for x in args if x.startswith('https:'))
if os.environ.get('TEST_DOWNLOAD_FAIL') == '1' and '?mode=' not in url:
    sys.exit(22)
source=os.environ['TEST_METADATA' if '?mode=' in url else 'TEST_ARCHIVE']
shutil.copyfile(source,args[args.index('-o')+1])
""")
        # macOS lacks flock/sha256sum and GNU mv. These stubs keep the real
        # checksum/extraction/filesystem behavior while adapting those commands.
        if not shutil.which("flock"):
            self.stub("flock", "#!/bin/sh\nexit 0\n")
        if not shutil.which("sha256sum"):
            self.stub("sha256sum", "#!/bin/sh\nexec shasum -a 256 \"$@\"\n")
        self.stub("mv", """#!/usr/bin/env python3
import os, pathlib, subprocess, sys
args=[x for x in sys.argv[1:] if x != '-T']; src,dst=map(pathlib.Path,args)
if os.environ.get('TEST_PUBLISH_FAIL') == '1' and src.name == 'go' and src.parent.name.startswith('.go-install.'):
    sys.exit(1)
if os.environ.get('TEST_ROLLBACK_FAIL') == '1' and src.name == 'previous':
    sys.exit(1)
if os.environ.get('TEST_NATIVE_MV'):
    sys.exit(subprocess.run([os.environ['TEST_NATIVE_MV'], *sys.argv[1:]]).returncode)
if dst.exists() or dst.is_symlink():
    sys.exit(1)
os.rename(src,dst)
""")
        if not shutil.which("jq"):
            self.skipTest("jq is required by the installer")
        source = SCRIPT.read_text()
        function = source.split("install_go() {", 1)[1].split(
            "# ------------------------------------------------------------------- node", 1
        )[0]
        self.harness = self.root / "harness.sh"
        self.harness.write_text("""#!/usr/bin/env bash
set -uo pipefail
log() { :; }
ok() { :; }
warn() { printf '%s\\n' "$*" >&2; }
have() { command -v "$1" >/dev/null 2>&1; }
go() { printf 'go1.24.0\\n'; }
version_ge() { return 1; }
persist_path() { :; }
as_root() { local args=("$@"); args[${#args[@]}-1]="$TEST_DESTINATION"; "${args[@]}"; }
GO_MIN=1.25
GO_VERSION=1.26.9
GOARCH=amd64
PKG=apt
OS=Linux
DRY_RUN=${TEST_DRY_RUN:-0}
install_go() {""" + function + "\ninstall_go\n")

    def stub(self, name, contents):
        path = self.bin / name
        path.write_text(contents)
        path.chmod(0o755)

    def make_archive(self, version):
        payload = f"#!/bin/sh\nprintf 'go{version}\\n'\n".encode()
        with tarfile.open(self.archive, "w:gz") as archive:
            entry = tarfile.TarInfo("go/bin/go")
            entry.mode = 0o755
            entry.size = len(payload)
            archive.addfile(entry, io.BytesIO(payload))

    def write_metadata(self, checksum=None):
        checksum = checksum or hashlib.sha256(self.archive.read_bytes()).hexdigest()
        self.metadata.write_text(json.dumps([{"files": [{
            "filename": "go1.26.9.linux-amd64.tar.gz", "sha256": checksum,
        }]}]))

    def run_installer(self, **overrides):
        env = dict(os.environ, PATH=str(self.bin) + os.pathsep + os.environ["PATH"],
                   TEST_ARCHIVE=str(self.archive), TEST_METADATA=str(self.metadata),
                   TEST_DESTINATION=str(self.destination), TMPDIR=str(self.root), **overrides)
        if sys.platform.startswith("linux"):
            env["TEST_NATIVE_MV"] = shutil.which("mv")
        return subprocess.run(["bash", str(self.harness)], env=env,
                              capture_output=True, text=True, timeout=20)

    def assert_original(self):
        self.assertEqual((self.destination / "go/original").read_text(), "retained toolchain\n")
        self.assertFalse((self.destination / "go/bin/go").exists())

    def test_download_failure_preserves_existing(self):
        result = self.run_installer(TEST_DOWNLOAD_FAIL="1")
        self.assertNotEqual(result.returncode, 0)
        self.assert_original()

    def test_bad_checksum_preserves_existing(self):
        self.write_metadata("0" * 64)
        result = self.run_installer()
        self.assertNotEqual(result.returncode, 0)
        self.assert_original()

    def test_bad_archive_preserves_existing(self):
        self.archive.write_bytes(b"not a tar archive")
        self.write_metadata()
        result = self.run_installer()
        self.assertNotEqual(result.returncode, 0)
        self.assert_original()

    def test_wrong_version_preserves_existing(self):
        self.make_archive("1.25.0")
        self.write_metadata()
        result = self.run_installer()
        self.assertNotEqual(result.returncode, 0)
        self.assert_original()

    def test_publish_failure_restores_previous(self):
        result = self.run_installer(TEST_PUBLISH_FAIL="1")
        self.assertNotEqual(result.returncode, 0)
        self.assert_original()

    def test_rollback_failure_retains_recovery_directory(self):
        result = self.run_installer(TEST_PUBLISH_FAIL="1", TEST_ROLLBACK_FAIL="1")
        self.assertNotEqual(result.returncode, 0)
        originals = list(self.destination.glob(".go-install.*/previous/original"))
        self.assertEqual(len(originals), 1)
        self.assertEqual(originals[0].read_text(), "retained toolchain\n")
        self.assertIn("original installation retained at", result.stderr)

    def test_valid_install_replaces_old_after_validation(self):
        result = self.run_installer()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertFalse((self.destination / "go/original").exists())
        self.assertEqual(subprocess.check_output(
            [str(self.destination / "go/bin/go"), "env", "GOVERSION"], text=True), "go1.26.9\n")
        self.assertEqual(list(self.destination.glob(".go-install.*/*")), [])

    def test_dry_run_preserves_existing(self):
        result = self.run_installer(TEST_DRY_RUN="1")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assert_original()
        self.assertFalse((self.destination / ".go-install.lock").exists())


if __name__ == "__main__":
    unittest.main()
