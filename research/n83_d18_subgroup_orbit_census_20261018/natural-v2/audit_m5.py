#!/usr/bin/env python3
"""Fail-closed audit of the frozen N83 m=5 paired Spot gate."""

from __future__ import annotations

from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parent
SOURCE = "5604fb5ed63fa8dcfcc5e838b517637f0075eaec"
BINARY = "c8f6849ee98d2552709e232a85f3933c1735326f78f57648f1e96ac84c23b705"
IMAGE = "sha256:8ed3c0a996841537f75607e7d1de2114d8150391f75792e8da9268738547e73f"
BASES = {
    "U": ("1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d",
          "564c6bb72b346b73112240bb0bddc4ef68b7525f7ef2556d6046e52c32f976e0"),
    "D": ("e0c24980207cbb15fa57ec76bcfc6a628a763a80834cba1117ca83ea04678682",
          "8c1c245cda15bdd0019b487a5535a3b7eac642c259c4dc0bae25875d87d345b0"),
}
CORPUS = "831eaa7be2387634fd610f6706103b2af9b4e274f0db59923c0280af3cbed74b"
TARGETS = {
    0: ("ad77539c5624533970a6df1abf339a0f4f90e54259da33ba275c740e4262cfb4",
        {"x_decimal": "1834913121178197242856238", "y_decimal": "8797964190732846213981377"}),
    1: ("25e15462e2cb86a928dd024c2bb5403218bd14259d893b6d9cfff7f07e06fb7e",
        {"x_decimal": "5809312440517013698248638", "y_decimal": "3568442427236746854022671"}),
}
PREFLIGHTS = (("p-u0", "U", 0), ("p-d0", "D", 0), ("p-u1", "U", 1), ("p-d1", "D", 1))
CONTROLS = (("a-u", "U", 0), ("a-d", "D", 0))
NATURAL = (("n-u0", "U", 0), ("n-d0", "D", 0),
           ("n-d1", "D", 1), ("n-u1", "U", 1))


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def json_file(path: Path) -> dict:
    return json.loads(path.read_text())


def jsonl(path: Path) -> list[dict]:
    rows = [json.loads(line) for line in path.read_text().splitlines()]
    require(bool(rows), f"empty sidecar: {path}")
    return rows


def utc(path: Path) -> datetime:
    return datetime.fromisoformat(path.read_text().strip().replace("Z", "+00:00")).astimezone(timezone.utc)


def container_check(p: Path, exit_code: int, arm: str) -> dict:
    items = json_file(p / "container-inspect.json")
    require(len(items) == 1, f"{p.name}: inspect count")
    item = items[0]
    state, host = item["State"], item["HostConfig"]
    require(state["ExitCode"] == exit_code and state["OOMKilled"] is False,
            f"{p.name}: exit/OOM mismatch")
    require(host["Memory"] == host["MemorySwap"] == 4 * 1024**3,
            f"{p.name}: memory/swap cgroup")
    require(host["NanoCpus"] == 1_000_000_000 and host["CpusetCpus"] == "7",
            f"{p.name}: CPU cgroup")
    require(host["NetworkMode"] == "none" and host["ReadonlyRootfs"],
            f"{p.name}: network/root")
    require(host["CapDrop"] == ["ALL"] and host["PidsLimit"] == 256
            and "no-new-privileges" in host["SecurityOpt"],
            f"{p.name}: privileges")
    mounts = {m["Destination"]: m["RW"] for m in item["Mounts"]}
    require(mounts == {"/opt/worker": False, "/inputs/base": False,
                       "/inputs/evidence": False, "/inputs/fixture": False,
                       "/snapshot": False, "/out": True}, f"{p.name}: mounts")
    sources = {m["Destination"]: m["Source"] for m in item["Mounts"]}
    require(sources["/inputs/base"].endswith(f"/inputs/{arm}")
            and sources["/inputs/fixture"].endswith("/inputs/U")
            and sources["/inputs/evidence"].endswith(
                "/source/research/n83_d18_subgroup_orbit_census_20261018")
            and sources["/snapshot"].endswith("/snapshot")
            and sources["/opt/worker"].endswith(
                "/target/release/examples/koblitz_n83_factor_base_export")
            and sources["/out"].endswith(f"/runs/{p.name}"),
            f"{p.name}: mount sources")
    require(item["Config"]["Image"].endswith(IMAGE), f"{p.name}: image")
    return {"oom_killed": False, "memory_limit_bytes": host["Memory"],
            "cpu_nanos": host["NanoCpus"]}


def sidecar_check(p: Path, arm: str, fixture: int, mode: str) -> tuple[list[dict], list[dict]]:
    obj, points = BASES[arm]
    phases = jsonl(p / "worker.json.phases.jsonl")
    progress_path = p / "worker.json.progress.jsonl"
    progress = jsonl(progress_path) if progress_path.exists() else []
    for kind, rows in (("phase", phases), ("progress", progress)):
        require([r["sequence"] for r in rows] == list(range(len(rows))), f"{p.name}: {kind} sequence")
        require(all(a["elapsed_wall_ms"] <= b["elapsed_wall_ms"] for a, b in zip(rows, rows[1:])),
                f"{p.name}: {kind} clock")
        require(all(r["source_commit"] == SOURCE and r["object_blake3"] == obj
                    and r["point_set_blake3"] == points for r in rows),
                f"{p.name}: {kind} identity")
    for field in ("completed_decisions", "completed_conflicts", "completed_restarts",
                  "completed_xor_passes", "completed_xor_nanoseconds"):
        require(all(a[field] <= b[field] for a, b in zip(progress, progress[1:])),
                f"{p.name}: nonmonotone {field}")
    named = {r["phase"]: r for r in phases}
    require("inputs_checked" in named and "model_domain_done" in named,
            f"{p.name}: missing build phases")
    inputs, model = named["inputs_checked"]["details"], named["model_domain_done"]["details"]
    require(inputs["arm"] == arm and inputs["mode"] == mode
            and inputs["fixture_index"] == fixture and inputs["summands"] == 5
            and inputs["binary_sha256"] == BINARY
            and inputs["public_corpus_blake3"] == CORPUS
            and (mode == "positive" or inputs["target_blake3"] == TARGETS[fixture][0]),
            f"{p.name}: input identity")
    require(model["variables"] == 257268 and model["clauses"] == 7409527
            and model["xor_rows"] == 4980, f"{p.name}: model shape")
    if mode != "preflight":
        require("root_propagation_entered" in named and "root_propagation_returned" in named,
                f"{p.name}: root phases")
        require(named["root_propagation_returned"]["details"]["result"] == "Fixpoint",
                f"{p.name}: root result")
        require("sat_entered" in named and progress,
                f"{p.name}: search/progress absent")
        require(progress[-1]["elapsed_wall_ms"] >= named["sat_entered"]["elapsed_wall_ms"],
                f"{p.name}: progress before search")
    return phases, progress


def audit_one(root: Path, name: str, arm: str, fixture: int, mode: str) -> dict:
    p = root / "spot-arms" / name
    require((p / "arm.txt").read_text().strip() == arm
            and (p / "mode.txt").read_text().strip() == mode
            and int((p / "fixture-index.txt").read_text()) == fixture,
            f"{name}: wrapper identity")
    start, end = utc(p / "start.utc"), utc(p / "end.utc")
    require(start < end, f"{name}: UTC interval")
    exit_code = int((p / "host-exit.txt").read_text())
    envelope = container_check(p, exit_code, arm)
    phases, progress = sidecar_check(p, arm, fixture, mode)
    cap_path, worker_path = p / "worker.json.cap.json", p / "worker.json"
    if cap_path.exists():
        require(mode == "natural" and exit_code == 124 and not worker_path.exists(),
                f"{name}: cap/exit mismatch")
        cap = json_file(cap_path)
        require(cap["schema"] == "n83.paired-orbit-m5-cap/v1"
                and cap["status"] == "UNKNOWN_budget"
                and cap["arm"] == arm and cap["mode"] == mode
                and cap["fixture_index"] == fixture and cap["summands"] == 5
                and cap["budget_seconds"] == 600
                and 599000 <= cap["elapsed_process_wall_ms"] <= 610000,
                f"{name}: cap receipt")
        receipt, status = cap, "UNKNOWN_budget"
    else:
        require(exit_code == 0 and worker_path.exists(), f"{name}: worker absent")
        receipt = json_file(worker_path)
        require(receipt["schema"] == "n83.paired-orbit-m5-worker/v1"
                and receipt["source_commit"] == SOURCE and receipt["binary_sha256"] == BINARY
                and receipt["arm"] == arm and receipt["mode"] == mode
                and receipt["fixture_index"] == fixture and receipt["summands"] == 5
                and receipt["object_blake3"] == BASES[arm][0]
                and receipt["point_set_blake3"] == BASES[arm][1],
                f"{name}: worker identity")
        require(receipt["public_corpus_blake3"] == CORPUS,
                f"{name}: public corpus")
        status = receipt["status"]
        expected = {"preflight": {"PASS_PREFLIGHT"},
                    "positive": {"PASS_CONSTRUCTED_CONTROL"},
                    "natural": {"UNKNOWN_search_cap", "UNVERIFIED_MODEL_UNSAT",
                                "INVALID_MODEL", "INVALID_GROUP_LIFT", "INVALID_RELATION",
                                "PASS_NATURAL_RELATION_PENDING_REPLAY"}}[mode]
        require(status in expected, f"{name}: unexpected status {status}")
        require(receipt["memory_cgroup_peak_bytes"] <= 4 * 1024**3,
                f"{name}: memory peak over cap")
        if mode == "preflight":
            require(receipt["solver_stage_executed"] is False, f"{name}: preflight searched")
            require(receipt["target_blake3"] == TARGETS[fixture][0]
                    and receipt["public_point"] == TARGETS[fixture][1],
                    f"{name}: public target")
    require(receipt["candidate_id"] is None and receipt["selected_best_total_runtime"] is None,
            f"{name}: premature selection")
    usage = receipt["process_usage"]
    require(isinstance(usage["cpu_seconds"], (int, float))
            and 0 < usage["cpu_seconds"] <= 610
            and isinstance(usage["peak_rss_bytes"], int)
            and 0 < usage["peak_rss_bytes"] <= 4 * 1024**3,
            f"{name}: process resource accounting")
    if mode == "positive":
        relation = receipt["relation"]
        require(relation["rank_after_insert"] == 1 and relation["group_verified"]
                and relation["tamper_rejected"] and relation["matches_constructed_row"]
                and relation["matches_constructed_indices"], f"{name}: relation control")
        replay = json_file(p / "replay-arm64.json")
        require(replay["schema"] == "n83.paired-orbit-m5-replay/v1"
                and replay["status"] == "PASS" and replay["worker_source_commit"] == SOURCE
                and replay["rank_after_insert"] == 1
                and replay["changed_point_rejected"] and replay["changed_row_rejected"],
                f"{name}: independent replay")
        for tamper in ("changed-point", "changed-row"):
            require(int((p / f"{tamper}.exit.txt").read_text()) != 0
                    and not (p / f"{tamper}-unexpected-success.json").exists(),
                    f"{name}: forged {tamper} accepted")
    if mode == "natural" and status == "PASS_NATURAL_RELATION_PENDING_REPLAY":
        replay = json_file(p / "replay-arm64.json")
        require(replay["status"] == "PASS" and replay["rank_after_insert"] == 1,
                f"{name}: natural relation lacks replay")
    return {
        "name": name, "arm": arm, "mode": mode, "fixture_index": fixture,
        "status": status, "host_exit": exit_code,
        "start_utc": start.isoformat(), "end_utc": end.isoformat(),
        "charged_host_wall_s": (end - start).total_seconds(),
        "base_import_ms": phases[0]["details"]["base_import_ms"],
        "model_domain_ms": phases[1]["details"]["model_ms"],
        "last_progress_ms": progress[-1]["elapsed_wall_ms"] if progress else None,
        "decisions_lower_bound": progress[-1]["completed_decisions"] if progress else None,
        "conflicts_lower_bound": progress[-1]["completed_conflicts"] if progress else None,
        "sampled_memory_peak_bytes": max((r["memory_current_bytes"] for r in progress), default=None),
        "cgroup_peak_bytes": receipt["memory_cgroup_peak_bytes"],
        "process_usage": receipt.get("process_usage"),
        "container": envelope,
        "candidate_id": None, "selected_best_total_runtime": None,
    }


def audit(root: Path = ROOT) -> dict:
    rows = [audit_one(root, name, arm, fixture, mode)
            for mode, group in (("preflight", PREFLIGHTS), ("positive", CONTROLS),
                                ("natural", NATURAL)) for name, arm, fixture in group]
    require(all(a["end_utc"] <= b["start_utc"] for a, b in zip(rows, rows[1:])),
            "overlapping or misordered arms")
    require([r["name"] for r in rows] == [x[0] for x in PREFLIGHTS + CONTROLS + NATURAL],
            "arm order")
    raw = root / "spot-arms"
    require({p.name for p in raw.iterdir()} == {r["name"] for r in rows}, "raw directory set")
    entries = []
    for file in sorted(raw.rglob("*")):
        if file.is_file():
            entries.append(f"{hashlib.sha256(file.read_bytes()).hexdigest()}  {file.relative_to(root)}")
    report = {
        "schema": "n83.paired-orbit-m5-audit/v1", "status": "AUDITED_BOUNDED",
        "source_commit": SOURCE, "binary_sha256": BINARY,
        "rows": rows, "raw_file_count": len(entries),
        "candidate_id": None, "selected_best_total_runtime": None,
    }
    (root / "raw-files.sha256").write_text("\n".join(entries) + "\n")
    (root / "audit.json").write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    return report


if __name__ == "__main__":
    result = audit()
    print(json.dumps({"status": result["status"], "raw_file_count": result["raw_file_count"],
                      "arms": [(r["name"], r["status"]) for r in result["rows"]]}))
