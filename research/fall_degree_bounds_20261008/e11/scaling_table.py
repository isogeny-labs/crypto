#!/usr/bin/env python3
"""Scaling table from e11/scaling/{window,dense}_*.jsonl (measured), plus the
agreement checks on the fresh draws: degree-2/3 dims equal, window certifies
=> dense d_F <= 4."""
import glob
import json
import statistics as st
import sys

S = "research/fall_degree_bounds_20261008/e11/scaling"
out = []
bad = 0
for wf in sorted(glob.glob(f"{S}/window_*.jsonl")):
    tag = wf.split("window_")[1][:-6]
    W = [json.loads(x) for x in open(wf) if x.startswith("{")]
    D = {}
    df = f"{S}/dense_{tag}.jsonl"
    try:
        for x in open(df):
            if x.startswith("{"):
                r = json.loads(x)
                D[r["draw"]] = r
    except FileNotFoundError:
        pass
    rows = []
    for r in W:
        w = r["window"].get("w1x", {})
        d = D.get(r["draw"])
        if d:
            for k, v in r["per_degree"].items():
                if d["per_degree"][k]["dim"] != v["dim"]:
                    bad += 1
                    print("MISMATCH", tag, r["draw"], k, file=sys.stderr)
            if w.get("certified") and not (d["d_f"] is not None and d["d_f"] <= 4):
                bad += 1
                print("VIOLATION", tag, r["draw"], file=sys.stderr)
        rows.append((r, w, d))
    r0 = W[0]
    ws = [w for _, w, _ in rows if w]
    ds = [d for _, _, d in rows if d]
    out.append({
        "cell": f"{r0['m']}:{r0['n']}:{r0['ell']}", "N": r0["n_vars"], "draws": len(W),
        "d_f_le3": sum(1 for r, _, _ in rows if r["d_f_le3"]),
        "window_certified": sum(1 for w in ws if w.get("certified")),
        "dense_d_f": [d["d_f"] for d in ds],
        "window_cols": ws[0]["cols"] if ws else None,
        "dense_cols": ds[0]["per_degree"]["4"]["of"] if ds and "4" in ds[0]["per_degree"] else None,
        "window_secs_median": st.median(w["secs"] for w in ws) if ws else None,
        "window_secs_max": max(w["secs"] for w in ws) if ws else None,
        "dense_deg4_secs_median": st.median(d["per_degree"]["4"]["secs"] for d in ds if "4" in d["per_degree"]) if ds else None,
        "window_peak_rss_mb": max(r["vmhwm_kb"] for r, _, _ in rows) / 1024,
        "dense_peak_rss_mb": max(d["maxrss_kb"] for d in ds) / 1024 if ds else None,
        "window_basis_mb_median": st.median(w["basis_bytes"] for w in ws) / 1e6 if ws else None,
        "count_secs_median": st.median(r["count_secs"] for r, _, _ in rows),
        "counter": r0["counter"],
    })
print(json.dumps(out, indent=1))
print("agreement problems:", bad, file=sys.stderr)
