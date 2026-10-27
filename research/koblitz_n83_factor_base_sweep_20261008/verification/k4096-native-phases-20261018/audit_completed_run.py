#!/usr/bin/env python3
"""Replay the frozen K4096 native diagnostic from fetched runner artifacts."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import sys

HERE = Path(__file__).resolve().parent
STUDY = HERE.parents[1]
sys.path.insert(0, str(STUDY))
import chain_search_supervisor as group  # noqa: E402
import audit_phase_trace as phase  # noqa: E402

OBJECT = "a84572e6ca52efa7867082261bf0543723d52c0d136061a45c970970716aa779.jsonl.gz"
SHA = "ccc1196d75de406b1daba634b8011f08308c37bd1e674758f9efeabb0ff0bcad"
SOURCE_FILES = (
    "examples/koblitz_n83_factor_base_export.rs",
    "research/koblitz_n83_factor_base_sweep_20261008/primary_adapter.rs",
    "research/koblitz_n83_factor_base_sweep_20261008/size-frontier-v2.json",
    "src/cryptanalysis/binary_semaev_chain_sat.rs",
    "src/cryptanalysis/koblitz_index_calculus.rs",
)


def require(ok: bool, reason: str) -> None:
    if not ok:
        raise ValueError(reason)


def sha256(path: Path) -> str:
    value = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            value.update(block)
    return value.hexdigest()


def read_json(path: Path) -> dict:
    return json.loads(path.read_text())


def audit(fetched: Path) -> dict:
    out = fetched / "out/k4096-native-phases"
    worker_dir = out / "run/worker"
    worker = read_json(worker_dir / "summary.json")
    config = read_json(worker_dir / "config.json")
    job = read_json(fetched / "job.json")
    status = read_json(fetched / "status.json")
    result = read_json(out / "result.json")
    attestation = read_json(out / "source-snapshot/attestation.json")
    frozen = read_json(STUDY / "verification/cloud-matched-k2048-k4096-20261018/input/fresh-inputs.json")
    base = next(row for row in frozen["bases"] if row["columns"] == 4096)
    commit = job["commit"]
    require(len(commit) == 40 and job["patch"] is False and
            job["instance_type"] == "c7i.xlarge" and job["ttl"] == "2h",
            "runner source or resource envelope differs")
    require(status["phase"] == "done" and status["exit_code"] == 0 and
            (out / "worker-exit-code.txt").read_text().strip() == "0",
            "runner or worker did not finish normally")
    require(attestation == {
        "schema": "n83.chain-s3-source-attestation/v1",
        "source_commit": commit, "status_clean": True,
    }, "clean source snapshot attestation differs")
    require(worker["source_commit"] == commit and config["source_commit"] == commit and
            result["source_commit"] == commit, "worker source commit differs")
    for name in SOURCE_FILES:
        expected = subprocess.run(["git", "show", f"{commit}:{name}"],
                                  check=True, stdout=subprocess.PIPE).stdout
        require((out / "source-snapshot" / name).read_bytes() == expected,
                f"source snapshot differs from pushed commit: {name}")
    object_path = out / "panel/objects" / OBJECT
    require(sha256(object_path) == SHA == base["sha256"],
            "selected archive differs from frozen S3 bytes")
    for name, expected in base["panel_sha256"].items():
        require(sha256(out / "panel" / name) == expected,
                f"panel sidecar SHA-256 differs: {name}")
    for name, expected in frozen["fixture_sha256"].items():
        require(sha256(out / "fixtures" / name) == expected,
                f"fixture SHA-256 differs: {name}")
    require(config["orbit_columns"] == 4096 and config["summands"] == 5 and
            config["max_trials"] == 1 and config["strategy"] == "orbit-chain-s3" and
            config["trace_chained_s3_phases"] is True and
            config["memory_cgroup_limit_bytes"] == 4 * 1024**3 and
            config["memory_cgroup_swap_limit_bytes"] == 0 and
            config["chain_s3_limits"] == {
                "max_variables": 150000, "max_domain_clauses": 2000000,
                "max_models": 32, "conflict_budget": 1000,
            }, "worker config differs from the frozen diagnostic")
    fixture = out / "fixtures/probe-corpus.json"
    phases = phase.native(out / "worker.stderr", worker_dir / "events.jsonl", fixture)
    require(phases == read_json(out / "independent-phase-audit.json") and
            phases["last_phase"] == "solver_unknown" and
            phases["solver_entry_observed"] and
            phases["terminal_solver_phase_observed"],
            "flushed phase audit differs or lacks a terminal unknown")
    raw_phases = phase.phases_from_text((out / "worker.stderr").read_text())
    values = {row["phase"]: row for row in raw_phases}
    require(values["solver_unknown"]["conflicts"] == 1000 and
            values["solver_enter"]["solver_calls"] == 1,
            "terminal SAT conflict receipt differs")
    group_replay = group.replay_group(worker, fixture, object_path)
    require(group_replay == {"status": "PASS", "queries": 1, "relations": 0},
            "independent attempt replay differs")
    stats = worker["report"]
    require(worker["status"] == "UNKNOWN_solver_cap" and
            worker["selected_best_total_runtime"] is None and
            result["selected_best_total_runtime"] is None and
            worker["verified_log"] is None and
            stats["sat_calls"] == 1 and stats["sat_conflicts"] == 1000 and
            stats["sat_unknowns"] == 1 and stats["sat_models"] == 0 and
            stats["sat_refutations"] == 0 and stats["relations"] == 0 and
            stats["independent_relations"] == 0 and
            stats["terminal_matrix_rank"] == 0 and stats["trials"] == 1 and
            worker["attempt_records"][0]["disposition"] == "Unknown",
            "worker summary is inconsistent with one capped unknown trial")
    require(stats["factor_base_size"] == 679936 and stats["orbit_count"] == 4096 and
            stats["matrix_columns"] == 4097 and
            worker["memory_cgroup_peak_bytes"] <= 4 * 1024**3,
            "factor base or cgroup peak differs")
    return {
        "schema": "n83.k4096-completed-run-audit/v1",
        "status": "PASS_BOUNDED_NATIVE_DIAGNOSTIC",
        "job_id": job["id"],
        "source_commit": commit,
        "object_sha256": SHA,
        "query_replay": group_replay,
        "phase_audit": phases,
        "model_ready_ms_from_decomposition_entry": values["domain_installed"]["elapsed_ms"],
        "sat_elapsed_ms": values["solver_unknown"]["elapsed_ms"] - values["solver_enter"]["elapsed_ms"],
        "solver_terminal": "UNKNOWN_solver_cap",
        "conflicts": 1000,
        "worker_pre_summary_wall_ms": worker["pre_summary_process_wall_ms"],
        "worker_peak_cgroup_bytes": worker["memory_cgroup_peak_bytes"],
        "claim_scope": "one capped public query; no relation-yield or total-runtime selection",
        "selected_best_total_runtime": None,
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--fetched", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    result = audit(args.fetched)
    with args.output.open("x") as stream:
        json.dump(result, stream, indent=2, sort_keys=True)
        stream.write("\n")
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()
