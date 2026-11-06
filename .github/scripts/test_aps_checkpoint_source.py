#!/usr/bin/env python3
"""Administrative custody tests; no upload, token request or experiment."""
import base64
import datetime as dt
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location(
    "aps_checkpoint_source", Path(__file__).with_name("aps_checkpoint_source.py"))
custody = importlib.util.module_from_spec(spec)
spec.loader.exec_module(custody)


class CustodyTests(unittest.TestCase):
    def repository(self, root, ledger_change=False, extra_manifest=False):
        def git(*args):
            return subprocess.check_output(["git", "-C", str(root), *args], text=True,
                                           stderr=subprocess.DEVNULL).strip()
        git("init", "-q")
        git("config", "user.name", "synthetic custody fixture")
        git("config", "user.email", "custody@example.invalid")
        git("remote", "add", "origin", "https://github.com/isogeny-labs/crypto.git")
        for name, body in [(".gitignore", ".resume/\n"), ("pcc/ledger.jsonl", "synthetic baseline\n"),
                           ("scripts/checkpoint.sh", "synthetic source; never executed\n")]:
            file = root / name
            file.parent.mkdir(parents=True, exist_ok=True)
            file.write_text(body)
        git("add", ".")
        git("commit", "-qm", "synthetic baseline")
        baseline = git("rev-parse", "HEAD")
        git("update-ref", "refs/remotes/origin/main", baseline)
        admission = self.admission()
        admission["lease_expires_at"] = (dt.datetime.now(dt.timezone.utc)
                                         + dt.timedelta(minutes=10)).isoformat()
        data = {custody.ADMISSION: json.dumps(admission),
                custody.STUDY + "CHECKPOINT_RESUME.md": "synthetic SOURCE_COMMIT resume\n",
                "src/cryptanalysis/f4_fp/reuse.rs": "synthetic source fixture\n"}
        files = []
        for name, body in data.items():
            file = root / name
            file.parent.mkdir(parents=True, exist_ok=True)
            file.write_text(body)
            files.append({"path": name, "sha256": custody.digest(file.read_bytes())})
        manifest = {"schema": "aps.checkpoint-selection/v1", "baseline_commit": baseline,
                    "checkpoint_script_sha256": custody.digest((root / "scripts/checkpoint.sh").read_bytes()),
                    "files": files}
        if extra_manifest:
            manifest["unreviewed_metadata"] = "synthetic denied payload"
        (root / custody.SELECTION).write_text(json.dumps(manifest))
        if ledger_change:
            (root / "pcc/ledger.jsonl").write_text("synthetic changed ledger\n")
        git("add", ".")
        git("commit", "-qm", "synthetic selected source")
        source = git("rev-parse", "HEAD")
        git("update-ref", "refs/remotes/origin/source", source)
        return source, baseline

    def admission(self):
        return {"schema": "aps.checkpoint-admission/v1", "task_ref": "T-10",
                "authorization_ref": "user-explicit-2026-10-30",
                "native_fence_sha256": "0" * 64,
                "lease_expires_at": "2026-11-06T12:10:00Z"}

    def test_fixed_window_never_authorizes_late_transfer(self):
        admission = self.admission()
        custody.deadline(admission, dt.datetime(2026, 11, 6, 12, 0, tzinfo=dt.timezone.utc))
        with self.assertRaises(ValueError):
            custody.deadline(admission, dt.datetime(2026, 11, 6, 12, 9, tzinfo=dt.timezone.utc))
        admission["authorization_ref"] = "generic-continuation"
        with self.assertRaises(ValueError):
            custody.deadline(admission, dt.datetime(2026, 11, 6, 12, 0, tzinfo=dt.timezone.utc))
        admission = self.admission()
        admission["unreviewed_metadata"] = "synthetic denied payload"
        with self.assertRaises(ValueError):
            custody.deadline(admission, dt.datetime(2026, 11, 6, 12, 0, tzinfo=dt.timezone.utc))

    def test_scoped_paths_exclude_ledger_shared_logs_and_private_history(self):
        for name in ["pcc/ledger.jsonl", "docs/session-logs/2026-11-06.md",
                     "problems/README.md", "/tmp/private", "../private",
                     custody.STUDY + "../other", custody.STUDY + "./note"]:
            self.assertFalse(custody.allowed(name), name)
        self.assertTrue(custody.allowed("src/cryptanalysis/f4_fp/reuse.rs"))
        self.assertTrue(custody.allowed(custody.STUDY + "ACCOUNTING.md"))

    def test_selected_bytes_are_pinned_and_cannot_escape(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            names = [custody.ADMISSION, custody.STUDY + "CHECKPOINT_RESUME.md"]
            rows = []
            for name in names:
                file = root / name
                file.parent.mkdir(parents=True, exist_ok=True)
                file.write_bytes(b"synthetic custody fixture")
                rows.append({"path": name, "sha256": custody.digest(file.read_bytes())})
            manifest = {"schema": "aps.checkpoint-selection/v1", "files": rows}
            self.assertEqual(len(custody.selected(root, manifest)), 2)
            (root / names[0]).write_bytes(b"changed")
            with self.assertRaises(ValueError):
                custody.selected(root, manifest)
            rows[0]["path"] = "pcc/ledger.jsonl"
            with self.assertRaises(ValueError):
                custody.selected(root, manifest)

    def test_token_is_never_accepted_for_another_endpoint(self):
        def token(url):
            return self.token({"url": url})
        for good in ["https://setec-astronomy-artifacts.s3.us-east-2.amazonaws.com/",
                     "https://setec-astronomy-artifacts.s3.us-west-2.amazonaws.com/",
                     "https://s3.eu-central-1.amazonaws.com/setec-astronomy-artifacts"]:
            with patch.dict(os.environ, {"CHECKPOINT_UPLOAD_TOKEN": token(good)}):
                custody.token_destination()
        for url in ["https://other.s3.amazonaws.com/", "http://setec-astronomy-artifacts.s3.amazonaws.com/",
                    "https://setec-astronomy-artifacts.s3.amazonaws.com.evil.invalid/",
                    "https://setec-astronomy-artifacts.s3.amazonaws.com/?token=private"]:
            with patch.dict(os.environ, {"CHECKPOINT_UPLOAD_TOKEN": token(url)}):
                with self.assertRaises(ValueError) as error:
                    custody.token_destination()
                self.assertNotIn(url, str(error.exception))

    def token(self, changes=None):
        value = {"url": "https://setec-astronomy-artifacts.s3.us-east-2.amazonaws.com/",
                 "prefix": "checkpoints/", "fields": {
                     "x-amz-algorithm": "AWS4-HMAC-SHA256",
                     "x-amz-credential": "SYNTHETIC/20261106/us-east-2/s3/aws4_request",
                     "x-amz-date": "20261106T120000Z", "policy": "e30=",
                     "x-amz-signature": "0" * 64}}
        value.update(changes or {})
        return base64.b64encode(json.dumps(value).encode()).decode()

    def test_token_cannot_add_files_override_keys_or_inject_curl_form_syntax(self):
        original = json.loads(base64.b64decode(self.token()))
        mutations = [dict(original["fields"], key="elsewhere/private"),
                     dict(original["fields"], file="@/private"),
                     dict(original["fields"], policy="@/private"),
                     dict(original["fields"], policy="e30=;type=text/plain"),
                     dict(original["fields"], policy="e30=\nprivate"),
                     dict(original["fields"], policy=1), {}]
        for fields in mutations:
            with patch.dict(os.environ, {"CHECKPOINT_UPLOAD_TOKEN": self.token({"fields": fields})}):
                with self.assertRaises(ValueError):
                    custody.token_destination()
        with patch.dict(os.environ, {"CHECKPOINT_UPLOAD_TOKEN": self.token({"prefix": "other/"})}):
            with self.assertRaises(ValueError):
                custody.token_destination()

    def test_source_namespace_is_fixed_even_without_an_upload_token(self):
        for field, value in [("CHECKPOINT_BUCKET", "other"),
                             ("CHECKPOINT_PREFIX", "other"),
                             ("CHECKPOINT_AWS_REGION", "other")]:
            with patch.dict(os.environ, {"CHECKPOINT_UPLOAD_TOKEN": "", field: value}):
                with self.assertRaisesRegex(ValueError, "namespace differs"):
                    custody.token_destination()

    def test_failed_gate_cannot_be_reported_as_uploaded(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            log = root / "private.log"
            log.write_text("checkpoint: no way to upload: no credential\n")
            value = custody.finish(root, "0" * 40, 1, log)
            self.assertFalse(value["checkpoint_uploaded"])
            self.assertEqual(value["failure_class"], "missing_auth")
            self.assertNotIn("no credential", json.dumps(value))
            with self.assertRaises(FileNotFoundError):
                custody.finish(root, "0" * 40, 0, log)

    def test_success_requires_the_standard_gate_receipt_in_the_approved_prefix(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / ".resume").mkdir()
            receipt = root / ".resume/.last-push"
            receipt.write_text("0" * 64 + " stamp " + custody.DESTINATION + "source/stamp/\n")
            value = custody.finish(root, "0" * 40, 0, root / "absent.log")
            self.assertTrue(value["checkpoint_uploaded"])
            self.assertFalse(value["stored_objects_independently_read_back"])
            self.assertFalse(value["benchmark_launched"])
            receipt.write_text("0" * 64 + " stamp s3://other/checkpoints/\n")
            with self.assertRaises(ValueError):
                custody.finish(root, "0" * 40, 0, root / "absent.log")

    def test_real_git_custody_closes_the_snapshot_without_copying_the_ledger(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source, _ = self.repository(root)
            with patch.dict(os.environ, {"CHECKPOINT_UPLOAD_TOKEN": ""}):
                value = custody.prepare(root, source)
            self.assertEqual(value["source_files"], 3)
            self.assertEqual(value["unpushed_commits"], 0)
            self.assertFalse((root / custody.CAPTURE / "pcc").exists())
            self.assertIn(source, (root / ".resume/RESUME.md").read_text())
            self.assertFalse((root / ".resume/.last-push").exists())

    def test_a_changed_native_ledger_blocks_even_a_clean_published_checkout(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source, _ = self.repository(root, ledger_change=True)
            with self.assertRaisesRegex(ValueError, "native ledger differs"):
                custody.prepare(root, source)
            self.assertFalse((root / custody.CAPTURE).exists())

    def test_unpushed_history_is_refused_before_any_payload_is_created(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source, baseline = self.repository(root)
            subprocess.run(["git", "-C", str(root), "update-ref", "refs/remotes/origin/source", baseline],
                           check=True)
            with self.assertRaisesRegex(ValueError, "unpushed history"):
                custody.prepare(root, source)
            self.assertFalse((root / custody.CAPTURE).exists())

    def test_manifest_cannot_smuggle_unreviewed_metadata_into_its_copied_bytes(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source, _ = self.repository(root, extra_manifest=True)
            with self.assertRaisesRegex(ValueError, "unsupported metadata"):
                custody.prepare(root, source)
            self.assertFalse((root / custody.CAPTURE).exists())


if __name__ == "__main__":
    unittest.main()
