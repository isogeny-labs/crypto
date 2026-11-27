#!/usr/bin/env python3
"""Resume the prospectively fixed 13-bit degree-16 PKM F4 panel."""
import argparse
import hashlib
import json
import random
from pathlib import Path

import f4_height4 as f4

HERE = Path(__file__).resolve().parent
EXPECTED_BINARY = "06439ae5119b5ed69c413df90e7a65dc67541dd1dfb9f6ac8d77cf2bd4441c59"
REPEATS = 32
SEED = 20261123


def command(binary, fb, oracle, preflight):
    cmd = [str(binary), "bench", "--bits", "13", "--family", "generic",
           "--seed", str(SEED), "--repeats", str(REPEATS), "--targets", "walk",
           "--factor-base", fb, "--oracle", "subtract" if preflight else oracle,
           "--max-trials", "1" if preflight else "1000",
           "--rho-runs", "0" if preflight else "1", "--json"]
    if not preflight:
        cmd += ["--solver-budget-seconds", "5"]
    return cmd


def base_size(record):
    rows = (record.get("result") or {}).get("rows") or []
    sizes = {r.get("factor_base", {}).get("abscissae") for r in rows}
    return sizes.pop() if len(rows) == REPEATS and len(sizes) == 1 else None


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--bin", type=Path, required=True)
    ap.add_argument("--store", type=Path, default=HERE / "scale13.jsonl")
    args = ap.parse_args()
    digest = hashlib.sha256(args.bin.read_bytes()).hexdigest()
    if digest != EXPECTED_BINARY:
        raise ValueError("binary SHA-256 differs from frozen protocol")
    records = {}
    if args.store.exists():
        for line in args.store.read_text().splitlines():
            record = json.loads(line)
            if record["id"] in records:
                raise ValueError(f"duplicate record {record['id']}")
            records[record["id"]] = record
    if "header" not in records:
        record = {"id": "header", "schema": "pkm.f4-scale13/v1", "binary_sha256": digest,
                  "protocol": "PROTOCOL_F4_SCALE13_20261123.md", "seed": SEED, "repeats": REPEATS}
        f4.append(args.store, record)
        records["header"] = record
    if records["header"]["binary_sha256"] != digest:
        raise ValueError("store binary mismatch")
    if "verdict" in records:
        print(json.dumps(records["verdict"], sort_keys=True))
        return
    if "preflight:baseline" not in records:
        record = {"id": "preflight:baseline", **f4.run(command(args.bin, f4.INTERVAL, "", True), 30)}
        f4.append(args.store, record)
        records[record["id"]] = record
    baseline = records["preflight:baseline"]
    size = base_size(baseline)
    if baseline["exit_code"] not in (0, 1) or size is None:
        raise RuntimeError("baseline preflight incomplete")
    instance = baseline["result"]["instance"]
    selected = None
    for aux in range(64):
        key = f"preflight:{aux}"
        if key not in records:
            fb = f"prime-isogeny:t=4,seed={aux},aux=ordinary"
            record = {"id": key, "aux_seed": aux, **f4.run(command(args.bin, fb, "", True), 30)}
            f4.append(args.store, record)
            records[key] = record
        record = records[key]
        if record["exit_code"] in (0, 1) and record["result"].get("instance") == instance and base_size(record) == size:
            selected = aux
            break
    if selected is None:
        record = {"id": "verdict", "complete": False, "reason": "no-size-match", "baseline_size": size}
        f4.append(args.store, record)
        return
    for arm in ("baseline", "candidate"):
        key = f"timed:{arm}"
        if key in records:
            continue
        fb = f4.INTERVAL if arm == "baseline" else f"prime-isogeny:t=4,seed={selected},aux=ordinary"
        oracle = f4.INTERVAL_ORACLE if arm == "baseline" else f4.ISOGENY_ORACLE
        record = {"id": key, "arm": arm, "aux_seed": selected if arm == "candidate" else None,
                  **f4.run(command(args.bin, fb, oracle, False), 180)}
        f4.append(args.store, record)
        records[key] = record
    a, b = records["timed:baseline"], records["timed:candidate"]
    ar, br = (a.get("result") or {}).get("rows") or [], (b.get("result") or {}).get("rows") or []
    complete = (a["exit_code"] == b["exit_code"] == 0 and
                a["result"].get("instance") == b["result"].get("instance") == instance and
                base_size(a) == base_size(b) == size and len(ar) == len(br) == REPEATS and
                all(x.get("verified") and y.get("verified") and
                    x.get("recovered") is not None and x.get("recovered") == y.get("recovered")
                    for x, y in zip(ar, br)))
    verdict = {"id": "verdict", "complete": complete, "instance": instance,
               "base_size": size, "aux_seed": selected, "n_pairs": REPEATS if complete else 0}
    if complete:
        ag, bg = [r["total_gae"] for r in ar], [r["total_gae"] for r in br]
        rng = random.Random(SEED)
        draws = []
        for _ in range(10000):
            ids = [rng.randrange(REPEATS) for __ in range(REPEATS)]
            draws.append(sum(bg[i] for i in ids) / sum(ag[i] for i in ids))
        draws.sort()
        verdict.update({"gae_ratio": sum(bg) / sum(ag),
                        "bootstrap_99_percentile": [draws[50], draws[9950]],
                        "wall_ratio": b["wall_s"] / a["wall_s"],
                        "cpu_ratio": b["child_cpu_s"] / a["child_cpu_s"],
                        "paired_wins": sum(y < x for x, y in zip(ag, bg)),
                        "totals_gae": [sum(ag), sum(bg)],
                        "f4_multiply_adds": [sum(r.get("decomposition", {}).get("solver", {}).get("ops", 0) for r in arm) for arm in (ar, br)],
                        "target_trials": [sum(r.get("decomposition", {}).get("targets_tried", 0) for r in arm) for arm in (ar, br)],
                        "raw_relations": [sum(r.get("decomposition", {}).get("relations_found", 0) for r in arm) for arm in (ar, br)]})
        verdict["component_lead"] = (verdict["gae_ratio"] < .9 and draws[9950] < 1 and
                                      verdict["wall_ratio"] < 1 and verdict["cpu_ratio"] < 1)
    else:
        verdict["component_lead"] = False
    f4.append(args.store, verdict)
    print(json.dumps(verdict, sort_keys=True))


if __name__ == "__main__":
    main()
