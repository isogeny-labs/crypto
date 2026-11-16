"""Analyse cells.jsonl per PRESET.txt.  [pcc:unverified]"""
import json, os, sys, math, statistics
from collections import defaultdict
import numpy as np
HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "..", "out")
rows = [json.loads(l) for l in open(os.path.join(OUT, "cells.jsonl"))]
seen = {}
for r in rows: seen[r["tag"]] = r          # last write wins (no duplicates expected)
rows = list(seen.values())
PROX = ["sum_rows", "rows_reduced", "sum_nnz", "ms_time"]

def svar(r):
    return 2 ** r["size"] if r["family"] == "bits" else r["size"]

bad = [r["tag"] + ":" + r["ms"]["status"] for r in rows if r["ms"]["status"] != "ok"]
groups = defaultdict(lambda: defaultdict(list))
for r in rows:
    if r["ms"]["status"] != "ok": continue
    groups[(r["family"], r["m"])][svar(r)].append(r)

lines = []
P = lines.append
P("# Per-cell table (medians over seeds and primes)\n")
P("| family | m | s | cells | |B_x| med | D_F4 | D_ff | D_XL | D_XL-D_F4 | D_F4-s | max mat | sum_rows | rows_reduced | sum_nnz | time s | XL cols | ThmB rank bnd | V_R>0 |")
P("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
summary = {}
for (fam, m) in sorted(groups):
    g = groups[(fam, m)]
    fit_pts = []
    for s in sorted(g):
        cs = g[s]
        med = lambda f: statistics.median([f(c) for c in cs if f(c) is not None]) if any(f(c) is not None for c in cs) else None
        dF4 = med(lambda c: c["ms"].get("D_F4"))
        dff = med(lambda c: (c.get("xl") or {}).get("D_ff"))
        dxl = med(lambda c: (c.get("xl") or {}).get("D_XL"))
        gap = med(lambda c: (c["xl"]["D_XL"] - c["ms"]["D_F4"]) if c.get("xl", {}) and c["xl"].get("D_XL") else None)
        mm = max(cs, key=lambda c: c["ms"].get("max_rc", 0))["ms"]
        vals = {pr: med(lambda c, pr=pr: c["ms"].get(pr)) for pr in PROX}
        xlc = med(lambda c: (c.get("xl") or {}).get("XL_cols"))
        tb = med(lambda c: (c.get("xl") or {}).get("thmB_rank_bound"))
        vr = sum(1 for c in cs if (c.get("V_R_grid") or 0) > 0)
        bx = med(lambda c: c["Bx"])
        P(f"| {fam} | {m} | {s} | {len(cs)} | {bx} | {dF4} | {dff} | {dxl} | {gap} | {None if dF4 is None else dF4 - (s if fam!='bits' else 0)} | {mm.get('max_rows')}x{mm.get('max_cols')} | {vals['sum_rows']} | {vals['rows_reduced']} | {vals['sum_nnz']} | {vals['ms_time']} | {xlc} | {tb} | {vr}/{len(cs)} |")
        fit_pts.append((s, vals, xlc, tb, dF4, dxl, dff))
    # fits over the largest 4 sizes
    pts = fit_pts[-4:]
    ks = {}
    for pr in PROX:
        xs = [math.log(s) for s, v, *_ in pts if v[pr] and v[pr] > 0]
        ys = [math.log(v[pr]) for s, v, *_ in pts if v[pr] and v[pr] > 0]
        if len(xs) >= 3 and (pr != "ms_time" or min(v["ms_time"] for _, v, *_ in pts) >= 0.05):
            ks[pr] = float(np.polyfit(xs, ys, 1)[0])
    xs = [math.log(s) for s, v, x, *_ in pts if x]
    kxl = float(np.polyfit(xs, [math.log(x) for s, v, x, *_ in pts if x], 1)[0]) if len(xs) >= 3 else None
    # degree slopes over all sizes
    dd = [(s, d) for s, v, x, t, d, *_ in fit_pts if d is not None]
    slope_D = float(np.polyfit([a for a, _ in dd], [b for _, b in dd], 1)[0]) if len(dd) >= 3 else None
    dx = [(s, d) for s, v, x, t, _, d, _ in fit_pts if d is not None]
    slope_XL = float(np.polyfit([a for a, _ in dx], [b for _, b in dx], 1)[0]) if len(dx) >= 3 else None
    k = min(ks.values()) if ks else None
    if k is None:
        e = None; verdict = "NO FIT"
    else:
        e = 1.0 if k >= m - 1 else max((1 + k) / m, 2 / m)
        verdict = "SURVIVES" if e >= 0.55 else ("INCONCLUSIVE" if e >= 0.45 else "FAILS")
    summary[f"{fam}_m{m}"] = {"family": fam, "m": m, "sizes": [s for s, *_ in fit_pts], "fit_sizes": [s for s, *_ in pts],
        "k": ks, "k_min": k, "k_XLcols": kxl, "dD_F4/ds": slope_D, "dD_XL/ds": slope_XL,
        "thmB_exp1_transfers": (k is not None and k >= m - 1 - 0.1), "e_F4": e, "verdict": verdict,
        # post-preset additions (not part of the frozen rule): LA-free exponent per proxy, min(1,(1+k)/m)
        "e_LAfree": {pr: min(1.0, (1 + kv) / m) for pr, kv in ks.items()}}

P("\n# Fits (largest 4 sizes) and verdicts per PRESET.txt\n")
P("| route | fit sizes | k sum_rows | k rows_reduced | k sum_nnz | k time | k XL cols | dD_F4/ds | dD_XL/ds | k_min | Thm B exp-1 transfers (k>=m-1.1) | e_F4 | verdict vs rho |")
P("|---|---|---|---|---|---|---|---|---|---|---|---|---|")
f = lambda x: "-" if x is None else f"{x:.2f}"
for key, v in summary.items():
    k = v["k"]
    P(f"| {key} | {v['fit_sizes']} | {f(k.get('sum_rows'))} | {f(k.get('rows_reduced'))} | {f(k.get('sum_nnz'))} | {f(k.get('ms_time'))} | {f(v['k_XLcols'])} | {f(v['dD_F4/ds'])} | {f(v['dD_XL/ds'])} | {f(v['k_min'])} | {v['thmB_exp1_transfers']} | {f(v['e_F4'])} | {v['verdict']} |")
P("\n# Post-preset: LA-free implied exponent min(1,(1+k)/m) per proxy (most IC-favourable reading)\n")
P("| route | e sum_rows | e rows_reduced | e sum_nnz | e time |")
P("|---|---|---|---|---|")
for key, v in summary.items():
    e = v["e_LAfree"]
    P(f"| {key} | {f(e.get('sum_rows'))} | {f(e.get('rows_reduced'))} | {f(e.get('sum_nnz'))} | {f(e.get('ms_time'))} |")
P(f"\nnon-ok runs ({len(bad)}): " + ", ".join(bad))
open(os.path.join(OUT, "tables.md"), "w").write("\n".join(lines) + "\n")
json.dump(summary, open(os.path.join(OUT, "summary.json"), "w"), indent=1)
print("\n".join(lines))
