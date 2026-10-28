#!/usr/bin/env python3
"""Run one (bits, curve) sweep job and append its rows.

`ic sweep --curves N` always starts at curve 0, so re-running a study to fill
in a missing curve repeats the curves already done.  This calls the same job
body the CLI uses, for one curve index only.

    python3 run_curve.py --bits 32 --curve 2 --m 5 6 7 8 9 \
        --out prime/sweep-mitm-32.jsonl [--engine mitm]

Run it with crypto_autoresearcher importable (from the repository root).
Rows have the same schema as `ic sweep --out`.
"""
import argparse
import json
import time

from crypto_autoresearcher.index_calculus.__main__ import _sweep_core


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--bits", type=int, required=True)
    ap.add_argument("--curve", type=int, required=True)
    ap.add_argument("--m", type=int, nargs="+", required=True)
    ap.add_argument("--engine", default="mitm", choices=["enumerate", "mitm"])
    ap.add_argument("--fb", default="small_x")
    ap.add_argument("--la-pivot", default="min_fill")
    ap.add_argument("--out", required=True)
    args = ap.parse_args()

    job = {
        "bits": args.bits, "curve": args.curve, "ms": args.m, "ic_ms": args.m,
        "fbs": [args.fb], "rho": True, "subgroup": False, "tolerance": 0.15,
        "engines": [args.engine], "la_pivot": args.la_pivot,
        "engine_caps": {}, "accelerate": None, "harvest": "off",
    }
    t0 = time.perf_counter()
    rows, _ = _sweep_core(job)
    with open(args.out, "a") as fh:
        for r in rows:
            fh.write(json.dumps(r) + "\n")
    print(f"bits={args.bits} curve={args.curve} rows={len(rows)} "
          f"ok={sum(r['ok'] for r in rows)} seconds={time.perf_counter()-t0:.1f}")
    return 0 if all(r["ok"] for r in rows) else 1


if __name__ == "__main__":
    raise SystemExit(main())
