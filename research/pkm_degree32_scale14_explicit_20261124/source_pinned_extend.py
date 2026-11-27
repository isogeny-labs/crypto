#!/usr/bin/env python3
"""Extend the explicit EFD roster deterministically until three size matches."""
import argparse
import hashlib
import json
import sys
from pathlib import Path

from sympy import isprime

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent / "pkm_adaptive_sweep_20261123"))
import f4_height4 as f4
import run as old

EXPECTED = "c98e290c7c7d5aee30ed70e1c9aa19726e830854b09ae15921f02b09c648de2d"


def roster(limit):
    out = []
    for p in range(2**14 - 1, 2, -2):
        if not isprime(p):
            continue
        # Store the least nonnegative square root for each nonzero residue.
        roots = {}
        for y in range(1, p):
            roots.setdefault((y*y) % p, y)
        squares = set(roots)
        nonsquares = set(range(1, p)) - squares
        found = None
        for a in range(1, 30):
            for b in range(1, 80):
                if (4*a*a*a + 27*b*b) % p == 0:
                    continue
                order = p + 1
                generator = None
                for x in range(p):
                    rhs = (x*x*x + a*x + b) % p
                    if rhs in squares:
                        order += 1
                        if rhs and generator is None:
                            generator = (x, roots[rhs])
                    elif rhs in nonsquares:
                        order -= 1
                if isprime(order):
                    found = {"index": len(out)+1, "p": p, "a": a, "b": b,
                             "r": order, "x": generator[0], "y": generator[1],
                             "z": 1, "seed": 20262000+len(out)+1}
                    break
            if found:
                break
        if found:
            out.append(found)
            if len(out) >= limit:
                return out
    return out


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bin", type=Path, required=True)
    parser.add_argument("--store", type=Path, default=HERE / "source_pinned_explicit14_20261125.jsonl")
    args = parser.parse_args()
    digest = hashlib.sha256(args.bin.read_bytes()).hexdigest()
    if digest != EXPECTED:
        raise ValueError("binary hash mismatch")
    rows = old.load(args.store)
    if rows["header"]["binary_sha256"] != digest:
        raise ValueError("store binary mismatch")
    if "amended_selection" in rows:
        print(json.dumps(rows["amended_selection"], sort_keys=True))
        return
    curves = roster(64)
    frozen = [{k: c[k] for k in ("index", "p", "a", "b", "r", "x", "y", "z", "seed")}
              for c in old.CURVES]
    if curves[:16] != frozen:
        raise ValueError("deterministic enumeration does not reproduce frozen 16-curve roster")
    selected = list(rows["selection"]["all_equal"])
    for curve in curves[16:]:
        if len(selected) >= 3:
            break
        for arm, fb in (("baseline", old.BASE), ("candidate", old.CAND)):
            key = f"preflight:{curve['index']}:{arm}"
            if key not in rows:
                item = {"id": key, "curve": curve, "arm": arm,
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
        item = {"id": f"size-check:{curve['index']}", "curve": curve,
                "sizes": sizes, "qualifies": bool(valid)}
        f4.append(args.store, item)
        if valid:
            selected.append({"curve_index": curve["index"], "instance": a["result"]["instance"],
                             "base_size": sizes[0]})
    screened = [int(key.split(":")[1]) for key in rows if key.startswith("size-check:")]
    item = {"id": "amended_selection", "first_three": selected[:3],
            "qualifying_count": len(selected), "complete": len(selected) >= 3,
            "last_screened_index": max(screened, default=16)}
    f4.append(args.store, item)
    print(json.dumps(item, sort_keys=True))


if __name__ == "__main__":
    main()
