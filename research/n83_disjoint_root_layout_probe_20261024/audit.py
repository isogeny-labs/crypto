#!/usr/bin/env python3
"""Independently audit local N83 root-layout raw process receipts."""

import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import statistics

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
EXPECTED_BASE_SHA256 = "08608f6ad2b8c66510246348933740c78d0937cae783d521514f467940edc3dc"
EXPECTED_BASE_BLAKE3 = "b0aea97c1c8b9b46e13df67e6af9c33ca73034d2ec3eff33a2392ceb1452b40e"
EXPECTED_TARGET_SHA256 = "5378b8e1b3b5e01f4590ddd89610ab39b3fca28ecaaa050013f2031dc3c1bb1b"
ARMS = ("root11_a", "root22_a", "root22_b", "root11_b")


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def load_group():
    spec = importlib.util.spec_from_file_location(
        "n83_disjoint_audit_group", ROOT / "research/n83_indexed5_20261013/analyze.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def audit(raw):
    group = load_group()
    target_path = HERE / "target.jsonl"
    target = tuple(map(int, json.loads(target_path.read_text())))
    answer = json.loads((HERE / "answer.jsonl").read_text())
    scalar = int(answer["scalar"])
    if (digest(target_path) != EXPECTED_TARGET_SHA256 or
            group.multiply(group.G, scalar) != target or
            not group.on_curve(target) or group.multiply(target, group.R) is not None):
        raise ValueError("public point and independent scalar replay differ")
    host = json.loads((raw / "host.json").read_text())
    if (host["source_commit"] != "8bf07aefb3fb2a65248ce379da7284cb58a5581d" or
            host["input_sha256"]["target"] != EXPECTED_TARGET_SHA256):
        raise ValueError("source or input receipt differs")
    summary = json.loads((raw / "summary.json").read_text())
    if [row["name"] for row in summary] != [*ARMS, "rho"]:
        raise ValueError("ABBA and rho order differs")
    audited = []
    for row in summary:
        name = row["name"]
        folder = raw / name
        receipt = json.loads((folder / "receipt.json").read_text())
        if (row != {"name": name, **receipt} or receipt["status"] != "COMPLETE" or
                receipt["exit_code"] != 0 or receipt["wall_seconds"] > receipt["cap_seconds"]):
            raise ValueError(f"{name}: process receipt differs or did not complete")
        for relative, expected in receipt["files"].items():
            path = folder / relative
            if not path.is_file() or path.stat().st_size != expected["bytes"] or digest(path) != expected["sha256"]:
                raise ValueError(f"{name}: file hash differs: {relative}")
        if name == "rho":
            result = json.loads((folder / "rho_result.json").read_text())
            if (int(result["recovered_scalar"]) != scalar or
                    tuple(map(int, result["public_target_q"])) != target or
                    result["generic_scalar_replay_verified"] is not True):
                raise ValueError("rho answer differs")
            timing = {"rho_target_ms": result["target_ms"]}
        else:
            if (receipt["files"]["base_dump.jsonl"]["sha256"] != EXPECTED_BASE_SHA256 or
                    digest(folder / "base_dump.jsonl") != EXPECTED_BASE_SHA256):
                raise ValueError(f"{name}: base differs from S3-custodied K1107")
            source = json.loads((folder / "stdout.jsonl").read_text().splitlines()[0])
            result = json.loads((folder / "targets_result.jsonl").read_text())
            if (source["base_hash"] != EXPECTED_BASE_BLAKE3 or
                    source["factor_base_points"] != 183762 or source["rank"] != 1107 or
                    result["recovered_scalar"] != scalar or
                    tuple(map(int, result["target"])) != target or
                    result["group_verified"] is not True):
                raise ValueError(f"{name}: base, rank, or scalar differs")
            expected_layout = "root22" if "root22" in name else "root11"
            if receipt["environment"]["KIC_ROOT_ONLY22"] != ("1" if expected_layout == "root22" else "0"):
                raise ValueError(f"{name}: layout flag differs")
            timing = source["timing_ms"]
        audited.append({"name": name, "wall_seconds": receipt["wall_seconds"],
                        "timing": timing, "status": "PASS_REPLAY"})
    root11 = [item["wall_seconds"] for item in audited if item["name"].startswith("root11")]
    root22 = [item["wall_seconds"] for item in audited if item["name"].startswith("root22")]
    result = {"schema": "n83.disjoint-root-layout-local-audit/v1",
              "status": "PASS_EXPLORATORY_LOCAL_REPLAY", "selected_best_total_runtime": None,
              "target_sha256": EXPECTED_TARGET_SHA256, "base_sha256": EXPECTED_BASE_SHA256,
              "base_points": 183762, "rows": audited,
              "root11_median_wall_seconds": statistics.median(root11),
              "root22_median_wall_seconds": statistics.median(root22),
              "root22_minus_root11_median_wall_seconds": statistics.median(root22) - statistics.median(root11),
              "root11_pair_spread_percent": 100 * (max(root11) - min(root11)) / statistics.median(root11),
              "root22_pair_spread_percent": 100 * (max(root22) - min(root22)) / statistics.median(root22)}
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("raw", type=Path)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    result = audit(args.raw.resolve())
    args.out.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps({key: result[key] for key in
                      ("status", "root11_median_wall_seconds", "root22_median_wall_seconds",
                       "root22_minus_root11_median_wall_seconds", "root11_pair_spread_percent",
                       "root22_pair_spread_percent")}, sort_keys=True))


if __name__ == "__main__":
    main()
