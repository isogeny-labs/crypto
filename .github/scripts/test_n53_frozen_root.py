"""Offline regression coverage for complete, tamper-detecting N53 source reports."""
from __future__ import annotations

import hashlib
import json
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch

import n53_frozen_root as frozen_root


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


class FrozenRootReportTests(unittest.TestCase):
    def test_verify_rejects_omitted_rows_changed_bytes_and_unrecorded_build_inputs(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            repo = root / "repo"
            repo.mkdir()
            subprocess.run(["git", "init", "-q"], cwd=repo, check=True)
            subprocess.run(["git", "config", "user.name", "Fixture"], cwd=repo, check=True)
            subprocess.run(["git", "config", "user.email", "fixture@example.invalid"], cwd=repo, check=True)
            (repo / "tracked").write_bytes(b"head\n")
            subprocess.run(["git", "add", "tracked"], cwd=repo, check=True)
            subprocess.run(["git", "commit", "-qm", "fixture"], cwd=repo, check=True)
            head = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=repo, text=True).strip()
            target = root / "source-root"
            target.mkdir()
            baseline = b"baseline bytes\n"
            pinned = b"pinned bytes\n"
            (target / "base.rs").write_bytes(baseline)
            (target / "pin.rs").write_bytes(pinned)
            report = root / "selection.json"
            value = {
                "schema": "n53-frozen-source-root-v2",
                "checkout_head": head,
                "source_root": str(target),
                "build_baseline_commit": frozen_root.BUILD_BASE,
                "baseline_files": [{"path": "base.rs", "sha256": sha(baseline),
                                    "source_commit": frozen_root.BUILD_BASE}],
                "selected_sources": [{"path": "pin.rs", "key": "pin",
                                      "source_commit": head, "expected_sha256": sha(pinned)}],
                "current_code_parity_status": "not_assessed",
            }
            report.write_text(json.dumps(value))
            with patch.object(frozen_root, "archive_hashes", return_value={"base.rs": sha(baseline)}), \
                 patch.object(frozen_root, "manifest_entries", return_value=({}, {"pin.rs": (pinned, "pin", head)})):
                frozen_root.verify(repo, target, report)
                value["baseline_files"] = []
                report.write_text(json.dumps(value))
                with self.assertRaisesRegex(AssertionError, "baseline source inventory"):
                    frozen_root.verify(repo, target, report)
                value["baseline_files"] = [{"path": "base.rs", "sha256": sha(baseline),
                                             "source_commit": frozen_root.BUILD_BASE}]
                value["selected_sources"] = []
                report.write_text(json.dumps(value))
                with self.assertRaisesRegex(AssertionError, "inventory is incomplete"):
                    frozen_root.verify(repo, target, report)
                value["selected_sources"] = [{"path": "pin.rs", "key": "pin",
                                               "source_commit": head, "expected_sha256": sha(pinned)}]
                report.write_text(json.dumps(value))
                (target / "pin.rs").write_bytes(b"changed\n")
                with self.assertRaisesRegex(AssertionError, "selected frozen source changed"):
                    frozen_root.verify(repo, target, report)
                (target / "pin.rs").write_bytes(pinned)
                (target / "build.rs").write_text("fn main() {}\n")
                report.write_text(json.dumps(value))
                with self.assertRaisesRegex(AssertionError, "unrecorded or missing"):
                    frozen_root.verify(repo, target, report)
                (target / "build.rs").unlink()
                (target / ".cargo").mkdir()
                (target / ".cargo/config.toml").write_text("[build]\n")
                with self.assertRaisesRegex(AssertionError, "unrecorded or missing"):
                    frozen_root.verify(repo, target, report)

    def test_head_metadata_reads_committed_bytes_when_worktree_is_dirty(self):
        with tempfile.TemporaryDirectory() as temp:
            repo = Path(temp)
            subprocess.run(["git", "init", "-q"], cwd=repo, check=True)
            subprocess.run(["git", "config", "user.name", "Fixture"], cwd=repo, check=True)
            subprocess.run(["git", "config", "user.email", "fixture@example.invalid"], cwd=repo, check=True)
            path = repo / "metadata.json"
            path.write_bytes(b'{"committed":true}\n')
            subprocess.run(["git", "add", "metadata.json"], cwd=repo, check=True)
            subprocess.run(["git", "commit", "-qm", "metadata"], cwd=repo, check=True)
            path.write_bytes(b'{"dirty":true}\n')
            self.assertEqual(frozen_root.head_bytes(repo, "metadata.json"), b'{"committed":true}\n')


if __name__ == "__main__":
    unittest.main()
