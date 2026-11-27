#!/usr/bin/env python3
"""Replay the original degree-32 cells with a commit-pinned ic executable."""
import argparse
import hashlib
import json
from pathlib import Path

import f4_degree32 as d32
import f4_degree32_replication as rep
import f4_height4 as f4

HERE = Path(__file__).resolve().parent
EXPECTED = "c98e290c7c7d5aee30ed70e1c9aa19726e830854b09ae15921f02b09c648de2d"
CASES = {
    "degree32": (20261128, ("baseline:1", "candidate:1")),
    "degree32_replication_extend": (20261149, ("baseline:1", "candidate:1", "candidate:2", "baseline:2")),
}


def load(path):
    rows = {}
    if path.exists():
        for line in path.read_text().splitlines():
            item = json.loads(line)
            if item["id"] in rows:
                raise ValueError(f"duplicate row {item['id']}")
            rows[item["id"]] = item
    return rows


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("case", choices=CASES)
    ap.add_argument("--bin", required=True, type=Path)
    ap.add_argument("--store", required=True, type=Path)
    args = ap.parse_args()
    digest = hashlib.sha256(args.bin.read_bytes()).hexdigest()
    if digest != EXPECTED:
        raise ValueError("source-pinned executable SHA-256 mismatch")
    seed, order = CASES[args.case]
    original = load(HERE / (args.case + ".jsonl"))
    records = load(args.store)
    if "header" not in records:
        item = {"id": "header", "schema": "pkm.degree32-original-source-replay/v1",
                "case": args.case, "seed": seed, "binary_sha256": digest,
                "source_commit": "a5503f4bb66ac3aa4eac3125fba01534d2574096",
                "protocol": "PROTOCOL_F4_DEGREE32_ORIGINAL_SOURCE_REPLAY_20261125.md"}
        f4.append(args.store, item)
        records[item["id"]] = item
    if records["header"]["binary_sha256"] != digest or records["header"]["case"] != args.case:
        raise ValueError("store header mismatch")
    if "verdict" in records:
        print(json.dumps(records["verdict"], sort_keys=True))
        return
    for arm, fb in (("baseline", d32.INTERVAL),
                    ("candidate", "prime-isogeny:t=5,seed=5,aux=ordinary")):
        key = "preflight:" + arm
        if key not in records:
            item = {"id": key, "arm": arm,
                    **f4.run(rep.command(args.bin, seed, fb, "", True), 30)}
            f4.append(args.store, item)
            records[key] = item
    base = records["preflight:baseline"]
    cand = records["preflight:candidate"]
    new_sizes = [d32.size(base), d32.size(cand)]
    old_base_key = "preflight:baseline"
    old_cand_key = "preflight:5" if args.case == "degree32" else "preflight:20261149:candidate"
    if args.case != "degree32":
        old_base_key = "preflight:20261149:baseline"
    old_sizes = [d32.size(original[old_base_key]), d32.size(original[old_cand_key])]
    valid = (all(x["exit_code"] in (0, 1) and x.get("result") for x in (base, cand))
             and base["result"].get("instance") == cand["result"].get("instance")
             and new_sizes[0] is not None and new_sizes[0] == new_sizes[1])
    if not valid or new_sizes != old_sizes:
        item = {"id": "verdict", "complete": False, "reason": "preflight-size-or-instance-mismatch",
                "old_sizes": old_sizes, "new_sizes": new_sizes,
                "old_instance": original[old_base_key].get("result", {}).get("instance"),
                "new_instances": [x.get("result", {}).get("instance") for x in (base, cand)]}
        f4.append(args.store, item)
        print(json.dumps(item, sort_keys=True))
        return
    for arm_rep in order:
        key = "timed:" + arm_rep
        if key in records:
            continue
        arm = arm_rep.split(":")[0]
        fb = d32.INTERVAL if arm == "baseline" else "prime-isogeny:t=5,seed=5,aux=ordinary"
        oracle = d32.INTERVAL_ORACLE if arm == "baseline" else f4.ISOGENY_ORACLE
        item = {"id": key, "arm": arm,
                **f4.run(rep.command(args.bin, seed, fb, oracle, False), 240)}
        f4.append(args.store, item)
        records[key] = item
    calls = [records["timed:" + key] for key in order]
    rowsets = [(x.get("result") or {}).get("rows") or [] for x in calls]
    complete = (all(x["exit_code"] == 0 and d32.size(x) == new_sizes[0] for x in calls)
                and all(len(rows) == d32.REPEATS and all(r.get("verified") for r in rows)
                        for rows in rowsets)
                and all(len({rows[i].get("recovered") for rows in rowsets}) == 1
                        for i in range(d32.REPEATS)))
    item = {"id": "verdict", "complete": complete,
            "reason": "complete" if complete else "timed-invocation-failed-or-censored",
            "old_sizes": old_sizes, "new_sizes": new_sizes}
    if complete:
        item["runs"] = [{"arm": key, "total_gae": sum(r["total_gae"] for r in rows),
                         "relations": sum(r["decomposition"]["relations_found"] for r in rows),
                         "trials": sum(r["decomposition"]["targets_tried"] for r in rows),
                         "wall_s": call["wall_s"], "cpu_s": call["child_cpu_s"],
                         "median_s_over_rho": sorted(r["s_over_rho"] for r in rows)[len(rows)//2]}
                        for key, call, rows in zip(order, calls, rowsets)]
    f4.append(args.store, item)
    print(json.dumps(item, sort_keys=True))


if __name__ == "__main__":
    main()
