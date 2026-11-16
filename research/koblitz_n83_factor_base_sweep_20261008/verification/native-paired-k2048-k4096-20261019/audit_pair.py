#!/usr/bin/env python3
"""Independently audit the frozen same-host N83 K2048/K4096 ABBA run."""

from __future__ import annotations

import argparse
from datetime import datetime
import hashlib
import json
from pathlib import Path
import subprocess
import sys

HERE = Path(__file__).resolve().parent
STUDY = HERE.parents[1]
sys.path.insert(0, str(STUDY))
sys.path.insert(0, str(STUDY / "verification/k4096-native-phases-20261018"))
import chain_search_supervisor as group  # noqa: E402
import audit_phase_trace as phase  # noqa: E402
from prepare_inputs import SOURCES  # noqa: E402

ORDER = ("a1-k2048", "b1-k4096", "b2-k4096", "a2-k2048")
MODEL = {
    2048: {"points": 339968, "x_coordinates": 169984, "orbits": 2048,
           "domain_clauses": 892470, "variables": 111478,
           "clauses": 1140557, "xors": 4980},
    4096: {"points": 679936, "x_coordinates": 339968, "orbits": 4096,
           "domain_clauses": 1773110, "variables": 131958,
           "clauses": 2021197, "xors": 4980},
}


def require(ok: bool, reason: str) -> None:
    if not ok:
        raise ValueError(reason)


def read(path: Path) -> dict:
    return json.loads(path.read_text())


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def audit(fetched: Path) -> dict:
    out = fetched / "out/native-paired"
    job, status, result = (read(fetched / name) for name in
                           ("job.json", "status.json", "out/native-paired/result.json"))
    commit = job["commit"]
    require(job["patch"] is False and job["instance_type"] == "c7i.xlarge"
            and job["ttl"] == "2h" and status["phase"] == "done"
            and status["exit_code"] == 0 and result["source_commit"] == commit,
            "job source, resource envelope, or terminal state differs")
    require(result["schema"] == "n83.native-paired-abba/v1"
            and result["selected_best_total_runtime"] is None
            and [row["label"] for row in result["arms"]] == list(ORDER),
            "paired order or selection field differs")
    preflight = read(out / "preflight.json")
    frozen = read(STUDY / "verification/cloud-matched-k2048-k4096-20261018/input/fresh-inputs.json")
    bases = {row["columns"]: row for row in frozen["bases"]}
    require(preflight["status"] == "PASS_FROZEN_INPUTS"
            and preflight["source_clean"] is True
            and preflight["source_commit"] == commit,
            "host frozen-input preflight differs")
    require((out / "container-image-id.txt").read_text().strip() ==
            "sha256:541096ae9b05d3b850341b78790e2ed31e94d5aa0ea086933c6ff0c09ffa2e63",
            "container image identity differs")
    require(read(out / "source-snapshot/attestation.json") == {
        "schema": "n83.chain-s3-source-attestation/v1",
        "source_commit": commit, "status_clean": True},
        "source attestation differs")
    for name in SOURCES:
        source = subprocess.run(["git", "show", f"{commit}:{name}"],
                                check=True, stdout=subprocess.PIPE).stdout
        require((out / "source-snapshot" / name).read_bytes() == source
                and preflight["source_sha256"][name] == hashlib.sha256(source).hexdigest(),
                f"source snapshot differs: {name}")
    for k in (2048, 4096):
        row = bases[k]
        panel = out / f"panel-k{k}"
        require(sha256(panel / "objects" / (row["blake3"] + ".jsonl.gz"))
                == row["sha256"], f"K{k} compressed archive differs")
        for name, digest in row["panel_sha256"].items():
            require(sha256(panel / name) == digest,
                    f"K{k} panel sidecar differs: {name}")
    for name, digest in frozen["fixture_sha256"].items():
        require(sha256(out / "fixtures" / name) == digest,
                f"fixture sidecar differs: {name}")

    checked = []
    for label, arm_result in zip(ORDER, result["arms"]):
        k = int(label.rsplit("k", 1)[1])
        model = MODEL[k]
        arm = out / "arms" / label
        worker_dir = arm / "worker"
        summary = read(worker_dir / "summary.json")
        config = read(worker_dir / "config.json")
        require(arm_result["exit_code"] == 0 and summary["source_commit"] == commit
                and config["source_commit"] == commit
                and summary["selected_best_total_runtime"] is None,
                f"{label} worker or source state differs")
        require((arm / "exit-code.txt").read_text().strip() == "0"
                and int((arm / "container-wall-ms.txt").read_text().strip())
                == arm_result["container_wall_ms"]
                and arm_result["summary_status"] == summary["status"]
                and arm_result["cap_status"] is None,
                f"{label} arm wrapper receipt differs")
        require(config["orbit_columns"] == k and config["seed"] == 2026100801
                and summary["seed"] == 2026100801
                and config["summands"] == 5
                and config["max_trials"] == 1 and config["strategy"] == "orbit-chain-s3"
                and config["trace_chained_s3_phases"] is True
                and config["memory_cgroup_limit_bytes"] == 4 * 1024**3
                and config["memory_cgroup_swap_limit_bytes"] == 0
                and config["chain_s3_limits"] == {
                    "max_variables": 150000, "max_domain_clauses": 2000000,
                    "max_models": 32, "conflict_budget": 1000},
                f"{label} worker config differs")
        phases = phase.phases_from_text((arm / "worker.stderr").read_text())
        phase.check_phase_order(phases)
        values = {row["phase"]: row for row in phases}
        require(len([row for row in phases if row["phase"] == "entry"]) == 1
                and values["entry"]["points"] == model["points"]
                and values["entry"]["orbits"] == k
                and values["entry"]["summands"] == 5
                and values["x_domain_ready"]["x_coordinates"] == model["x_coordinates"]
                and values["closure_ready"]["representatives"] == k
                and values["closure_ready"]["closure_x_coordinates"] == model["x_coordinates"]
                and values["domain_counted"]["domain_clauses"] == model["domain_clauses"]
                and all(values["domain_installed"][field] == model[field]
                        for field in ("variables", "clauses", "xors"))
                and values["solver_enter"]["remaining_conflicts"] == 1000,
                f"{label} exact model or SAT entry differs")
        query = phase.query_from_events(worker_dir / "events.jsonl",
                                        out / "fixtures/probe-corpus.json")
        replay = group.replay_group(
            summary, out / "fixtures/probe-corpus.json",
            out / f"panel-k{k}/objects/{bases[k]['blake3']}.jsonl.gz")
        stats = summary["report"]
        require(replay["status"] == "PASS" and replay["queries"] == 1
                and stats["sat_calls"] == 1 and stats["trials"] == 1
                and stats["factor_base_size"] == model["points"]
                and stats["orbit_count"] == k
                and summary["memory_cgroup_peak_bytes"] <= 4 * 1024**3,
                f"{label} independent replay or counters differ")
        require(summary["status"] == "UNKNOWN_solver_cap"
                and phases[-1]["phase"] == "solver_unknown"
                and values["solver_unknown"]["conflicts"] == 1000
                and stats["sat_conflicts"] == 1000
                and stats["sat_unknowns"] == 1
                and stats["sat_models"] == 0
                and stats["relations"] == 0
                and stats["independent_relations"] == 0
                and stats["terminal_matrix_rank"] == 0
                and replay["relations"] == 0,
                f"{label} capped terminal receipt differs")
        checked.append({
            "label": label, "status": summary["status"],
            "query": query, "replay": replay,
            "phase_markers": len(phases), "last_phase": phases[-1]["phase"],
            "model_ready_ms": values["domain_installed"]["elapsed_ms"],
            "base_import_ms": summary["base_import_ms"],
            "sat_elapsed_ms": max(0, phases[-1]["elapsed_ms"] -
                                  values["solver_enter"]["elapsed_ms"]),
            "worker_pre_summary_wall_ms": summary["pre_summary_process_wall_ms"],
            "container_wall_ms": arm_result["container_wall_ms"],
            "peak_cgroup_bytes": summary["memory_cgroup_peak_bytes"],
            "sat_conflicts": stats["sat_conflicts"],
            "sat_models": stats["sat_models"],
            "relations": stats["relations"],
            "independent_relations": stats["independent_relations"],
            "matrix_rank": stats["terminal_matrix_rank"],
        })
    require(all(row["query"] == checked[0]["query"] for row in checked),
            "paired public queries differ")
    job_wall_ms = int((datetime.fromisoformat(status["updated"].replace("Z", "+00:00"))
                       - datetime.fromisoformat(status["started"].replace("Z", "+00:00")))
                      .total_seconds() * 1000)
    return {"schema": "n83.native-paired-abba-audit/v1",
            "status": "PASS_BOUNDED_PAIRED_STAGE_CONTROL",
            "source_commit": commit, "job_id": job["id"],
            "job_wall_ms": job_wall_ms, "arms": checked,
            "selected_best_total_runtime": None}


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
