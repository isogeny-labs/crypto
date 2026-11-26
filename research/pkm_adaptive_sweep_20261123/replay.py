#!/usr/bin/env python3
"""Independently enumerate ordered signed-point pairs for one sweep target."""
import argparse
import importlib.util
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("pkm_relations", HERE.parent / "pkm_map_sweep_20261123" / "sweep.py")
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)


def add_direct(u, v):
    if u is None:
        return v
    if v is None:
        return u
    x, y = u
    z, w = v
    if x == z and (y + w) % m.P == 0:
        return None
    den = (z - x) % m.P if x != z else (2 * y) % m.P
    if not den:
        return None
    num = (w - y) % m.P if x != z else (3 * x * x + m.A) % m.P
    lam = num * pow(den, -1, m.P) % m.P
    out_x = (lam * lam - x - z) % m.P
    return (out_x, (lam * (x - out_x) - y) % m.P)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--source", type=Path, required=True)
    ap.add_argument("--store", type=Path, required=True)
    ap.add_argument("--seed", type=int, required=True)
    ap.add_argument("--target-index", type=int, required=True)
    args = ap.parse_args()
    rows = [json.loads(line) for line in args.store.read_text().splitlines()]
    row = next(r for r in rows if r["id"] == f"seed:{args.seed}")
    raw = args.source.read_bytes()
    import hashlib
    assert hashlib.sha256(raw).hexdigest() == row["source_sha256"]
    t = json.loads(raw)
    t = t.get("tower", t)
    assert all(m.map_value(t["steps"], x) == t["c"] for x in t["V"])
    base = [x for x in t["V"] if m.lift(x) is not None]
    points = []
    for x in base:
        y = m.lift(x)
        points.extend([(x, y), (x, -y % m.P)] if y else [(x, 0)])
    target = m.targets()[args.target_index]
    count = sum(add_direct(u, v) == target for u in points for v in points)
    assert count == row["ordered_counts"][args.target_index]
    print(json.dumps({"seed": args.seed, "target_index": args.target_index,
                      "target": target, "ordered_pairs": count,
                      "source_sha256": row["source_sha256"], "matched": True}, sort_keys=True))


if __name__ == "__main__":
    main()
