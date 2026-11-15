#!/usr/bin/env python3
"""Fast vs SgeBw on line-sieve rows (plan track T3; not under protocol).

    python3 summarise.py fast.txt sgebw.txt > summary-table.md

fast.txt / sgebw.txt: `sieve_probe` output (HEC_SKIP_WALK=1 HEC_VERIFY=0,
HEC_SIEVE_SOLVER=fast / sgebw); sgebw.txt also holds the BWTRACE lines
(HEC_BW_TRACE=1, stderr), one per solve call, in run order.  Per (genus, p):
medians over seeds.  Counts are the measurement; wall clock comes from a
shared host and is descriptive only.
"""
import json
import math
import statistics as st
import sys
from collections import defaultdict


def load(path):
    runs, traces = [], []
    for line in open(path):
        if line.startswith("JSON "):
            j = json.loads(line[5:])
            j["traces"] = traces
            runs.append(j)
            traces = []
        elif line.startswith("BWTRACE "):
            traces.append(json.loads(line[8:]))
    return runs


def fit(xs, ys):
    lx = [math.log(x) for x in xs]
    ly = [math.log(y) for y in ys]
    mx, my = st.mean(lx), st.mean(ly)
    return sum((a - mx) * (b - my) for a, b in zip(lx, ly)) / sum((a - mx) ** 2 for a in lx)


def main():
    fast, bw = load(sys.argv[1]), load(sys.argv[2])
    G = defaultdict(lambda: {"fast": [], "bw": []})
    for r in fast:
        G[(r["genus"], r["p"])]["fast"].append(r)
    for r in bw:
        G[(r["genus"], r["p"])]["bw"].append(r)
    med = lambda xs, f: st.median(f(x) for x in xs)
    print("| g | p | m | Fast mulmods | SgeBw mulmods | SgeBw/Fast | SGE share | residual N (N/m) | nnz/N | matvecs "
          "| Fast solve ms | SgeBw solve ms | rounds F/B (max) | correct F/B |")
    print("|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|:-:|")
    series = defaultdict(list)
    bad = []
    for (g, p), d in sorted(G.items()):
        f, b = d["fast"], d["bw"]
        m = b[0]["m"]
        fm, bm = med(f, lambda x: x["solve_mulmods"]), med(b, lambda x: x["solve_mulmods"])
        tr = [t for x in b for t in x["traces"]]
        share = med(tr, lambda t: t["sge_mulmods"] / t["mulmods"])
        n = med(tr, lambda t: t["sge_cols_left"])
        nnz = med(tr, lambda t: t["sge_nnz_left"] / max(1, t["sge_cols_left"]))
        mv = med(tr, lambda t: t["matvecs"])
        rf, rb = max(x["solve_rounds"] for x in f), max(x["solve_rounds"] for x in b)
        okf, okb = all(x["correct"] for x in f), all(x["correct"] for x in b)
        if rf > 1 or rb > 1 or not okf or not okb:
            bad.append((g, p))
        series[g].append((m, fm, bm))
        print(
            f"| {g} | {p} | {m} | {fm:.0f} | {bm:.0f} | {bm / fm:.2f} | {share:.3f} | {n:.0f} ({n / m:.3f}) "
            f"| {nnz:.1f} | {mv:.0f} | {med(f, lambda x: x['solve_ms']):.2f} | {med(b, lambda x: x['solve_ms']):.2f} "
            f"| {rf}/{rb} | {'yes' if okf else 'NO'}/{'yes' if okb else 'NO'} |"
        )
    print()
    print("Log-log slope of median solve mulmods against m:")
    print()
    for g, s in sorted(series.items()):
        if len(s) < 3:
            continue
        s.sort()
        ms = [x[0] for x in s]
        hi = s[len(s) // 2:]
        print(
            f"- g={g}, m {ms[0]}..{ms[-1]} ({len(s)} rows): Fast {fit(ms, [x[1] for x in s]):.2f}, "
            f"SgeBw {fit(ms, [x[2] for x in s]):.2f}; upper half (m ≥ {hi[0][0]}): "
            f"Fast {fit([x[0] for x in hi], [x[1] for x in hi]):.2f}, SgeBw {fit([x[0] for x in hi], [x[2] for x in hi]):.2f}"
        )
    print()
    print("Rows with a run that took >1 solve round or returned a wrong k:", bad if bad else "none")


main()
