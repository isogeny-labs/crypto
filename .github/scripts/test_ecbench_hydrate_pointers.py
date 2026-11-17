"""Real file-store recovery controls for frozen ecbench pointers."""
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location("hydrate_pointers", Path(__file__).with_name("ecbench-hydrate-pointers.py"))
loader = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(loader)


class Recovery(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        base = Path(self.tmp.name).resolve()
        self.root = base / "repo"
        self.root.mkdir()
        subprocess.run(["git", "init", "-q", str(self.root)], check=True)
        self.directory = self.root / "results/session"
        self.directory.mkdir(parents=True)
        self.payload = b"original sealed records\n"
        self.digest = hashlib.sha256(self.payload).hexdigest()
        self.remote, self.cache = base / "remote", base / "cache"
        obj = self.remote / "sha256" / self.digest[:2] / self.digest
        obj.parent.mkdir(parents=True)
        obj.write_bytes(self.payload)
        self.obj = obj
        self.source = self.directory / "records.jsonl.pointer"
        self.source.write_text(f"{self.digest}  session/records.jsonl  {len(self.payload)} bytes  in s3://archive/results.tgz (out/session/records.jsonl)\n")
        self.capsule = self.directory / "session.json"
        self.capsule.write_text(json.dumps({"schema":"ecbench.session/v1", "records_sha256":self.digest}))
        self.dest = self.directory / "records.jsonl"
        self.environment = patch.dict(os.environ, {"ARTIFACTS_REMOTE":"file://" + str(self.remote), "ARTIFACTS_CACHE":str(self.cache)})
        self.environment.start()
        self.addCleanup(self.environment.stop)

    def hydrate(self):
        return loader.hydrate(self.root, ["results/session"])

    def test_exact_bytes_recovered_without_changing_original_metadata(self):
        original = self.source.read_bytes(), self.capsule.read_bytes()
        self.assertEqual(self.hydrate(), 1)
        self.assertEqual(self.dest.read_bytes(), self.payload)
        self.assertEqual((self.source.read_bytes(), self.capsule.read_bytes()), original)
        self.assertEqual(self.hydrate(), 0)

    def test_corrupt_store_refused_before_destination_write(self):
        self.obj.write_bytes(b"wrong original bytes")
        with self.assertRaises(SystemExit):
            self.hydrate()
        self.assertFalse(self.dest.exists())

    def test_existing_local_bytes_are_never_replaced(self):
        self.dest.write_bytes(b"local edit")
        self.assertEqual(self.hydrate(), 0)
        self.assertEqual(self.dest.read_bytes(), b"local edit")

    def test_omitted_staged_source_and_case_alias_stay_owned(self):
        for name in ("records.jsonl", "RECORDS.JSONL"):
            with self.subTest(name=name):
                path = self.directory / name
                path.write_bytes(b"tracked successor")
                subprocess.run(["git", "-C", str(self.root), "add", str(path)], check=True)
                path.unlink()
                self.assertEqual(self.hydrate(), 0)
                self.assertFalse(self.dest.exists())
                subprocess.run(["git", "-C", str(self.root), "rm", "--cached", "-q", str(path)], check=True)

    def test_capsule_mismatch_refused(self):
        self.capsule.write_text(json.dumps({"schema":"ecbench.session/v1", "records_sha256":"0"*64}))
        with self.assertRaisesRegex(ValueError, "differs"):
            self.hydrate()
        self.assertFalse(self.dest.exists())

    def test_pointer_paths_and_metadata_refused(self):
        text = self.source.read_text()
        for bad in (text.replace("session/records", "../session/records"), text.replace("session/records", "other/records"), text.replace("out/session", "/out/session"), text.replace(" bytes", "oops bytes"), text + "extra\n"):
            with self.subTest(pointer=bad):
                self.source.write_text(bad)
                with self.assertRaises(ValueError):
                    self.hydrate()
                self.assertFalse(self.dest.exists())

    def test_pointer_symlink_refused(self):
        saved = self.source.with_suffix(".original")
        self.source.rename(saved)
        self.source.symlink_to(saved)
        with self.assertRaisesRegex(ValueError, "symbolic"):
            self.hydrate()

    def test_outside_session_directory_refused(self):
        saved = self.root.parent / "outside"
        self.directory.rename(saved)
        self.directory.symlink_to(saved, target_is_directory=True)
        with self.assertRaisesRegex(ValueError, "outside"):
            self.hydrate()
        self.assertFalse((saved / "records.jsonl").exists())

    def test_bad_cache_replaced_only_with_exact_remote_object(self):
        cached = self.cache / "sha256" / self.digest[:2] / self.digest
        cached.parent.mkdir(parents=True)
        cached.write_bytes(b"corrupt cached bytes")
        self.assertEqual(self.hydrate(), 1)
        self.assertEqual(cached.read_bytes(), self.payload)


if __name__ == "__main__":
    unittest.main()
