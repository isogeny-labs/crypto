#!/usr/bin/env python3
"""Run the selected fixed31 base against rho on 16 fresh one-target points."""

import argparse
from datetime import datetime, timezone
import json
import os
from pathlib import Path
import subprocess

from research.n83_highk_beyond4405_20261021 import run, runpod_runner
from research.n83_highk_beyond4405_20261021 import select as selector
from research.n83_online_finalists_20261018.run import ic_environment as lowerk_environment

HERE = Path(__file__).resolve().parent
RHO_SEED_BASE = 2026120300
K1107_CANDIDATE_ID = "IC1N83Ckb1a1fb183762PDP4rootRCguidedLAgaussTDdirectISO0hf1851768e85a"
K1107_MANIFEST_SHA256 = "67cbd5d4f12ab5bfe703714ab66f0c779b5d708aaa79a6399b13d4217b4f9176"
K1107_ARM = {"k": 1107, "candidate_id": K1107_CANDIDATE_ID,
             "layout": "root11_dynamic"}


def events(candidate_id: str) -> list[dict]:
    result = []
    for index in range(16):
        arms = (("highk", candidate_id), ("k1107", K1107_CANDIDATE_ID))
        if index % 2:
            arms = tuple(reversed(arms))
        for arm, arm_id in arms:
            for rep in range(3):
                result.append({"point_index": index, "kind": "calibration",
                               "side": "before", "arm": arm,
                               "candidate_id": arm_id,
                               "name": f"p{index:02d}_{arm}_cal_before_{rep}"})
            kinds = (("ic", "rho") if
                     (index + (0 if arm == "highk" else 1)) % 2 == 0
                     else ("rho", "ic"))
            for kind in kinds:
                result.append({"point_index": index, "kind": kind,
                               "arm": arm, "candidate_id": arm_id,
                               "name": f"p{index:02d}_{arm}_{kind}"})
            for rep in range(3):
                result.append({"point_index": index, "kind": "calibration",
                               "side": "after", "arm": arm,
                               "candidate_id": arm_id,
                               "name": f"p{index:02d}_{arm}_cal_after_{rep}"})
    if len(result) != 256:
        raise ValueError("holdout event count differs")
    return result


def ic_environment(folder: Path, arm: dict) -> dict[str, str]:
    if arm["candidate_id"] != K1107_CANDIDATE_ID:
        return run.ic_environment(folder, arm)
    path = HERE / "identities" / f"candidate_{K1107_CANDIDATE_ID}.json"
    record = json.loads(path.read_text())
    if (run.digest(path) != K1107_MANIFEST_SHA256 or
            record.get("candidate_id") != K1107_CANDIDATE_ID or
            record["implementation"]["source_sha256"]["ic"] !=
            run.digest(run.ROOT / "examples/koblitz_orbit_dlp_fast.rs") or
            record["factor_base"]["actual_usable_points"] != 183762 or
            record["factor_base"]["effective_columns"] != 1107):
        raise ValueError("current-source K1107 candidate differs")
    env = lowerk_environment(folder, "root11_dynamic", "k1107_root11")
    flags = record["implementation"]["algorithm_flags"]
    mapping = {
        "compact_state26": "KIC_STATE26",
        "compact_state28": "KIC_STATE28",
        "dense_request": "KIC_PACKED_ROOT_TABLE_DENSE",
        "indexed4_fixed31_request": "KIC_INDEXED4_FIXED31",
        "indexed5_fixed30_request": "KIC_INDEXED5_FIXED30",
        "indexed5_id30_request": "KIC_INDEXED5_ID30",
        "indexed5_load75_request": "KIC_INDEXED5_LOAD75",
        "indexed_root4": "KIC_INDEXED_ROOT4",
        "indexed_root5": "KIC_INDEXED_ROOT5",
        "load75_request": "KIC_PACKED_ROOT_TABLE_LOAD75",
        "packed_root_table": "KIC_PACKED_ROOT_TABLE",
        "root_batch": "KIC_INDEX_ROOT_BATCH",
        "root_bloom": "KIC_ROOT_BLOOM",
        "root_only11": "KIC_ROOT_ONLY11",
        "root_only21": "KIC_ROOT_ONLY21",
        "root_only22": "KIC_ROOT_ONLY22",
        "sparse_canonical": "KIC_SPARSE_CANONICAL",
        "symmetric_s3_states": "KIC_SYMMETRIC_S3",
        "target_cancel": "KIC_TARGET_CANCEL",
    }
    if (any(not isinstance(flags[key], bool) or
            env.get(name) != str(int(flags[key]))
            for key, name in mapping.items()) or
            env.get("KIC_RANK_THREADS") != str(flags["rank_threads"]) or
            env.get("KIC_TARGET_THREADS") != str(flags["target_threads"]) or
            env.get("KIC_DUMP_BASE") != str(folder / "base_dump.jsonl")):
        raise ValueError("current-source K1107 runtime flags differ")
    return env


def run_holdout(output: Path, allocation_path: Path) -> int:
    dispatch, streams = run.fixture()
    allocation = runpod_runner.read_allocation(allocation_path)
    source = json.loads((output / "source.json").read_text())
    selection_path = output / "screen_selection.json"
    selection = json.loads(selection_path.read_text())
    validated_path = output / "screen_validated.json"
    analysis_path = output / "screen_analysis.json"
    validated = json.loads(validated_path.read_text())
    analysis = json.loads(analysis_path.read_text())
    candidate_id = selection.get("selected_candidate_id")
    arm = next((item for item in dispatch["arms"]
                if item["candidate_id"] == candidate_id and item["fixed31"]), None)
    if (selection.get("schema") != "n83.fixed31-screen-selection/v1" or
            selection.get("admission_status") != "PASS" or
            selection.get("status") != "SELECTED" or arm is None or
            selection != selector.select(validated, dispatch) or
            analysis.get("status") != "SCREEN_ADMITTED" or
            source.get("schema") != "n83.fixed31-tier1-source/v1" or
            source.get("source_sha256") != dispatch["source_sha256"] or
            source.get("dispatch_sha256") != run.digest(HERE / "dispatch.json") or
            source.get("workload_manifest_sha256") !=
            dispatch["workload_manifest_sha256"] or
            source.get("runpod_allocation_sha256") != run.digest(allocation_path) or
            (output / "holdout").exists()):
        raise ValueError("screen selection, source or holdout output differs")
    ic_environment(output / "preflight_k1107", K1107_ARM)
    bins = output / "executed_bins"
    ic_binary = bins / "koblitz_orbit_dlp_fast"
    rho_binary = bins / "n83_rho_optimization_eval"
    if (run.digest(ic_binary) != source["binary_sha256"] or
            run.digest(rho_binary) != source["rho_binary_sha256"]):
        raise ValueError("executed binaries differ")
    deadline = runpod_runner.timestamp(allocation["stop_after_utc"])
    schedule = events(candidate_id)
    holdout = output / "holdout"
    holdout.mkdir()
    run.parent.write_json(holdout / "source.json", {
        "schema": "n83.fixed31-cross-family-one-target-source/v2",
        "runner_sha256": run.digest(Path(__file__)),
        "candidate_id": candidate_id,
        "k1107_candidate_id": K1107_CANDIDATE_ID,
        "k1107_manifest_sha256": K1107_MANIFEST_SHA256,
        "selection_sha256": run.digest(selection_path),
        "screen_validated_sha256": run.digest(validated_path),
        "screen_analysis_sha256": run.digest(analysis_path),
        "workload_manifest_sha256": run.digest(HERE / "workloads.json"),
        "source_sha256": dispatch["source_sha256"],
        "binary_sha256": source["binary_sha256"],
        "rho_binary_sha256": source["rho_binary_sha256"],
        "targets_sha256": dispatch["holdout_targets_sha256"],
        "answers_sha256": run.digest(HERE / "holdout_answers.jsonl"),
        "runpod_allocation_sha256": run.digest(allocation_path),
        "events": schedule,
        "rho_seed_rule": "2026120300+2*point_index+(0 highk,1 k1107)",
    })
    run.parent.write_json(holdout / "host_telemetry_before.json",
                          run.telemetry.cpu_telemetry())
    group = run.parent.cgroup_root()
    for event in schedule:
        kind, index = event["kind"], event["point_index"]
        point_dir = holdout / "points" / f"p{index:02d}"
        point_dir.mkdir(parents=True, exist_ok=True)
        target = point_dir / "target.jsonl"
        if target.exists() and target.read_text() != streams["holdout"][index]:
            raise ValueError("one-target public point differs")
        target.write_text(streams["holdout"][index])
        folder = holdout / "runs" / event["name"]
        if kind == "ic":
            chosen = arm if event["arm"] == "highk" else K1107_ARM
            command = [str(ic_binary), f"construct:83:1:{chosen['k']}",
                       str(target), "29", str(folder / "targets_result.jsonl")]
            env = ic_environment(folder, chosen)
            cap = run.IC_CAP_SECONDS
        elif kind == "rho":
            command = [str(rho_binary), str(target),
                       str(RHO_SEED_BASE + 2 * index +
                           (0 if event["arm"] == "highk" else 1)),
                       str(run.cold.RHO_CONFIG),
                       str(folder / "receipt.json")]
            env = {"PATH": os.environ.get("PATH", "/usr/bin:/bin")}
            cap = run.RHO_CAP_SECONDS
        else:
            command = [str(rho_binary), str(run.cold.TARGETS),
                       str(run.cold.RHO_SEED), str(run.cold.RHO_CONFIG),
                       str(folder / "receipt.json")]
            env = {"PATH": os.environ.get("PATH", "/usr/bin:/bin")}
            cap = run.RHO_CAP_SECONDS
        remaining = (deadline - datetime.now(timezone.utc)).total_seconds()
        if remaining < cap + 1200:
            run.parent.write_json(holdout / "producer_stop.json", {
                "status": "PROVIDER_DEADLINE_BEFORE_NEXT_EVENT",
                "next_event": event, "seconds_remaining": remaining})
            break
        print(f"starting holdout/{event['name']} at {run.parent.now()}", flush=True)
        usage = run.telemetry.run_with_telemetry(folder, command, env, cap, group)
        print(f"holdout/{event['name']}: {usage['status']} after "
              f"{usage['wall_seconds']:.3f}s", flush=True)
    run.parent.write_json(holdout / "host_telemetry_after.json",
                          run.telemetry.cpu_telemetry())
    return 0


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", type=Path)
    parser.add_argument("--provider-receipt", type=Path)
    parser.add_argument("--schedule-preflight", action="store_true")
    args = parser.parse_args()
    if args.schedule_preflight:
        print(json.dumps({"event_count": len(events("candidate")),
                          "primary_point_index": 0,
                          "holdout_point_count": 16,
                          "candidate_count": 2}, sort_keys=True))
        return 0
    if not args.output or not args.provider_receipt:
        parser.error("holdout needs --output and --provider-receipt")
    receipt_path = args.provider_receipt.resolve()
    allocation = runpod_runner.read_allocation(receipt_path)
    head = subprocess.check_output(
        ["git", "rev-parse", "HEAD"], cwd=run.ROOT, text=True).strip()
    if head != allocation["source_head_sha"]:
        raise ValueError("Pod checkout differs from frozen allocation")
    runpod_runner.bind_provider(allocation, receipt_path, head)
    return run_holdout(args.output.resolve(), receipt_path)


if __name__ == "__main__":
    raise SystemExit(main())
