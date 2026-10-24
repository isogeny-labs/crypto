#!/usr/bin/env python3
"""Independently replay N83 local five-base process evidence."""

import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import statistics

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
TARGET_SHA256 = "b1a9b1bc398b1c73867b8b0c19ef8a7438225917989e08b1c15aef2c7640c0bf"
NAMES = ("k992_root11", "k1024_root11", "k3300_wide3",
         "k2304_root11", "k1107_root11")
KS = (992, 1024, 3300, 2304, 1107)


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def group_module():
    spec = importlib.util.spec_from_file_location(
        "n83_five_base_audit_group", ROOT / "research/n83_indexed5_20261013/analyze.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def custody():
    prior = ROOT / "research/n83_fresh_cold_finalists_20261019"
    records = []
    for name in ("base_custody_prior_three_receipt.json", "base_custody_missing_receipt.json"):
        source = json.loads((prior / name).read_text())
        if source["status"] != "PASS_S3_ROUNDTRIP":
            raise ValueError(f"prior factor-base custody failed: {name}")
        records.extend(source["objects"])
    by_k = {item["k"]: item for item in records}
    if set(by_k) != set(KS) or len(records) != 5:
        raise ValueError("custodied five-base family differs")
    return by_k


def audit(raw):
    group = group_module()
    target_path = HERE / "target.jsonl"
    answer = json.loads((HERE / "answer.jsonl").read_text())
    scalar = int(answer["scalar"])
    target = tuple(map(int, json.loads(target_path.read_text())))
    if (digest(target_path) != TARGET_SHA256 or
            group.multiply(group.G, scalar) != target or
            not group.on_curve(target) or group.multiply(target, group.R) is not None):
        raise ValueError("target and independent scalar replay differ")
    dispatch_path = ROOT / "research/n83_fresh_cold_finalists_20261019/dispatch.json"
    dispatch = json.loads(dispatch_path.read_text())
    host = json.loads((raw / "host.json").read_text())
    if (host["source_commit"] != "8bf07aefb3fb2a65248ce379da7284cb58a5581d" or
            host["dispatch_sha256"] != digest(dispatch_path) or
            host["input_sha256"]["target"] != TARGET_SHA256):
        raise ValueError("source, dispatch or target receipt differs")
    records = custody()
    expected_names = (["rho_before"] + ["a_" + name for name in NAMES] +
                      ["b_" + name for name in reversed(NAMES)] + ["rho_after"])
    summary = json.loads((raw / "summary.json").read_text())
    if [row["name"] for row in summary] != expected_names:
        raise ValueError("fixed forward/reverse schedule differs")
    arm_ids = dict(zip(NAMES, dispatch["ordered_candidate_ids"]))
    audited = []
    for row in summary:
        name = row["name"]
        folder = raw / name
        receipt = json.loads((folder / "receipt.json").read_text())
        if (row != receipt or receipt["status"] != "COMPLETE" or
                receipt["exit_code"] != 0 or receipt["wall_seconds"] > receipt["cap_seconds"]):
            raise ValueError(f"{name}: incomplete or altered process")
        for relative, expected in receipt["files"].items():
            path = folder / relative
            if (not path.is_file() or path.stat().st_size != expected["bytes"] or
                    digest(path) != expected["sha256"]):
                raise ValueError(f"{name}: altered raw file {relative}")
        if receipt["kind"] == "rho":
            result = json.loads((folder / "rho_result.json").read_text())
            if (int(result["recovered_scalar"]) != scalar or
                    tuple(map(int, result["public_target_q"])) != target or
                    result["generic_scalar_replay_verified"] is not True):
                raise ValueError(f"{name}: rho scalar differs")
            timing = {"rho_target_ms": result["target_ms"]}
            candidate = None
        else:
            arm = name[2:]
            k = receipt["k"]
            record = records[k]
            candidate = receipt["candidate_id"]
            manifest_dir = (ROOT / "research/n83_fresh_cold_finalists_20261019"
                            if k in (992, 1024, 3300) else
                            ROOT / "research/n83_online_finalists_20261018")
            manifest_path = manifest_dir / f"candidate_{candidate}.json"
            manifest = json.loads(manifest_path.read_text())
            factor_base = manifest["factor_base"]
            if (arm not in arm_ids or candidate != arm_ids[arm] or
                    manifest["candidate_id"] != candidate or
                    digest(manifest_path) != dispatch["choice"]["candidate_manifest_sha256"][candidate] or
                    record["raw_sha256"] != factor_base["ordered_base_dump_sha256"] or
                    receipt["files"]["base_dump.jsonl"]["sha256"] != record["raw_sha256"]):
                raise ValueError(f"{name}: factor-base identity differs")
            source = json.loads((folder / "stdout.jsonl").read_text().splitlines()[0])
            result = json.loads((folder / "targets_result.jsonl").read_text())
            if (source["base_hash"] != factor_base["source_base_hash_blake3"] or
                    source["factor_base_points"] != factor_base["actual_usable_points"] or
                    source["rank"] != k or source["rank_relations"] != k or
                    result["recovered_scalar"] != scalar or
                    tuple(map(int, result["target"])) != target or
                    result["group_verified"] is not True):
                raise ValueError(f"{name}: rank or recovered scalar differs")
            timing = source["timing_ms"]
        audited.append({"name": name, "status": "PASS_REPLAY", "candidate_id": candidate,
                        "wall_seconds": receipt["wall_seconds"], "timing": timing})
    medians = {}
    spreads = {}
    for name in NAMES:
        pair = [item["wall_seconds"] for item in audited if item["name"] in ("a_" + name, "b_" + name)]
        medians[name] = statistics.median(pair)
        spreads[name] = 100 * (max(pair) - min(pair)) / medians[name]
    rho_pair = [item["wall_seconds"] for item in audited if item["name"].startswith("rho_")]
    return {"schema": "n83.local-five-base-audit/v1",
            "status": "PASS_EXPLORATORY_LOCAL_REPLAY",
            "selected_best_total_runtime": None,
            "target_sha256": TARGET_SHA256, "rows": audited,
            "median_whole_process_seconds": medians,
            "pair_spread_percent": spreads,
            "rho_median_wall_seconds": statistics.median(rho_pair),
            "rho_pair_spread_percent": 100 * (max(rho_pair) - min(rho_pair)) / statistics.median(rho_pair),
            "descriptive_lowest_median_arm": min(medians, key=medians.get)}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("raw", type=Path)
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()
    result = audit(args.raw.resolve())
    args.out.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps({key: result[key] for key in
                      ("status", "median_whole_process_seconds", "pair_spread_percent",
                       "rho_median_wall_seconds", "rho_pair_spread_percent",
                       "descriptive_lowest_median_arm")}, sort_keys=True))


if __name__ == "__main__":
    main()
