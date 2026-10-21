"""Check that scientific admission requires a complete provider exit trail."""

import hashlib
import io
import json
import tarfile
import tempfile
import unittest
from pathlib import Path
from unittest import mock

from research.n83_highk_beyond4405_20261021 import verify_provider_custody


RECEIPT = {"pod_id": "pod123", "created_utc": "2030-01-01T00:00:00Z",
           "stop_after_utc": "2030-01-01T10:00:00Z"}


def write(path: Path, data: dict):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, sort_keys=True) + "\n")


def fixture(root: Path) -> tuple[Path, Path]:
    allocation = root / "allocation.json"
    archive = root / "archive"
    archive.mkdir()
    write(allocation, RECEIPT)
    digest = hashlib.sha256(allocation.read_bytes()).hexdigest()
    started = {"schema": "n83.fixed31-watchdog-start/v1",
               "pod_id": "pod123", "allocation_sha256": digest,
               "recorded_utc": "2030-01-01T00:01:00Z",
               "provider_deadline_utc": "2030-01-01T10:00:00Z",
               "retire_at_utc": "2030-01-01T09:55:00Z"}
    ready = {"schema": "n83.fixed31-watchdog-ready/v1",
             "pod_id": "pod123", "allocation_sha256": digest,
             "recorded_utc": "2030-01-01T00:02:00Z",
             "retire_at_utc": "2030-01-01T09:55:00Z"}
    transfer = {"pod_id": "pod123", "recorded_utc": "2030-01-01T02:00:00Z",
                "reason": "PRODUCER_DONE", "spool_succeeded": True,
                "archive_ready": True, "error_type": None}
    absent = {"pod_id": "pod123", "recorded_utc": "2030-01-01T02:01:00Z",
              "pod_present": False}
    for name, data in (("watchdog_started", started),
                       ("watchdog_ready", ready),
                       ("watchdog_transfer", transfer),
                       ("provider_post_terminate", absent)):
        write(archive / f"{name}.json", data)
    write(archive / "results" / "runpod_allocation.json", RECEIPT)
    files = {
        "watchdog_ready.json": (archive / "watchdog_ready.json").read_bytes(),
        "results/runpod_allocation.json":
            (archive / "results" / "runpod_allocation.json").read_bytes(),
        "results/source.json": b"{}\n",
        "results/host.json": b"{}\n",
        "results/screen_analysis.json":
            b'{"status":"SCREEN_ADMITTED"}\n',
        "results/holdout/analysis.json":
            b'{"primary_status":"PASS_VERIFIED_CROSS_FAMILY_TIER1",'
            b'"cross_family_selector":{"status":"SELECTED"}}\n',
    }
    tar_path = archive / "raw_results.tar.gz"
    with tarfile.open(tar_path, "w:gz") as tar:
        for name, content in files.items():
            info = tarfile.TarInfo(name)
            info.size = len(content)
            tar.addfile(info, io.BytesIO(content))
    (archive / "raw_results.sha256").write_text(
        hashlib.sha256(tar_path.read_bytes()).hexdigest() +
        "  raw_results.tar.gz\n")
    return allocation, archive


class ProviderCustodyTests(unittest.TestCase):
    def test_verified_archive_and_provider_absence_pass(self):
        with tempfile.TemporaryDirectory() as temp:
            allocation, archive = fixture(Path(temp))
            with mock.patch.object(verify_provider_custody.runpod_runner,
                                   "read_allocation", return_value=RECEIPT):
                result = verify_provider_custody.validate(allocation, archive)
        self.assertEqual(result["status"], "PASS_PROVIDER_CUSTODY")
        self.assertTrue(all(result["checks"].values()))
        self.assertEqual(result["final_primary_status"],
                         "PASS_VERIFIED_CROSS_FAMILY_TIER1")

    def test_missing_absence_or_modified_archive_is_unknown(self):
        with tempfile.TemporaryDirectory() as temp:
            allocation, archive = fixture(Path(temp))
            (archive / "provider_post_terminate.json").unlink()
            with mock.patch.object(verify_provider_custody.runpod_runner,
                                   "read_allocation", return_value=RECEIPT):
                missing = verify_provider_custody.validate(allocation, archive)
            self.assertEqual(missing["status"], "BOUNDED_UNKNOWN")
            self.assertEqual(missing["final_primary_status"],
                             "BOUNDED_UNKNOWN")
            write(archive / "provider_post_terminate.json", {
                "pod_id": "pod123", "recorded_utc": "2030-01-01T02:01:00Z",
                "pod_present": False})
            with (archive / "raw_results.tar.gz").open("ab") as out:
                out.write(b"changed")
            with mock.patch.object(verify_provider_custody.runpod_runner,
                                   "read_allocation", return_value=RECEIPT):
                modified = verify_provider_custody.validate(allocation, archive)
        self.assertEqual(modified["status"], "BOUNDED_UNKNOWN")
        self.assertFalse(modified["checks"]["archive_sha256"])


if __name__ == "__main__":
    unittest.main()
