#!/usr/bin/env python3
"""Regenerate the prime factor bases a sweep used, and save them.

The sweep rows store only the curve, the arity and |F|.  The bases themselves
are deterministic: `small_x` is the first |F| x-coordinates that lift, with one
point per +/- pair, and its seed is ignored.  This script rebuilds each base
from the same instance generator the sweep used, checks the curve fields and
|F| against the recorded row, and writes one JSON file per (bits, curve, m).

    python3 dump_factor_bases.py \
        --rows prime/sweep.jsonl prime/sweep-mitm.jsonl prime/sweep-mitm-32.jsonl \
        --out /Volumes/data/ic-arity-sweep-20261027

`--out` is the data root: bases go to OUT/factor-bases/, the manifest to
OUT/manifest.json.  Run it with crypto_autoresearcher importable (from the
repository root, or with PYTHONPATH set).
"""
import argparse
import hashlib
import json
import os
import time

from crypto_autoresearcher.index_calculus.__main__ import _instance
from crypto_autoresearcher.index_calculus.factor_base import build_factor_base, default_fb_size


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--rows", nargs="+", required=True, help="sweep .jsonl files")
    ap.add_argument("--out", required=True, help="data root for factor bases")
    ap.add_argument("--kind", default="small_x")
    args = ap.parse_args()

    rows = []
    for path in args.rows:
        with open(path) as fh:
            rows.extend(json.loads(line) for line in fh if line.strip())

    recorded, curve_fields = {}, {}
    for r in rows:
        if r["method"] == "rho":
            curve_fields[(r["bits"], r["curve"])] = (r["p"], r["a"], r["b"], r["N"])
            continue
        m = int(r["method"].split("m")[1])
        key = (r["bits"], r["curve"], m)
        if key in recorded and recorded[key] != r["fb_size"]:
            raise SystemExit(f"conflicting |F| for {key}: {recorded[key]} vs {r['fb_size']}")
        recorded[key] = r["fb_size"]

    fudir = os.path.join(args.out, "factor-bases")
    os.makedirs(fudir, exist_ok=True)
    entries = []
    t0 = time.perf_counter()
    for (bits, curve, m), want in sorted(recorded.items()):
        E, P, _, _ = _instance(bits, curve, 0, None)
        got = (E.p, E.a, E.b, E.order)
        if (bits, curve) in curve_fields and got != curve_fields[(bits, curve)]:
            raise SystemExit(f"curve mismatch at {bits}/{curve}: {got} != {curve_fields[(bits, curve)]}")
        size = default_fb_size(E.order, m)
        fb = build_factor_base(E, args.kind, size, curve)
        if len(fb) != want:
            raise SystemExit(f"|F| mismatch at {bits}/{curve}/m{m}: {len(fb)} != {want}")
        doc = {
            "schema": "prime-ic-factor-base/1",
            "bits": bits, "curve": curve, "m": m, "kind": args.kind,
            "p": E.p, "a": E.a, "b": E.b, "N": E.order,
            "generator": [int(P[0]), int(P[1])],
            "requested_size": size, "size": len(fb), "seed": curve,
            "points": [[int(Q[0]), int(Q[1])] for Q in fb.points],
        }
        name = f"n{bits}-c{curve}-m{m}.json"
        path = os.path.join(fudir, name)
        tmp = path + ".tmp"
        with open(tmp, "w") as fh:
            json.dump(doc, fh, separators=(",", ":"))
        os.replace(tmp, path)
        with open(path, "rb") as fh:
            digest = hashlib.sha256(fh.read()).hexdigest()
        entries.append({"file": f"factor-bases/{name}", "sha256": digest,
                        "bytes": os.path.getsize(path), "bits": bits, "curve": curve,
                        "m": m, "size": len(fb)})

    manifest = {
        "schema": "prime-ic-factor-base-manifest/1",
        "source_tool": "crypto_autoresearcher.index_calculus sweep",
        "curve_model": "generate_prime_order_curve(bits, curve_seed, p_filter=None)",
        "kind": args.kind,
        "arities": sorted({m for (_, _, m) in recorded}),
        "bits": sorted({b for (b, _, _) in recorded}),
        "count": len(entries),
        "total_bytes": sum(e["bytes"] for e in entries),
        "elapsed_seconds": round(time.perf_counter() - t0, 3),
        "entries": entries,
    }
    with open(os.path.join(args.out, "manifest.json"), "w") as fh:
        json.dump(manifest, fh, indent=1)
    print(f"saved {len(entries)} factor bases ({manifest['total_bytes']} bytes) under {fudir}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
