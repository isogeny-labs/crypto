"""No-launch controls for the hosted six-summand receipt and resource gates."""

from __future__ import annotations

import json
import hashlib
from pathlib import Path
import sys
from tempfile import TemporaryDirectory
from unittest import TestCase, main, mock
from zipfile import ZipFile

import blake3

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent / "k600-gray-hash-paired-protocol-20261014"))
import receipt_audit as pinned  # noqa: E402
import audit  # noqa: E402
import protocol  # noqa: E402
import run  # noqa: E402
import replay_artifact  # noqa: E402


class SixSummandControls(TestCase):
    def setUp(self) -> None:
        self.plan = json.loads(protocol.OUTPUT.read_text())
        self.arm = self.plan["arms"][0]
        self.config = {
            "schema": "n83.cross-stream-pilot-partial-config/v1",
            "study": "koblitz_n83_factor_base_sweep_20261008",
            "base_object_blake3": self.arm["base_object_blake3"],
            "pool_object_blake3": self.arm["pool_object_blake3"],
            "pilot_manifest_blake3": blake3.blake3(
                pinned.preflight.PINNED_SHA256["manifest"][0].read_bytes()).hexdigest(),
            "source_blake3": blake3.blake3(
                pinned.preflight.PINNED_SHA256["worker_source"][0].read_bytes()).hexdigest(),
            "public_corpus_path": "/corpus.json",
            "public_corpus_canonical_json_blake3": pinned.PRIOR.CORPUS_BLAKE3,
            "budget_seconds": 1800, "conflict_budget": 20000,
            "max_models": 32, "base_summands": 4, "residual_summands": 2,
            "order_first_pair": True, "max_variables": 150000,
            "max_domain_clauses": 12000000,
            "memory_cgroup_limit_bytes": 4294967296,
            "memory_cgroup_swap_limit_bytes": 0,
        }
        shape = self.arm["source_capacity"]
        self.result = {
            "schema": "n83.cross-stream-pilot-partial-result/v1",
            "study": "koblitz_n83_factor_base_sweep_20261008",
            "base_object_blake3": self.arm["base_object_blake3"],
            "pool_object_blake3": self.arm["pool_object_blake3"],
            "base_policy": "public_x_sequential", "base_seed": 2026100802,
            "pool_policy": "public_x_gray_prefix", "pool_seed": 2026100801,
            "public_corpus_canonical_json_blake3": pinned.PRIOR.CORPUS_BLAKE3,
            "base_signed_orbits": 600, "pool_signed_orbits": 600,
            "shared_signed_orbits": self.arm["shared_signed_orbits"],
            "residual_signed_orbits": self.arm["residual_signed_orbits"],
            "trial": 0, "a": self.plan["first_trial_a"],
            "b": self.plan["first_trial_b"],
            "graph_rows": None, "matrix_rank": None,
            "total_index_calculus_runtime_ms": None,
            "selected_best_total_runtime": None,
            "base_import_elapsed_ms_L0": 100, "worker_wall_ms_L0": 1000,
            "memory_cgroup_peak_bytes": 1000000,
            "search": {
                "domain_clauses": shape["domain_clauses"],
                "peak_variables": shape["total_variables"],
                "peak_xors": shape["xor_rows"], "peak_clauses": shape["domain_clauses"],
                "conflicts": 20000, "models": 0, "invalid_models": 0,
                "group_rejected_models": 0, "solver_calls": 1,
                "patterns_attempted": 1, "exhausted": True, "refuted": False,
            },
            "status": "UNKNOWN_search_cap", "candidate": None,
        }

    def replay(self) -> str:
        return audit.search(self.config, self.result, self.arm, [(1, 2)],
                            {(3, 4)}, pinned, self.plan)

    def test_frozen_protocol_and_six_summand_unknown_receipt(self) -> None:
        self.assertEqual(protocol.make(), self.plan)
        self.assertEqual(self.plan["schema"], "n83.hosted-sixsum-access-protocol/v2")
        self.assertEqual(self.plan["build_cpus"], 2)
        self.assertEqual(self.replay(), "UNKNOWN_search_cap")

    def test_failed_host_admission_receipt_binds_raw_stderr(self) -> None:
        attempt = json.loads((HERE / "attempt-1.json").read_text())
        stderr = (HERE / "attempt-1-exporter-build.stderr").read_bytes()
        self.assertEqual(hashlib.sha256(stderr).hexdigest(),
                         attempt["exporter_build_stderr_sha256"])
        self.assertIn(b"there are only 2 CPUs available", stderr)
        self.assertFalse(attempt["solver_search_executed"])
        archive = HERE / "attempt-1.zip"
        self.assertEqual(hashlib.sha256(archive.read_bytes()).hexdigest(),
                         attempt["artifact_zip_sha256"])
        files, _, size, _ = replay_artifact.members(archive)
        self.assertEqual(size, attempt["artifact_size_bytes"])
        self.assertEqual(len(files), attempt["artifact_zip_member_count"])

    def test_artifact_replay_refuses_path_traversal(self) -> None:
        with TemporaryDirectory() as directory:
            path = Path(directory) / "artifact.zip"
            with ZipFile(path, "w") as archive:
                archive.writestr("../outside.json", "{}")
            with self.assertRaisesRegex(ValueError, "unsafe or duplicate ZIP member"):
                replay_artifact.members(path)

    def test_artifact_replay_refuses_wrong_dispatch_head(self) -> None:
        jobs = {"total_count": 1, "jobs": [{
            "name": "six-summand-access", "run_id": replay_artifact.RUN_ID,
            "run_attempt": 1, "head_sha": "0" * 40,
            "status": "completed", "conclusion": "success", "id": 1,
            "steps": [],
        }]}
        with TemporaryDirectory() as directory:
            jobs_path = Path(directory) / "jobs.json"
            metadata_path = Path(directory) / "metadata.json"
            jobs_path.write_text(json.dumps(jobs))
            metadata_path.write_text("{}")
            with self.assertRaisesRegex(ValueError, "provider job head_sha changed"):
                replay_artifact.provider(jobs_path, metadata_path, 0, "0" * 64)

    def test_hosted_build_cpu_limit_is_applied_to_pinned_command(self) -> None:
        supervisor = mock.Mock()
        supervisor.stage_command.return_value = (
            ["docker", "run", "--cpus", "4", "--network", "none"],
            Path("/out"), "frozen-container")
        args = (self.arm, HERE.parents[3], Path("/archives"), Path("/build"),
                Path("/run"), Path("/worker"), Path("/registry"),
                "sha256:" + "a" * 64, "token", 2)
        command, _, _ = run.bounded_stage_command(supervisor, "build", *args)
        self.assertEqual(command[command.index("--cpus") + 1], "2")
        supervisor.stage_command.return_value = (
            ["docker", "run", "--cpus", "1", "--network", "none"],
            Path("/out"), "frozen-container")
        command, _, _ = run.bounded_stage_command(supervisor, "search", *args)
        self.assertEqual(command[command.index("--cpus") + 1], "1")

    def test_five_summand_config_is_rejected(self) -> None:
        self.config["base_summands"] = 3
        with self.assertRaisesRegex(ValueError, "base_summands"):
            self.replay()

    def test_candidate_outside_domain_is_rejected(self) -> None:
        self.result["status"] = "PASS_one_bounded_partial"
        self.result["candidate"] = {
            "source_id": 1, "a": self.plan["first_trial_a"],
            "b": self.plan["first_trial_b"],
            "base_point_indices": [0, 0, 0],
            "residual_points": [self.plan["public_target"], self.plan["public_target"]],
        }
        with self.assertRaisesRegex(ValueError, "outside exact domains"):
            self.replay()

    def test_worker_wall_cap_is_shape_specific(self) -> None:
        receipt = {"cleanup_verified": True, "timed_out": False,
                   "container_state": {"ExitCode": 124}, "exit_code": 124}
        with mock.patch.object(Path, "read_text", return_value=json.dumps({
            "schema": "n83.cross-stream-pilot-partial-cap/v1",
            "status": "UNKNOWN_budget", "budget_seconds": 1100,
            "elapsed_process_wall_ms": 1100000,
        })):
            self.assertEqual(run.stage_issue(receipt, "search", Path("/unused"), 1800),
                             "PRODUCER_FAILURE_worker_exit")
        with mock.patch.object(Path, "read_text", return_value=json.dumps({
            "schema": "n83.cross-stream-pilot-partial-cap/v1",
            "status": "UNKNOWN_budget", "budget_seconds": 1800,
            "elapsed_process_wall_ms": 1800000,
        })):
            self.assertEqual(run.stage_issue(receipt, "search", Path("/unused"), 1800),
                             "UNKNOWN_worker_wall_cap")

    def test_preparation_must_bind_workflow_head_and_archive_bytes(self) -> None:
        archives = {"/archives/base.jsonl.gz": "a" * 64}
        row = {
            "schema": "n83.hosted-regenerated-archives/v1",
            "status": "PASS_3_FROZEN_OBJECTS",
            "source_commit": protocol.SOURCE_COMMIT,
            "workflow_dispatch_head_sha": "b" * 40,
            "archive_sha256": {"base.jsonl.gz": "a" * 64},
            "pilot_completed_base_count": 54,
            "hosted_preparation_wall_seconds_L0": 20.5,
        }
        with TemporaryDirectory() as directory, \
             mock.patch.dict(run.os.environ, {"GITHUB_SHA": "b" * 40}):
            path = Path(directory) / "preparation.json"
            path.write_text(json.dumps(row))
            self.assertEqual(run.checked_preparation(path, protocol.SOURCE_COMMIT,
                                                     archives, 1800), row)
            row["archive_sha256"]["base.jsonl.gz"] = "c" * 64
            path.write_text(json.dumps(row))
            with self.assertRaisesRegex(ValueError, "does not bind frozen inputs"):
                run.checked_preparation(path, protocol.SOURCE_COMMIT, archives, 1800)

    def test_non_linux_host_rejected_before_pinned_io(self) -> None:
        with mock.patch.object(run.platform, "system", return_value="Darwin"), \
             mock.patch.object(run, "pinned_modules") as modules:
            with self.assertRaisesRegex(ValueError, "requires Linux arm64"):
                run.run(HERE.parents[3], Path("/tmp"),
                        Path("/tmp/n83-hosted-sixsum-output-does-not-exist"),
                        Path("/tmp/n83-hosted-sixsum-scratch-does-not-exist"))
            modules.assert_not_called()


if __name__ == "__main__":
    main()
