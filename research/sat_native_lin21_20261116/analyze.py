#!/usr/bin/env python3
"""Tables, the pre-registered decision and the figure from results.json.

  python3 analyze.py [DIR]     # DIR holds results.json; writes tables.md, summary.json, fig_*.svg/png
"""
import json, math, os, statistics as st, sys
from collections import defaultdict

D = sys.argv[1] if len(sys.argv) > 1 else os.path.dirname(os.path.abspath(__file__))
R = json.load(open(os.path.join(D, "results.json")))
rows = R["rows"]
ARMS = ["lin", "lin-nolex", "lin21", "lin21-pu", "scan"]
SAT_ARMS = ARMS[:4]
ls = sorted({r["l"] for r in rows})
by = defaultdict(dict)  # (l, arm) -> idx -> row
for r in rows:
    by[(r["l"], r["arm"])][r["idx"]] = r


def refut(l, arm):
    """uncensored refutation rows on non-decomposable uniform targets"""
    return [r for r in by[(l, arm)].values() if r["kind"] == "uniform" and r["status"] == "unsat"]


def med(xs):
    return st.median(xs) if xs else float("nan")


def fmt(x, d=3):
    if isinstance(x, float) and math.isnan(x):
        return "n/a"
    return f"{x:.{d}g}" if isinstance(x, float) else str(x)


out, summ = [], {"cells": {}}
# integrity
bad = [r for r in rows if r.get("agree") is False or r.get("verified") is False or r["status"] == "error"]
cens = [r for r in rows if r["status"] == "censored"]
nsat = [r for r in rows if r["status"] == "sat"]
out.append(f"Rows: {len(rows)}; verdict disagreements with the pair oracle, unverified SAT models or errors: {len(bad)}; "
           f"censored at the cap: {len(cens)}; SAT models lifted to points summing to the target: "
           f"{sum(1 for r in nsat if r['verified'])} of {len(nsat)}. [pcc:unverified]\n")
summ["integrity"] = {"rows": len(rows), "bad": len(bad), "censored": len(cens), "sat_models": len(nsat),
                     "sat_verified": sum(1 for r in nsat if r["verified"])}

out.append("## Conflicts on non-decomposable uniform targets (median [min, max]) [pcc:unverified]\n")
out.append("| l | n | arm | targets | conflicts | / 2^l | / C(2^l,2) | decisions | GJE calls | gate linearisations | wall ms |")
out.append("|---|---|---|---|---|---|---|---|---|---|---|")
medc = {}
for l in ls:
    n = 3 * l + 2
    for arm in SAT_ARMS:
        rs = refut(l, arm)
        ncens = sum(1 for r in by[(l, arm)].values() if r["kind"] == "uniform" and r["status"] == "censored")
        if not rs and not ncens:
            continue
        c = [r["conflicts"] for r in rs]
        P, C2 = 2 ** l, 2 ** l * (2 ** l - 1) // 2
        m = med(c)
        medc[(l, arm)] = m
        summ["cells"][f"{l}/{arm}"] = {
            "targets": len(rs), "censored": ncens, "median_conflicts": m, "min": min(c, default=None),
            "max": max(c, default=None), "per_2l": m / P, "per_pairs": m / C2,
            "median_decisions": med([r["decisions"] for r in rs]),
            "median_gje_calls": med([r["gje_calls"] for r in rs]),
            "median_propagations": med([r["propagations"] for r in rs]),
            "median_wall_ms": med([r["wall_ms"] for r in rs])}
        if rs:
            out.append(f"| {l} | {n} | {arm} | {len(rs)}{' (+%d censored)' % ncens if ncens else ''} | "
                       f"{fmt(m, 6)} [{min(c)}, {max(c)}] | {m/P:.3f} [{min(c)/P:.3f}, {max(c)/P:.3f}] | "
                       f"{m/C2:.4f} [{min(c)/C2:.4f}, {max(c)/C2:.4f}] | {fmt(med([r['decisions'] for r in rs]), 6)} | "
                       f"{fmt(med([r['gje_calls'] for r in rs]), 6)} | {fmt(med([r['gate_linearisations'] for r in rs]), 6)} | "
                       f"{med([r['wall_ms'] for r in rs]):.4g} |")
        else:
            out.append(f"| {l} | {n} | {arm} | 0 (+{ncens} censored) | censored | | | | | | > {R['meta']['cap_s']*1000} |")

out.append("\n## Plain native 2+1 scan (no SAT), non-decomposable uniform targets, medians [pcc:unverified]\n")
out.append("| l | targets | x1 visited | leaves | leaves with consistent (s,p) | field mul | field inv | Artin-Schreier solves | shifts | row XORs | wall ms (median of 21 repeats) |")
out.append("|---|---|---|---|---|---|---|---|---|---|---|")
for l in ls:
    rs = refut(l, "scan")
    if not rs:
        continue
    g = lambda k: med([r[k] for r in rs])
    summ["cells"][f"{l}/scan"] = {"targets": len(rs), "median_leaves": g("leaves"), "median_wall_ms": g("wall_ms"),
                                  "median_field_muls": g("field_muls"), "median_row_xors": g("row_xors")}
    out.append(f"| {l} | {len(rs)} | {fmt(g('x1_visited'),6)} | {fmt(g('leaves'),6)} | {fmt(g('leaves_consistent'),6)} | {fmt(g('field_muls'),6)} | "
               f"{fmt(g('field_invs'),6)} | {fmt(g('as_solves'),6)} | {fmt(g('xtimes'),7)} | {fmt(g('row_xors'),7)} | {g('wall_ms'):.4g} |")

out.append("\n## Paired ratios on common non-decomposable targets (median of per-target ratios [min, max]) [pcc:unverified]\n")
out.append("| l | common targets lin & lin21 | conflicts lin / lin21 | 2^(l-3) | wall lin / lin21 | 2^(l-4) | wall lin21 / scan | wall lin / scan | lin21 conflicts minus scan count D [min, max] |")
out.append("|---|---|---|---|---|---|---|---|---|")
pair = {}
for l in ls:
    a = {r["idx"]: r for r in refut(l, "lin")}
    b = {r["idx"]: r for r in refut(l, "lin21")}
    s = {r["idx"]: r for r in refut(l, "scan")}
    common = sorted(set(a) & set(b))
    rc = [a[i]["conflicts"] / b[i]["conflicts"] for i in common]
    rw = [a[i]["wall_ms"] / b[i]["wall_ms"] for i in common]
    rs_ = [b[i]["wall_ms"] / s[i]["wall_ms"] for i in sorted(set(b) & set(s))]
    rl = [a[i]["wall_ms"] / s[i]["wall_ms"] for i in sorted(set(a) & set(s))]
    # D = conflicts - (1 [x1 = 0] + x1 without roots + leaves)
    dd = [b[i]["conflicts"] - (1 + s[i]["x1_visited"] - s[i]["x1_with_roots"] + s[i]["leaves"]) for i in sorted(set(b) & set(s))]
    pair[l] = {"common": len(common), "conf_ratio": med(rc), "wall_ratio": med(rw), "lin21_over_scan": med(rs_),
               "lin_over_scan": med(rl), "D_min": min(dd, default=None), "D_max": max(dd, default=None),
               "ratio_of_medians": (med([a[i]["conflicts"] for i in common]) / med([b[i]["conflicts"] for i in common])) if common else None}
    rng = lambda v: f"{med(v):.4g} [{min(v):.4g}, {max(v):.4g}]" if v else "n/a"
    out.append(f"| {l} | {len(common)} | {rng(rc)} | {2**(l-3)} | {rng(rw)} | {2**(l-4)} | {rng(rs_)} | {rng(rl)} | "
               f"{'[%d, %d]' % (min(dd), max(dd)) if dd else 'n/a'} |")
summ["paired"] = pair

out.append("\n## Planted (decomposable) targets and decomposable uniform draws [pcc:unverified]\n")
out.append("| l | arm | targets | SAT and lifted | median conflicts | median wall ms |")
out.append("|---|---|---|---|---|---|")
for l in ls:
    for arm in ARMS:
        rs = [r for r in by[(l, arm)].values() if r["kind"] != "uniform"]
        if not rs:
            continue
        ok = [r for r in rs if r["status"] == "sat" and r["verified"]]
        c = [r["conflicts"] for r in ok if "conflicts" in r]
        out.append(f"| {l} | {arm} | {len(rs)} | {len(ok)} | {fmt(med(c), 6) if c else 'n/a'} | {med([r['wall_ms'] for r in ok]):.4g} |")

out.append("\n## lin21: learnt clauses by number of x1 priority literals, summed over non-decomposable uniform targets [pcc:unverified]\n")
out.append("Width w < l means the clause excludes 2^(l-w) values of x1. Priority set of `lin21` = the l bits of x1 only.\n")
out.append("| l | targets | " + " | ".join(f"w={w}" for w in range(0, max(ls) + 1)) + " | clauses |")
out.append("|---|---|" + "---|" * (max(ls) + 2))
hist = {}
for l in ls:
    rs = refut(l, "lin21")
    h = [0] * (max(ls) + 1)
    for r in rs:
        for w, k in enumerate(r["learnt_priority_width"]):
            h[w] += k
    hist[l] = h
    out.append(f"| {l} | {len(rs)} | " + " | ".join(str(x) if w <= l else "" for w, x in enumerate(h)) + f" | {sum(h)} |")
summ["lin21_width_hist"] = hist

# decision
big = [l for l in ls if l >= 7 and (l, "lin21") in medc]
condA = all(medc[(l, "lin21")] <= 4 * 2 ** l for l in big) and bool(big)
common_l = [l for l in ls if pair[l]["common"] >= 4]
Ls = max(common_l) if common_l else None
ratio = pair[Ls]["ratio_of_medians"] if Ls else None
if Ls is None:
    dec = "inconclusive"
elif ratio < 4:
    dec = "fails"
elif condA and ratio >= 2 ** (Ls - 3):
    dec = "lead holds"
else:
    dec = "inconclusive"
summ["decision"] = {"condA_all_l_ge_7_median_le_4x2l": condA, "largest_common_l": Ls,
                    "ratio_of_medians_lin_over_lin21": ratio, "threshold": 2 ** (Ls - 3) if Ls else None, "decision": dec}
out.append(f"\n## Decision by the pre-registered rule\n\nCondition A (lin21 median conflicts <= 4 * 2^l at every measured l >= 7): {condA}. "
           f"Largest l with >= 4 common uncensored refutations: {Ls}; median lin conflicts / median lin21 conflicts there: "
           f"{fmt(float(ratio), 5) if ratio else 'n/a'}; threshold 2^(l-3) = {2**(Ls-3) if Ls else 'n/a'}. **Decision: {dec}.** [pcc:unverified]\n")

open(os.path.join(D, "tables.md"), "w").write("\n".join(out) + "\n")
json.dump(summ, open(os.path.join(D, "summary.json"), "w"), indent=1)
print("\n".join(out))

# figure
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

fig, ax = plt.subplots(figsize=(7.6, 5.0), dpi=150)
col = {"lin": "#1f77b4", "lin-nolex": "#7f7f7f", "lin21": "#d62728", "lin21-pu": "#ff7f0e"}
mk = {"lin": "o", "lin-nolex": "s", "lin21": "D", "lin21-pu": "^"}
for arm in SAT_ARMS:
    xs = [l for l in ls if refut(l, arm)]
    if not xs:
        continue
    m = [med([r["conflicts"] for r in refut(l, arm)]) for l in xs]
    lo = [min(r["conflicts"] for r in refut(l, arm)) for l in xs]
    hi = [max(r["conflicts"] for r in refut(l, arm)) for l in xs]
    ax.errorbar(xs, m, yerr=[[a - b for a, b in zip(m, lo)], [b - a for a, b in zip(m, hi)]], color=col[arm],
                marker=mk[arm], ms=5, lw=1.4, capsize=2, label=f"{arm} (median, min-max)")
xs = [l for l in ls if refut(l, "scan")]
ax.plot(xs, [med([r["leaves"] for r in refut(l, "scan")]) for l in xs], color="#2ca02c", marker="x", lw=1.0, ls="-.",
        label="plain 2+1 scan: leaves solved")
grid = list(range(min(ls), max(ls) + 1))
ax.plot(grid, [2 ** l for l in grid], "k:", lw=1, label="2^l")
ax.plot(grid, [4 * 2 ** l for l in grid], color="k", ls=(0, (1, 4)), lw=1, label="4 * 2^l (rule bound)")
ax.plot(grid, [2 ** l * (2 ** l - 1) / 2 for l in grid], "k--", lw=1, label="C(2^l, 2)")
ax.set_yscale("log", base=2)
ax.set_xlabel("l = dim V   (n = 3l + 2)")
ax.set_ylabel("conflicts per non-decomposable target")
ax.set_title("3-summand PDP refutation in LinCdcl: chained pair encoding (lin) vs 2+1 leaf (lin21)", fontsize=9)
ax.grid(True, which="major", alpha=0.3)
ax.legend(fontsize=7, loc="upper left")
fig.tight_layout()
fig.savefig(os.path.join(D, "fig_conflicts_vs_l.svg"))
fig.savefig(os.path.join(D, "fig_conflicts_vs_l.png"))
