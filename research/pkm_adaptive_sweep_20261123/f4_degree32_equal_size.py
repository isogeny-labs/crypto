#!/usr/bin/env python3
"""Degree-32 seed-5 vs equal-size interval, parameterised by bit size.

Same frozen shape as f4_degree32_source_pinned_equal18.py (order interval,
candidate, candidate, interval; preflights must match instance and abscissa
count), but the field size, interval bound and expected base size are explicit
arguments so a 14-bit equal-size rung can be run without editing an evidence
script. The binary SHA-256 is still checked against --expected-sha.
"""
import argparse
import hashlib
import json
import statistics
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
RESEARCH = Path("/Volumes/SSD990-2/.codex/worktrees/crypto-pkm-map-sweep-20261123"
                "/research/pkm_adaptive_sweep_20261123")
for p in (HERE, RESEARCH):
    if str(p) not in sys.path:
        sys.path.insert(0, str(p))
import f4_height4 as f4  # noqa: E402

CANDIDATE_ORACLE = "pkm-semaev:m=2,engine=f4,degree=8,mode=chain"
INTERVAL_ORACLE = "pkm-semaev:m=2,engine=f4,degree=8,bits=5,lo=0,mode=chain"


def command(args, seed, fb, oracle, preflight):
    cmd = [str(args.bin), "bench", "--bits", str(args.bits), "--family", "generic",
           "--seed", str(seed), "--repeats", str(args.repeats), "--targets", "walk",
           "--factor-base", fb, "--oracle", "subtract" if preflight else oracle,
           "--max-trials", "1" if preflight else str(args.max_trials), "--rho-runs",
           "0" if preflight else "1", "--json"]
    if not preflight:
        cmd += ["--solver-budget-seconds", str(args.solver_budget)]
    return cmd


def size(item):
    rows = (item.get("result") or {}).get("rows") or []
    values = {r.get("factor_base", {}).get("abscissae") for r in rows}
    return values.pop() if len(rows) == item.get("_repeats") and len(values) == 1 else None


def median(rows, key):
    vals = [r.get(key) for r in rows if isinstance(r.get(key), (int, float))]
    return statistics.median(vals) if vals else None


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--bin", type=Path, required=True)
    ap.add_argument("--expected-sha", required=True)
    ap.add_argument("--seed", type=int, required=True)
    ap.add_argument("--bits", type=int, required=True)
    ap.add_argument("--base-size", type=int, required=True)
    ap.add_argument("--interval", required=True)
    ap.add_argument("--candidate", required=True)
    ap.add_argument("--repeats", type=int, default=16)
    ap.add_argument("--max-trials", type=int, default=1000)
    ap.add_argument("--solver-budget", type=int, default=5)
    ap.add_argument("--call-cap", type=int, default=900)
    ap.add_argument("--protocol", required=True)
    ap.add_argument("--store", type=Path, required=True)
    args = ap.parse_args()

    digest = hashlib.sha256(args.bin.read_bytes()).hexdigest()
    if digest != args.expected_sha:
        raise SystemExit(f"binary differs from protocol: {digest}")

    def sz(item):
        item["_repeats"] = args.repeats
        return size(item)

    records = {}
    if args.store.exists():
        for line in args.store.read_text().splitlines():
            item = json.loads(line)
            if item["id"] in records:
                raise SystemExit(f"duplicate {item['id']}")
            records[item["id"]] = item
    if "header" not in records:
        item = {"id": "header", "schema": "pkm.f4-degree32-equal-size/v1",
                "binary_sha256": digest, "protocol": args.protocol, "seed": args.seed,
                "bits": args.bits, "repeats": args.repeats,
                "expected_base_size": args.base_size, "interval": args.interval}
        f4.append(args.store, item)
        records["header"] = item
    if "verdict" in records:
        print(json.dumps(records["verdict"], sort_keys=True))
        return

    for key, fb in (("preflight:baseline", args.interval), ("preflight:candidate", args.candidate)):
        if key not in records:
            item = {"id": key, "arm": key.split(":")[1],
                    **f4.run(command(args, args.seed, fb, "", True), 60)}
            f4.append(args.store, item)
            records[key] = item

    a = records["preflight:baseline"]
    b = records["preflight:candidate"]
    if a["exit_code"] not in (0, 1) or b["exit_code"] not in (0, 1):
        fail = {"id": "verdict", "complete": False, "reason": "preflight-failed"}
        f4.append(args.store, fail)
        print(json.dumps(fail, sort_keys=True))
        return
    if a["result"].get("instance") != b["result"].get("instance") or sz(a) != sz(b) or sz(a) != args.base_size:
        fail = {"id": "verdict", "complete": False, "reason": "size-mismatch",
                "instance": a["result"].get("instance"), "sizes": [sz(a), sz(b)]}
        f4.append(args.store, fail)
        print(json.dumps(fail, sort_keys=True))
        return

    order = ("baseline:1", "candidate:1", "candidate:2", "baseline:2")
    for arm_rep in order:
        key = f"timed:{arm_rep}"
        if key in records:
            continue
        arm = arm_rep.split(":")[0]
        fb = args.interval if arm == "baseline" else args.candidate
        oracle = INTERVAL_ORACLE if arm == "baseline" else CANDIDATE_ORACLE
        item = {"id": key, "arm": arm, "seed": args.seed,
                **f4.run(command(args, args.seed, fb, oracle, False), args.call_cap)}
        f4.append(args.store, item)
        records[key] = item

    runs = [records[f"timed:{x}"] for x in order]
    rowsets = [(x.get("result") or {}).get("rows") or [] for x in runs]
    complete = (
        all(x["exit_code"] == 0 and x["result"].get("instance") == a["result"].get("instance")
            and sz(x) == args.base_size for x in runs)
        and all(len(rs) == args.repeats for rs in rowsets)
        and all(all(r.get("verified") and r.get("recovered") is not None for r in rs) for rs in rowsets)
        and all(len({rowsets[j][i]["recovered"] for j in range(4)}) == 1 for i in range(args.repeats))
    )
    item = {"id": "verdict", "complete": complete, "seed": args.seed,
            "instance": a["result"].get("instance"), "base_size": args.base_size,
            "bits": args.bits}
    if complete:
        pairs = []
        for ai, bi in ((0, 1), (3, 2)):
            ag = sum(r["total_gae"] for r in rowsets[ai])
            bg = sum(r["total_gae"] for r in rowsets[bi])
            pairs.append({
                "gae_ratio": bg / ag,
                "wall_ratio": runs[bi]["wall_s"] / runs[ai]["wall_s"],
                "cpu_ratio": runs[bi]["child_cpu_s"] / runs[ai]["child_cpu_s"],
                "totals_gae": [ag, bg],
                "wall_s": [runs[ai]["wall_s"], runs[bi]["wall_s"]],
                "cpu_s": [runs[ai]["child_cpu_s"], runs[bi]["child_cpu_s"]],
            })
        item["pairs"] = pairs
        item["combined_gae_ratio"] = (
            sum(p["totals_gae"][1] for p in pairs) / sum(p["totals_gae"][0] for p in pairs)
        )
        item["raw_relations"] = [
            sum(r["decomposition"]["relations_found"] for rs in (rowsets[0], rowsets[3]) for r in rs),
            sum(r["decomposition"]["relations_found"] for rs in (rowsets[1], rowsets[2]) for r in rs),
        ]
        item["target_trials"] = [
            sum(r["decomposition"]["targets_tried"] for rs in (rowsets[0], rowsets[3]) for r in rs),
            sum(r["decomposition"]["targets_tried"] for rs in (rowsets[1], rowsets[2]) for r in rs),
        ]
        item["median_s_over_rho"] = [
            median(rowsets[0] + rowsets[3], "s_over_rho"),
            median(rowsets[1] + rowsets[2], "s_over_rho"),
        ]
        item["robust_component_lead"] = (
            item["combined_gae_ratio"] < 0.9
            and all(p["gae_ratio"] < 0.9 and p["wall_ratio"] < 1 and p["cpu_ratio"] < 1 for p in pairs)
        )
    else:
        item["robust_component_lead"] = False
    f4.append(args.store, item)
    print(json.dumps(item, sort_keys=True))


if __name__ == "__main__":
    main()
