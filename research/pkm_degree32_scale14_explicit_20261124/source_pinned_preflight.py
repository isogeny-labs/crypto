#!/usr/bin/env python3
"""Screen the frozen explicit EFD roster with the retained source-pinned ic."""
import argparse
import hashlib
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent / "pkm_adaptive_sweep_20261123"))
import f4_height4 as f4
import run as old

EXPECTED = "c98e290c7c7d5aee30ed70e1c9aa19726e830854b09ae15921f02b09c648de2d"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bin", type=Path, required=True)
    parser.add_argument("--store", type=Path, default=HERE / "source_pinned_explicit14_20261125.jsonl")
    args = parser.parse_args()
    digest = hashlib.sha256(args.bin.read_bytes()).hexdigest()
    if digest != EXPECTED:
        raise ValueError("binary hash mismatch")
    rows = old.load(args.store)
    if "header" not in rows:
        item = {"id": "header", "schema": "pkm.explicit14-source-pinned/v1",
                "binary_sha256": digest,
                "source_commit": "a5503f4bb66ac3aa4eac3125fba01534d2574096",
                "protocol": "PROTOCOL_SOURCE_PINNED_20261125.md", "curves": old.CURVES}
        f4.append(args.store, item)
        rows["header"] = item
    if rows["header"]["binary_sha256"] != digest:
        raise ValueError("store binary mismatch")
    if "selection" in rows:
        print(json.dumps(rows["selection"], sort_keys=True))
        return
    selected = []
    for curve in old.CURVES:
        for arm, fb in (("baseline", old.BASE), ("candidate", old.CAND)):
            key = f"preflight:{curve['index']}:{arm}"
            if key not in rows:
                item = {"id": key, "curve_index": curve["index"], "arm": arm,
                        **f4.run(old.cmd(args.bin, curve, fb, "", True), 30)}
                f4.append(args.store, item)
                rows[key] = item
        a = rows[f"preflight:{curve['index']}:baseline"]
        b = rows[f"preflight:{curve['index']}:candidate"]
        sizes = [old.size(a), old.size(b)]
        valid = (all(x["exit_code"] in (0, 1) and x.get("result") for x in (a, b))
                 and a["result"].get("instance") == b["result"].get("instance")
                 and sizes[0] is not None and sizes[0] == sizes[1]
                 and all(len(x["result"]["rows"]) == old.REPEATS for x in (a, b))
                 and all(all(r["r"] == curve["r"] for r in x["result"]["rows"]) for x in (a, b)))
        if valid:
            selected.append({"curve_index": curve["index"], "instance": a["result"]["instance"],
                             "base_size": sizes[0]})
    item = {"id": "selection", "first_three": selected[:3], "all_equal": selected,
            "qualifying_count": len(selected)}
    f4.append(args.store, item)
    print(json.dumps(item, sort_keys=True))


if __name__ == "__main__":
    main()
