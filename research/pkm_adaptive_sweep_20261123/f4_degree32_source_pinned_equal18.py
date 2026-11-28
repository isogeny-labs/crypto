#!/usr/bin/env python3
"""Source-pinned degree-32 seed-5 vs equal-18 interval, one selected 13-bit instance."""
import argparse
import hashlib
import json
import statistics
from pathlib import Path

import f4_height4 as f4

HERE = Path(__file__).resolve().parent
EXPECTED = "c98e290c7c7d5aee30ed70e1c9aa19726e830854b09ae15921f02b09c648de2d"
SEED = 20261185
BITS = 13
REPEATS = 16
BASE_SIZE = 18
INTERVAL = "prime-abscissa:below=32"
INTERVAL_ORACLE = "pkm-semaev:m=2,engine=f4,degree=8,bits=5,lo=0,mode=chain"
CANDIDATE = "prime-isogeny:t=5,seed=5,aux=ordinary"
CANDIDATE_ORACLE = "pkm-semaev:m=2,engine=f4,degree=8,mode=chain"
PROTOCOL = "PROTOCOL_F4_DEGREE32_SOURCE_PINNED_EQUAL18_20261125.md"


def command(binary, seed, fb, oracle, preflight):
    cmd = [str(binary), "bench", "--bits", str(BITS), "--family", "generic",
           "--seed", str(seed), "--repeats", str(REPEATS), "--targets", "walk",
           "--factor-base", fb, "--oracle", "subtract" if preflight else oracle,
           "--max-trials", "1" if preflight else "1000",
           "--rho-runs", "0" if preflight else "1", "--json"]
    if not preflight:
        cmd += ["--solver-budget-seconds", "5"]
    return cmd


def size(item):
    rows = (item.get("result") or {}).get("rows") or []
    values = {r.get("factor_base", {}).get("abscissae") for r in rows}
    return values.pop() if len(rows) == REPEATS and len(values) == 1 else None


def median(rows, key):
    vals = [r.get(key) for r in rows if isinstance(r.get(key), (int, float))]
    return statistics.median(vals) if vals else None


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--bin", type=Path, required=True)
    ap.add_argument("--seed", type=int, default=SEED)
    ap.add_argument("--base-size", type=int, default=BASE_SIZE)
    ap.add_argument("--store", type=Path, default=HERE / "degree32_source_pinned_equal18.jsonl")
    args = ap.parse_args()

    digest = hashlib.sha256(args.bin.read_bytes()).hexdigest()
    if digest != EXPECTED:
        raise ValueError(f"binary differs from protocol: {digest}")

    records = {}
    if args.store.exists():
        for line in args.store.read_text().splitlines():
            item = json.loads(line)
            if item["id"] in records:
                raise ValueError(f"duplicate {item['id']}")
            records[item["id"]] = item
    if "header" not in records:
        item = {"id": "header", "schema": "pkm.f4-degree32-source-pinned/v1",
                "binary_sha256": digest, "protocol": PROTOCOL, "seed": args.seed,
                "repeats": REPEATS, "expected_base_size": args.base_size}
        f4.append(args.store, item)
        records["header"] = item
    if "verdict" in records:
        print(json.dumps(records["verdict"], sort_keys=True))
        return

    for key, fb in (("preflight:baseline", INTERVAL), ("preflight:candidate", CANDIDATE)):
        if key not in records:
            item = {"id": key, "arm": key.split(":")[1],
                    **f4.run(command(args.bin, args.seed, fb, "", True), 30)}
            f4.append(args.store, item)
            records[key] = item

    a = records["preflight:baseline"]
    b = records["preflight:candidate"]
    if a["exit_code"] not in (0, 1) or b["exit_code"] not in (0, 1):
        fail = {"id": "verdict", "complete": False, "reason": "preflight-failed"}
        f4.append(args.store, fail)
        print(json.dumps(fail, sort_keys=True))
        return
    if a["result"].get("instance") != b["result"].get("instance") or size(a) != size(b) or size(a) != args.base_size:
        fail = {"id": "verdict", "complete": False, "reason": "size-mismatch",
                "instance": a["result"].get("instance"), "sizes": [size(a), size(b)]}
        f4.append(args.store, fail)
        print(json.dumps(fail, sort_keys=True))
        return

    order = ("baseline:1", "candidate:1", "candidate:2", "baseline:2")
    for arm_rep in order:
        key = f"timed:{arm_rep}"
        if key in records:
            continue
        arm = arm_rep.split(":")[0]
        fb = INTERVAL if arm == "baseline" else CANDIDATE
        oracle = INTERVAL_ORACLE if arm == "baseline" else CANDIDATE_ORACLE
        item = {"id": key, "arm": arm, "seed": args.seed,
                **f4.run(command(args.bin, args.seed, fb, oracle, False), 240)}
        f4.append(args.store, item)
        records[key] = item

    runs = [records[f"timed:{x}"] for x in order]
    rowsets = [(x.get("result") or {}).get("rows") or [] for x in runs]
    complete = (
        all(x["exit_code"] == 0 and x["result"].get("instance") == a["result"].get("instance")
            and size(x) == args.base_size for x in runs)
        and all(len(rs) == REPEATS for rs in rowsets)
        and all(all(r.get("verified") and r.get("recovered") is not None for r in rs) for rs in rowsets)
        and all(len({rowsets[j][i]["recovered"] for j in range(4)}) == 1 for i in range(REPEATS))
    )
    item = {"id": "verdict", "complete": complete, "seed": args.seed,
            "instance": a["result"].get("instance"), "base_size": args.base_size}
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
