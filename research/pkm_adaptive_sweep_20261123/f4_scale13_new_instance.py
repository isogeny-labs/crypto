#!/usr/bin/env python3
"""Resume the seed-10 PKM map on a fresh fixed 13-bit instance."""
import argparse
import hashlib
import json
from pathlib import Path

import f4_height4 as f4

HERE = Path(__file__).resolve().parent
EXPECTED_BINARY = "06439ae5119b5ed69c413df90e7a65dc67541dd1dfb9f6ac8d77cf2bd4441c59"
SEED = 20261124
REPEATS = 32


def command(binary, fb, oracle, preflight):
    cmd = [str(binary), "bench", "--bits", "13", "--family", "generic",
           "--seed", str(SEED), "--repeats", str(REPEATS), "--targets", "walk",
           "--factor-base", fb, "--oracle", "subtract" if preflight else oracle,
           "--max-trials", "1" if preflight else "1000", "--rho-runs", "0" if preflight else "1", "--json"]
    if not preflight:
        cmd += ["--solver-budget-seconds", "5"]
    return cmd


def size(item):
    rows = (item.get("result") or {}).get("rows") or []
    sizes = {r.get("factor_base", {}).get("abscissae") for r in rows}
    return sizes.pop() if len(rows) == REPEATS and len(sizes) == 1 else None


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--bin", type=Path, required=True)
    ap.add_argument("--store", type=Path, default=HERE / "scale13_new_instance.jsonl")
    args = ap.parse_args()
    digest = hashlib.sha256(args.bin.read_bytes()).hexdigest()
    if digest != EXPECTED_BINARY:
        raise ValueError("binary differs from frozen protocol")
    records = {}
    if args.store.exists():
        for line in args.store.read_text().splitlines():
            item = json.loads(line)
            if item["id"] in records:
                raise ValueError(f"duplicate {item['id']}")
            records[item["id"]] = item
    if "header" not in records:
        item = {"id": "header", "schema": "pkm.f4-scale13-new-instance/v1", "binary_sha256": digest,
                "protocol": "PROTOCOL_F4_SCALE13_NEW_INSTANCE_20261123.md", "seed": SEED, "repeats": REPEATS}
        f4.append(args.store, item)
        records["header"] = item
    if records["header"]["binary_sha256"] != digest:
        raise ValueError("binary changed")
    if "verdict" in records:
        print(json.dumps(records["verdict"], sort_keys=True))
        return
    if "preflight:candidate" not in records:
        item = {"id": "preflight:candidate", **f4.run(command(args.bin, "prime-isogeny:t=4,seed=10,aux=ordinary", "", True), 30)}
        f4.append(args.store, item)
        records[item["id"]] = item
    candidate = records["preflight:candidate"]
    if candidate["exit_code"] not in (0, 1) or size(candidate) is None:
        raise RuntimeError("candidate preflight incomplete")
    fb_size = size(candidate)
    instance = candidate["result"]["instance"]
    below = None
    for b in (8, 16, 32):
        key = f"preflight:interval:{b}"
        if key not in records:
            item = {"id": key, "below": b, **f4.run(command(args.bin, f"prime-abscissa:below={b}", "", True), 30)}
            f4.append(args.store, item)
            records[key] = item
        item = records[key]
        if item["exit_code"] in (0, 1) and item["result"].get("instance") == instance and size(item) == fb_size:
            below = b
            break
    if below is None:
        item = {"id": "verdict", "complete": False, "reason": "no-size-match", "candidate_size": fb_size}
        f4.append(args.store, item)
        return
    order = ("baseline:1", "candidate:1", "candidate:2", "baseline:2")
    for arm_rep in order:
        key = f"timed:{arm_rep}"
        if key in records:
            continue
        arm = arm_rep.split(":")[0]
        fb = f"prime-abscissa:below={below}" if arm == "baseline" else "prime-isogeny:t=4,seed=10,aux=ordinary"
        oracle = f4.INTERVAL_ORACLE.replace("bits=4", f"bits={below.bit_length() - 1}") if arm == "baseline" else f4.ISOGENY_ORACLE
        item = {"id": key, "arm": arm, **f4.run(command(args.bin, fb, oracle, False), 180)}
        f4.append(args.store, item)
        records[key] = item
    runs = [records[f"timed:{k}"] for k in order]
    rowsets = [(x.get("result") or {}).get("rows") or [] for x in runs]
    complete = (all(x["exit_code"] == 0 and x["result"].get("instance") == instance and size(x) == fb_size for x in runs)
                and all(len(rs) == REPEATS for rs in rowsets)
                and all(all(r.get("verified") and r.get("recovered") is not None for r in rs) for rs in rowsets)
                and all(len({rowsets[j][i]["recovered"] for j in range(4)}) == 1 for i in range(REPEATS)))
    item = {"id": "verdict", "complete": complete, "instance": instance, "below": below, "base_size": fb_size}
    if complete:
        pairs = []
        for baseline_index, candidate_index in ((0, 1), (3, 2)):
            a, b = runs[baseline_index], runs[candidate_index]
            ag = sum(r["total_gae"] for r in rowsets[baseline_index])
            bg = sum(r["total_gae"] for r in rowsets[candidate_index])
            pairs.append({"gae_ratio": bg / ag, "wall_ratio": b["wall_s"] / a["wall_s"],
                          "cpu_ratio": b["child_cpu_s"] / a["child_cpu_s"],
                          "totals_gae": [ag, bg], "wall_s": [a["wall_s"], b["wall_s"]],
                          "cpu_s": [a["child_cpu_s"], b["child_cpu_s"]]})
        total_a = sum(p["totals_gae"][0] for p in pairs)
        total_b = sum(p["totals_gae"][1] for p in pairs)
        item.update({"pairs": pairs, "combined_gae_ratio": total_b / total_a,
                     "raw_relations": [sum(r["decomposition"]["relations_found"] for rs in (rowsets[0], rowsets[3]) for r in rs),
                                       sum(r["decomposition"]["relations_found"] for rs in (rowsets[1], rowsets[2]) for r in rs)],
                     "target_trials": [sum(r["decomposition"]["targets_tried"] for rs in (rowsets[0], rowsets[3]) for r in rs),
                                       sum(r["decomposition"]["targets_tried"] for rs in (rowsets[1], rowsets[2]) for r in rs)]})
        item["robust_component_lead"] = (total_b / total_a < .9 and all(p["gae_ratio"] < .9 and
                                         p["wall_ratio"] < 1 and p["cpu_ratio"] < 1 for p in pairs))
    else:
        item["robust_component_lead"] = False
    f4.append(args.store, item)
    print(json.dumps(item, sort_keys=True))


if __name__ == "__main__":
    main()
