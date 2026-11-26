#!/usr/bin/env python3
"""Resume the predeclared 11-bit degree-16 PKM F4 diagnostic."""
import argparse
import hashlib
import json
import random
import time
from pathlib import Path

import f4_sweeper as f4

HERE = Path(__file__).resolve().parent
SCALARS = random.Random(20261123).sample(range(1, 1877), 32)
SEEDS = [x - 1 for x in SCALARS]


def command(binary, arm, seed):
    fb = "prime-abscissa:below=16" if arm == "baseline" else "prime-isogeny:t=4,seed=10,aux=ordinary"
    oracle = f4.INTERVAL_ORACLE.replace("bits=3", "bits=4") if arm == "baseline" else f4.ISOGENY_ORACLE
    return [str(binary), "bench", "--bits", "11", "--family", "generic",
            "--factor-base", fb, "--oracle", oracle, "--max-trials", "1000",
            "--rho-runs", "1", "--repeats", "1", "--seed", str(seed),
            "--targets", "walk", "--solver-budget-seconds", "5", "--json"]


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--bin", type=Path, required=True)
    ap.add_argument("--store", type=Path, default=HERE / "scale11.jsonl")
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
        item = {"id": "header", "schema": "pkm.f4-scale11/v1", "binary_sha256": binary_hash,
                "protocol": "PROTOCOL_F4_SCALE11_20261123.md", "scalars": SCALARS,
                "target_seeds": SEEDS, "instance": "icv1-fp11-t11-613e5f3a",
                "base_size": 10, "aux_seed": 10}
        f4.append(args.store, item); rows["header"] = item
    if rows["header"]["binary_sha256"] != binary_hash or rows["header"]["scalars"] != SCALARS:
        raise ValueError("source or scalar list changed")
    if "verdict" in rows:
        print(json.dumps(rows["verdict"], sort_keys=True)); return
    deadline = time.monotonic() + args.max_wall_seconds
    for seed in SEEDS:
        for arm in ("baseline", "candidate"):
            key = f"{arm}:{seed}"
            if key in rows:
                continue
            if deadline - time.monotonic() < 10:
                return
            item = {"id": key, "arm": arm, "target_seed": seed,
                    **f4.run(command(args.bin, arm, seed), 10)}
            f4.append(args.store, item); rows[key] = item
            size = f4.row(item.get("result")).get("factor_base", {}).get("abscissae")
            if size != 10:
                raise RuntimeError(f"{key}: expected ten abscissae, got {size}")
    pairs = [(rows[f"baseline:{s}"], rows[f"candidate:{s}"]) for s in SEEDS]
    complete = all(a["exit_code"] == b["exit_code"] == 0 and
                   f4.row(a["result"]).get("verified") and f4.row(b["result"]).get("verified") and
                   f4.row(a["result"]).get("recovered") == f4.row(b["result"]).get("recovered") == scalar and
                   a["result"].get("instance") == b["result"].get("instance") == "icv1-fp11-t11-613e5f3a"
                   for (a, b), scalar in zip(pairs, SCALARS))
    item = {"id": "verdict", "complete": complete, "n_pairs": len(pairs)}
    if complete:
        a = [f4.row(x["result"]) for x, _ in pairs]
        b = [f4.row(y["result"]) for _, y in pairs]
        item.update({"gae_ratio": sum(x["total_gae"] for x in b) / sum(x["total_gae"] for x in a),
                     "wall_ratio": sum(y["wall_s"] for _, y in pairs) / sum(x["wall_s"] for x, _ in pairs),
                     "cpu_ratio": sum(y["child_cpu_s"] for _, y in pairs) / sum(x["child_cpu_s"] for x, _ in pairs),
                     "trials": [sum(x["decomposition"]["targets_tried"] for x in a), sum(x["decomposition"]["targets_tried"] for x in b)],
                     "f4_ops": [sum(x["decomposition"]["solver"]["ops"] for x in a), sum(x["decomposition"]["solver"]["ops"] for x in b)]})
    f4.append(args.store, item)


if __name__ == "__main__":
    main()
