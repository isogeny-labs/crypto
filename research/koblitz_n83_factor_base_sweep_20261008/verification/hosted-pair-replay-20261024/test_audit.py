"""Fail-closed controls for the retained hosted-run custody audit."""

from __future__ import annotations

from io import BytesIO
import json
from pathlib import Path
import sys
import unittest
from zipfile import ZipFile


HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import audit  # noqa: E402


class HostedPairAuditTests(unittest.TestCase):
    def test_provider_rejects_changed_job_conclusion(self) -> None:
        jobs = json.loads((HERE / "provider_jobs.json").read_text())
        jobs["jobs"][0]["conclusion"] = "failure"
        metadata = (HERE / "artifact_metadata.json").read_bytes()
        with self.assertRaisesRegex(ValueError, "provider job conclusion changed"):
            audit.provider(json.dumps(jobs).encode(), metadata, 10283519)

    def test_rejects_duplicate_member_name(self) -> None:
        stream = BytesIO()
        with ZipFile(stream, "w") as archive:
            archive.writestr("n83-hosted-pair/outer.json", "{}")
            archive.writestr("n83-hosted-pair/outer.json", "{}")
        with ZipFile(BytesIO(stream.getvalue())) as archive:
            with self.assertRaisesRegex(ValueError, "unsafe or duplicate ZIP member"):
                audit.member_bytes(archive)

    def test_rejects_parent_path(self) -> None:
        stream = BytesIO()
        with ZipFile(stream, "w") as archive:
            archive.writestr("../outer.json", "{}")
        with ZipFile(BytesIO(stream.getvalue())) as archive:
            with self.assertRaisesRegex(ValueError, "unsafe or duplicate ZIP member"):
                audit.member_bytes(archive)


if __name__ == "__main__":
    unittest.main()
