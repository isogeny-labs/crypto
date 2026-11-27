#!/usr/bin/env python3
"""Resume the fresh-target seed-14 PKM F4 replication panel."""
import argparse
import hashlib
import json
import random
import statistics
import time
from pathlib import Path

import f4_height4 as f4

HERE = Path(__file__).resolve().parent
TARGETS = range(20261241, 20261369)
AUX = 1


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--bin", type=Path, required=True)
    ap.add_argument("--store", type=Path, default=HERE / "replication_height4_seed1_corrected.jsonl")
    ap.add_argument("--max-wall-seconds", type=float, default=120)
    args = ap.parse_args()
    rows = {}
    if args.store.exists():
        for line in args.store.read_text().splitlines():
            item = json.loads(line)
            if item["id"] in rows:
                raise ValueError(f"duplicate {item['id']}")
            rows[item["id"]] = item
    binary_hash = hashlib.sha256(args.bin.read_bytes()).hexdigest()
    if "header" not in rows:
        item = {"id": "header", "schema": "pkm.f4-height4-replication/v1", "binary_sha256": binary_hash,
                "aux_seed": AUX, "target_seeds": list(TARGETS),
                "protocol": "PROTOCOL_F4_HEIGHT4_REPLICATION_20261123.md"}
        f4.append(args.store, item); rows["header"] = item
    if rows["header"]["binary_sha256"] != binary_hash:
        raise ValueError("binary changed; use new store")
    if "verdict" in rows:
        print(json.dumps(rows["verdict"], sort_keys=True)); return
    deadline = time.monotonic() + args.max_wall_seconds
    for target in TARGETS:
        for arm in ("baseline", "candidate"):
            key = f"{arm}:{target}"
            if key in rows:
                continue
            if deadline - time.monotonic() < 10:
                return
            fb = f4.INTERVAL if arm == "baseline" else f"prime-isogeny:t=4,seed={AUX},aux=ordinary"
            oracle = f4.INTERVAL_ORACLE if arm == "baseline" else f4.ISOGENY_ORACLE
            item = {"id": key, "arm": arm, "target_seed": target,
                    **f4.run(f4.command(args.bin, fb, oracle, target), 10)}
            f4.append(args.store, item); rows[key] = item
            actual_size = f4.row(item.get("result")).get("factor_base", {}).get("abscissae")
            if actual_size != 6:
                raise RuntimeError(f"{key}: expected six lifting abscissae, got {actual_size}")
    pairs = [(rows[f"baseline:{s}"], rows[f"candidate:{s}"]) for s in TARGETS]
    complete = all(f4.verified_pair(a, b) for a, b in pairs)
    item = {"id": "verdict", "complete": complete, "n_pairs": len(pairs)}
    if complete:
        baseline = [f4.row(a["result"])["total_gae"] for a, _ in pairs]
        candidate = [f4.row(b["result"])["total_gae"] for _, b in pairs]
        ratio = sum(candidate) / sum(baseline)
        rng = random.Random(20261123)
        draws = []
        for _ in range(10000):
            indices = [rng.randrange(len(pairs)) for __ in pairs]
            draws.append(sum(candidate[i] for i in indices) / sum(baseline[i] for i in indices))
        draws.sort()
        wall_ratio = sum(b["wall_s"] for _, b in pairs) / sum(a["wall_s"] for a, _ in pairs)
        cpu_ratio = sum(b["child_cpu_s"] for _, b in pairs) / sum(a["child_cpu_s"] for a, _ in pairs)
        individual = [c / b for b, c in zip(baseline, candidate)]
        item.update({"ratio_of_sums": ratio, "bootstrap_99_percentile": [draws[50], draws[9950]],
                     "wall_ratio_of_sums": wall_ratio, "cpu_ratio_of_sums": cpu_ratio,
                     "paired_wins": sum(x < 1 for x in individual),
                     "paired_losses": sum(x > 1 for x in individual),
                     "median_pair_ratio": statistics.median(individual),
                     "min_pair_ratio": min(individual), "max_pair_ratio": max(individual),
                     "component_lead": ratio < .9 and draws[9950] < 1 and wall_ratio < 1 and cpu_ratio < 1})
    else:
        item["component_lead"] = False
    f4.append(args.store, item)


if __name__ == "__main__":
    main()
