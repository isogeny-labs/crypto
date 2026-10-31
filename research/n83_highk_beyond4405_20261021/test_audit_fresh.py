"""Check the artifact migration preserves the sealed exclusion input."""

import hashlib
import json
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch

from research.n83_highk_beyond4405_20261021 import audit_fresh


class MigratedFixtureTests(unittest.TestCase):
    def test_deleted_head_blob_requires_matching_manifest_and_ancestor(self):
        path = "research/n83_highk_online_tiebreak_20261020/screen_targets.jsonl"
        blob = b'["1", "2"]\n'
        sha = hashlib.sha256(blob).hexdigest()
        missing = subprocess.CalledProcessError(128, ["git", "show"])
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            manifest = root / Path(path).parent / ".artifacts.json"
            manifest.parent.mkdir(parents=True)
            manifest.write_text(json.dumps({
                "schema": "artifacts.manifest/v1",
                "files": {Path(path).name: {"sha256": sha, "bytes": len(blob)}},
            }))
            with patch.object(audit_fresh, "ROOT", root), patch.object(
                    audit_fresh.subprocess, "check_output",
                    side_effect=[missing, blob]) as show:
                self.assertEqual(audit_fresh.verified_git_points("HEAD", path, sha),
                                 [(1, 2)])
                self.assertEqual(show.call_args_list[1].args[0][2],
                                 f"{audit_fresh.ARTIFACT_MIGRATION_PARENT}:{path}")
            with patch.object(audit_fresh, "ROOT", root), patch.object(
                    audit_fresh.subprocess, "check_output", side_effect=missing):
                with self.assertRaisesRegex(ValueError, "artifact manifest differs"):
                    audit_fresh.verified_git_points("HEAD", path, "0" * 64)


if __name__ == "__main__":
    unittest.main()
