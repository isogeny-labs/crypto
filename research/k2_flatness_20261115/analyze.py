"""Mechanical readout of raw/grid.jsonl against PROTOCOL.md; writes results.json, the table
(results.md) and figures/k2_dsol.{svg,png}. Usage: analyze.py [raw/grid.jsonl]"""
import json
import sys
from collections import defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from dreg import semi_regular_degree  # noqa: E402

HERE = Path(__file__).resolve().parent
raw = Path(sys.argv[1]) if len(sys.argv) > 1 else HERE / "raw" / "grid.jsonl"
rows = [json.loads(l) for l in raw.read_text().splitlines() if l.strip()]

# one record per system: (cell, m, n, k, seed, target, pres)
systems = defaultdict(list)
for r in rows:
    systems[(r["cell"], r["m"], r["n"], r["k"], r["seed"], r["target"], r.get("pres", "chain"))].append(r)


def dsol(rs):
    """First solved D; else ('>', largest D computed); censoring when every D was skipped."""
    done = [r for r in rs if "solved" in r]
    for r in sorted(done, key=lambda r: r["D"]):
        if r["solved"]:
            return r["D"], False
    top = max((r["D"] for r in done), default=None)
    return (top + 1 if top is not None else None), True


cells = defaultdict(list)
incomplete = []
for key, rs in systems.items():
    cell, m, n, k, seed, t, pres = key
    # a run stopped by the budget leaves only the rows below D = 4 (no solved row, no skip row)
    if not any(r["D"] >= 4 for r in rs) and not any(r.get("solved") for r in rs):
        incomplete.append({"cell": cell, "m": m, "n": n, "k": k, "seed": seed, "target": t})
        continue
    d, cens = dsol(rs)
    d4 = next((r for r in rs if r["D"] == 4 and "rank" in r), None)
    cells[(cell, m, n, k)].append({
        "seed": seed, "target": t, "nsol": rs[0]["nsol"], "N": rs[0]["N"], "eqs": rs[0]["eqs"],
        "dsol": d, "censored": cens,
        "deficit_D4": d4["deficit"] if d4 else None,
        "secs_D4": d4["secs"] if d4 else None,
        "cols_D4": d4["cols"] if d4 else None,
    })

def med(xs):
    xs = sorted(xs)
    return xs[len(xs) // 2] if xs else None


out = []
for (cell, m, n, k), ts in sorted(cells.items(), key=lambda x: (x[0][0], x[0][1], x[0][2], x[0][3])):
    N, eqs = ts[0]["N"], ts[0]["eqs"]
    if m == 2:
        degs = [2] * eqs
    elif cell == "N":
        degs = None
    else:
        degs = [3] * (eqs - n) + [2] * n  # chain: the last S_3 (fixed xR) is quadratic
    dreg = semi_regular_degree(N, degs) if degs else None
    ds = [t["dsol"] for t in ts]
    out.append({
        "cell": cell, "m": m, "n": n, "k": k, "N": N, "eqs": eqs, "targets": len(ts),
        "sat_targets": sum(1 for t in ts if t["nsol"] > 0),
        "dsol": ds, "censored": [t["censored"] for t in ts],
        "share_dsol_le4": sum(1 for t in ts if t["dsol"] is not None and t["dsol"] <= 4 and not t["censored"]) / len(ts),
        "d_reg": dreg,
        "cols_D4": ts[0]["cols_D4"],
        "median_secs_D4": med([t["secs_D4"] for t in ts if t["secs_D4"] is not None]),
        "max_deficit_D4": max((t["deficit_D4"] for t in ts if t["deficit_D4"] is not None), default=None),
        "per_target": ts,
    })


def series_verdict(series):
    cs = sorted([c for c in out if c["cell"] in series], key=lambda c: (c["n"], c["k"]))
    if not cs:
        return "not run"
    if all(c["share_dsol_le4"] == 1 for c in cs):
        return "flat"
    ns = sorted({c["n"] for c in cs})
    lo = [c for c in cs if c["n"] in ns[:2]]
    hi = [c for c in cs if c["n"] in ns[-2:]]
    sh = lambda cc: sum(c["share_dsol_le4"] * c["targets"] for c in cc) / sum(c["targets"] for c in cc)
    if sh(lo) == 1 and sh(hi) <= 0.5:
        return "growing"
    return "inconclusive"


ctrl_p = all(c["share_dsol_le4"] == 1 for c in out if c["cell"] == "P")
ctrl_n = all(c["max_deficit_D4"] is not None and c["max_deficit_D4"] > 100 and c["share_dsol_le4"] == 0
             for c in out if c["cell"] == "N")
va = series_verdict({"A", "A-ext"})
vc = series_verdict({"C"})
overall = "growing" if "growing" in (va, vc) else ("flat" if va == vc == "flat" else "inconclusive")
summary = {"incomplete_targets": incomplete, "controls": {"positive_pass": ctrl_p, "negative_pass": ctrl_n},
           "verdict": {"A_m3": va, "C_m4": vc, "overall": overall}, "cells": out}
(HERE / "results.json").write_text(json.dumps(summary, indent=1))

lines = ["| series | m | n | k | N | eqs | targets (sat) | D_sol per target | share D_sol <= 4 | d_reg (semi-regular) | cols at D=4 | median s at D=4 |",
         "|:--|--:|--:|--:|--:|--:|:--|:--|--:|--:|--:|--:|"]
for c in out:
    ds = " ".join((">" + str(d - 1) if cen else str(d)) if d is not None else "skip" for d, cen in zip(c["dsol"], c["censored"]))
    lines.append(f"| {c['cell']} | {c['m']} | {c['n']} | {c['k']} | {c['N']} | {c['eqs']} | {c['targets']} ({c['sat_targets']}) | {ds} | "
                 f"{c['share_dsol_le4']:.2f} | {c['d_reg'] if c['d_reg'] else '-'} | {c['cols_D4'] or '-'} | "
                 f"{c['median_secs_D4'] if c['median_secs_D4'] is not None else '-'} |")
lines.append("")
if incomplete:
    lines.append("Not completed (stopped by the compute budget, excluded): " + ", ".join(
        f"{i['cell']} m={i['m']} n={i['n']} k={i['k']} seed {i['seed']} target {i['target']}" for i in incomplete) + ".")
    lines.append("")
lines.append(f"Controls: positive {'PASS' if ctrl_p else 'FAIL'}, negative {'PASS' if ctrl_n else 'FAIL'}. "
             f"Verdict: series A (m = 3) **{va}**, series C (m = 4) **{vc}**, overall **{overall}** (on the measured range).")
(HERE / "results.md").write_text("\n".join(lines) + "\n")
print("\n".join(lines))

# figure
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

fig, ax = plt.subplots(figsize=(7.2, 4.2))
styles = {("A", 3): ("#2a6fdb", "o", "m = 3 chain, k = round(n/3): closure D_sol"),
          ("C", 4): ("#d9622b", "s", "m = 4 chain, k ~ n/4: closure D_sol"),
          ("P", 2): ("#3a9a5b", "^", "m = 2 direct (positive control)")}
for (cell, m), (col, mk, lab) in styles.items():
    cs = [c for c in out if (c["cell"] == cell or (cell == "A" and c["cell"] == "A-ext")) and c["m"] == m]
    xs, ys = [], []
    for c in cs:
        for d, cen in zip(c["dsol"], c["censored"]):
            if d is None:
                continue
            xs.append(c["n"]); ys.append(d)
    if xs:
        ax.scatter(xs, ys, color=col, marker=mk, s=40, alpha=0.6, label=lab, zorder=3)
    cs2 = sorted([c for c in cs if c["d_reg"]], key=lambda c: c["n"])
    if cs2 and m != 2:
        ax.plot([c["n"] for c in cs2], [c["d_reg"] for c in cs2], color=col, ls="--", lw=1.2,
                label=f"m = {m}: semi-regular d_reg of the same shape")
ax.axhline(4, color="#888", lw=0.8, ls=":")
ax.set_xlabel("field degree n")
ax.set_ylabel("degree")
ax.set_title("K2: closure solving degree vs n  [pcc:unverified]", fontsize=10)
ax.legend(fontsize=7, loc="center right")
ax.grid(alpha=0.3)
(HERE / "figures").mkdir(exist_ok=True)
fig.tight_layout()
fig.savefig(HERE / "figures" / "k2_dsol.svg")
fig.savefig(HERE / "figures" / "k2_dsol.png", dpi=150)
