#!/usr/bin/env python3
"""Interface tests for the full-eight fixed-order launcher."""

from __future__ import annotations

import contextlib
import hashlib
import importlib.util
import io
import json
import os
import tempfile
import time
import unittest
from pathlib import Path
from unittest import mock


MODULE_PATH = Path(__file__).with_name("run_full8_f4_suite.py")
SPEC = importlib.util.spec_from_file_location("run_full8_f4_suite", MODULE_PATH)
assert SPEC is not None and SPEC.loader is not None
SUITE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(SUITE)


def make_inputs(root: Path) -> dict[str, Path]:
    binary = root / "producer"
    binary.write_text("#!/bin/sh\nexit 0\n", encoding="utf-8")
    binary.chmod(0o700)
    time_binary = root / "time"
    time_binary.write_text("#!/bin/sh\nexit 0\n", encoding="utf-8")
    time_binary.chmod(0o700)
    verifier_source = root / "verifier.py"
    verifier_source.write_text("print('verifier')\n", encoding="utf-8")
    verifier_executable = root / "python"
    verifier_executable.write_text("#!/bin/sh\nexit 0\n", encoding="utf-8")
    verifier_executable.chmod(0o700)
    protocol = root / "protocol.md"
    source = root / "source.rs"
    cargo_lock = root / "Cargo.lock"
    cargo_manifest = root / "Cargo.toml"
    protocol.write_text("frozen\n", encoding="utf-8")
    source.write_text("fn main() {}\n", encoding="utf-8")
    cargo_lock.write_text("# lock\n", encoding="utf-8")
    cargo_manifest.write_text("[package]\nname='test'\n", encoding="utf-8")
    return {
        "binary": binary,
        "protocol": protocol,
        "source": source,
        "verifier_source": verifier_source,
        "verifier_executable": verifier_executable,
        "time_binary": time_binary,
        "cargo_lock": cargo_lock,
        "cargo_manifest": cargo_manifest,
    }


def make_valid_execution_inputs(root: Path) -> dict[str, Path]:
    inputs = make_inputs(root)
    outputs: list[str] = []
    for case in SUITE.CASE_ORDER:
        input_system = {"schema": SUITE.EXPECTED_INPUT_SCHEMA, "case": case}
        compact = json.dumps(
            input_system, separators=(",", ":"), sort_keys=True
        ).encode("utf-8")
        outputs.append(
            json.dumps(
                {
                    "schema": SUITE.EXPECTED_PRODUCER_SCHEMA,
                    "case": case,
                    "input_system": input_system,
                    "input_system_sha256": hashlib.sha256(compact).hexdigest(),
                },
                separators=(",", ":"),
            )
        )
    inputs["binary"].write_text(
        "#!/bin/sh\n"
        "case \"$1\" in\n"
        f"0) printf '%s\\n' '{outputs[0]}' ;;\n"
        f"1) printf '%s\\n' '{outputs[1]}' ;;\n"
        f"2) printf '%s\\n' '{outputs[2]}' ;;\n"
        "esac\n"
        "exit 0\n",
        encoding="utf-8",
    )
    inputs["binary"].chmod(0o700)
    inputs["time_binary"].write_text(
        "#!/bin/sh\n"
        "shift 2\n"
        "report=$1\n"
        "shift\n"
        '"$@"\n'
        "status=$?\n"
        "{\n"
        "echo 'User time (seconds): 0.01'\n"
        "echo 'System time (seconds): 0.01'\n"
        "echo 'Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.02'\n"
        "echo 'Maximum resident set size (kbytes): 1024'\n"
        "echo \"Exit status: $status\"\n"
        "} > \"$report\"\n"
        "exit \"$status\"\n",
        encoding="utf-8",
    )
    inputs["time_binary"].chmod(0o700)
    return inputs


class Full8LauncherTests(unittest.TestCase):
    def test_defaults_bind_the_native_producer_crate(self) -> None:
        expected_crate = MODULE_PATH.parent.parent / "native"
        self.assertEqual(SUITE.DEFAULT_CARGO_LOCK, expected_crate / "Cargo.lock")
        self.assertEqual(
            SUITE.DEFAULT_CARGO_MANIFEST, expected_crate / "Cargo.toml"
        )

    def test_informational_git_provenance_never_invokes_subprocess(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            inputs = make_inputs(root)
            paths = SUITE.input_paths(**inputs)
            with mock.patch.object(
                SUITE.subprocess,
                "run",
                side_effect=AssertionError("git subprocess must not be invoked"),
            ):
                observed = SUITE.repository_receipt(root, paths)
            self.assertFalse(observed["admission_bearing"])
            self.assertFalse(observed["available"])
            self.assertEqual(
                observed["unavailable_reason"],
                "git_not_invoked_by_custody_wrapper",
            )
            self.assertIsNone(observed["head"])

    def test_hostile_thread_environment_is_overridden_and_receipt_bound(self) -> None:
        with mock.patch.dict(
            os.environ,
            {
                "RAYON_NUM_THREADS": "99",
                "OMP_NUM_THREADS": "77",
                "LD_PRELOAD": "/hostile/injection.so",
                "PATH": "/hostile/bin",
            },
            clear=False,
        ):
            observed = SUITE.environment_receipt("sha256:test-image")
        self.assertEqual(
            observed["child_environment"], SUITE.FROZEN_CHILD_ENVIRONMENT
        )
        self.assertFalse(observed["inherited_environment_used_for_children"])
        self.assertNotIn("LD_PRELOAD", observed["child_environment"])
        self.assertNotIn("PATH", observed["child_environment"])

    def test_observed_interface_checks_only_schema_case_and_input_hash(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            output = Path(temporary) / "producer.json"
            input_system = {
                "schema": SUITE.EXPECTED_INPUT_SCHEMA,
                "case": 1,
                "p": 2_147_483_647,
            }
            compact = json.dumps(
                input_system, separators=(",", ":"), sort_keys=True
            ).encode("utf-8")
            output.write_text(
                json.dumps(
                    {
                        "schema": SUITE.EXPECTED_PRODUCER_SCHEMA,
                        "case": 1,
                        "input_system": input_system,
                        "input_system_sha256": hashlib.sha256(compact).hexdigest(),
                        "positive_replay_passed": True,
                    }
                )
                + "\n",
                encoding="utf-8",
            )
            observed = SUITE.observed_header(output)
            self.assertEqual(SUITE.producer_interface_errors(observed, 1), [])
            self.assertEqual(
                SUITE.producer_interface_errors(observed, 0),
                ["producer_case_mismatch", "input_system_case_mismatch"],
            )
            for invalid_case in (False, 1.0):
                mutated = dict(observed)
                mutated["case"] = invalid_case
                mutated["input_system_case"] = invalid_case
                self.assertIn(
                    "producer_case_mismatch",
                    SUITE.producer_interface_errors(mutated, 1),
                )
                self.assertIn(
                    "input_system_case_mismatch",
                    SUITE.producer_interface_errors(mutated, 1),
                )

    def test_time_v_resource_fields_are_observations(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            report = Path(temporary) / "time-v.txt"
            report.write_text(
                "\tUser time (seconds): 1.25\n"
                "\tSystem time (seconds): 0.50\n"
                "\tElapsed (wall clock) time (h:mm:ss or m:ss): 0:01.80\n"
                "\tMaximum resident set size (kbytes): 4096\n"
                "\tExit status: 7\n",
                encoding="utf-8",
            )
            self.assertEqual(
                SUITE.time_v_observations(report),
                {
                    "user_seconds": 1.25,
                    "system_seconds": 0.5,
                    "elapsed_wall": "0:01.80",
                    "elapsed_wall_seconds": 1.8,
                    "maximum_resident_set_kib": 4096,
                    "time_exit_status": 7,
                },
            )

    def test_strict_json_rejects_duplicate_keys_and_nonfinite_numbers(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            output = Path(temporary) / "producer.json"
            output.write_text(
                '{"schema":"a","schema":"b","case":0}', encoding="utf-8"
            )
            self.assertIsNone(SUITE.observed_header(output))
            output.write_text(
                '{"schema":"a","case":0,"extra":NaN}', encoding="utf-8"
            )
            self.assertIsNone(SUITE.observed_header(output))
            with self.assertRaises(ValueError):
                SUITE.strict_json_loads('{"nested":{"x":1,"x":2}}')
            with self.assertRaises(ValueError):
                SUITE.strict_json_loads('{"x":Infinity}')
            with self.assertRaises(ValueError):
                SUITE.strict_json_loads('{"x":1e999}')
            with self.assertRaises(ValueError):
                SUITE.strict_json_loads('{"case":0.0}')
            with self.assertRaises(ValueError):
                SUITE.strict_json_loads('{"input_system":{"p":2147483647.0}}')

    def test_time_v_missing_malformed_nonfinite_and_inconsistent_fail(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            report = Path(temporary) / "time-v.txt"
            report.write_text(
                "\tUser time (seconds): nan\n"
                "\tSystem time (seconds): -0.5\n"
                "\tElapsed (wall clock) time (h:mm:ss or m:ss): 0:99.0\n"
                "\tMaximum resident set size (kbytes): -1\n"
                "\tExit status: 0\n",
                encoding="utf-8",
            )
            observations, errors = SUITE.time_v_report(report)
            self.assertEqual(observations, {"time_exit_status": 0})
            self.assertIn("time_v_malformed:user_seconds", errors)
            self.assertIn("time_v_malformed:system_seconds", errors)
            self.assertIn("time_v_malformed:elapsed_wall", errors)
            self.assertIn("time_v_malformed:maximum_resident_set_kib", errors)
            self.assertIn("time_v_missing:user_seconds", errors)
            self.assertIn(
                "time_v_exit_status_mismatch",
                SUITE.time_v_resource_errors(report, 1),
            )

    def test_staged_custody_ignores_original_replacement_and_detects_verifier_mutation(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            inputs = make_inputs(root)
            bindings = SUITE.validate_inputs(
                inputs["binary"],
                inputs["protocol"],
                inputs["source"],
                inputs["verifier_source"],
                inputs["verifier_executable"],
                inputs["time_binary"],
                inputs["cargo_lock"],
                inputs["cargo_manifest"],
            )
            paths = SUITE.input_paths(
                inputs["binary"],
                inputs["protocol"],
                inputs["source"],
                inputs["verifier_source"],
                inputs["verifier_executable"],
                inputs["time_binary"],
                inputs["cargo_lock"],
                inputs["cargo_manifest"],
            )
            custody = SUITE.stage_inputs(root / "custody", paths, bindings)
            inputs["binary"].write_text("#!/bin/sh\nexit 99\n", encoding="utf-8")
            self.assertEqual(SUITE.custody_errors(custody), [])
            staged_verifier = Path(custody["verifier_source"]["staged"]["path"])
            staged_verifier.chmod(0o600)
            staged_verifier.write_text("print('substituted')\n", encoding="utf-8")
            staged_verifier.chmod(0o400)
            self.assertIn(
                "verifier_source_custody_sha256_mismatch",
                SUITE.custody_errors(custody),
            )
            staged_verifier_runtime = Path(
                custody["verifier_executable"]["staged"]["path"]
            )
            staged_verifier_runtime.chmod(0o400)
            self.assertIn(
                "verifier_executable_custody_mode_mismatch",
                SUITE.custody_errors(custody),
            )

    def test_sealed_memfd_custody_survives_staged_replace_and_child_probe(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            inputs = make_inputs(root)
            paths = SUITE.input_paths(**inputs)
            bindings = SUITE.validate_inputs(**inputs)
            custody = SUITE.stage_inputs(root / "custody", paths, bindings)
            handles, receipts = SUITE.create_execution_memfds(custody)
            try:
                probe = SUITE.probe_memfd_child_access(receipts)
                self.assertEqual(probe["exit_code"], 0)
                self.assertFalse(probe["scientific_producer_invoked"])
                self.assertTrue(
                    probe[
                        "probe_grandchild_reexec_from_wrapper_owned_proc_path"
                    ]
                )
                self.assertEqual(
                    probe["probe_memfd"]["seal_mask"],
                    SUITE.REQUIRED_MEMFD_SEALS,
                )
                staged_producer = Path(custody["binary"]["staged"]["path"])
                original_bytes = staged_producer.read_bytes()
                staged_producer.chmod(0o700)
                replacement = root / "replacement-producer"
                replacement.write_text("#!/bin/sh\nexit 99\n", encoding="utf-8")
                replacement.chmod(0o500)
                os.replace(replacement, staged_producer)
                restored = root / "restored-producer"
                restored.write_bytes(original_bytes)
                restored.chmod(0o500)
                os.replace(restored, staged_producer)
                self.assertEqual(SUITE.sealed_memfd_errors(receipts), [])
                with self.assertRaises(OSError):
                    os.write(handles["binary"], b"x")
            finally:
                for descriptor in handles.values():
                    os.close(descriptor)

    def test_staged_custody_recheck_bounds_sparse_and_growing_files(self) -> None:
        for mutation in ("sparse_replace", "grow_same_inode"):
            with self.subTest(mutation=mutation), tempfile.TemporaryDirectory() as temporary:
                root = Path(temporary)
                inputs = make_inputs(root)
                paths = SUITE.input_paths(**inputs)
                bindings = SUITE.validate_inputs(**inputs)
                custody = SUITE.stage_inputs(root / "custody", paths, bindings)
                expected = custody["binary"]["staged"]
                target = Path(expected["path"])
                if mutation == "sparse_replace":
                    replacement = root / "sparse-producer"
                    with replacement.open("wb") as handle:
                        handle.truncate(64 * 1024 * 1024)
                    replacement.chmod(expected["mode"])
                    os.replace(replacement, target)
                else:
                    target.chmod(expected["mode"] | 0o200)
                original_read = SUITE.os.read
                observed_bytes = 0
                grew = False

                def bounded_read(descriptor: int, count: int) -> bytes:
                    nonlocal observed_bytes, grew
                    chunk = original_read(descriptor, count)
                    observed_bytes += len(chunk)
                    if mutation == "grow_same_inode" and not grew:
                        grew = True
                        with target.open("r+b") as handle:
                            handle.truncate(64 * 1024 * 1024)
                    return chunk

                with mock.patch.object(SUITE.os, "read", side_effect=bounded_read):
                    errors = SUITE.custody_errors(custody, ("binary",))
                self.assertLessEqual(observed_bytes, expected["bytes"] + 1)
                self.assertIn("binary_custody_size_mismatch", errors)
                self.assertIn("binary_custody_sha256_mismatch", errors)
                if mutation == "sparse_replace":
                    self.assertIn("binary_custody_inode_mismatch", errors)

    def test_bootstrap_recheck_bounds_sparse_and_growing_files(self) -> None:
        for mutation in ("sparse_replace", "grow_same_inode"):
            with self.subTest(mutation=mutation), tempfile.TemporaryDirectory() as temporary:
                root = Path(temporary)
                launcher = root / "launcher.py"
                runtime = root / "python-runtime"
                launcher.write_text("print('stable')\n", encoding="utf-8")
                runtime.write_bytes(b"stable-runtime\n")
                bootstrap = {
                    "launcher_source": SUITE.bootstrap_binding(launcher),
                    "python_executable": SUITE.bootstrap_binding(runtime),
                }
                expected = bootstrap["launcher_source"]
                if mutation == "sparse_replace":
                    replacement = root / "sparse-launcher"
                    with replacement.open("wb") as handle:
                        handle.truncate(64 * 1024 * 1024)
                    os.replace(replacement, launcher)
                original_read = SUITE.os.read
                observed_bytes = 0
                grew = False

                def bounded_read(descriptor: int, count: int) -> bytes:
                    nonlocal observed_bytes, grew
                    chunk = original_read(descriptor, count)
                    observed_bytes += len(chunk)
                    if mutation == "grow_same_inode" and not grew:
                        grew = True
                        with launcher.open("r+b") as handle:
                            handle.truncate(64 * 1024 * 1024)
                    return chunk

                with mock.patch.object(SUITE.os, "read", side_effect=bounded_read):
                    errors = SUITE.bootstrap_binding_errors(bootstrap)
                maximum_read = (
                    bootstrap["launcher_source"]["bytes"]
                    + 1
                    + bootstrap["python_executable"]["bytes"]
                )
                self.assertLessEqual(observed_bytes, maximum_read)
                self.assertIn("launcher_source_bootstrap_binding_changed", errors)

    def test_missing_image_digest_fails_preflight_with_durable_receipt(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            inputs = make_inputs(root)
            output = root / "attempt"
            receipt, status = SUITE.run_suite(
                inputs["binary"],
                inputs["protocol"],
                inputs["source"],
                inputs["verifier_source"],
                inputs["verifier_executable"],
                inputs["time_binary"],
                inputs["cargo_lock"],
                inputs["cargo_manifest"],
                output,
                root,
                None,
                SUITE.PROCESS_CONTAINMENT_GUARANTEE,
            )
            self.assertEqual(status, 1)
            self.assertFalse(receipt["producer_suite_started"])
            self.assertEqual(
                receipt["preflight_failure"]["message"],
                "container image digest is required",
            )
            on_disk = json.loads(
                (output / "suite-receipt.json").read_text(encoding="utf-8")
            )
            self.assertEqual(on_disk["launch_attempt_id"], receipt["launch_attempt_id"])

    def test_missing_or_invalid_containment_guarantee_fails_preflight(self) -> None:
        for index, guarantee in enumerate((None, "free-form-claim")):
            with self.subTest(guarantee=guarantee), tempfile.TemporaryDirectory() as temporary:
                root = Path(temporary)
                inputs = make_inputs(root)
                receipt, status = SUITE.run_suite(
                    inputs["binary"],
                    inputs["protocol"],
                    inputs["source"],
                    inputs["verifier_source"],
                    inputs["verifier_executable"],
                    inputs["time_binary"],
                    inputs["cargo_lock"],
                    inputs["cargo_manifest"],
                    root / f"attempt-{index}",
                    root,
                    "sha256:test-image",
                    guarantee,
                )
                self.assertEqual(status, 1)
                self.assertFalse(receipt["producer_suite_started"])
                self.assertIn(
                    "process containment guarantee",
                    receipt["preflight_failure"]["message"],
                )
                self.assertFalse(receipt["producer_invoked"])

    def test_child_procfd_probe_failure_fails_preflight_and_closes_memfds(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            inputs = make_inputs(root)
            output = root / "attempt"
            captured_handles: dict[str, int] = {}
            original_create = SUITE.create_execution_memfds

            def recording_create(
                custody: dict[str, dict[str, object]],
            ) -> tuple[dict[str, int], dict[str, dict[str, object]]]:
                handles, receipts = original_create(custody)
                captured_handles.update(handles)
                return handles, receipts

            with mock.patch.object(
                SUITE,
                "create_execution_memfds",
                side_effect=recording_create,
            ), mock.patch.object(
                SUITE,
                "probe_memfd_child_access",
                side_effect=PermissionError("procfd child access denied"),
            ):
                receipt, status = SUITE.run_suite(
                    inputs["binary"],
                    inputs["protocol"],
                    inputs["source"],
                    inputs["verifier_source"],
                    inputs["verifier_executable"],
                    inputs["time_binary"],
                    inputs["cargo_lock"],
                    inputs["cargo_manifest"],
                    output,
                    root,
                    "sha256:test-image",
                    SUITE.PROCESS_CONTAINMENT_GUARANTEE,
                )
            self.assertEqual(status, 1)
            self.assertFalse(receipt["producer_suite_started"])
            self.assertEqual(receipt["preflight_failure"]["type"], "PermissionError")
            for descriptor in captured_handles.values():
                with self.assertRaises(OSError):
                    os.fstat(descriptor)

    def test_popen_exception_leaves_partial_receipt_and_seal(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            inputs = make_inputs(root)
            inputs["time_binary"].write_text("not-an-executable-format\n", encoding="utf-8")
            inputs["time_binary"].chmod(0o700)
            output = root / "attempt"
            receipt, status = SUITE.run_suite(
                inputs["binary"],
                inputs["protocol"],
                inputs["source"],
                inputs["verifier_source"],
                inputs["verifier_executable"],
                inputs["time_binary"],
                inputs["cargo_lock"],
                inputs["cargo_manifest"],
                output,
                root,
                "sha256:test-image",
                SUITE.PROCESS_CONTAINMENT_GUARANTEE,
            )
            self.assertEqual(status, 1)
            self.assertEqual(receipt["launcher_exception"]["type"], "OSError")
            self.assertFalse(
                receipt["outer_cap_attestation"][
                    "complete_suite_finished_by_deadline"
                ]
            )
            self.assertTrue((output / "suite-receipt.json").is_file())
            self.assertFalse((output / "suite-seal.json").exists())
            self.assertFalse((output / "suite-seal-completion.json").exists())
            self.assertTrue((output / "suite-seal-failure.json").is_file())
            self.assertTrue(
                (output / "suite-seal-failure-completion.json").is_file()
            )
            on_disk = json.loads(
                (output / "suite-receipt.json").read_text(encoding="utf-8")
            )
            self.assertEqual(on_disk["launcher_exception"]["type"], "OSError")
            self.assertEqual(receipt["producer_invocation_count"], 0)
            self.assertFalse(receipt["producer_invoked"])
            self.assertFalse(receipt["scientific_evaluation_performed"])
            launch_failed = receipt["cases"][0]
            self.assertTrue(launch_failed["launch_failed_before_process_start"])
            self.assertEqual(launch_failed["launch_failure"]["type"], "OSError")
            self.assertFalse(launch_failed["process_started"])
            self.assertIsNone(launch_failed["process_pid"])
            self.assertIsNotNone(launch_failed["launch_attempted_utc"])
            self.assertIsNotNone(
                launch_failed["launch_attempted_monotonic_ns"]
            )
            self.assertIsNone(launch_failed["started_monotonic_ns"])
            self.assertIsNone(launch_failed["process_finished_monotonic_ns"])
            self.assertIsNone(launch_failed["evidence_finished_monotonic_ns"])
            self.assertIsNone(launch_failed["endpoint_fsync"])
            self.assertEqual(
                launch_failed["endpoint_fsync_errors"],
                ["launch_failed_before_process_start"],
            )
            self.assertNotIn("not_started_launcher_exception", launch_failed)
            self.assertTrue(
                all(
                    row.get("not_started_launcher_exception") is True
                    and row["process_started"] is False
                    and row["process_pid"] is None
                    and row["endpoint_fsync"] is None
                    and row["endpoint_fsync_errors"] == ["launcher_exception"]
                    for row in receipt["cases"][1:]
                )
            )
            failure_marker = json.loads(
                (output / "suite-seal-failure.json").read_text(encoding="utf-8")
            )
            failure_completion = json.loads(
                (output / "suite-seal-failure-completion.json").read_text(
                    encoding="utf-8"
                )
            )
            self.assertEqual(
                failure_marker["required_completion_artifact"],
                "suite-seal-failure-completion.json",
            )
            self.assertEqual(
                failure_completion["schema"],
                "prime31.rr9.full8-f4-suite-seal-failure-completion.v1",
            )
            self.assertEqual(
                set(failure_completion),
                {
                    "schema",
                    "launch_attempt_id",
                    "producer_cap_excluded",
                    "receipt",
                    "failure_marker",
                    "started_monotonic_ns",
                    "finished_monotonic_ns",
                    "elapsed_ns",
                    "resource_start",
                    "resource_finish",
                    "resource_delta",
                    "failure_marker_and_directory_fsync_completed",
                    "completion_receipt_emission_accounted_locally",
                    "external_whole_job_accounting_required",
                },
            )
            self.assertTrue(
                failure_completion[
                    "failure_marker_and_directory_fsync_completed"
                ]
            )
            self.assertFalse(
                failure_completion[
                    "completion_receipt_emission_accounted_locally"
                ]
            )
            self.assertTrue(
                failure_completion["external_whole_job_accounting_required"]
            )
            self.assertTrue(
                SUITE.artifact_matches(
                    failure_completion["receipt"],
                    SUITE.artifact(output / "suite-receipt.json"),
                )
            )
            self.assertTrue(
                SUITE.artifact_matches(
                    failure_completion["failure_marker"],
                    SUITE.artifact(output / "suite-seal-failure.json"),
                )
            )

    def test_unexpected_postcap_exception_persists_full_receipt_and_closes_memfds(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            inputs = make_inputs(root)
            output = root / "attempt"
            captured_handles: dict[str, int] = {}
            original_create = SUITE.create_execution_memfds

            def recording_create(
                custody: dict[str, dict[str, object]],
            ) -> tuple[dict[str, int], dict[str, dict[str, object]]]:
                handles, receipts = original_create(custody)
                captured_handles.update(handles)
                return handles, receipts

            with mock.patch.object(
                SUITE,
                "create_execution_memfds",
                side_effect=recording_create,
            ), mock.patch.object(
                SUITE,
                "custody_recheck",
                side_effect=OSError("injected-postcap-io-failure"),
            ):
                receipt, status = SUITE.run_suite(
                    inputs["binary"],
                    inputs["protocol"],
                    inputs["source"],
                    inputs["verifier_source"],
                    inputs["verifier_executable"],
                    inputs["time_binary"],
                    inputs["cargo_lock"],
                    inputs["cargo_manifest"],
                    output,
                    root,
                    "sha256:test-image",
                    SUITE.PROCESS_CONTAINMENT_GUARANTEE,
                )
            self.assertEqual(status, 1)
            self.assertEqual(
                receipt["unexpected_postcap_exception"]["type"], "OSError"
            )
            self.assertIn("outer_cap_attestation", receipt)
            self.assertTrue((output / "suite-receipt.json").is_file())
            self.assertTrue((output / "suite-seal-failure.json").is_file())
            on_disk = json.loads(
                (output / "suite-receipt.json").read_text(encoding="utf-8")
            )
            self.assertEqual(len(on_disk["cases"]), len(SUITE.CASE_ORDER))
            self.assertEqual(on_disk["receipt_seal_failure"]["type"], "OSError")
            failure_completion = json.loads(
                (output / "suite-seal-failure-completion.json").read_text(
                    encoding="utf-8"
                )
            )
            self.assertEqual(
                failure_completion["started_monotonic_ns"],
                receipt["suite_finished_monotonic_ns"],
            )
            self.assertTrue(failure_completion["producer_cap_excluded"])
            self.assertEqual(
                failure_completion["resource_start"],
                receipt["producer_suite_resource_accounting"]["finish"],
            )
            for descriptor in captured_handles.values():
                with self.assertRaises(OSError):
                    os.fstat(descriptor)

    def test_staged_custody_mutation_between_postcap_and_seal_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            inputs = make_inputs(root)
            output = root / "attempt"
            original_seal = SUITE.seal_suite_receipt

            def mutate_then_seal(
                seal_output: Path,
                receipt: dict[str, object],
                **kwargs: object,
            ) -> dict[str, object]:
                custody = receipt["execution_custody"]
                assert isinstance(custody, dict)
                protocol_entry = custody["protocol"]
                assert isinstance(protocol_entry, dict)
                staged = protocol_entry["staged"]
                assert isinstance(staged, dict)
                path = Path(staged["path"])
                path.chmod(0o600)
                path.write_text("mutated after postcap\n", encoding="utf-8")
                path.chmod(0o400)
                return original_seal(seal_output, receipt, **kwargs)

            with mock.patch.object(
                SUITE, "seal_suite_receipt", side_effect=mutate_then_seal
            ):
                receipt, status = SUITE.run_suite(
                    inputs["binary"],
                    inputs["protocol"],
                    inputs["source"],
                    inputs["verifier_source"],
                    inputs["verifier_executable"],
                    inputs["time_binary"],
                    inputs["cargo_lock"],
                    inputs["cargo_manifest"],
                    output,
                    root,
                    "sha256:test-image",
                    SUITE.PROCESS_CONTAINMENT_GUARANTEE,
                )
            self.assertEqual(status, 1)
            self.assertIn(
                "staged custody drift before seal",
                receipt["receipt_seal_failure"]["message"],
            )
            self.assertFalse((output / "suite-seal.json").exists())

    def test_drain_thread_start_failure_kills_and_reaps_child(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            inputs = make_inputs(root)
            output = root / "attempt"
            spawned_pids: list[int] = []
            original_popen = SUITE.subprocess.Popen

            def recording_popen(*args: object, **kwargs: object) -> object:
                process = original_popen(*args, **kwargs)
                spawned_pids.append(process.pid)
                return process

            with mock.patch.object(
                SUITE.subprocess, "Popen", side_effect=recording_popen
            ), mock.patch.object(
                SUITE.threading.Thread,
                "start",
                side_effect=RuntimeError("thread-start-failure"),
            ):
                receipt, status = SUITE.run_suite(
                    inputs["binary"],
                    inputs["protocol"],
                    inputs["source"],
                    inputs["verifier_source"],
                    inputs["verifier_executable"],
                    inputs["time_binary"],
                    inputs["cargo_lock"],
                    inputs["cargo_manifest"],
                    output,
                    root,
                    "sha256:test-image",
                    SUITE.PROCESS_CONTAINMENT_GUARANTEE,
                )
            self.assertEqual(status, 1)
            self.assertEqual(receipt["launcher_exception"]["type"], "RuntimeError")
            self.assertEqual(receipt["producer_invocation_count"], 1)
            self.assertTrue(receipt["scientific_evaluation_performed"])
            started_row = receipt["cases"][0]
            self.assertTrue(started_row["started_but_endpoint_failed"])
            self.assertTrue(started_row["process_started"])
            self.assertNotIn("not_started_launcher_exception", started_row)
            self.assertNotIn("launch_failed_before_process_start", started_row)
            self.assertEqual(started_row["endpoint_failure"]["type"], "RuntimeError")
            self.assertIsNotNone(started_row["started_monotonic_ns"])
            self.assertIsNotNone(started_row["process_finished_monotonic_ns"])
            self.assertIsNotNone(started_row["exit_code"])
            self.assertIn(started_row["process_pid"], spawned_pids)
            self.assertTrue(
                started_row["process_group_cleanup"]["absence_verified"]
            )
            self.assertIsNone(started_row["endpoint_fsync"])
            self.assertFalse(started_row["evidence_endpoint_complete"])
            self.assertFalse(
                receipt["outer_cap_attestation"][
                    "complete_suite_finished_by_deadline"
                ]
            )
            self.assertTrue(
                all(
                    row.get("not_started_launcher_exception") is True
                    for row in receipt["cases"][1:]
                )
            )
            self.assertTrue(spawned_pids)
            self.assertTrue(
                all(not SUITE.process_group_exists(pid) for pid in spawned_pids)
            )

    def test_post_popen_registration_failure_is_a_started_case(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            inputs = make_inputs(root)
            output = root / "attempt"
            original_run_case = SUITE.run_case

            def fail_before_receipt_registration(
                *args: object, **kwargs: object
            ) -> dict[str, object]:
                mutable_args = list(args)

                def injected_registration_failure(_: object) -> None:
                    raise RuntimeError("injected-pre-registration-failure")

                mutable_args[6] = injected_registration_failure
                return original_run_case(*mutable_args, **kwargs)

            with mock.patch.object(
                SUITE,
                "run_case",
                side_effect=fail_before_receipt_registration,
            ):
                receipt, status = SUITE.run_suite(
                    inputs["binary"],
                    inputs["protocol"],
                    inputs["source"],
                    inputs["verifier_source"],
                    inputs["verifier_executable"],
                    inputs["time_binary"],
                    inputs["cargo_lock"],
                    inputs["cargo_manifest"],
                    output,
                    root,
                    "sha256:test-image",
                    SUITE.PROCESS_CONTAINMENT_GUARANTEE,
                )
            self.assertEqual(status, 1)
            self.assertEqual(receipt["producer_invocation_count"], 1)
            started_row = receipt["cases"][0]
            self.assertTrue(started_row["process_started"])
            self.assertIsInstance(started_row["process_pid"], int)
            self.assertTrue(started_row["started_but_endpoint_failed"])
            self.assertEqual(
                started_row["endpoint_failure"],
                {
                    "type": "RuntimeError",
                    "message": "injected-pre-registration-failure",
                },
            )
            self.assertNotIn("not_started_launcher_exception", started_row)
            self.assertTrue(
                started_row["process_group_cleanup"]["absence_verified"]
            )
            self.assertTrue(
                all(
                    row.get("not_started_launcher_exception") is True
                    for row in receipt["cases"][1:]
                )
            )

    def test_outer_deadline_kill_never_uses_blocking_wait(self) -> None:
        process = mock.Mock()
        process.pid = 424242
        process.poll.return_value = None
        with mock.patch.object(SUITE.os, "killpg"):
            observation = SUITE.kill_group_at_outer_deadline(process)
        process.wait.assert_not_called()
        process.poll.assert_called_once_with()
        self.assertFalse(observation["blocking_wait_used"])
        self.assertFalse(observation["leader_terminal_observed"])
        self.assertIsNone(observation["leader_return_code"])

    def test_unobserved_leader_has_no_process_finished_timestamp(self) -> None:
        process = mock.Mock()
        process.pid = 424242
        process.returncode = None
        with mock.patch.object(
            SUITE.time,
            "monotonic_ns",
            side_effect=AssertionError(
                "nonterminal process must not receive a finish timestamp"
            ),
        ):
            observed_ns = SUITE.reconcile_process_terminal_observation(
                process, {"reaped": []}
            )
        self.assertIsNone(observed_ns)
        self.assertIsNone(process.returncode)

    def test_process_group_cleanup_never_sleeps_past_deadline(self) -> None:
        deadline_ns = 50_000
        with mock.patch.object(
            SUITE.time, "monotonic_ns", return_value=deadline_ns
        ), mock.patch.object(
            SUITE.time,
            "sleep",
            side_effect=AssertionError("must not sleep after deadline"),
        ), mock.patch.object(
            SUITE.os, "killpg"
        ), mock.patch.object(
            SUITE, "process_group_exists", return_value=True
        ), mock.patch.object(
            SUITE, "reap_process_group", return_value=[]
        ):
            cleanup = SUITE.terminate_surviving_process_group(
                424242, deadline_ns
            )
        self.assertFalse(cleanup["absence_verified"])
        self.assertEqual(cleanup["polls"], 0)

    def test_started_endpoint_failure_preserves_completed_helper_receipt(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            inputs = make_valid_execution_inputs(root)
            output = root / "attempt"
            original_endpoint = SUITE.fsync_and_snapshot_artifact
            endpoint_calls = 0

            def fail_second_endpoint(*args: object, **kwargs: object) -> object:
                nonlocal endpoint_calls
                endpoint_calls += 1
                if endpoint_calls == 2:
                    raise RuntimeError("injected-second-endpoint-failure")
                return original_endpoint(*args, **kwargs)

            with mock.patch.object(
                SUITE,
                "fsync_and_snapshot_artifact",
                side_effect=fail_second_endpoint,
            ):
                receipt, status = SUITE.run_suite(
                    inputs["binary"],
                    inputs["protocol"],
                    inputs["source"],
                    inputs["verifier_source"],
                    inputs["verifier_executable"],
                    inputs["time_binary"],
                    inputs["cargo_lock"],
                    inputs["cargo_manifest"],
                    output,
                    root,
                    "sha256:test-image",
                    SUITE.PROCESS_CONTAINMENT_GUARANTEE,
                )
            self.assertEqual(status, 1)
            started_row = receipt["cases"][0]
            self.assertTrue(started_row["process_started"])
            self.assertTrue(started_row["started_but_endpoint_failed"])
            self.assertEqual(
                started_row["endpoint_failure"]["message"],
                "injected-second-endpoint-failure",
            )
            self.assertEqual(set(started_row["endpoint_fsync"]), {"stdout"})
            self.assertTrue(started_row["endpoint_fsync"]["stdout"]["success"])
            self.assertFalse(started_row["evidence_endpoint_complete"])
            self.assertTrue(
                all(
                    row.get("not_started_launcher_exception") is True
                    for row in receipt["cases"][1:]
                )
            )

    def test_early_suite_failure_uses_initial_open_accounting_boundary(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            inputs = make_inputs(root)
            output = root / "attempt"
            with mock.patch.object(
                SUITE,
                "producer_cap_endpoint",
                side_effect=RuntimeError("injected-suite-endpoint-failure"),
            ):
                receipt, status = SUITE.run_suite(
                    inputs["binary"],
                    inputs["protocol"],
                    inputs["source"],
                    inputs["verifier_source"],
                    inputs["verifier_executable"],
                    inputs["time_binary"],
                    inputs["cargo_lock"],
                    inputs["cargo_manifest"],
                    output,
                    root,
                    "sha256:test-image",
                    SUITE.PROCESS_CONTAINMENT_GUARANTEE,
                )
            self.assertEqual(status, 1)
            failure_marker = json.loads(
                (output / "suite-seal-failure.json").read_text(encoding="utf-8")
            )
            failure_completion = json.loads(
                (output / "suite-seal-failure-completion.json").read_text(
                    encoding="utf-8"
                )
            )
            self.assertFalse(failure_marker["producer_cap_excluded"])
            self.assertFalse(failure_completion["producer_cap_excluded"])
            self.assertEqual(
                failure_completion["started_monotonic_ns"],
                receipt["started_monotonic_ns"],
            )
            self.assertEqual(
                failure_completion["resource_start"],
                receipt["preflight"]["resource_finish"],
            )

    def test_invocation_is_recorded_but_missing_metrics_is_nonadmissible(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            inputs = make_inputs(root)
            inputs["time_binary"].write_text(
                "#!/bin/sh\nshift 3\n\"$@\"\n", encoding="utf-8"
            )
            inputs["time_binary"].chmod(0o700)
            output = root / "attempt"
            original_seal = SUITE.seal_suite_receipt
            original_fsync = SUITE.os.fsync

            def reject_raw_or_manifest_refsync(descriptor: int) -> None:
                target = os.readlink(f"/proc/self/fd/{descriptor}")
                if "/case-" in target or target.endswith("custody-manifest.json"):
                    raise AssertionError(f"forbidden postcap refsync: {target}")
                original_fsync(descriptor)

            def guarded_seal(
                seal_output: Path,
                seal_receipt: dict[str, object],
                **kwargs: object,
            ) -> dict[str, object]:
                with mock.patch.object(
                    SUITE.os,
                    "fsync",
                    side_effect=reject_raw_or_manifest_refsync,
                ):
                    return original_seal(seal_output, seal_receipt, **kwargs)

            with mock.patch.object(
                SUITE, "seal_suite_receipt", side_effect=guarded_seal
            ):
                receipt, status = SUITE.run_suite(
                    inputs["binary"],
                    inputs["protocol"],
                    inputs["source"],
                    inputs["verifier_source"],
                    inputs["verifier_executable"],
                    inputs["time_binary"],
                    inputs["cargo_lock"],
                    inputs["cargo_manifest"],
                    output,
                    root,
                    "sha256:test-image",
                    SUITE.PROCESS_CONTAINMENT_GUARANTEE,
                )
            self.assertEqual(status, 1)
            self.assertTrue(receipt["producer_invoked"])
            self.assertEqual(receipt["producer_invocation_count"], 3)
            self.assertTrue(receipt["scientific_evaluation_performed"])
            self.assertTrue(
                receipt["outer_cap_attestation"][
                    "complete_suite_finished_by_deadline"
                ]
            )
            self.assertFalse(receipt["producer_suite_admissible"])
            self.assertTrue(
                all(row["time_v_resource_errors"] for row in receipt["cases"])
            )

    def test_normal_exit_drains_background_group_before_raw_capture(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            inputs = make_inputs(root)
            outputs: list[str] = []
            for case in SUITE.CASE_ORDER:
                input_system = {
                    "schema": SUITE.EXPECTED_INPUT_SCHEMA,
                    "case": case,
                }
                compact = json.dumps(
                    input_system, separators=(",", ":"), sort_keys=True
                ).encode("utf-8")
                outputs.append(
                    json.dumps(
                        {
                            "schema": SUITE.EXPECTED_PRODUCER_SCHEMA,
                            "case": case,
                            "input_system": input_system,
                            "input_system_sha256": hashlib.sha256(compact).hexdigest(),
                        },
                        separators=(",", ":"),
                    )
                )
            inputs["binary"].write_text(
                "#!/bin/sh\n"
                "case \"$1\" in\n"
                f"0) printf '%s\\n' '{outputs[0]}' ;;\n"
                f"1) printf '%s\\n' '{outputs[1]}' ;;\n"
                f"2) printf '%s\\n' '{outputs[2]}' ;;\n"
                "esac\n"
                "(sleep 0.20; printf 'LATE') &\n"
                "exit 0\n",
                encoding="utf-8",
            )
            inputs["binary"].chmod(0o700)
            inputs["time_binary"].write_text(
                "#!/bin/sh\n"
                "shift 2\n"
                "report=$1\n"
                "shift\n"
                '"$@"\n'
                "status=$?\n"
                "{\n"
                "echo 'User time (seconds): 0.01'\n"
                "echo 'System time (seconds): 0.01'\n"
                "echo 'Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.02'\n"
                "echo 'Maximum resident set size (kbytes): 1024'\n"
                "echo \"Exit status: $status\"\n"
                "} > \"$report\"\n"
                "exit \"$status\"\n",
                encoding="utf-8",
            )
            inputs["time_binary"].chmod(0o700)
            output = root / "attempt"
            seal_events: list[str] = []
            original_write_json_new = SUITE.write_json_new
            original_fsync_directory = SUITE.fsync_directory
            original_resource_snapshot = SUITE.resource_snapshot
            original_recheck_bound_artifact = SUITE.recheck_bound_artifact

            def tracking_write_json_new(
                path: Path, value: object
            ) -> dict[str, object]:
                binding = original_write_json_new(path, value)
                if path.name == "suite-seal.json":
                    seal_events.append("seal_written_and_file_fsynced")
                elif path.name == "suite-seal-completion.json":
                    seal_events.append("completion_written_and_file_fsynced")
                return binding

            def tracking_fsync_directory(path: Path) -> None:
                original_fsync_directory(path)
                if (
                    (output / "suite-seal.json").exists()
                    and not (output / "suite-seal-completion.json").exists()
                ):
                    seal_events.append("seal_directory_fsynced")

            def tracking_resource_snapshot() -> dict[str, object]:
                if (
                    (output / "suite-seal.json").exists()
                    and not (output / "suite-seal-completion.json").exists()
                ):
                    seal_events.append("seal_finish_resource_snapshot")
                return original_resource_snapshot()

            def reject_post_completion_recheck(
                path: Path, expected: dict[str, object], label: str
            ) -> dict[str, object]:
                if (output / "suite-seal-completion.json").exists():
                    raise AssertionError(
                        "no fallible validation may follow completion publication"
                    )
                return original_recheck_bound_artifact(path, expected, label)

            with mock.patch.object(
                SUITE, "write_json_new", side_effect=tracking_write_json_new
            ), mock.patch.object(
                SUITE, "fsync_directory", side_effect=tracking_fsync_directory
            ), mock.patch.object(
                SUITE, "resource_snapshot", side_effect=tracking_resource_snapshot
            ), mock.patch.object(
                SUITE,
                "recheck_bound_artifact",
                side_effect=reject_post_completion_recheck,
            ), mock.patch.object(
                SUITE,
                "validate_completion_artifact",
                side_effect=AssertionError(
                    "completion publication must be the terminal local action"
                ),
            ):
                receipt, status = SUITE.run_suite(
                    inputs["binary"],
                    inputs["protocol"],
                    inputs["source"],
                    inputs["verifier_source"],
                    inputs["verifier_executable"],
                    inputs["time_binary"],
                    inputs["cargo_lock"],
                    inputs["cargo_manifest"],
                    output,
                    root,
                    "sha256:test-image",
                    SUITE.PROCESS_CONTAINMENT_GUARANTEE,
                )
            self.assertEqual(status, 0)
            self.assertEqual(
                seal_events,
                [
                    "seal_written_and_file_fsynced",
                    "seal_directory_fsynced",
                    "seal_finish_resource_snapshot",
                    "completion_written_and_file_fsynced",
                ],
            )
            self.assertTrue(receipt["producer_suite_admissible"])
            self.assertEqual(receipt["outer_cap_scope"], SUITE.OUTER_CAP_SCOPE)
            self.assertIn(
                "stdout/stderr/time-v",
                receipt["outer_cap_attestation"]["boundary"],
            )
            self.assertNotIn("launcher", receipt["execution_custody"])
            for name in ("launcher_source", "python_executable"):
                self.assertTrue(
                    SUITE.artifact_matches(
                        receipt["launcher_bootstrap"][name],
                        receipt["postcap_bootstrap"][name],
                    )
                )
                self.assertTrue(
                    SUITE.artifact_matches(
                        receipt["launcher_bootstrap"][name],
                        receipt["final_seal_bootstrap"][name],
                    )
                )
            self.assertFalse(
                receipt["sealed_memfd_child_access_probe"][
                    "scientific_producer_invoked"
                ]
            )
            self.assertEqual(
                receipt["sealed_memfd_child_access_probe"]["exit_code"], 0
            )
            self.assertEqual(
                receipt["started_monotonic_ns"],
                receipt["preflight"]["finished_monotonic_ns"],
            )
            self.assertEqual(
                receipt["postcap_custody_recheck"]["started_monotonic_ns"],
                receipt["suite_finished_monotonic_ns"],
            )
            self.assertIn("children", receipt["preflight"]["resource_delta"])
            suite_resources = receipt["producer_suite_resource_accounting"]
            self.assertTrue(suite_resources["outer_cap_included"])
            self.assertTrue(suite_resources["not_additive_with_case_time_v"])
            self.assertTrue(suite_resources["includes_rusage_self"])
            self.assertTrue(suite_resources["includes_rusage_children"])
            self.assertEqual(
                suite_resources["started_monotonic_ns"],
                receipt["started_monotonic_ns"],
            )
            self.assertEqual(
                suite_resources["finished_monotonic_ns"],
                receipt["suite_finished_monotonic_ns"],
            )
            self.assertEqual(
                suite_resources["start"], receipt["preflight"]["resource_finish"]
            )
            self.assertEqual(
                receipt["postcap_custody_recheck"]["resource_start"],
                suite_resources["finish"],
            )
            self.assertIn("self", suite_resources["delta"])
            self.assertIn("children", suite_resources["delta"])
            self.assertTrue(
                all(
                    row["post_leader_group_cleanup_attempted"]
                    for row in receipt["cases"]
                )
            )
            time.sleep(0.25)
            self.assertEqual(SUITE.raw_artifact_errors(receipt, output), [])
            self.assertTrue(
                all(
                    b"LATE" not in (output / f"case-{case}.stdout").read_bytes()
                    for case in SUITE.CASE_ORDER
                )
            )
            seal = json.loads(
                (output / "suite-seal.json").read_text(encoding="utf-8")
            )
            completion = json.loads(
                (output / "suite-seal-completion.json").read_text(
                    encoding="utf-8"
                )
            )
            self.assertEqual(
                seal["required_completion_artifact"],
                "suite-seal-completion.json",
            )
            for premature_field in (
                "finished_monotonic_ns",
                "elapsed_ns",
                "resource_finish",
                "resource_delta",
                "receipt_fsynced",
                "containing_directory_fsynced",
            ):
                self.assertNotIn(premature_field, seal)
            self.assertEqual(
                completion["schema"],
                "prime31.rr9.full8-f4-suite-seal-completion.v1",
            )
            self.assertEqual(
                set(completion),
                {
                    "schema",
                    "launch_attempt_id",
                    "producer_cap_excluded",
                    "seal",
                    "receipt",
                    "started_monotonic_ns",
                    "finished_monotonic_ns",
                    "elapsed_ns",
                    "resource_start",
                    "resource_finish",
                    "resource_delta",
                    "seal_and_directory_fsync_completed",
                    "completion_receipt_emission_accounted_locally",
                    "external_whole_job_accounting_required",
                },
            )
            self.assertEqual(
                completion["launch_attempt_id"], receipt["launch_attempt_id"]
            )
            self.assertEqual(
                completion["elapsed_ns"],
                completion["finished_monotonic_ns"]
                - completion["started_monotonic_ns"],
            )
            self.assertEqual(
                completion["started_monotonic_ns"],
                receipt["postcap_custody_recheck"]["finished_monotonic_ns"],
            )
            self.assertEqual(
                completion["resource_start"],
                receipt["postcap_custody_recheck"]["resource_finish"],
            )
            self.assertTrue(completion["seal_and_directory_fsync_completed"])
            self.assertFalse(
                completion["completion_receipt_emission_accounted_locally"]
            )
            self.assertTrue(
                completion["external_whole_job_accounting_required"]
            )
            self.assertTrue(
                SUITE.artifact_matches(
                    completion["receipt"],
                    SUITE.artifact(output / "suite-receipt.json"),
                )
            )
            self.assertTrue(
                SUITE.artifact_matches(
                    completion["seal"],
                    SUITE.artifact(output / "suite-seal.json"),
                )
            )
            self.assertFalse((output / "suite-seal-failure.json").exists())
            self.assertFalse(
                (output / "suite-seal-failure-completion.json").exists()
            )
            self.assertEqual(
                len(seal["raw_artifact_endpoint_fsync_receipts"]), 9
            )
            self.assertTrue(
                all(
                    row["success"]
                    for row in seal["raw_artifact_endpoint_fsync_receipts"]
                )
            )
            self.assertFalse(seal["raw_artifacts_refsynced_postcap"])
            self.assertEqual(len(seal["raw_artifacts_final_rehash"]), 9)
            self.assertFalse(seal["custody_manifest_refsynced_postcap"])
            self.assertEqual(
                {
                    (row["case"], row["stream"])
                    for row in seal["raw_artifact_endpoint_fsync_receipts"]
                },
                {
                    (case, stream)
                    for case in SUITE.CASE_ORDER
                    for stream in ("stdout", "stderr", "time_v")
                },
            )
            for memfd in receipt["sealed_execution_memfds"].values():
                with self.assertRaises(OSError):
                    os.fstat(memfd["fd"])

    def test_mocked_monotonic_endpoint_is_inclusive_and_fails_one_ns_late(self) -> None:
        with mock.patch.object(SUITE.time, "monotonic_ns", return_value=1_000):
            self.assertEqual(SUITE.producer_cap_endpoint(1_000), (1_000, True))
        with mock.patch.object(SUITE.time, "monotonic_ns", return_value=1_001):
            self.assertEqual(SUITE.producer_cap_endpoint(1_000), (1_001, False))

    def test_bounded_pipe_accepts_cap_and_truncates_cap_plus_one(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            exact_state: dict[str, object] = {}
            SUITE.drain_bounded_stream(
                io.BytesIO(b"a" * 64),
                root / "exact",
                64,
                2_147_483_647,
                exact_state,
            )
            self.assertFalse(exact_state["cap_exceeded"])
            self.assertEqual((root / "exact").stat().st_size, 64)
            overflow_state: dict[str, object] = {}
            SUITE.drain_bounded_stream(
                io.BytesIO(b"b" * 65),
                root / "overflow",
                64,
                2_147_483_647,
                overflow_state,
            )
            self.assertTrue(overflow_state["cap_exceeded"])
            self.assertEqual(overflow_state["observed_bytes"], 65)
            self.assertEqual(overflow_state["observed_bytes_kind"], "exact")
            self.assertEqual(overflow_state["written_bytes"], 64)
            self.assertTrue(overflow_state["retained_prefix"])
            self.assertEqual(overflow_state["retained_prefix_bytes"], 64)
            self.assertEqual(
                overflow_state["termination_reason"],
                "write_cap_exceeded_process_group_killed",
            )
            self.assertEqual((root / "overflow").stat().st_size, 64)

    def test_overflow_prefix_is_never_parsed_as_json_or_time_evidence(self) -> None:
        input_system = {"schema": SUITE.EXPECTED_INPUT_SCHEMA, "case": 0}
        compact = json.dumps(
            input_system, separators=(",", ":"), sort_keys=True
        ).encode("utf-8")
        producer_json = (
            json.dumps(
                {
                    "schema": SUITE.EXPECTED_PRODUCER_SCHEMA,
                    "case": 0,
                    "input_system": input_system,
                    "input_system_sha256": hashlib.sha256(compact).hexdigest(),
                },
                separators=(",", ":"),
            )
            + "\n"
        )
        time_report = (
            "User time (seconds): 0.01\n"
            "System time (seconds): 0.01\n"
            "Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.02\n"
            "Maximum resident set size (kbytes): 1024\n"
            "Exit status: 0\n"
        )
        for overflow_stream in ("stdout", "time_v"):
            with self.subTest(stream=overflow_stream), tempfile.TemporaryDirectory() as temporary:
                root = Path(temporary)
                producer = root / "producer"
                producer_suffix = "X" if overflow_stream == "stdout" else ""
                producer.write_text(
                    "#!/bin/sh\nprintf '%s' '"
                    + producer_json
                    + producer_suffix
                    + "'\n",
                    encoding="utf-8",
                )
                producer.chmod(0o700)
                time_binary = root / "time"
                report_suffix = "X" if overflow_stream == "time_v" else ""
                time_binary.write_text(
                    "#!/bin/sh\n"
                    "shift 2\nreport=$1\nshift\n"
                    '"$@"\nstatus=$?\n'
                    "printf '%s' '"
                    + time_report
                    + report_suffix
                    + "' > \"$report\"\nexit \"$status\"\n",
                    encoding="utf-8",
                )
                time_binary.chmod(0o700)
                patches = (
                    mock.patch.object(
                        SUITE,
                        "RAW_PRODUCER_JSON_MAX_BYTES",
                        len(producer_json.encode("utf-8")),
                    )
                    if overflow_stream == "stdout"
                    else mock.patch.object(
                        SUITE, "TIME_V_MAX_BYTES", len(time_report.encode("utf-8"))
                    )
                )
                with patches:
                    row = SUITE.run_case(
                        0,
                        0,
                        producer,
                        time_binary,
                        root,
                        time.monotonic_ns() + 2_000_000_000,
                    )
                if overflow_stream == "stdout":
                    self.assertIsNone(row["observed_output_header"])
                    self.assertIn(
                        "stdout_write_cap_exceeded", row["stream_cap_errors"]
                    )
                else:
                    self.assertIsNone(row["time_v_observations"])
                    self.assertIn(
                        "time_v_write_cap_exceeded", row["stream_cap_errors"]
                    )

    def test_stdout_stderr_and_time_caps_terminate_and_bound_disk(self) -> None:
        configurations = (
            (
                "stdout",
                "RAW_PRODUCER_JSON_MAX_BYTES",
                "#!/bin/sh\nwhile :; do printf x; done\n",
            ),
            (
                "stderr",
                "STDERR_MAX_BYTES",
                "#!/bin/sh\nhead -c 65 /dev/zero >&2\n",
            ),
            ("time_v", "TIME_V_MAX_BYTES", "#!/bin/sh\nexit 0\n"),
        )
        for stream_name, constant_name, producer_source in configurations:
            with self.subTest(stream=stream_name), tempfile.TemporaryDirectory() as temporary:
                root = Path(temporary)
                inputs = make_inputs(root)
                inputs["binary"].write_text(producer_source, encoding="utf-8")
                inputs["binary"].chmod(0o700)
                time_payload = (
                    "head -c 65 /dev/zero > \"$report\"\n"
                    if stream_name == "time_v"
                    else "printf 'User time (seconds): 0.01\\n' > \"$report\"\n"
                )
                inputs["time_binary"].write_text(
                    "#!/bin/sh\n"
                    "shift 2\n"
                    "report=$1\n"
                    "shift\n"
                    '"$@"\n'
                    "status=$?\n"
                    + time_payload
                    + "exit \"$status\"\n",
                    encoding="utf-8",
                )
                inputs["time_binary"].chmod(0o700)
                output = root / "attempt"
                with mock.patch.object(SUITE, constant_name, 64):
                    receipt, status = SUITE.run_suite(
                        inputs["binary"],
                        inputs["protocol"],
                        inputs["source"],
                        inputs["verifier_source"],
                        inputs["verifier_executable"],
                        inputs["time_binary"],
                        inputs["cargo_lock"],
                        inputs["cargo_manifest"],
                        output,
                        root,
                        "sha256:test-image",
                        SUITE.PROCESS_CONTAINMENT_GUARANTEE,
                    )
                self.assertEqual(status, 1)
                self.assertTrue(
                    any(
                        f"{stream_name}_write_cap_exceeded"
                        in row["stream_cap_errors"]
                        for row in receipt["cases"]
                    )
                )
                suffix = {
                    "stdout": "stdout",
                    "stderr": "stderr",
                    "time_v": "time-v.txt",
                }[stream_name]
                self.assertTrue(
                    all(
                        (output / f"case-{case}.{suffix}").stat().st_size <= 64
                        for case in SUITE.CASE_ORDER
                    )
                )

    def test_raw_mutation_between_capture_and_seal_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            output = Path(temporary)
            stdout = output / "case-0.stdout"
            stdout.write_bytes(b"stable\n")
            receipt = {
                "launch_attempt_id": "test",
                "execution_custody": {},
                "cases": [
                    {
                        "case": 0,
                        "evidence_endpoint_complete": True,
                        "stdout": SUITE.artifact(stdout),
                        "stderr": SUITE.artifact(output / "case-0.stderr"),
                        "time_v": SUITE.artifact(output / "case-0.time-v.txt"),
                    }
                ],
            }
            self.assertEqual(SUITE.raw_artifact_errors(receipt, output), [])
            stdout.write_bytes(b"mutated\n")
            self.assertIn(
                "case_0_stdout_changed_after_capture",
                SUITE.raw_artifact_errors(receipt, output),
            )
            with mock.patch.object(
                SUITE, "custody_recheck", return_value=({}, [])
            ), self.assertRaisesRegex(ValueError, "raw artifact drift"):
                SUITE.seal_suite_receipt(
                    output,
                    receipt,
                    sealing_attempt_start={
                        "monotonic_ns": time.monotonic_ns(),
                        "resource_usage": SUITE.resource_snapshot(),
                        "stage": "test",
                    },
                )

    def test_capture_stops_at_cap_plus_one_on_oversized_sparse_file(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "oversized"
            with path.open("wb") as handle:
                handle.truncate(64 * 1024 * 1024)
            observed, buffered, errors = SUITE.capture_raw_artifact(
                path, buffer_limit=64
            )
            self.assertIsNone(buffered)
            self.assertEqual(observed["bytes"], 64 * 1024 * 1024)
            self.assertEqual(observed["observed_prefix_bytes"], 65)
            self.assertEqual(observed["observed_bytes_kind"], "lower_bound")
            self.assertIsNone(observed["sha256"])
            self.assertIn("raw_artifact_exceeds_buffer_limit:64", errors)

    def test_placeholder_capture_is_cap_bounded_and_missing_raw_fails_seal(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            hostile = root / "case-2.stdout"
            with hostile.open("wb") as handle:
                handle.truncate(64 * 1024 * 1024)
            with mock.patch.object(SUITE, "RAW_PRODUCER_JSON_MAX_BYTES", 64):
                artifacts, errors = SUITE.capture_case_raw_artifacts(root, 2)
            self.assertEqual(artifacts["stdout"]["observed_prefix_bytes"], 65)
            self.assertEqual(artifacts["stdout"]["observed_bytes_kind"], "lower_bound")
            self.assertIn("stdout:raw_artifact_exceeds_buffer_limit:64", errors)
            helper, _, _, endpoint_errors = SUITE.fsync_and_snapshot_artifact(
                root / "case-2.stderr",
                {},
                SUITE.STDERR_MAX_BYTES,
                time.monotonic_ns() + 1_000_000_000,
                retain_bytes=False,
            )
            self.assertFalse(helper["success"])
            self.assertTrue(
                any("endpoint_open_failed" in item for item in endpoint_errors)
            )

    def test_fifo_substitution_is_rejected_without_blocking(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            raw_fifo = root / "case-0.stdout"
            os.mkfifo(raw_fifo)
            started = time.monotonic()
            observed, buffered, errors = SUITE.capture_raw_artifact(
                raw_fifo, buffer_limit=64
            )
            self.assertLess(time.monotonic() - started, 0.5)
            self.assertFalse(observed["present"])
            self.assertIsNone(buffered)
            self.assertIn("raw_artifact_not_regular", errors)

            inputs = make_inputs(root)
            paths = SUITE.input_paths(**inputs)
            bindings = SUITE.validate_inputs(**inputs)
            custody = SUITE.stage_inputs(root / "custody", paths, bindings)
            staged_producer = Path(custody["binary"]["staged"]["path"])
            staged_producer.unlink()
            os.mkfifo(staged_producer)
            started = time.monotonic()
            custody_failures = SUITE.custody_errors(custody, ("binary",))
            self.assertLess(time.monotonic() - started, 0.5)
            self.assertTrue(
                any("raw_artifact_not_regular" in item for item in custody_failures)
            )

            launcher = root / "bootstrap-launcher.py"
            runtime = root / "bootstrap-python"
            launcher.write_text("pass\n", encoding="utf-8")
            runtime.write_bytes(b"runtime\n")
            bootstrap = {
                "launcher_source": SUITE.bootstrap_binding(launcher),
                "python_executable": SUITE.bootstrap_binding(runtime),
            }
            launcher.unlink()
            os.mkfifo(launcher)
            started = time.monotonic()
            bootstrap_failures = SUITE.bootstrap_binding_errors(bootstrap)
            self.assertLess(time.monotonic() - started, 0.5)
            self.assertTrue(
                any(
                    "raw_artifact_not_regular" in item
                    for item in bootstrap_failures
                )
            )

            started = time.monotonic()
            helper, _, _, endpoint_errors = SUITE.fsync_and_snapshot_artifact(
                raw_fifo,
                {
                    "written_bytes": 0,
                    "retained_device": 0,
                    "retained_inode": 0,
                    "retained_prefix_sha256": hashlib.sha256(b"").hexdigest(),
                },
                64,
                time.monotonic_ns() + 1_000_000_000,
                retain_bytes=False,
            )
            self.assertFalse(helper["success"])
            self.assertIn("endpoint_not_regular", endpoint_errors)
            self.assertLess(time.monotonic() - started, 0.5)

    def test_containment_and_custody_manifest_rechecks_fail_on_drift(self) -> None:
        expected = SUITE.process_containment_receipt(
            SUITE.PROCESS_CONTAINMENT_GUARANTEE
        )
        changed = json.loads(json.dumps(expected))
        changed["wrapper_process_group_id"] += 1
        self.assertIn(
            "containment_wrapper_process_group_id_mismatch",
            SUITE.containment_stability_errors(expected, changed),
        )
        with tempfile.TemporaryDirectory() as temporary:
            manifest = Path(temporary) / "custody-manifest.json"
            manifest.write_text('{"stable":true}\n', encoding="utf-8")
            binding = SUITE.bounded_file_binding(
                manifest, 1024, "custody_manifest"
            )
            manifest.write_text('{"stable":null}\n', encoding="utf-8")
            self.assertIn(
                "custody_manifest_binding_changed",
                SUITE.bounded_file_binding_errors(
                    binding, 1024, "custody_manifest"
                ),
            )

    def test_drain_binding_detects_direct_same_inode_overwrite(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "stdout"
            state: dict[str, object] = {}
            SUITE.drain_bounded_stream(
                io.BytesIO(b"trusted\n"),
                path,
                64,
                2_147_483_647,
                state,
            )
            path.write_bytes(b"altered\n")
            observed, _, _ = SUITE.capture_raw_artifact(path, buffer_limit=64)
            errors = SUITE.stream_snapshot_binding_errors(
                {"stdout": state}, {"stdout": observed}
            )
            self.assertIn("stdout_retained_sha256_mismatch", errors)

    def test_endpoint_fsync_rejects_inode_swap_before_fork(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            path = root / "case-0.stdout"
            path.write_bytes(b"same\n")
            expected = SUITE.artifact(path)
            state = {
                "written_bytes": expected["bytes"],
                "retained_device": expected["device"],
                "retained_inode": expected["inode"],
                "retained_prefix_sha256": expected["sha256"],
            }
            replacement = root / "replacement"
            replacement.write_bytes(b"same\n")
            os.replace(replacement, path)
            with mock.patch.object(
                SUITE.os,
                "fork",
                side_effect=AssertionError("invalid inode must not be fsynced"),
            ):
                helper, _, _, errors = SUITE.fsync_and_snapshot_artifact(
                    path,
                    state,
                    64,
                    time.monotonic_ns() + 1_000_000_000,
                    retain_bytes=False,
                )
            self.assertFalse(helper["success"])
            self.assertIn("endpoint_inode_mismatch_before_fsync", errors)

    def test_endpoint_fsync_rejects_same_inode_growth_before_fork(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "case-0.stdout"
            path.write_bytes(b"stable\n")
            expected = SUITE.artifact(path)
            state = {
                "written_bytes": expected["bytes"],
                "retained_device": expected["device"],
                "retained_inode": expected["inode"],
                "retained_prefix_sha256": expected["sha256"],
            }
            with path.open("ab") as handle:
                handle.write(b"growth")
            with mock.patch.object(
                SUITE.os, "fork", side_effect=AssertionError("must not fork")
            ):
                helper, _, _, errors = SUITE.fsync_and_snapshot_artifact(
                    path,
                    state,
                    64,
                    time.monotonic_ns() + 1_000_000_000,
                    retain_bytes=False,
                )
            self.assertFalse(helper["success"])
            self.assertIn("endpoint_size_mismatch_before_fsync", errors)

    def test_failure_receipt_replace_ignores_fixed_temp_collisions(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            output = Path(temporary)
            receipt_path = output / "suite-receipt.json"
            marker_path = output / "suite-seal-failure.json"
            receipt_path.write_text('{"attacker":true}\n', encoding="utf-8")
            marker_path.write_text('{"attacker":true}\n', encoding="utf-8")
            fixed_receipt_temp = output / "suite-receipt.json.replacement"
            fixed_marker_temp = output / "suite-seal-failure.json.replacement"
            fixed_receipt_temp.write_text("collision", encoding="utf-8")
            fixed_marker_temp.write_text("collision", encoding="utf-8")
            receipt = {
                "schema": SUITE.SCHEMA,
                "launch_attempt_id": "durable-failure-test",
                "outer_cap_attestation": {
                    "complete_suite_finished_by_deadline": True
                },
            }
            with mock.patch.object(
                SUITE,
                "validate_completion_artifact",
                side_effect=AssertionError(
                    "failure completion must be the terminal local action"
                ),
            ):
                SUITE.persist_full_failure_receipt(
                    output,
                    receipt,
                    OSError("injected seal failure"),
                    sealing_attempt_start={
                        "monotonic_ns": time.monotonic_ns(),
                        "resource_usage": SUITE.resource_snapshot(),
                        "stage": "test",
                    },
                )
            stored_receipt = json.loads(receipt_path.read_text(encoding="utf-8"))
            stored_marker = json.loads(marker_path.read_text(encoding="utf-8"))
            completion_path = output / "suite-seal-failure-completion.json"
            stored_completion = json.loads(
                completion_path.read_text(encoding="utf-8")
            )
            self.assertEqual(
                stored_receipt["launch_attempt_id"], "durable-failure-test"
            )
            self.assertEqual(
                stored_receipt["receipt_seal_failure"]["type"], "OSError"
            )
            self.assertEqual(
                stored_marker["launch_attempt_id"], "durable-failure-test"
            )
            self.assertEqual(
                stored_marker["required_completion_artifact"],
                "suite-seal-failure-completion.json",
            )
            self.assertTrue(
                stored_completion[
                    "failure_marker_and_directory_fsync_completed"
                ]
            )
            self.assertTrue(
                SUITE.artifact_matches(
                    stored_completion["receipt"], SUITE.artifact(receipt_path)
                )
            )
            self.assertTrue(
                SUITE.artifact_matches(
                    stored_completion["failure_marker"],
                    SUITE.artifact(marker_path),
                )
            )
            self.assertEqual(fixed_receipt_temp.read_text(encoding="utf-8"), "collision")
            self.assertEqual(fixed_marker_temp.read_text(encoding="utf-8"), "collision")
            self.assertEqual(
                list(output.glob(".*.replacement-*.tmp")), []
            )

    def test_completion_replacement_directory_fsync_is_terminal_action(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            output = Path(temporary)
            path = output / "suite-seal-failure-completion.json"
            path.write_text('{"old":true}\n', encoding="utf-8")
            directory_fsync_finished = False
            original_fsync_directory = SUITE.fsync_directory
            original_close = SUITE.os.close
            original_stat = SUITE.os.stat

            def terminal_fsync_directory(directory: Path) -> None:
                nonlocal directory_fsync_finished
                original_fsync_directory(directory)
                directory_fsync_finished = True

            def reject_late_close(descriptor: int) -> None:
                if directory_fsync_finished:
                    raise AssertionError("FD close followed terminal directory fsync")
                original_close(descriptor)

            def reject_late_stat(*args: object, **kwargs: object) -> object:
                if directory_fsync_finished:
                    raise AssertionError("stat followed terminal directory fsync")
                return original_stat(*args, **kwargs)

            with mock.patch.object(
                SUITE,
                "fsync_directory",
                side_effect=terminal_fsync_directory,
            ), mock.patch.object(
                SUITE.os, "close", side_effect=reject_late_close
            ), mock.patch.object(
                SUITE.os, "stat", side_effect=reject_late_stat
            ):
                binding = SUITE.write_json_replace(
                    path,
                    {
                        "schema": (
                            "prime31.rr9.full8-f4-suite-seal-failure-"
                            "completion.v1"
                        ),
                        "launch_attempt_id": "replacement-terminal-test",
                    },
                )
            self.assertTrue(directory_fsync_finished)
            self.assertTrue(
                SUITE.artifact_matches(binding, SUITE.artifact(path))
            )

    def test_completion_validation_rejects_missing_and_tampered_bytes(self) -> None:
        schema = "prime31.rr9.full8-f4-suite-seal-completion.v1"
        launch_attempt_id = "completion-validation-test"
        for mutation in ("missing", "tampered"):
            with self.subTest(mutation=mutation), tempfile.TemporaryDirectory() as temporary:
                path = Path(temporary) / "suite-seal-completion.json"
                binding = SUITE.write_json_new(
                    path,
                    {
                        "schema": schema,
                        "launch_attempt_id": launch_attempt_id,
                    },
                )
                SUITE.validate_completion_artifact(
                    path,
                    binding,
                    schema=schema,
                    launch_attempt_id=launch_attempt_id,
                    label="test completion",
                )
                if mutation == "missing":
                    path.unlink()
                else:
                    path.write_text(
                        json.dumps(
                            {
                                "schema": schema,
                                "launch_attempt_id": "wrong-attempt",
                            }
                        )
                        + "\n",
                        encoding="utf-8",
                    )
                with self.assertRaises(ValueError):
                    SUITE.validate_completion_artifact(
                        path,
                        binding,
                        schema=schema,
                        launch_attempt_id=launch_attempt_id,
                        label="test completion",
                    )

    def test_missing_success_completion_fails_closed_with_failure_completion(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            inputs = make_valid_execution_inputs(root)
            output = root / "attempt"
            original_write_json_new = SUITE.write_json_new

            def fail_success_completion(
                path: Path, value: object
            ) -> dict[str, object]:
                if path.name == "suite-seal-completion.json":
                    raise OSError("injected-success-completion-write-failure")
                return original_write_json_new(path, value)

            with mock.patch.object(
                SUITE, "write_json_new", side_effect=fail_success_completion
            ):
                receipt, status = SUITE.run_suite(
                    inputs["binary"],
                    inputs["protocol"],
                    inputs["source"],
                    inputs["verifier_source"],
                    inputs["verifier_executable"],
                    inputs["time_binary"],
                    inputs["cargo_lock"],
                    inputs["cargo_manifest"],
                    output,
                    root,
                    "sha256:test-image",
                    SUITE.PROCESS_CONTAINMENT_GUARANTEE,
                )
            self.assertEqual(status, 1)
            self.assertFalse(receipt["producer_suite_admissible"])
            self.assertTrue((output / "suite-seal.json").is_file())
            self.assertFalse((output / "suite-seal-completion.json").exists())
            self.assertTrue((output / "suite-seal-failure.json").is_file())
            self.assertTrue(
                (output / "suite-seal-failure-completion.json").is_file()
            )
            on_disk = json.loads(
                (output / "suite-receipt.json").read_text(encoding="utf-8")
            )
            self.assertEqual(on_disk["receipt_seal_failure"]["type"], "OSError")
            failure_completion = json.loads(
                (output / "suite-seal-failure-completion.json").read_text(
                    encoding="utf-8"
                )
            )
            self.assertEqual(
                failure_completion["started_monotonic_ns"],
                receipt["postcap_custody_recheck"]["finished_monotonic_ns"],
            )
            self.assertTrue(failure_completion["producer_cap_excluded"])
            self.assertEqual(
                failure_completion["resource_start"],
                receipt["postcap_custody_recheck"]["resource_finish"],
            )

    def test_receipt_binding_recheck_rejects_missing_swap_and_oversize(self) -> None:
        for mutation in ("missing", "inode_swap", "oversize"):
            with self.subTest(mutation=mutation), tempfile.TemporaryDirectory() as temporary:
                root = Path(temporary)
                path = root / "suite-receipt.json"
                expected = SUITE.write_json_new(path, {"stable": True})
                encoded = path.read_bytes()
                if mutation == "missing":
                    path.unlink()
                elif mutation == "inode_swap":
                    replacement = root / "replacement"
                    replacement.write_bytes(encoded)
                    os.replace(replacement, path)
                else:
                    path.unlink()
                    with path.open("wb") as handle:
                        handle.truncate(64 * 1024 * 1024)
                with self.assertRaisesRegex(ValueError, "suite receipt changed"):
                    SUITE.recheck_bound_artifact(path, expected, "suite receipt")

    def test_delayed_endpoint_fsync_uses_remaining_suite_budget(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            input_system = {"schema": SUITE.EXPECTED_INPUT_SCHEMA, "case": 0}
            compact = json.dumps(
                input_system, separators=(",", ":"), sort_keys=True
            ).encode("utf-8")
            producer_output = json.dumps(
                {
                    "schema": SUITE.EXPECTED_PRODUCER_SCHEMA,
                    "case": 0,
                    "input_system": input_system,
                    "input_system_sha256": hashlib.sha256(compact).hexdigest(),
                },
                separators=(",", ":"),
            )
            producer = root / "producer"
            producer.write_text(
                "#!/bin/sh\nprintf '%s\\n' '" + producer_output + "'\n",
                encoding="utf-8",
            )
            producer.chmod(0o700)
            time_binary = root / "time"
            time_binary.write_text(
                "#!/bin/sh\n"
                "shift 2\nreport=$1\nshift\n"
                '"$@"\nstatus=$?\n'
                "{\n"
                "echo 'User time (seconds): 0.01'\n"
                "echo 'System time (seconds): 0.01'\n"
                "echo 'Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.02'\n"
                "echo 'Maximum resident set size (kbytes): 1024'\n"
                "echo \"Exit status: $status\"\n"
                "} > \"$report\"\nexit \"$status\"\n",
                encoding="utf-8",
            )
            time_binary.chmod(0o700)
            original_fsync = SUITE.os.fsync

            def delayed_helper_fsync(descriptor: int) -> None:
                time.sleep(0.20)
                original_fsync(descriptor)

            started = time.monotonic()
            with mock.patch.object(
                SUITE.os, "fsync", side_effect=delayed_helper_fsync
            ):
                row = SUITE.run_case(
                    0,
                    0,
                    producer,
                    time_binary,
                    root,
                    time.monotonic_ns() + 3_000_000_000,
                )
            self.assertGreaterEqual(time.monotonic() - started, 0.60)
            self.assertTrue(row["evidence_endpoint_complete"])
            self.assertEqual(row["endpoint_fsync_errors"], [])
            self.assertTrue(
                all(entry["success"] for entry in row["endpoint_fsync"].values())
            )
            self.assertTrue(
                all(
                    entry["finished_monotonic_ns"]
                    - entry["started_monotonic_ns"]
                    >= 180_000_000
                    for entry in row["endpoint_fsync"].values()
                )
            )
            self.assertEqual(row["stream_cap_errors"], [])
            self.assertEqual(row["producer_interface_errors"], [])

    def test_endpoint_fsync_timeout_uses_only_nonblocking_waitpid(self) -> None:
        waitpid_calls: list[tuple[int, int]] = []

        def nonblocking_waitpid(pid: int, options: int) -> tuple[int, int]:
            waitpid_calls.append((pid, options))
            if len(waitpid_calls) == 1:
                raise InterruptedError("injected EINTR at deadline")
            return 0, 0

        with mock.patch.object(SUITE.os, "fork", return_value=4242), mock.patch.object(
            SUITE.os, "waitpid", side_effect=nonblocking_waitpid
        ), mock.patch.object(SUITE.os, "kill") as kill, mock.patch.object(
            SUITE.time, "monotonic_ns", side_effect=(0, 100, 101)
        ):
            observed = SUITE.fork_fsync_until_deadline(99, 100)
        self.assertTrue(observed["timed_out"])
        self.assertTrue(observed["kill_sent"])
        self.assertFalse(observed["success"])
        kill.assert_called_once_with(4242, SUITE.signal.SIGKILL)
        self.assertTrue(waitpid_calls)
        self.assertTrue(
            all(options == SUITE.os.WNOHANG for _, options in waitpid_calls)
        )

    def test_endpoint_fsync_nonzero_and_schema_are_fail_closed(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "raw"
            path.write_bytes(b"stable\n")
            expected = SUITE.artifact(path)
            state = {
                "written_bytes": expected["bytes"],
                "retained_device": expected["device"],
                "retained_inode": expected["inode"],
                "retained_prefix_sha256": expected["sha256"],
            }
            with mock.patch.object(
                SUITE.os, "fsync", side_effect=OSError("injected fsync failure")
            ):
                helper, _, _, errors = SUITE.fsync_and_snapshot_artifact(
                    path,
                    state,
                    64,
                    time.monotonic_ns() + 1_000_000_000,
                    retain_bytes=False,
                )
            self.assertFalse(helper["success"])
            self.assertEqual(helper["exit_code"], 111)
            self.assertIsNone(helper["post_fsync_snapshot"])
            self.assertIn("endpoint_fsync_failed", errors)
            self.assertEqual(
                tuple(helper),
                (
                    "attempted",
                    "helper_pid",
                    "cap_bytes",
                    "started_monotonic_ns",
                    "finished_monotonic_ns",
                    "deadline_monotonic_ns",
                    "timed_out",
                    "kill_sent",
                    "reaped",
                    "wait_status",
                    "exit_code",
                    "termination_reason",
                    "success",
                    "error",
                    "pre_fsync_binding",
                    "post_fsync_snapshot",
                ),
            )

    def test_endpoint_fsync_detects_growth_after_helper(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "raw"
            path.write_bytes(b"stable\n")
            expected = SUITE.artifact(path)
            state = {
                "written_bytes": expected["bytes"],
                "retained_device": expected["device"],
                "retained_inode": expected["inode"],
                "retained_prefix_sha256": expected["sha256"],
            }

            def grow_after_helper(
                descriptor: int, deadline: int
            ) -> dict[str, object]:
                del descriptor
                with path.open("ab") as handle:
                    handle.write(b"growth")
                now = time.monotonic_ns()
                return {
                    "attempted": True,
                    "helper_pid": 123,
                    "started_monotonic_ns": now,
                    "finished_monotonic_ns": now,
                    "deadline_monotonic_ns": deadline,
                    "timed_out": False,
                    "kill_sent": False,
                    "reaped": True,
                    "wait_status": 0,
                    "exit_code": 0,
                    "termination_reason": "helper_exit_zero",
                    "success": True,
                }

            with mock.patch.object(
                SUITE,
                "fork_fsync_until_deadline",
                side_effect=grow_after_helper,
            ):
                helper, _, _, errors = SUITE.fsync_and_snapshot_artifact(
                    path,
                    state,
                    64,
                    time.monotonic_ns() + 1_000_000_000,
                    retain_bytes=False,
                )
            self.assertFalse(helper["success"])
            self.assertIn("endpoint_changed_during_fsync", errors)
            self.assertIn("endpoint_changed_during_snapshot", errors)

    def test_setsid_fd_holder_cannot_block_deadline_safe_drain(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            escaped_pid_path = root / "escaped.pid"
            producer = root / "producer"
            producer.write_text(
                "#!/bin/sh\n"
                "printf '{}\\n'\n"
                f"setsid sh -c 'echo $$ > \"{escaped_pid_path}\"; sleep 5' &\n"
                "i=0\n"
                f"while [ ! -s \"{escaped_pid_path}\" ] && [ \"$i\" -lt 50 ]; "
                "do i=$((i+1)); sleep 0.01; done\n"
                "exit 0\n",
                encoding="utf-8",
            )
            producer.chmod(0o700)
            time_binary = root / "time"
            time_binary.write_text(
                "#!/bin/sh\n"
                "shift 2\nreport=$1\nshift\n"
                '"$@"\nstatus=$?\n'
                "{\n"
                "echo 'User time (seconds): 0.01'\n"
                "echo 'System time (seconds): 0.01'\n"
                "echo 'Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.02'\n"
                "echo 'Maximum resident set size (kbytes): 1024'\n"
                "echo \"Exit status: $status\"\n"
                "} > \"$report\"\nexit \"$status\"\n",
                encoding="utf-8",
            )
            time_binary.chmod(0o700)
            SUITE.enable_child_subreaper()
            started = time.monotonic()
            row = SUITE.run_case(
                0,
                0,
                producer,
                time_binary,
                root,
                time.monotonic_ns() + 2_000_000_000,
            )
            self.assertLess(time.monotonic() - started, 1.5)
            self.assertTrue(
                any(
                    "drain_incomplete" in error
                    for error in row["stream_cap_errors"]
                )
            )
            escaped_pid = int(escaped_pid_path.read_text(encoding="utf-8"))
            cleanup = SUITE.terminate_surviving_process_group(escaped_pid)
            self.assertTrue(cleanup["absence_verified"])

    def test_dry_run_is_fixed_order_and_never_invokes_binary(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            marker = root / "invoked"
            time_marker = root / "time-invoked"
            binary = root / "producer"
            binary.write_text(f"#!/bin/sh\ntouch '{marker}'\n", encoding="utf-8")
            binary.chmod(0o700)
            time_binary = root / "time"
            time_binary.write_text(
                f"#!/bin/sh\ntouch '{time_marker}'\nexit 99\n", encoding="utf-8"
            )
            time_binary.chmod(0o700)
            protocol = root / "protocol.md"
            source = root / "source.rs"
            verifier_source = root / "verifier.py"
            verifier_executable = root / "python"
            cargo_lock = root / "Cargo.lock"
            cargo_manifest = root / "Cargo.toml"
            protocol.write_text("frozen\n", encoding="utf-8")
            source.write_text("fn main() {}\n", encoding="utf-8")
            verifier_source.write_text("print('ok')\n", encoding="utf-8")
            verifier_executable.write_text("#!/bin/sh\nexit 0\n", encoding="utf-8")
            verifier_executable.chmod(0o700)
            cargo_lock.write_text("# lock\n", encoding="utf-8")
            cargo_manifest.write_text("[package]\nname='test'\n", encoding="utf-8")
            output = root / "must-not-exist"
            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                status = SUITE.main(
                    [
                        "--binary",
                        str(binary),
                        "--protocol",
                        str(protocol),
                        "--source",
                        str(source),
                        "--verifier-source",
                        str(verifier_source),
                        "--verifier-executable",
                        str(verifier_executable),
                        "--time-binary",
                        str(time_binary),
                        "--cargo-lock",
                        str(cargo_lock),
                        "--cargo-manifest",
                        str(cargo_manifest),
                        "--repo",
                        str(root),
                        "--image-digest",
                        "sha256:test-image",
                        "--process-containment-guarantee",
                        SUITE.PROCESS_CONTAINMENT_GUARANTEE,
                        "--output-dir",
                        str(output),
                        "--dry-run",
                    ]
                )
            value = json.loads(stdout.getvalue())
            self.assertEqual(status, 0)
            self.assertEqual(value["schema"], SUITE.SCHEMA)
            self.assertEqual(value["case_order"], [0, 1, 2])
            self.assertEqual(
                value["outer_cap_scope"],
                "producer_commands_plus_process_group_drain_raw_endpoint_fsync_"
                "snapshot_hashing_strict_interface_and_resource_validation",
            )
            self.assertEqual([row[-1] for row in value["commands"]], ["0", "1", "2"])
            self.assertTrue(
                all(
                    row[3] == "<capped-time-v-pipe:/proc/self/fd/N>"
                    for row in value["commands"]
                )
            )
            self.assertFalse(value["producer_invoked"])
            self.assertEqual(
                value["planned_container_image_digest"], "sha256:test-image"
            )
            self.assertFalse(value["scientific_evaluation_performed"])
            self.assertFalse(value["comparison_admissible"])
            self.assertIsNone(
                value["required_followup_links"]["independent_full8_verifier"][
                    "receipt"
                ]["sha256"]
            )
            self.assertEqual(
                value["bindings"]["launcher"]["sha256"],
                SUITE.sha256_file(MODULE_PATH),
            )
            self.assertEqual(
                value["bindings"]["cargo_lock"]["sha256"],
                SUITE.sha256_file(cargo_lock),
            )
            self.assertEqual(
                value["bindings"]["cargo_manifest"]["sha256"],
                SUITE.sha256_file(cargo_manifest),
            )
            self.assertFalse(marker.exists())
            self.assertFalse(time_marker.exists())
            self.assertFalse(output.exists())

    def test_execution_refuses_existing_output_directory(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            inputs = make_inputs(root)
            output = root / "existing"
            output.mkdir()
            with self.assertRaises(FileExistsError):
                SUITE.run_suite(
                    inputs["binary"],
                    inputs["protocol"],
                    inputs["source"],
                    inputs["verifier_source"],
                    inputs["verifier_executable"],
                    inputs["time_binary"],
                    inputs["cargo_lock"],
                    inputs["cargo_manifest"],
                    output,
                    root,
                    "sha256:test-image",
                    SUITE.PROCESS_CONTAINMENT_GUARANTEE,
                )


if __name__ == "__main__":
    unittest.main()
