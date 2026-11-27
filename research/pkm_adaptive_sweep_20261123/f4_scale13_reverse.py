#!/usr/bin/env python3
"""Reverse-order timing check for the adaptive 13-bit seed-10 map."""
import argparse
import hashlib
import json
from pathlib import Path

import f4_height4 as f4
import f4_scale13_alt as alt

HERE = Path(__file__).resolve().parent


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--bin", type=Path, required=True)
    ap.add_argument("--store", type=Path, default=HERE / "scale13_reverse.jsonl")
    args = ap.parse_args()
    digest = hashlib.sha256(args.bin.read_bytes()).hexdigest()
    if digest != alt.EXPECTED_BINARY:
        raise ValueError("binary differs from frozen protocol")
    rows = {}
    if args.store.exists():
        for line in args.store.read_text().splitlines():
            item = json.loads(line)
            if item["id"] in rows:
                raise ValueError(f"duplicate {item['id']}")
            rows[item["id"]] = item
    if "header" not in rows:
        item = {"id": "header", "schema": "pkm.f4-scale13-reverse/v1", "binary_sha256": digest,
                "protocol": "PROTOCOL_F4_SCALE13_REVERSE_20261123.md", "aux_seed": 10}
        f4.append(args.store, item)
        rows["header"] = item
    if rows["header"]["binary_sha256"] != digest:
        raise ValueError("binary changed")
    if "verdict" in rows:
        print(json.dumps(rows["verdict"], sort_keys=True))
        return
    for arm in ("candidate", "baseline"):
        key = f"timed:{arm}"
        if key not in rows:
            fb = "prime-isogeny:t=4,seed=10,aux=ordinary" if arm == "candidate" else f4.INTERVAL
            oracle = f4.ISOGENY_ORACLE if arm == "candidate" else f4.INTERVAL_ORACLE
            item = {"id": key, "arm": arm, **f4.run(alt.command(args.bin, fb, oracle, False), 180)}
            f4.append(args.store, item)
            rows[key] = item
    a, b = rows["timed:baseline"], rows["timed:candidate"]
    ar, br = (a.get("result") or {}).get("rows") or [], (b.get("result") or {}).get("rows") or []
    complete = (a["exit_code"] == b["exit_code"] == 0 and len(ar) == len(br) == 32 and
                a["result"].get("instance") == b["result"].get("instance") == "icv1-fp13-t113-d65ed37b" and
                alt.base_size(a) == alt.base_size(b) == 9 and
                all(x.get("verified") and y.get("verified") and x.get("recovered") is not None and
                    x.get("recovered") == y.get("recovered") for x, y in zip(ar, br)))
    item = {"id": "verdict", "complete": complete}
    if complete:
        ag, bg = [r["total_gae"] for r in ar], [r["total_gae"] for r in br]
        item.update({"gae_ratio": sum(bg) / sum(ag), "wall_ratio": b["wall_s"] / a["wall_s"],
                     "cpu_ratio": b["child_cpu_s"] / a["child_cpu_s"],
                     "totals_gae": [sum(ag), sum(bg)],
                     "raw_relations": [sum(r["decomposition"]["relations_found"] for r in arm) for arm in (ar, br)],
                     "target_trials": [sum(r["decomposition"]["targets_tried"] for r in arm) for arm in (ar, br)]})
    f4.append(args.store, item)
    print(json.dumps(item, sort_keys=True))


if __name__ == "__main__":
    main()
