"""Keep live producer evolution separate from immutable R0 evidence."""
import hashlib
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location(
    "frozen_report", Path(__file__).with_name("check_frozen_report.py"))
report = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(report)


class FrozenReportControls(unittest.TestCase):
    def setUp(self):
        self.scratch = tempfile.TemporaryDirectory()
        self.addCleanup(self.scratch.cleanup)
        self.root = Path(self.scratch.name)
        files = {p: "original producer" for p in report.PRODUCER_INPUTS}
        files["research/control/records.jsonl"] = "original failed and successful records\n"
        self.lock = {"files": {}, "outputs": {"leaderboard.json": ""}}
        for name, content in files.items():
            path = self.root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(content)
            self.lock["files"][name] = report.sha256(path.read_bytes())
        path = self.root / report.ARCHIVE / "leaderboard.json"
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text('{"frozen": true}\n')
        self.lock["outputs"]["leaderboard.json"] = report.sha256(path.read_bytes())

    def test_current_producer_may_advance_while_frozen_data_remains_identical(self):
        report.verify_live_evidence(self.root, self.lock)
        (self.root / "src/cryptanalysis/ecbench/methods.rs").write_text("new current code")
        report.verify_live_evidence(self.root, self.lock)

    def test_changed_scientific_record_is_refused(self):
        (self.root / "research/control/records.jsonl").write_text("replacement")
        with self.assertRaisesRegex(ValueError, "frozen scientific input changed"):
            report.verify_live_evidence(self.root, self.lock)

    def test_changed_published_output_is_refused(self):
        (self.root / report.ARCHIVE / "leaderboard.json").write_text('{}')
        with self.assertRaisesRegex(ValueError, "frozen report output changed"):
            report.verify_live_evidence(self.root, self.lock)

    def test_new_current_report_does_not_replace_the_pinned_historical_output(self):
        current = self.root / "docs/backend-leaderboard/leaderboard.json"
        current.parent.mkdir(parents=True, exist_ok=True)
        current.write_text('{"new_producer_revision": true}\n')
        report.verify_live_evidence(self.root, self.lock)
        (self.root / report.ARCHIVE / "leaderboard.json").write_bytes(current.read_bytes())
        with self.assertRaisesRegex(ValueError, "frozen report output changed"):
            report.verify_live_evidence(self.root, self.lock)

    def test_incomplete_producer_inventory_is_refused(self):
        self.lock["files"].pop("src/cryptanalysis/ecbench/methods.rs")
        with self.assertRaisesRegex(ValueError, "producer inventory incomplete"):
            report.verify_live_evidence(self.root, self.lock)

    def test_input_path_escape_is_refused(self):
        self.lock["files"]["../escape"] = "0" * 64
        with self.assertRaisesRegex(ValueError, "unsafe frozen report path"):
            report.verify_live_evidence(self.root, self.lock)

    def test_modified_lock_cannot_supply_its_own_trusted_hash(self):
        path = self.root / report.LOCK
        changed = json.dumps(self.lock).encode()
        path.write_bytes(changed)
        with patch.object(report, "ROOT", self.root), \
                patch.object(report.subprocess, "check_output", return_value=changed), \
                patch.object(report, "run") as run:
            with self.assertRaisesRegex(ValueError, "reviewed frozen report lock changed"):
                report.main()
            run.assert_not_called()


if __name__ == "__main__":
    unittest.main()
