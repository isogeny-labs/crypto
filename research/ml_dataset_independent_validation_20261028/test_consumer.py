"""Adversarial format fixtures, not cryptographic experiments or training data."""
import copy
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest import mock

import consumer as c


class ConsumerTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name) / "run"
        self.root.mkdir()
        protocol = c.compact({"schema": "experiment.protocol/v1", "fixture": "format tests only"})
        self.plan = dict(zip(c.PLAN_FIELDS, ["crypto.ml-dataset.plan/v1", [23], [2], ["prime"],
                        ["bsgs"], 1, 1, 1, 20261027, 2, 3, 1000, True,
                        c.digest(protocol), "curve-twist-component-sha256-v1"]))
        self.plan_sha = c.digest(c.compact(self.plan))
        self.binding = {"schema": "crypto.ml-dataset.run/v1", "plan_sha256": self.plan_sha,
                        "source_commit": "a" * 40, "binary_sha256": "b" * 64,
                        "shard_index": 0, "shard_count": 1}
        (self.root / "plan.json").write_bytes(c.compact(self.plan))
        (self.root / "run.json").write_bytes(c.compact(self.binding))
        (self.root / "protocol.json").write_bytes(protocol)
        self.cell = self.root / "cells" / "000000000000"
        self.cell.mkdir(parents=True)
        self.config = c.config_at(self.plan, 0)
        self.members = {}
        for name in ("plan.json", "run.json", "protocol.json"):
            self.add(name, (self.root / name).read_bytes(), "restricted-provenance")
        self.add("config.json", self.config, "restricted-provenance")
        for name in ("attempt.json", "worker-process.json"):
            self.add(name, {}, "process-provenance")
        for name in ("stdout.log", "stderr.log"):
            self.add(name, b"", "process-log")
        self.outcome = {"status": "unsupported", "solver_completed": False}
        self.seal()

    def add(self, name, value, role):
        data = value if isinstance(value, bytes) else c.compact(value)
        path = self.cell / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data)
        self.members[name] = {"path": name, "role": role, "encoding": "json",
                              "bytes": len(data), "sha256": c.digest(data)}

    def seal(self):
        (self.cell / "manifest.json").write_bytes(c.compact({"schema": "crypto.ml-dataset.cell/v1",
            "cell_index": 0, "plan_sha256": self.plan_sha, "config": self.config,
            "outcome": self.outcome, "artifacts": list(self.members.values())}))

    def add_public(self):
        self.public = {"schema": "ml-ecdlp-instance/v1", "family": "prime", "n": 23, "m": 2,
            "twist": False, "split_component": "field-size-n23",
            "field": {"characteristic": "17", "degree": 1, "modulus": "17", "basis": "canonical-residue"},
            "curve": {"model": "short-weierstrass", "a": "1", "b": "1", "subgroup_order": "19"},
            "generator": {"x": "0", "y": "1"}, "target": {"x": "6", "y": "3"}}
        self.private = {"schema": "ml-ecdlp-private-witness/v1", "scalar": "PRIVATE-SCALAR",
            "sampling_provenance": {k: self.config[k] for k in c.PROVENANCE}}
        self.base = {"schema": "ml-ecdlp-factor-base/v1", "requested_size": 2,
            "rule": "small-x-cofactor-projection", "construction_status": "verified",
            "points": [{"x": "0", "y": "1"}, {"x": "0", "y": "16"}]}
        self.save_public()

    def save_public(self):
        self.add("public.json", self.public, "public-instance")
        self.add("witness.json", self.private, "restricted-known-answer")
        self.add("factor-base.json", self.base, "factor-base")
        self.seal()

    def reject(self, message):
        with self.assertRaisesRegex(ValueError, message):
            c.audit(self.root)

    def test_incomplete_and_censored_are_retained(self):
        (self.root / "cells" / "000000000001").mkdir()
        report, rows = c.audit(self.root)
        self.assertEqual(report["checked_cells"], 1)
        self.assertEqual(report["incomplete_attempts"], ["000000000001"])
        self.assertFalse(report["mathematics_independently_verified"])
        self.assertFalse(report["full_scope_complete"])
        self.assertEqual(rows[0]["outcome"]["status"], "unsupported")

    def test_fake_success_without_inputs_rejected(self):
        self.add("recovery.json", {}, "restricted-independent-scalar-label")
        self.outcome.update(status="verified", scalar_recovered_without_witness=True,
                            scalar_replay_verified=True, solver_completed=True)
        self.seal()
        self.reject("lacks public/witness/factor-base")

    def test_public_arity_and_split_must_match_plan(self):
        self.add_public()
        self.public["m"] = 15
        self.save_public()
        self.reject("public/config m mismatch")
        self.public["m"] = 2
        self.public["split_component"] = "attacker-test-split"
        self.save_public()
        self.reject("split component")

    def test_transplanted_sampling_provenance_rejected(self):
        self.add_public()
        self.private["sampling_provenance"]["key_index"] = 1
        self.save_public()
        self.reject("sampling provenance")

    def test_false_factor_base_completion_rejected(self):
        self.add_public()
        self.base["points"] = []
        self.save_public()
        self.reject("completion falsely asserted")

    def test_contradictory_outcomes_rejected(self):
        self.add_public()
        self.outcome.update(status="verified", solver_completed=True,
                            pdp_witness_verified=True, exhaustive_no_decomposition=True)
        self.add("pdp.json", {"target": self.public["target"], "summands": self.base["points"]},
                 "independently-verified-pdp")
        self.seal()
        self.reject("contradictory")

    def test_certificate_arity_and_domain_binding(self):
        self.add_public()
        self.outcome.update(status="verified", solver_completed=True, pdp_witness_verified=True)
        cert = {"target": self.public["target"], "summands": self.base["points"] * 2}
        self.add("pdp.json", cert, "independently-verified-pdp")
        self.seal()
        self.reject("PDP arity")
        cert["summands"] = self.base["points"]
        cert["factor_base_definition"] = {}
        self.add("pdp.json", cert, "independently-verified-pdp")
        self.seal()
        self.reject("outside PKM")

    def test_corruption_and_duplicate_members_rejected(self):
        (self.cell / "stdout.log").write_bytes(b"bad")
        self.reject("artifact size")
        (self.cell / "stdout.log").write_bytes(b"")
        manifest = c.load(self.cell / "manifest.json")
        manifest["artifacts"].append(manifest["artifacts"][0])
        (self.cell / "manifest.json").write_bytes(c.compact(manifest))
        self.reject("duplicate artifact")

    def test_symlinks_and_parent_paths_rejected(self):
        (self.cell / "stdout.log").unlink()
        (self.cell / "stdout.log").symlink_to(self.root / "plan.json")
        self.reject("symlink")
        with self.assertRaisesRegex(ValueError, "path"):
            c.contained(self.root, "../escape")

    def test_trusted_root_and_json_duplicate_keys(self):
        with self.assertRaisesRegex(ValueError, "trusted run"):
            c.audit(self.root, "0" * 64)
        path = self.root / "duplicate.json"
        path.write_text('{"key":1,"key":2}')
        with self.assertRaisesRegex(ValueError, "duplicate JSON"):
            c.load(path)

    def test_allowlist_excludes_nested_provenance_and_terminal_trace(self):
        self.add_public()
        self.public["generator"]["scalar"] = "NESTED-PRIVATE"
        self.public["curve"]["secret_seed"] = "CURVE-PRIVATE"
        self.public["field"]["seed"] = "FIELD-PRIVATE"
        self.base["points"][0]["private"] = "POINT-PRIVATE"
        self.base["construction_candidates"] = [{"seed": "CANDIDATE-PRIVATE"}]
        self.save_public()
        self.add("bsgs-giants.json", {"baby_index": 17, "i": 21, "secret": "TRACE-PRIVATE"},
                 "independent-scalar-search-probes")
        self.seal()
        _, rows = c.audit(self.root)
        features = json.dumps(c.static_features(rows[0]))
        for bad in ("PRIVATE", "seed", "scalar", "baby_index", "key_index"):
            self.assertNotIn(bad, features)
        output = self.root / "export.jsonl"
        c.export_rows(rows, output)
        self.assertFalse(json.loads(output.read_text())["labels"]["independently_verified"])
        with self.assertRaises(FileExistsError):
            c.export_rows(rows, output)

    def test_twists_share_split_and_failures_are_not_dropped(self):
        self.add_public()
        _, rows = c.audit(self.root)
        twist = copy.deepcopy(rows[0])
        twist["config"]["twist"] = True
        twist["public"]["twist"] = True
        failed = copy.deepcopy(rows[0])
        failed["public"] = None
        failed["outcome"]["status"] = "timeout"
        path = self.root / "export.jsonl"
        c.export_rows([*rows, twist, failed], path)
        exported = [json.loads(line) for line in path.read_text().splitlines()]
        self.assertEqual(len(exported), 3)
        self.assertEqual(len({r["split"] for r in exported}), 1)
        self.assertIsNone(exported[-1]["features"])

    def test_nested_feature_payload_and_partial_export_rejected(self):
        self.add_public()
        _, rows = c.audit(self.root)
        corrupted = copy.deepcopy(rows[0])
        corrupted["factor_base"]["negation_action"] = [{"private_seed": 42}]
        path = self.root / "view.jsonl"
        with self.assertRaisesRegex(ValueError, "factor-base action"):
            c.export_rows([*rows, corrupted], path)
        self.assertFalse(path.exists())
        self.assertEqual(list(self.root.glob(".consumer-export-*")), [])
        corrupted = copy.deepcopy(rows[0])
        corrupted["public"]["curve"]["a"] = {"seed": 42}
        with self.assertRaisesRegex(ValueError, "public decimal"):
            c.static_features(corrupted)

    def test_current_prime_primary_factor_base_rule_is_exported(self):
        self.add_public()
        self.base["rule"] = "small-x-prime-primary-projection-with-cyclic-membership"
        self.save_public()
        _, rows = c.audit(self.root)
        self.assertEqual(c.static_features(rows[0])["factor_base"]["rule"], self.base["rule"])

    def test_native_replay_requires_coverage_and_records_identity(self):
        verifier = self.root / "native-verifier-fixture"
        status = {"schema": "crypto.ml-dataset.status/v1", "known_answer_replay_performed": True,
                  "artifact_hashes_verified": True, "completed_receipts": 0}
        command = [sys.executable, str(Path(c.__file__)), "audit", "--run", str(self.root),
                   "--native-verifier", str(verifier)]
        def install(value):
            verifier.write_text('#!' + sys.executable + '\nprint(' + repr(json.dumps(value)) + ')\n')
            verifier.chmod(0o700)
        install(status)
        denied = subprocess.run(command, capture_output=True, text=True)
        self.assertEqual(denied.returncode, 1)
        self.assertIn("coverage mismatch", denied.stderr)
        status["completed_receipts"] = 1
        install(status)
        allowed = subprocess.run(command, capture_output=True, text=True)
        self.assertEqual(allowed.returncode, 0, allowed.stderr)
        report = json.loads(allowed.stdout)
        self.assertTrue(report["producer_native_replay_passed"])
        self.assertFalse(report["mathematics_independently_verified"])
        self.assertEqual(report["native_verifier_sha256"], c.digest(verifier.read_bytes()))

    def test_json_object_required(self):
        path = self.root / "array.json"
        path.write_text('[]')
        with self.assertRaisesRegex(ValueError, "expected JSON object"):
            c.load(path)

    def add_phase(self, multiplexed=False):
        self.metrics = {"schema": "crypto.ml-dataset.phase-metrics/v1", "phase": "solver",
            "returned_ok": True, "returned_error": None,
            "instructions_status": "measured_multiplexed_estimate" if multiplexed else "measured_unmultiplexed",
            "retired_instructions_raw": 100, "retired_instructions_exact_in_scope": None if multiplexed else 100,
            "hardware": {"instructions": 100, "instructions_scaled": 200 if multiplexed else 100,
                         "instructions_time_enabled_ns": 20 if multiplexed else 10,
                         "instructions_time_running_ns": 10, "multiplexed": multiplexed,
                         "inherit": True, "grouped": True, "error": None},
            "clock_valid_for_comparison": False, "counter_scope": "calling thread user space",
            "excludes": ["startup", "upload", "preexisting threads"], "interrupted_phase_readback": False}
        self.add("telemetry/solver-start.json", {"schema": "crypto.ml-dataset.phase-start/v1",
            "phase": "solver", "pid": 123, "usage_before": None}, "phase-start")
        self.save_phase()

    def save_phase(self):
        self.add("telemetry/solver.json", self.metrics, "hardware-and-resource-telemetry")
        self.seal()

    def test_instruction_labels_preserve_scope_and_stay_out_of_features(self):
        self.add_public()
        self.add_phase()
        _, rows = c.audit(self.root)
        path = self.root / "telemetry-view.jsonl"
        c.export_rows(rows, path)
        record = json.loads(path.read_text())
        phase = record["labels"]["phase_measurements"][0]
        self.assertEqual(phase["producer_reported"]["retired_instructions_exact_in_scope"], 100)
        self.assertEqual(phase["producer_reported"]["excludes"], self.metrics["excludes"])
        self.assertFalse(phase["independently_verified"])
        self.assertIsNone(record["labels"]["whole_worker_instruction_count"])
        self.assertNotIn("instructions", json.dumps(record["features"]))

    def test_multiplexed_estimates_are_not_exact_counts(self):
        self.add_phase(multiplexed=True)
        _, rows = c.audit(self.root)
        metrics = rows[0]["phase_measurements"][0]["producer_reported"]
        self.assertEqual(metrics["retired_instructions_raw"], 100)
        self.assertEqual(metrics["hardware"]["instructions_scaled"], 200)
        self.assertIsNone(metrics["retired_instructions_exact_in_scope"])
        self.metrics["retired_instructions_exact_in_scope"] = 200
        self.save_phase()
        self.reject("exact instruction label mismatch")

    def test_unavailable_pmu_and_interrupted_phase_remain_missing(self):
        self.add_phase()
        self.metrics.update(instructions_status="unavailable", retired_instructions_raw=None,
                            retired_instructions_exact_in_scope=None)
        self.metrics["hardware"] = {"instructions": None, "error": "Permission denied"}
        self.save_phase()
        _, rows = c.audit(self.root)
        metrics = rows[0]["phase_measurements"][0]["producer_reported"]
        self.assertIsNone(metrics["retired_instructions_raw"])
        self.assertEqual(metrics["hardware"]["error"], "Permission denied")
        del self.members["telemetry/solver.json"]
        self.seal()
        _, rows = c.audit(self.root)
        phase = rows[0]["phase_measurements"][0]
        self.assertEqual(phase["measurement_state"], "interrupted_or_unfinished")
        self.assertIsNone(phase["producer_reported"])

    def test_hardware_labels_reject_inconsistent_scope_and_values(self):
        self.add_phase()
        original = copy.deepcopy(self.metrics)
        changes = [
            (lambda m: m["hardware"].update(instructions_scaled=101), "scaling"),
            (lambda m: m["hardware"].update(instructions_time_running_ns=11), "running/enabled"),
            (lambda m: m["hardware"].update(instructions_time_enabled_ns=20, instructions_scaled=200), "multiplexing"),
            (lambda m: m.update(retired_instructions_exact_in_scope=True), "label type"),
            (lambda m: m.update(clock_valid_for_comparison=True), "comparative clock"),
            (lambda m: m.update(excludes=[]), "scope/exclusions"),
            (lambda m: m.update(returned_ok=False), "phase error/status"),
        ]
        for change, message in changes:
            with self.subTest(message=message):
                self.metrics = copy.deepcopy(original)
                change(self.metrics)
                self.save_phase()
                self.reject(message)

    def test_phase_readback_requires_start_artifact(self):
        self.add_phase()
        del self.members["telemetry/solver-start.json"]
        self.seal()
        self.reject("lack durable start")

    def test_cycles_and_missing_or_zero_running_counters_are_consistent(self):
        self.add_phase()
        original = copy.deepcopy(self.metrics)
        for updates, message in [
            ({"cycles": 100, "cycles_scaled": 999, "cycles_time_enabled_ns": 10, "cycles_time_running_ns": 10}, "cycles scaling"),
            ({"cycles": 100, "cycles_scaled": 100, "cycles_time_enabled_ns": 10, "cycles_time_running_ns": 11}, "running/enabled"),
            ({"instructions": None, "instructions_scaled": 123}, "instructions scaling"),
        ]:
            with self.subTest(message=message):
                self.metrics = copy.deepcopy(original)
                self.metrics["hardware"].update(updates)
                self.save_phase()
                self.reject(message)
        for enabled, scaled in [(0, 100), (10, None)]:
            self.metrics = copy.deepcopy(original)
            self.metrics.update(instructions_status="unavailable", retired_instructions_raw=None,
                                retired_instructions_exact_in_scope=None)
            self.metrics["hardware"].update(instructions_time_enabled_ns=enabled, instructions_time_running_ns=0,
                                            instructions_scaled=scaled, multiplexed=enabled > 0)
            self.save_phase()
            _, rows = c.audit(self.root)
            self.assertIsNone(rows[0]["phase_measurements"][0]["producer_reported"]["retired_instructions_raw"])

    def test_mutation_during_native_replay_rejected(self):
        verifier = self.root / "verifier"
        verifier.write_bytes(b"before")
        report, rows = c.audit(self.root)
        status = {"schema": "crypto.ml-dataset.status/v1", "known_answer_replay_performed": True,
                  "artifact_hashes_verified": True, "completed_receipts": 1}
        def mutate_manifest(argv, **kwargs):
            self.outcome["status"] = "timeout"
            self.seal()
            return subprocess.CompletedProcess(argv, 0, json.dumps(status), "")
        with mock.patch.object(c.subprocess, "run", side_effect=mutate_manifest), self.assertRaisesRegex(ValueError, "cell contents changed"):
            c.replay_native(self.root, verifier, report, rows)
        report, rows = c.audit(self.root)
        def mutate_binary(argv, **kwargs):
            verifier.write_bytes(b"after")
            return subprocess.CompletedProcess(argv, 0, json.dumps(status), "")
        with mock.patch.object(c.subprocess, "run", side_effect=mutate_binary), self.assertRaisesRegex(ValueError, "verifier changed"):
            c.replay_native(self.root, verifier, report, rows)

    def test_cli_export_requires_replay_or_explicit_preview(self):
        script = Path(c.__file__)
        command = [sys.executable, str(script), "export", "--run", str(self.root), "--out", str(self.root / "view.jsonl")]
        denied = subprocess.run(command, capture_output=True, text=True)
        self.assertEqual(denied.returncode, 1)
        self.assertIn("requires native replay", denied.stderr)
        allowed = subprocess.run([*command, "--allow-unreplayed-preview"], capture_output=True, text=True)
        self.assertEqual(allowed.returncode, 0, allowed.stderr)
        self.assertFalse(json.loads(allowed.stdout)["producer_native_replay_passed"])

    def test_remote_fetch_downloads_and_rehashes_every_object(self):
        raw_manifest = (self.cell / "manifest.json").read_bytes()
        artifacts = list(self.members.values()) + [{"path": "manifest.json", "role": "cell-manifest",
                     "encoding": "json", "bytes": len(raw_manifest), "sha256": c.digest(raw_manifest)}]
        record = {"schema": "ml-dataset.publication/v1", "remote": "s3://setec-astronomy-artifacts/corpus",
                  "namespace": "fixture", "manifest_sha256": c.digest(raw_manifest),
                  "objects": [{"artifact": a, "object_key": f"sha256/{a['sha256'][:2]}/{a['sha256']}"} for a in artifacts]}
        receipt = self.root / "completion.json"
        receipt.write_bytes(c.compact(record))
        objects = {f"corpus/sha256/{a['sha256'][:2]}/{a['sha256']}": (self.cell / a["path"]).read_bytes() for a in artifacts}
        calls = []
        def get(argv, **kwargs):
            key = argv[argv.index("--key") + 1]
            if "head-object" in argv:
                return subprocess.CompletedProcess(argv, 0,
                    json.dumps({"ContentLength": len(objects[key]), "ETag": '"fixture-etag"'}), "")
            calls.append(key)
            self.assertEqual(argv[argv.index("--if-match") + 1], '"fixture-etag"')
            Path(argv[-1]).write_bytes(objects[key])
            return subprocess.CompletedProcess(argv, 0, "{}", "")
        destination = self.root.parent / "restored"
        with mock.patch.object(c.subprocess, "run", side_effect=get):
            report = c.fetch_cell(receipt, c.digest(receipt.read_bytes()), destination)
        self.assertTrue(report["remote_rechecked"])
        self.assertEqual(len(calls), len(artifacts))
        self.assertEqual(report["checked_cells"], 1)
        objects[next(iter(objects))] = b"corrupt"
        with mock.patch.object(c.subprocess, "run", side_effect=get), self.assertRaisesRegex(ValueError, "remote size|artifact (size|hash)"):
            c.fetch_cell(receipt, c.digest(receipt.read_bytes()), self.root.parent / "corrupt-restore")


if __name__ == "__main__":
    unittest.main()
