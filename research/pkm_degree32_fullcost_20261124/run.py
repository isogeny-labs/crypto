#!/usr/bin/env python3
"""Resume the frozen PKM degree-32 cold full-rank comparison."""
import argparse
import hashlib
import json
import math
import os
import resource
import subprocess
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
EXPECTED_BINARY = "06439ae5119b5ed69c413df90e7a65dc67541dd1dfb9f6ac8d77cf2bd4441c59"
EXPECTED_INSTANCE = "icv1-fp13-t19-0d05ae31"
REPEATS = 16


def append(path, item):
    with path.open("a") as handle:
        handle.write(json.dumps(item, sort_keys=True) + "\n")
        handle.flush()
        os.fsync(handle.fileno())
    print(item["id"], item.get("exit_code"), flush=True)


def run(command, timeout):
    before = resource.getrusage(resource.RUSAGE_CHILDREN)
    started = time.perf_counter()
    try:
        proc = subprocess.run(command, capture_output=True, text=True, timeout=timeout)
        code, stdout, stderr = proc.returncode, proc.stdout, proc.stderr
    except subprocess.TimeoutExpired as exc:
        code = 124
        stdout = exc.stdout.decode(errors="replace") if isinstance(exc.stdout, bytes) else (exc.stdout or "")
        stderr = exc.stderr.decode(errors="replace") if isinstance(exc.stderr, bytes) else (exc.stderr or "")
    after = resource.getrusage(resource.RUSAGE_CHILDREN)
    try:
        result = json.loads(stdout)
    except json.JSONDecodeError:
        result = None
    return {
        "command": command,
        "exit_code": code,
        "wall_s": time.perf_counter() - started,
        "child_cpu_s": (after.ru_utime + after.ru_stime) - (before.ru_utime + before.ru_stime),
        "result": result,
        "stderr_tail": stderr[-2000:],
        "stdout_tail": stdout[-2000:] if result is None else None,
    }


def command(binary, arm):
    if arm == "baseline":
        factor_base = "prime-abscissa:below=32"
        oracle = "pkm-semaev:m=2,engine=f4,degree=8,bits=5,lo=0,mode=chain"
    else:
        factor_base = "prime-isogeny:t=5,seed=5,aux=ordinary"
        oracle = "pkm-semaev:m=2,engine=f4,degree=8,mode=chain"
    return [
        str(binary), "bench", "--bits", "13", "--family", "generic",
        "--seed", "20261149", "--repeats", str(REPEATS), "--targets", "walk",
        "--factor-base", factor_base, "--oracle", oracle,
        "--linalg", "incremental-gauss-full-rank-checked",
        "--max-trials", "5000", "--solver-budget-seconds", "5",
        "--rho-runs", str(REPEATS), "--json",
    ]


def rows(item):
    return (item.get("result") or {}).get("rows") or []


def rho(item):
    return (item.get("result") or {}).get("rho_reference") or {}


def complete_invocation(item):
    got = rows(item)
    result = item.get("result") or {}
    if item.get("exit_code") != 0 or result.get("instance") != EXPECTED_INSTANCE:
        return False
    if result.get("status") != "complete" or not result.get("all_verified"):
        return False
    if result.get("cost_qualification") is not None or len(got) != REPEATS:
        return False
    recovered = [row.get("recovered") for row in got]
    if any(value is None for value in recovered):
        return False
    for row in got:
        fb = row.get("factor_base", {})
        la = row.get("linear_algebra", {})
        native = la.get("cost", {}).get("native", {})
        if not row.get("verified") or row.get("exhausted"):
            return False
        if fb.get("abscissae") != 16 or fb.get("columns") != 16:
            return False
        if la.get("rank") != 17 or native.get("full_rank_reached") != 1:
            return False
    reference = rho(item)
    if not reference.get("all_verified") or reference.get("runs") != REPEATS:
        return False
    return [entry.get("planted") for entry in reference.get("per_run", [])] == recovered


def totals(item):
    got = rows(item)
    reference = rho(item)
    sqrt_r = math.sqrt(got[0]["r"])
    rho_total_gae = REPEATS * reference["mean_s"] * sqrt_r
    return {
        "total_gae": sum(row["total_gae"] for row in got),
        "row_wall_s": sum(row["wall_seconds"] for row in got),
        "external_wall_s": item["wall_s"],
        "child_cpu_s": item["child_cpu_s"],
        "relations": sum(row["decomposition"]["relations_found"] for row in got),
        "target_trials": sum(row["decomposition"]["targets_tried"] for row in got),
        "f4_multiply_adds": sum(row["decomposition"]["solver"]["ops"] for row in got),
        "la_row_ops": sum(row["linear_algebra"]["work"] for row in got),
        "la_gae": sum(row["linear_algebra"]["cost"]["gae"] for row in got),
        "rho_total_gae": rho_total_gae,
        "vs_rho": sum(row["total_gae"] for row in got) / rho_total_gae,
        "recovered": [row["recovered"] for row in got],
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bin", type=Path, required=True)
    parser.add_argument("--store", type=Path, default=HERE / "fullrank.jsonl")
    args = parser.parse_args()
    digest = hashlib.sha256(args.bin.read_bytes()).hexdigest()
    if digest != EXPECTED_BINARY:
        raise ValueError("binary SHA-256 differs from frozen protocol")
    records = {}
    if args.store.exists():
        for line in args.store.read_text().splitlines():
            item = json.loads(line)
            if item["id"] in records:
                raise ValueError(f"duplicate record {item['id']}")
            records[item["id"]] = item
    if "header" not in records:
        item = {
            "id": "header", "schema": "pkm.degree32-fullrank/v1",
            "binary_sha256": digest, "protocol": "PROTOCOL.md",
            "order": ["baseline:1", "candidate:1", "candidate:2", "baseline:2"],
        }
        append(args.store, item)
        records["header"] = item
    if records["header"]["binary_sha256"] != digest:
        raise ValueError("store binary mismatch")
    if "verdict" in records:
        print(json.dumps(records["verdict"], sort_keys=True))
        return
    order = ["baseline:1", "candidate:1", "candidate:2", "baseline:2"]
    for arm_rep in order:
        key = f"timed:{arm_rep}"
        if key in records:
            continue
        arm = arm_rep.split(":")[0]
        item = {"id": key, "arm": arm, **run(command(args.bin, arm), 600)}
        append(args.store, item)
        records[key] = item
    invocations = [records[f"timed:{arm_rep}"] for arm_rep in order]
    complete = all(complete_invocation(item) for item in invocations)
    verdict = {"id": "verdict", "complete": complete, "n_invocations": 4}
    if complete:
        arms = [totals(item) for item in invocations]
        same_scalars = all(arm["recovered"] == arms[0]["recovered"] for arm in arms[1:])
        pairs = []
        for baseline_index, candidate_index in ((0, 1), (3, 2)):
            baseline, candidate = arms[baseline_index], arms[candidate_index]
            pairs.append({
                "gae_ratio": candidate["total_gae"] / baseline["total_gae"],
                "row_wall_ratio": candidate["row_wall_s"] / baseline["row_wall_s"],
                "external_wall_ratio": candidate["external_wall_s"] / baseline["external_wall_s"],
                "cpu_ratio": candidate["child_cpu_s"] / baseline["child_cpu_s"],
                "candidate_vs_rho": candidate["vs_rho"],
                "baseline_vs_rho": baseline["vs_rho"],
            })
        candidate_gae = arms[1]["total_gae"] + arms[2]["total_gae"]
        baseline_gae = arms[0]["total_gae"] + arms[3]["total_gae"]
        verdict.update({
            "same_scalars": same_scalars,
            "arms": arms,
            "pairs": pairs,
            "combined_gae_ratio": candidate_gae / baseline_gae,
            "component_gate": same_scalars and all(
                pair["gae_ratio"] < 0.9 and pair["row_wall_ratio"] < 1 and pair["cpu_ratio"] < 1
                for pair in pairs
            ),
            "cold_rho_crossover": same_scalars and all(pair["candidate_vs_rho"] < 1 for pair in pairs),
        })
    else:
        verdict.update({"component_gate": False, "cold_rho_crossover": False})
    append(args.store, verdict)
    print(json.dumps(verdict, sort_keys=True))


if __name__ == "__main__":
    main()
