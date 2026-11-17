from __future__ import annotations

import ast
import hashlib
import json
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from typing import Callable


ROOT = Path(__file__).resolve().parents[1]
GATE = ROOT / "intake_gate.py"
FIXTURES = ROOT / "intake" / "fixtures"
VALID = FIXTURES / "valid"
NEGATIVE_CASES = FIXTURES / "negative-cases.json"


def load_json(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, value) -> None:
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def load_rows(path: Path):
    return [json.loads(line) for line in path.read_text(encoding="utf-8").splitlines()]


def write_rows(path: Path, rows) -> None:
    path.write_text(
        "".join(json.dumps(row, sort_keys=True, separators=(",", ":")) + "\n" for row in rows),
        encoding="utf-8",
    )


def refresh_integrity(bundle: Path) -> None:
    manifest_path = bundle / "manifest.json"
    manifest = load_json(manifest_path)
    payload_names = ("primary", "process_receipts", "replay", "setup")
    for name in payload_names:
        entry = manifest["files"][name]
        payload = (bundle / entry["path"]).read_bytes()
        entry["sha256"] = hashlib.sha256(payload).hexdigest()
        entry["bytes"] = len(payload)
    checksum_text = "".join(
        f"{manifest['files'][name]['sha256']}  {manifest['files'][name]['path']}\n"
        for name in payload_names
    )
    checksum_path = bundle / manifest["files"]["checksums"]["path"]
    checksum_path.write_text(checksum_text, encoding="ascii")
    checksum_payload = checksum_path.read_bytes()
    manifest["files"]["checksums"]["sha256"] = hashlib.sha256(checksum_payload).hexdigest()
    manifest["files"]["checksums"]["bytes"] = len(checksum_payload)
    write_json(manifest_path, manifest)


class IntakeGateTests(unittest.TestCase):
    maxDiff = None

    def run_gate(self, cwd: Path, *arguments: str):
        completed = subprocess.run(
            [sys.executable, str(GATE), *arguments],
            cwd=cwd,
            check=False,
            text=True,
            capture_output=True,
        )
        self.assertEqual(completed.stderr, "")
        lines = completed.stdout.splitlines()
        self.assertEqual(len(lines), 1, completed.stdout)
        receipt = json.loads(lines[0])
        self.assertEqual(receipt["schema"], "pencil-intake-validation/v1")
        self.assertIs(receipt["synthetic_only"], True)
        self.assertIn(receipt["status"], {"synthetic_fixture_accepted", "rejected"})
        for error in receipt["errors"]:
            self.assertEqual(
                list(error), ["code", "path", "workload_id", "field", "detail"]
            )
        return completed, receipt

    def copied_bundle(self, temporary: Path) -> Path:
        bundle = temporary / "bundle"
        shutil.copytree(VALID, bundle)
        return bundle

    def test_valid_public_fixture_and_written_receipt(self):
        with tempfile.TemporaryDirectory() as raw:
            temporary = Path(raw)
            self.copied_bundle(temporary)
            completed, receipt = self.run_gate(
                temporary, "--bundle", "bundle", "--receipt", "receipt.json"
            )
            self.assertEqual(completed.returncode, 0)
            self.assertIs(receipt["accepted"], True)
            self.assertEqual(receipt["status"], "synthetic_fixture_accepted")
            self.assertEqual(receipt["errors"], [])
            self.assertEqual(
                receipt["counts"], {"primary": 3, "replay": 3, "process_receipts": 6}
            )
            self.assertEqual(load_json(temporary / "receipt.json"), receipt)

    def test_usage_failure_still_emits_standard_receipt(self):
        with tempfile.TemporaryDirectory() as raw:
            completed, receipt = self.run_gate(Path(raw))
            self.assertEqual(completed.returncode, 1)
            self.assertIs(receipt["accepted"], False)
            self.assertEqual(receipt["status"], "rejected")
            self.assertTrue(receipt["errors"])
            self.assertEqual(receipt["errors"][0]["code"], "usage_error")

    def test_gate_ast_has_no_execution_surface(self):
        tree = ast.parse(GATE.read_text(encoding="utf-8"), filename=str(GATE))
        allowed_import_roots = {
            "__future__",
            "hashlib",
            "json",
            "sys",
            "pathlib",
            "typing",
        }
        imports = set()
        calls = []
        for node in ast.walk(tree):
            if isinstance(node, ast.Import):
                imports.update(alias.name.split(".")[0] for alias in node.names)
            elif isinstance(node, ast.ImportFrom):
                imports.add((node.module or "").split(".")[0])
            elif isinstance(node, ast.Call):
                if isinstance(node.func, ast.Name):
                    calls.append(node.func.id)
                elif isinstance(node.func, ast.Attribute):
                    calls.append(node.func.attr)
        self.assertLessEqual(imports, allowed_import_roots)
        forbidden_calls = {
            "exec",
            "eval",
            "compile",
            "__import__",
            "system",
            "popen",
            "spawn",
            "fork",
            "run",
            "call",
            "check_call",
            "check_output",
        }
        self.assertEqual(sorted(forbidden_calls & set(calls)), [])
        forbidden_call_fragments = {
            "collector",
            "matrix",
            "nullspace",
            "recover",
            "replay",
            "solver",
        }
        implicated = sorted(
            name
            for name in calls
            if any(fragment in name.lower() for fragment in forbidden_call_fragments)
        )
        self.assertEqual(implicated, [])
        source = GATE.read_text(encoding="utf-8")
        self.assertNotIn("--mode", source)
        self.assertNotIn("--command", source)

    def test_negative_fixture_catalog_is_complete(self):
        catalog = load_json(NEGATIVE_CASES)
        self.assertEqual(catalog["schema"], "pencil-intake-negative-fixtures/v1")
        self.assertEqual(catalog["base"], "valid")
        expected = {case["id"]: case for case in catalog["cases"]}
        self.assertEqual(set(expected), set(self.mutations()))
        for case_id, mutate in self.mutations().items():
            with self.subTest(case=case_id), tempfile.TemporaryDirectory() as raw:
                temporary = Path(raw)
                bundle = self.copied_bundle(temporary)
                arguments = mutate(bundle, temporary)
                completed, receipt = self.run_gate(temporary, *arguments)
                self.assertEqual(completed.returncode, 1)
                self.assertIs(receipt["accepted"], False)
                self.assertEqual(receipt["status"], "rejected")
                self.assertTrue(receipt["errors"])
                error = receipt["errors"][0]
                self.assertEqual(error["code"], expected[case_id]["expected_error_code"])
                if "expected_field" in expected[case_id]:
                    self.assertEqual(error["field"], expected[case_id]["expected_field"])

    @staticmethod
    def mutations() -> dict[str, Callable[[Path, Path], tuple[str, ...]]]:
        def record_mutation(role: str, change):
            def mutate(bundle: Path, _temporary: Path):
                path = bundle / f"{role}.jsonl"
                rows = load_rows(path)
                change(rows)
                write_rows(path, rows)
                refresh_integrity(bundle)
                return "--bundle", "bundle"

            return mutate

        def process_mutation(change):
            def mutate(bundle: Path, _temporary: Path):
                path = bundle / "process-receipts.jsonl"
                rows = load_rows(path)
                change(rows)
                write_rows(path, rows)
                refresh_integrity(bundle)
                return "--bundle", "bundle"

            return mutate

        def manifest_mutation(change):
            def mutate(bundle: Path, _temporary: Path):
                path = bundle / "manifest.json"
                manifest = load_json(path)
                change(manifest)
                write_json(path, manifest)
                return "--bundle", "bundle"

            return mutate

        def file_hash_drift(bundle: Path, _temporary: Path):
            with (bundle / "primary.jsonl").open("a", encoding="utf-8") as stream:
                stream.write(" ")
            return "--bundle", "bundle"

        def missing_setup(bundle: Path, _temporary: Path):
            (bundle / "setup.json").unlink()
            return "--bundle", "bundle"

        def unlisted_file(bundle: Path, _temporary: Path):
            (bundle / "unlisted.txt").write_text("not declared\n", encoding="utf-8")
            return "--bundle", "bundle"

        def missing_process_receipts(bundle: Path, _temporary: Path):
            (bundle / "process-receipts.jsonl").unlink()
            return "--bundle", "bundle"

        def setup_substitution(bundle: Path, _temporary: Path):
            setup = load_json(bundle / "setup.json")
            setup["factor_base_size"] += 1
            write_json(bundle / "setup.json", setup)
            refresh_integrity(bundle)
            return "--bundle", "bundle"

        def checksum_index_mismatch(bundle: Path, _temporary: Path):
            path = bundle / "files.sha256"
            path.write_text(path.read_text(encoding="ascii") + "0" * 64 + "  extra\n", encoding="ascii")
            manifest_path = bundle / "manifest.json"
            manifest = load_json(manifest_path)
            payload = path.read_bytes()
            manifest["files"]["checksums"]["sha256"] = hashlib.sha256(payload).hexdigest()
            manifest["files"]["checksums"]["bytes"] = len(payload)
            write_json(manifest_path, manifest)
            return "--bundle", "bundle"

        def absolute_receipt(_bundle: Path, temporary: Path):
            return "--bundle", "bundle", "--receipt", str(temporary / "receipt.json")

        def absolute_bundle(bundle: Path, _temporary: Path):
            return "--bundle", str(bundle)

        def escaping_bundle(_bundle: Path, _temporary: Path):
            return "--bundle", "../bundle"

        def duplicate(rows):
            rows.append(dict(rows[0]))

        def missing(rows):
            rows.pop()

        def unknown(rows):
            rows[0]["workload_id"] = "SYNTH-PENCIL-UNKNOWN"

        def seed_drift(rows):
            rows[0]["seed"] += 1

        def protocol_drift(rows):
            rows[0]["protocol_digest"] = "0" * 64

        def bits_drift(rows):
            rows[0]["bits"] += 1

        def p_drift(rows):
            rows[0]["p"] += 2

        def target_index_drift(rows):
            rows[0]["target_index"] += 1

        def workload_hash_drift(rows):
            rows[0]["workload_sha256"] = "0" * 64

        def solver_mode_drift(rows):
            rows[0]["solver_mode"] = "synthetic-different-mode"

        def source_revision_drift(rows):
            rows[0]["source_revision"] = "SYNTHETIC-DIFFERENT-REVISION"

        def archive_drift(rows):
            rows[0]["archive_sha256"] = "0" * 64

        def plan_drift(rows):
            rows[0]["plan_set_sha256"] = "0" * 64

        def missing_resource_guard(rows):
            rows[0]["resource_guards"].pop("memory_limit_bytes")

        def resource_guard_drift(rows):
            rows[0]["resource_guards"]["memory_limit_bytes"] += 1

        def setup_drift(rows):
            rows[0]["setup_sha256"] = "0" * 64

        def overlap(rows):
            rows[0]["process_receipt_id"] = "SYNTH-PROCESS-PRIMARY-0001"

        def replay_mismatch(rows):
            rows[0]["coefficient_visits"] += 1

        def unverified_rho(rows):
            rows[0]["rho"]["verified"] = False

        def failed_price(rows):
            rows[1]["candidate"] = {
                "verified": False,
                "scalar": 0,
                "prices": {"M1-S1-I1": 1, "M1-S0.1-I10": 1, "M1-S1-I100": 1},
            }

        def capped_price(rows):
            rows[2]["candidate"] = {
                "verified": False,
                "scalar": 0,
                "prices": {"M1-S1-I1": 1, "M1-S0.1-I10": 1, "M1-S1-I100": 1},
            }

        def real_id(rows):
            rows[0]["workload_id"] = "Wa0fc1e72d7a0"

        def candidate_scalar_disagreement(rows):
            rows[0]["candidate"]["scalar"] += 1

        def rho_scalar_disagreement(rows):
            rows[0]["rho"]["scalar"] += 1

        def binary_digest_drift(rows):
            rows[0]["binary_sha256"] = "0" * 64

        def process_binary_drift(rows):
            rows[0]["binary_sha256"] = "0" * 64

        def missing_process_receipt_row(rows):
            rows.pop()

        def duplicate_process_receipt_row(rows):
            rows.append(dict(rows[0]))

        def guard_outcome_drift(rows):
            rows[0]["guard_outcome"]["wall_limit_exceeded"] = True

        def solved_wall_exceeded(bundle: Path, _temporary: Path):
            primary_path = bundle / "primary.jsonl"
            primary = load_rows(primary_path)
            primary[0]["wall_ns"] = 6_000_000_000
            write_rows(primary_path, primary)
            process_path = bundle / "process-receipts.jsonl"
            process = load_rows(process_path)
            process[0]["wall_ns"] = 6_000_000_000
            process[0]["guard_outcome"]["wall_limit_exceeded"] = True
            write_rows(process_path, process)
            refresh_integrity(bundle)
            return "--bundle", "bundle"

        def solved_memory_exceeded(bundle: Path, _temporary: Path):
            primary_path = bundle / "primary.jsonl"
            primary = load_rows(primary_path)
            primary[0]["peak_memory_bytes"] = 1_048_577
            write_rows(primary_path, primary)
            process_path = bundle / "process-receipts.jsonl"
            process = load_rows(process_path)
            process[0]["peak_memory_bytes"] = 1_048_577
            process[0]["guard_outcome"]["memory_limit_exceeded"] = True
            write_rows(process_path, process)
            refresh_integrity(bundle)
            return "--bundle", "bundle"

        def solved_coefficient_cap(bundle: Path, _temporary: Path):
            primary_path = bundle / "primary.jsonl"
            primary = load_rows(primary_path)
            primary[0]["peak_stored_coefficients"] = 128
            write_rows(primary_path, primary)
            process_path = bundle / "process-receipts.jsonl"
            process = load_rows(process_path)
            process[0]["peak_stored_coefficients"] = 128
            process[0]["guard_outcome"]["matrix_coefficient_cap_reached"] = True
            write_rows(process_path, process)
            refresh_integrity(bundle)
            return "--bundle", "bundle"

        def same_role_process_alias_and_deleted_receipt(bundle: Path, _temporary: Path):
            primary_path = bundle / "primary.jsonl"
            primary = load_rows(primary_path)
            removed_process_id = primary[1]["process_receipt_id"]
            primary[1]["process_receipt_id"] = primary[0]["process_receipt_id"]
            write_rows(primary_path, primary)
            process_path = bundle / "process-receipts.jsonl"
            process = [
                row
                for row in load_rows(process_path)
                if row["process_receipt_id"] != removed_process_id
            ]
            write_rows(process_path, process)
            refresh_integrity(bundle)
            return "--bundle", "bundle"

        return {
            "duplicate-primary": record_mutation("primary", duplicate),
            "missing-primary": record_mutation("primary", missing),
            "unknown-workload": record_mutation("primary", unknown),
            "seed-drift": record_mutation("primary", seed_drift),
            "bits-drift": record_mutation("primary", bits_drift),
            "p-drift": record_mutation("primary", p_drift),
            "target-index-drift": record_mutation("primary", target_index_drift),
            "workload-hash-drift": record_mutation("primary", workload_hash_drift),
            "protocol-hash-drift": record_mutation("primary", protocol_drift),
            "solver-mode-drift": record_mutation("primary", solver_mode_drift),
            "source-revision-drift": record_mutation("primary", source_revision_drift),
            "archive-drift": record_mutation("primary", archive_drift),
            "plan-drift": record_mutation("primary", plan_drift),
            "missing-resource-guard": record_mutation("primary", missing_resource_guard),
            "resource-guard-drift": record_mutation("primary", resource_guard_drift),
            "file-hash-drift": file_hash_drift,
            "file-length-drift": manifest_mutation(
                lambda manifest: manifest["files"]["primary"].__setitem__(
                    "bytes", manifest["files"]["primary"]["bytes"] + 1
                )
            ),
            "unlisted-file": unlisted_file,
            "missing-process-receipts": missing_process_receipts,
            "manifest-missing-file": manifest_mutation(
                lambda manifest: manifest["files"].pop("checksums")
            ),
            "manifest-extra-file": manifest_mutation(
                lambda manifest: manifest["files"].__setitem__(
                    "extra", {"path": "setup.json", "sha256": "0" * 64, "bytes": 0}
                )
            ),
            "manifest-path-alias": manifest_mutation(
                lambda manifest: manifest["files"]["primary"].__setitem__(
                    "path", "replay.jsonl"
                )
            ),
            "checksum-index-mismatch": checksum_index_mismatch,
            "missing-setup": missing_setup,
            "setup-substitution": setup_substitution,
            "setup-drift": record_mutation("primary", setup_drift),
            "absolute-input-path": manifest_mutation(
                lambda manifest: manifest["files"]["setup"].__setitem__(
                    "path", "/tmp/setup.json"
                )
            ),
            "escaping-input-path": manifest_mutation(
                lambda manifest: manifest["files"]["setup"].__setitem__(
                    "path", "../setup.json"
                )
            ),
            "absolute-bundle-path": absolute_bundle,
            "escaping-bundle-path": escaping_bundle,
            "overlapping-process": record_mutation("replay", overlap),
            "incomplete-replay": record_mutation("replay", missing),
            "non-wall-replay-mismatch": record_mutation("replay", replay_mismatch),
            "unverified-rho": record_mutation("primary", unverified_rho),
            "candidate-scalar-disagreement": record_mutation(
                "primary", candidate_scalar_disagreement
            ),
            "rho-scalar-disagreement": record_mutation("primary", rho_scalar_disagreement),
            "binary-digest-drift": record_mutation("primary", binary_digest_drift),
            "process-binary-drift": process_mutation(process_binary_drift),
            "missing-process-receipt-row": process_mutation(missing_process_receipt_row),
            "duplicate-process-receipt-row": process_mutation(
                duplicate_process_receipt_row
            ),
            "same-role-process-alias-delete": same_role_process_alias_and_deleted_receipt,
            "guard-outcome-drift": process_mutation(guard_outcome_drift),
            "solved-wall-guard-exceeded": solved_wall_exceeded,
            "solved-memory-guard-exceeded": solved_memory_exceeded,
            "solved-coefficient-guard-reached": solved_coefficient_cap,
            "failed-candidate-pricing": record_mutation("primary", failed_price),
            "capped-candidate-pricing": record_mutation("primary", capped_price),
            "real-cohort-id": record_mutation("primary", real_id),
            "absolute-receipt-path": absolute_receipt,
        }


if __name__ == "__main__":
    unittest.main()
