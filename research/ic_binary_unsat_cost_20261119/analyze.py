#!/usr/bin/env python3
"""Derive per-cell paired ratios from raw/*.jsonl -> results.json, figure.svg, sha256 list."""
import json, glob, math, random, hashlib, os, statistics as st
random.seed(7)
here = os.path.dirname(os.path.abspath(__file__))
ARMS = ["base2", "freq_hi", "freq_lo"]

def gm(xs): return math.exp(sum(math.log(x) for x in xs) / len(xs))
def boot(ratios, B=2000):
    n = len(ratios); out = []
    for _ in range(B):
        out.append(gm([ratios[random.randrange(n)] for _ in range(n)]))
    out.sort(); return out[int(.025 * B)], out[int(.975 * B) - 1]

cells = {}; sha = {}; gate = {"attempts": 0}
for f in sorted(glob.glob(f"{here}/raw/*.jsonl")):
    sha[os.path.basename(f)] = hashlib.sha256(open(f, "rb").read()).hexdigest()
    rows = [json.loads(l) for l in open(f) if l.strip()]
    gate["attempts"] += len(rows)
    key = f"a{rows[0]['a']}_n{rows[0]['n']}"
    by = {}
    for r in rows:
        by.setdefault((r["target"], r["arm"]), []).append(r)
    targets = sorted({r["target"] for r in rows})
    def med(t, arm, k): return st.median(x[k] for x in by[(t, arm)])
    cell = {"n": rows[0]["n"], "a": rows[0]["a"], "dim": rows[0]["dim"], "targets": len(targets), "arms": {}}
    unsat = [t for t in targets if by[(t, "base")][0]["roots"] == 0]
    cell["unsat_targets"] = len(unsat)
    tot = sum(med(t, "base", "word_ops") for t in targets)
    cell["unsat_share_base_word_ops"] = sum(med(t, "base", "word_ops") for t in unsat) / tot
    cell["base_ms_median_per_target"] = st.median(med(t, "base", "ms") for t in targets)
    cell["base_word_ops_per_target_mean"] = tot / len(targets)
    for arm in ARMS:
        d = {}
        for metric in ("word_ops", "ms", "reductions"):
            rat = [med(t, arm, metric) / med(t, "base", metric) for t in targets]
            lo, hi = boot(rat)
            d[metric + "_gmean_ratio"] = gm(rat); d[metric + "_ci95"] = [lo, hi]
        tw = sum(med(t, arm, "word_ops") for t in targets)
        d["total_word_ops_ratio"] = tw / tot
        d["unsat_total_word_ops_ratio"] = (sum(med(t, arm, "word_ops") for t in unsat) /
                                           max(1, sum(med(t, "base", "word_ops") for t in unsat)))
        d["censored_attempts"] = sum(1 for t in targets for x in by[(t, arm)] if x["exhausted"])
        d["attempts"] = sum(len(by[(t, arm)]) for t in targets)
        cell["arms"][arm] = d
    # base repeatability of counted work across reps (native word ops deterministic?)
    cell["base_native_ops_repeatable"] = all(len({x["native_word_ops"] for x in by[(t, "base")]}) == 1 for t in targets)
    cells[key] = cell
res = {"status": "measured (single host, L0 development, [pcc:unverified])", "raw_sha256": sha, "attempts": gate["attempts"], "cells": cells}
json.dump(res, open(f"{here}/results.json", "w"), indent=1)

# figure: gmean word-op ratio per cell, log scale
keys = sorted(cells, key=lambda k: (cells[k]["n"], cells[k]["a"]))
W, H, m = 760, 330, 50
svg = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" font-family="sans-serif" font-size="11">',
       '<rect width="100%" height="100%" fill="white"/>',
       f'<text x="{m}" y="18" font-size="13">Counted word ops, arm / base (geometric mean over targets, log scale; 1 = no change)</text>']
lo, hi = -0.3, 3.0
def y(v): return H - 40 - (math.log10(max(v, 1e-3)) - lo) / (hi - lo) * (H - 80)
for e in (0, 1, 2, 3):
    svg.append(f'<line x1="{m}" x2="{W-10}" y1="{y(10**e):.1f}" y2="{y(10**e):.1f}" stroke="#ddd"/><text x="5" y="{y(10**e)+4:.1f}">{10**e}x</text>')
svg.append(f'<line x1="{m}" x2="{W-10}" y1="{y(1):.1f}" y2="{y(1):.1f}" stroke="#000"/>')
colors = {"base2": "#888", "freq_hi": "#c33", "freq_lo": "#36c"}
gw = (W - m - 20) / len(keys)
for i, k in enumerate(keys):
    for j, arm in enumerate(ARMS):
        v = cells[k]["arms"][arm]["word_ops_gmean_ratio"]
        x = m + i * gw + 8 + j * 20
        svg.append(f'<rect x="{x:.1f}" y="{min(y(v), y(1)):.1f}" width="16" height="{abs(y(v)-y(1)):.1f}" fill="{colors[arm]}"/>')
    svg.append(f'<text x="{m+i*gw+6:.1f}" y="{H-24}">K({cells[k]["a"]},{cells[k]["n"]})</text>')
for j, arm in enumerate(ARMS):
    svg.append(f'<rect x="{m+j*110}" y="{H-14}" width="10" height="10" fill="{colors[arm]}"/><text x="{m+j*110+14}" y="{H-5}">{arm}</text>')
svg.append("</svg>")
open(f"{here}/figure.svg", "w").write("\n".join(svg))
print(json.dumps({k: {a: (round(v["word_ops_gmean_ratio"], 3), round(v["ms_gmean_ratio"], 3), v["censored_attempts"]) for a, v in c["arms"].items()} | {"unsat_share": round(c["unsat_share_base_word_ops"], 3)} for k, c in cells.items()}, indent=1))
