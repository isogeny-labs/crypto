#!/usr/bin/env python3
"""Lay out the field-level rows (rows/field-level-*.json): a Markdown table and
an SVG/PNG graph of the recorded widths against n. Computes nothing about any
system; reads recorded values only."""
import glob, json, math, os
HERE = os.path.dirname(os.path.abspath(__file__))
rows = []
for path in sorted(glob.glob(os.path.join(HERE, "rows", "field-level-*.json"))):
    rows.extend(json.load(open(path)))
rows.sort(key=lambda r: r["n"])

lines = ["| n | n' | vars 2n' | eqs | quad. monomials | bb factors / max arity | bb width lower | bb width upper | split vars | split lower | split upper | split log2 cost | enum one slot n' | enum both 2n' | D* (d_max) | first fall | Macaulay cols at D* (log2) | target decomposable |",
         "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|"]
for r in rows:
    bb, sp, rf = r["bb"], r["split"], r.get("refutation")
    dstar = rf and rf.get("refutation_degree")
    cols = "-"
    if rf and dstar is not None:
        for m in rf["macaulay"]:
            if m["degree"] == dstar:
                cols = "%.1f" % math.log2(m["cols"])
    lines.append("| %d | %d | %d | %d | %d | %d / %d | %d | %d | %d | %d | %d | %.1f | %d | %d | %s | %s | %s | %s |" % (
        r["n"], r["n_sub"], r["num_vars"], r["num_eqs"], r["quadratic_monomials"], bb["factors"], bb["max_arity"],
        bb["width"]["lower_bound"], bb["width"]["upper_bound"], sp["variables"], sp["width"]["lower_bound"],
        sp["width"]["upper_bound"], sp["width"]["log2_cost_best"], r["enumeration_exponent_one_slot"],
        r["enumeration_exponent_both_slots"],
        ("%s (%d)" % (dstar, rf["d_max"])) if rf else "-", (rf and rf.get("first_fall")) if rf else "-", cols,
        r.get("decomposable_target")))
open(os.path.join(HERE, "FIELD_LEVEL_TABLE.md"), "w").write("# Field-level (Weil-descended binary S3) rows\n\n" + "\n".join(lines) + "\n")

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
fig, ax = plt.subplots(figsize=(7.5, 5))
ns = [r["n"] for r in rows]
ax.fill_between(ns, [r["bb"]["width"]["lower_bound"] for r in rows], [r["bb"]["width"]["upper_bound"] for r in rows], color="#1f77b4", alpha=0.15)
ax.plot(ns, [r["bb"]["width"]["upper_bound"] for r in rows], "o-", color="#1f77b4", label="black-box factors: best upper bound")
ax.plot(ns, [r["bb"]["width"]["lower_bound"] for r in rows], "v--", color="#1f77b4", label="black-box factors: lower bound")
ax.fill_between(ns, [r["split"]["width"]["lower_bound"] for r in rows], [r["split"]["width"]["upper_bound"] for r in rows], color="#2ca02c", alpha=0.15)
ax.plot(ns, [r["split"]["width"]["upper_bound"] for r in rows], "s-", color="#2ca02c", label="arity-3 gadgets (split): best upper bound")
ax.plot(ns, [r["split"]["width"]["lower_bound"] for r in rows], "^--", color="#2ca02c", label="arity-3 gadgets (split): lower bound")
ax.plot(ns, [r["num_vars"] for r in rows], "k:", label="2n' (both slots enumerated)")
ax.plot(ns, [r["n_sub"] for r in rows], "k-.", label="n' (one slot enumerated)")
dn = [(r["n"], r["refutation"]["refutation_degree"]) for r in rows if r.get("refutation") and r["refutation"].get("refutation_degree") is not None]
if dn:
    ax.plot([d[0] for d in dn], [d[1] for d in dn], "d-", color="#d62728", label="refutation degree D* (F4 reaches it)")
ax.set_xlabel("field degree n (factor base V of dimension n' = n/2)")
ax.set_ylabel("treewidth bound / exponent (bits)")
ax.set_title("Field-level binary S3 PDP system: treewidth of its primal graph vs enumeration and D*", fontsize=9)
ax.grid(True, alpha=0.3)
ax.legend(fontsize=7)
fig.tight_layout()
fig.savefig(os.path.join(HERE, "field_level_vs_n.svg"))
fig.savefig(os.path.join(HERE, "field_level_vs_n.png"), dpi=150)
print("\n".join(lines))
