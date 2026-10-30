"""Local Python boundary checks only; no native CLI, solver, build, tar or cloud calls.

Archive subprocess outcomes are explicitly mocked; they are not archive evidence.
All generated fixture files live in temporary directories and are removed after each test.
"""
import copy
import json
from pathlib import Path
import subprocess
import tempfile
import types
import unittest
from unittest.mock import patch

from common import canonical, load, sha, write
from driver import (Driver, checked_censored_prefix, corrupt_propagation,
                    fixture_exit_code, CHILD_ENVIRONMENT, Stop)
from verify import EXPORT_ROLES, checked_export_inventory, transition

SCRATCH = Path(__file__).resolve().parent


class CorruptionChecks(unittest.TestCase):
    def event(self, sequence, pattern=1, reason=None):
        return {"sequence": sequence, "pattern": pattern,
                "native": {"kind": "assignment", "initial": False,
                           "literal": 3, "reason": reason or {"kind": "clause", "clause_id": 8}}}

    def test_clause_mutation_keeps_real_reason_and_forges_unit_implication(self):
        events = [self.event(2, pattern=0), self.event(9)]
        before = copy.deepcopy(events)
        recipe = corrupt_propagation(events, {"pattern": 1, "decision_after": 12})
        self.assertEqual(events[0], before[0])
        self.assertEqual(events[1]["native"]["reason"], {"kind": "clause", "clause_id": 8})
        self.assertEqual(events[1]["native"]["literal"], -3)
        # The actual clause [-1,3] under x1=true implies +3, never -3.
        self.assertIn(before[1]["native"]["literal"], [-1, 3])
        self.assertNotIn(events[1]["native"]["literal"], [-1, 3])
        self.assertEqual(recipe["expected_error"],
                         "SAT pattern 1 event 9: reason is not unit on preceding trail")
        self.assertEqual(recipe["original_assignment"], before[1]["native"])

    def test_xor_mutation_changes_checked_explanation_not_literal_or_reference(self):
        events = [self.event(9, reason={"kind": "xor", "explanation": [3, -1]})]
        recipe = corrupt_propagation(events, {"pattern": 1, "decision_after": 12})
        self.assertEqual(events[0]["native"]["literal"], 3)
        self.assertEqual(events[0]["native"]["reason"], {"kind": "xor", "explanation": [-3, -1]})
        self.assertEqual(recipe["expected_error"], "SAT pattern 1 event 9: unchecked XOR explanation")

    def test_no_other_pattern_or_future_propagation_substitutes_for_control(self):
        for events in ([self.event(2, pattern=0)], [self.event(20)]):
            with self.assertRaisesRegex(ValueError, "lacks a propagation"):
                corrupt_propagation(events, {"pattern": 1, "decision_after": 12})


class CensorChecks(unittest.TestCase):
    def setUp(self):
        self.before = {"pattern": 1, "decision_sequence": 3, "assignment_sequence": 4, "feature_id": "first"}
        self.after = {"pattern": 1, "decision_sequence": 11, "assignment_sequence": 12, "feature_id": "last"}
        self.audit = {"decision_index": [self.before, self.after]}
        self.witness = {"pattern": 1}

    def changed(self, dynamic=False):
        return {"decision_index": [self.before, self.after] if dynamic else [self.before],
                "unknown_suffixes": [{"scope": "pattern", "pattern": 1,
                  "event_sequence": 13 if dynamic else None,
                  "reason": {"kind": "unsupported", "transition": "dynamic_clause"} if dynamic else {"kind": "truncated"},
                  "work": {"raw_events": 13 if dynamic else 11}}]}

    def check(self, value, dynamic=False):
        return checked_censored_prefix(self.audit, value, {"cut_sequence": 12 if dynamic else 11}, self.witness,
                                       "dynamic-clause" if dynamic else "unpaired-decision")

    def test_exact_censor_accepts_both_frozen_controls(self):
        for dynamic in (False, True):
            result = self.check(self.changed(dynamic), dynamic)
            self.assertTrue(result["passed"])
            self.assertEqual(result["preserved_decision_occurrences"], 2 if dynamic else 1)

    def test_wrong_pattern_scope_sequence_or_byte_prefix_rejected(self):
        for dynamic in (False, True):
            for key, value in (("pattern", 0), ("scope", "global"), ("event_sequence", 99),
                               ("reason", {"kind": "truncated", "extra": True}), ("work", {"raw_events": 100})):
                changed = self.changed(dynamic)
                changed["unknown_suffixes"][0][key] = value
                with self.subTest(dynamic=dynamic, key=key), self.assertRaisesRegex(ValueError, "exact mutated-pattern"):
                    self.check(changed, dynamic)

    def test_duplicate_or_changed_or_extra_decisions_rejected(self):
        for mode in ("duplicate", "changed", "suffix", "empty"):
            changed = self.changed()
            if mode == "duplicate":
                changed["decision_index"] *= 2
            elif mode == "changed":
                changed["decision_index"] = [dict(self.before, feature_id="altered")]
            elif mode == "suffix":
                changed["decision_index"].append(self.after)
            else:
                changed["decision_index"] = []
            with self.subTest(mode=mode), self.assertRaises(ValueError):
                self.check(changed)

    def test_earlier_unrelated_censor_cannot_replace_expected_terminal(self):
        changed = self.changed()
        changed["unknown_suffixes"][0].update(pattern=0, work={"raw_events": 2})
        with self.assertRaisesRegex(ValueError, "exact mutated-pattern"):
            self.check(changed)


class ManifestChecks(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="manifest-test-", dir=SCRATCH)
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.manifest = {"artifacts": []}
        for name, role in EXPORT_ROLES.items():
            path = self.root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(b'{}\n')
            self.manifest["artifacts"].append({"path": name, "sha256": sha(path), "bytes": 3,
                                                "role": role, "encoding": "jsonl"})
        for key in ("prefix_feature_occurrences", "unique_prefix_feature_rows", "restricted_prefix_record_rows",
                    "algebraic_feature_occurrences", "unique_algebraic_feature_rows", "restricted_algebraic_record_rows"):
            self.manifest[key] = 0
        self.manifest["algebraic_outcomes"] = {key: 0 for key in (
            "verified_nonzero_target_rows", "verified_zero_target_rows", "witnessed_degree_reduction_rows", "bounded_unknown_suffix_records")}

    def test_exact_nine_hashed_artifacts_accepted(self):
        self.assertEqual(len(checked_export_inventory(self.root, self.manifest)), 9)

    def test_omitted_restricted_manifest_binding_rejected(self):
        for name in ("restricted/targets.jsonl", "restricted/index.jsonl", "restricted/prefix-records.jsonl",
                     "restricted/algebraic-records.jsonl"):
            bad = copy.deepcopy(self.manifest)
            bad["artifacts"] = [a for a in bad["artifacts"] if a["path"] != name]
            with self.subTest(name=name), self.assertRaisesRegex(ValueError, "exactly all nine"):
                checked_export_inventory(self.root, bad)

    def test_role_encoding_hash_extra_member_and_zero_counters_rejected(self):
        for field, value in (("role", "wrong"), ("encoding", "json"), ("sha256", "0"*64)):
            bad = copy.deepcopy(self.manifest)
            bad["artifacts"][0][field] = value
            with self.subTest(field=field), self.assertRaises(ValueError):
                checked_export_inventory(self.root, bad)
        extra = self.root / "unexpected.jsonl"
        extra.write_bytes(b'{}\n')
        bad = copy.deepcopy(self.manifest)
        bad["artifacts"].append({"path": extra.name, "sha256": sha(extra), "bytes": 3, "role": "extra", "encoding": "jsonl"})
        with self.assertRaisesRegex(ValueError, "exactly all nine"):
            checked_export_inventory(self.root, bad)
        for key, value in (("prefix_feature_occurrences", 1), ("restricted_algebraic_record_rows", False)):
            bad = copy.deepcopy(self.manifest)
            bad[key] = value
            with self.subTest(key=key), self.assertRaisesRegex(ValueError, "unrelated channel"):
                checked_export_inventory(self.root, bad)


class ArchiveChecks(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="archive-test-", dir=SCRATCH)
        self.addCleanup(self.temporary.cleanup)
        self.out = Path(self.temporary.name) / "attempt"
        self.out.mkdir()
        (self.out / "payload.txt").write_text("retained mock-test payload")
        self.driver = Driver(self.out, self.out / "not-executed", self.out / "not-used.json")
        self.driver.receipt["status"] = "native_fixture_passed_unified_export_pending"
        self.driver.receipt["fixture_bindings"] = {"unified_export_tooling": {
            "path": "mock-tooling-not-executed", "checker_python": "mock-python-not-executed"}}
        self.export_patch = patch("driver.export_attempt", return_value={
            "status": "passed", "fixture_acceptance": True, "software_unit_mock": True})
        self.export_mock = self.export_patch.start()
        self.addCleanup(self.export_patch.stop)
        self.driver.checkpoint()

    def assert_custody(self, complete, expected_status=None, before_retention="native_fixture_passed"):
        receipt = load(self.out / "receipt.json")
        inv = load(self.out / "inventory.json")
        self.assertNotIn("inventory.json", inv["files"])
        for name, artifact in inv["files"].items():
            self.assertEqual(artifact, {"sha256": sha(self.out/name), "bytes": (self.out/name).stat().st_size})
        external = load(self.out.with_name("attempt.archive.json"))
        self.assertEqual(external["source_receipt_sha256"], sha(self.out / "receipt.json"))
        self.assertEqual(external["source_inventory_sha256"], sha(self.out / "inventory.json"))
        self.assertEqual(external["status"], "complete" if complete else "failed_or_interrupted")
        expected_status = expected_status or ("native_fixture_passed" if complete else "failed_retention")
        self.assertEqual(receipt["status"], expected_status)
        self.assertEqual(fixture_exit_code(receipt), 0 if complete and expected_status == "native_fixture_passed" else 2)
        if not complete:
            self.assertEqual(receipt["status"], "failed_retention")
            self.assertEqual(receipt["status_before_retention_failure"], before_retention)
            self.assertTrue(receipt["retention_error"])

    def test_mock_archive_nonzero_is_failed_and_partial_output_preserved(self):
        archive = self.out.with_name("attempt.tar")
        def failed(*args, **kwargs):
            archive.write_bytes(b"partial mock tar")
            return types.SimpleNamespace(returncode=2, stderr=b"mock archive failure")
        with patch("driver.subprocess.run", side_effect=failed):
            self.driver.finish()
        self.assertEqual(archive.read_bytes(), b"partial mock tar")
        self.assert_custody(False)

    def test_mock_timeout_and_no_remaining_budget_do_not_pass(self):
        with patch("driver.subprocess.run", side_effect=subprocess.TimeoutExpired(["mock-tar"], 1)):
            self.driver.finish()
        self.assert_custody(False)

    def test_no_remaining_budget_never_calls_archive(self):
        with patch.object(self.driver, "remaining", return_value=0), patch("driver.subprocess.run") as run:
            self.driver.finish()
            run.assert_not_called()
        self.export_mock.assert_not_called()
        self.assert_custody(False, before_retention="failed_unified_export")

    def test_existing_archive_is_preserved_and_rejected(self):
        archive = self.out.with_name("attempt.tar")
        archive.write_bytes(b"existing evidence")
        with patch("driver.subprocess.run") as run:
            self.driver.finish()
            run.assert_not_called()
        self.assertEqual(archive.read_bytes(), b"existing evidence")
        self.assert_custody(False)

    def test_mock_success_binds_final_inventory_and_receipt(self):
        def passed(*args, **kwargs):
            self.out.with_name("attempt.tar").write_bytes(b"mock complete tar; not a real archive")
            return types.SimpleNamespace(returncode=0, stderr=b"")
        with patch("driver.subprocess.run", side_effect=passed):
            self.driver.finish()
        self.assert_custody(True)
        self.export_mock.assert_called_once()
        self.assertEqual(load(self.out / "receipt.json")["unified_export_gate"]["status"], "passed")

    def successful_mock_archive(self, *args, **kwargs):
        self.out.with_name("attempt.tar").write_bytes(b"mock archive, never native tar")
        return types.SimpleNamespace(returncode=0, stderr=b"")

    def test_failed_unified_gate_cannot_be_rescued_by_successful_archive(self):
        self.export_mock.return_value = {"status": "failed", "error": "constructed checker rejection"}
        with patch("driver.subprocess.run", side_effect=self.successful_mock_archive):
            self.driver.finish()
        self.assert_custody(True, expected_status="failed_unified_export")
        self.assertEqual(load(self.out / "receipt.json")["unified_export_gate"]["status"], "failed")
        self.assertTrue((self.out / "payload.txt").is_file())

    def test_export_exception_retains_raw_evidence_and_fails_gate(self):
        self.export_mock.side_effect = ValueError("constructed admission rejection")
        with patch("driver.subprocess.run", side_effect=self.successful_mock_archive):
            self.driver.finish()
        self.assert_custody(True, expected_status="failed_unified_export")
        self.assertTrue(load(self.out / "receipt.json")["unified_export_gate"]["raw_evidence_retained"])

    def test_passing_schema_gate_with_unaccepted_fixture_is_not_success(self):
        self.export_mock.return_value = {"status": "passed", "fixture_acceptance": False}
        with patch("driver.subprocess.run", side_effect=self.successful_mock_archive):
            self.driver.finish()
        self.assert_custody(True, expected_status="failed_unified_export")

    def test_missing_fixture_stays_missing_after_valid_export_and_archive(self):
        self.driver.receipt["status"] = "missing_fixture"
        self.export_mock.return_value = {"status": "passed", "fixture_acceptance": False}
        with patch("driver.subprocess.run", side_effect=self.successful_mock_archive):
            self.driver.finish()
        self.assert_custody(True, expected_status="missing_fixture")
        self.export_mock.assert_called_once()


class ProtocolCustodyChecks(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(dir=SCRATCH)
        self.addCleanup(self.temp.cleanup)
        root = Path(self.temp.name)
        external = root / "external"
        external.mkdir()
        self.out = root / "attempt"
        self.out.mkdir()
        (self.out / "logs").mkdir()
        binary, protocol_path = external / "crypto", external / "protocol.json"
        binary.write_bytes(b"fake binary; never executable")
        protocol_path.write_bytes(b'{"frozen":"test bytes"}\n')
        receipt = {}
        for label in ("checker", "stdout", "stderr", "schema_file"):
            source = external / label
            source.write_bytes((label + " exact retained evidence\n").encode())
            receipt[label] = {"path": label, "sha256": sha(source)}
        receipt_path = external / "protocol.validation.json"
        write(receipt_path, receipt)
        self.protocol = {"extensions": {"ml_dataset_sat_fixture": {"binary_sha256": sha(binary)}}}
        self.binding = {"validation_receipt": str(receipt_path),
                        "validation_receipt_sha256": sha(receipt_path), "file_sha256": sha(protocol_path)}
        self.driver = Driver(self.out, binary, protocol_path)
        self.external = external

    def retain(self):
        self.driver.retain_protocol(self.protocol, self.binding)

    def test_launcher_thread_environment_matches_frozen_child_environment(self):
        launcher = (SCRATCH / "run.sh").read_text()
        for key, value in CHILD_ENVIRONMENT.items():
            self.assertIn("export " + key + "=" + value + "\n", launcher)
        self.assertNotIn("RAYON_NUM_THREADS=8", launcher)

    def test_retains_exact_evidence_and_uses_local_protocol_when_original_changes(self):
        self.retain()
        expected = {item["label"]: item for item in load(self.out / "protocol-evidence/retained.json")["files"]}
        self.assertEqual(set(expected), {"binary", "protocol", "validation_receipt", "checker", "stdout", "stderr", "schema_file"})
        for item in expected.values():
            self.assertEqual((self.out / item["path"]).read_bytes(), Path(item["original_path"]).read_bytes())
        self.assertEqual(self.driver.protocol_path, self.out / "protocol.json")
        for source in self.external.iterdir():
            source.write_bytes(b"external source now changed")
        self.driver.check_frozen_custody()
        with patch("driver.subprocess.Popen") as popen:
            popen.return_value.wait.return_value = 0
            row = self.driver.command("mock-preflight", ["dataset", "preflight", "--protocol", self.driver.protocol_path], 1)
        self.assertEqual(row["argv"][-1], str(self.out / "protocol.json"))
        self.assertEqual(row["child_environment_overrides"], {"RAYON_NUM_THREADS": "1", "OMP_NUM_THREADS": "1"})
        for key, value in CHILD_ENVIRONMENT.items():
            self.assertEqual(popen.call_args.kwargs["env"][key], value)
        self.assertEqual(row["frozen_custody_after"], "passed")

    def test_changed_admitted_receipt_or_source_fails_before_commands(self):
        for target in (self.external / "protocol.validation.json", self.external / "checker"):
            original = target.read_bytes()
            target.write_bytes(b"tampered")
            # Each attempt must be a fresh output, as partial copies are retained.
            case = self.out / target.name
            case.mkdir()
            candidate = Driver(case, self.driver.original_binary, self.driver.protocol_path)
            with self.subTest(path=target.name), self.assertRaises(ValueError):
                candidate.retain_protocol(self.protocol, self.binding)
            target.write_bytes(original)

    def test_every_retained_file_is_checked_before_spawn(self):
        self.retain()
        for relative in self.driver.frozen_custody:
            target = self.out / relative
            original = target.read_bytes()
            target.chmod(0o600)
            target.write_bytes(b"tampered retained file")
            with self.subTest(path=relative), patch("driver.subprocess.Popen") as popen, self.assertRaises(ValueError):
                self.driver.command("never-start", [], 1)
            popen.assert_not_called()
            target.write_bytes(original)
        self.driver.check_frozen_custody()

    def test_changed_binary_after_command_fails_and_is_retained(self):
        self.retain()
        def changed(timeout):
            self.driver.binary.chmod(0o600)
            self.driver.binary.write_bytes(b"changed during mock command")
            return 0
        with patch("driver.subprocess.Popen") as popen:
            popen.return_value.wait.side_effect = changed
            with self.assertRaisesRegex(Stop, "custody changed"):
                self.driver.command("mock-tamper", [], 1)
        row = load(self.out / "receipt.json")["commands"][0]
        self.assertEqual(row["exit_code"], 0)
        self.assertEqual(row["frozen_custody_after"], "failed")
        self.assertIn("frozen-crypto", row["custody_error"])

    def test_nonzero_command_still_checks_protocol_and_retains_failure(self):
        self.retain()
        def changed(timeout):
            self.driver.protocol_path.chmod(0o600)
            self.driver.protocol_path.write_bytes(b"changed")
            return 2
        with patch("driver.subprocess.Popen") as popen:
            popen.return_value.wait.side_effect = changed
            with self.assertRaisesRegex(Stop, "custody changed"):
                self.driver.command("mock-nonzero", [], 1, expected_failure=True)
        row = load(self.out / "receipt.json")["commands"][0]
        self.assertEqual(row["status"], "nonzero_exit")
        self.assertEqual(row["frozen_custody_after"], "failed")

    def test_symlink_replacement_rejected_even_when_bytes_match(self):
        self.retain()
        target = self.out / "protocol-evidence/checker"
        target.unlink()
        target.symlink_to(self.external / "checker")
        with self.assertRaisesRegex(ValueError, "custody mismatch"):
            self.driver.check_frozen_custody()


if __name__ == "__main__":
    unittest.main(verbosity=2)
