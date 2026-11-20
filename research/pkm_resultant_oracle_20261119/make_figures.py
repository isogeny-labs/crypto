#!/usr/bin/env python3
"""Figure for EXP-PKM-RESULTANT-001 from results.json (and the EXP-PKM-DENSE-DEC-001 results for F4 references)."""
import json
import math
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
R = json.load(open(os.path.join(HERE, "results.json")))
DD = os.path.join(HERE, "..", "pkm_dense_decompose_20261115")
F = json.load(open(os.path.join(DD, "results.json")))["cells"]
L = json.load(open(os.path.join(DD, "results_large.json")))
cells = [c for c in R["cells"] if c["vkind"] == "coset"]
fig, (ax, bx) = plt.subplots(1, 2, figsize=(10.5, 4))
res = sorted({c["S"]: c["res_ops"] for c in cells}.items())
enum = sorted({c["S"]: c["enum_ops"] for c in cells}.items())
ax.plot(*zip(*res), "o-", color="#7a3e9d", label="resultant / multipoint oracle (this study)", ms=4)
ax.plot(*zip(*enum), "d-", color="#555555", label="enumeration over V", ms=4)
dense = {}
tower = {}
for c in F + L["cells"]:
    if c["m"] == 2 and c["vkind"] == "coset":
        if c.get("dense_planted_ops_median"):
            dense.setdefault(c["S"], c["dense_planted_ops_median"])
        if c.get("tower_planted_ops_median"):
            tower.setdefault(c["S"], c["tower_planted_ops_median"])
ax.plot(*zip(*sorted(tower.items())), "o:", color="#3b6ea8", label="F4, tower membership", ms=4)
ax.plot(*zip(*sorted(dense.items())), "^:", color="#7a9e4b", label="F4, dense membership", ms=4)
lb = [(v["S"], v["ops_at_cutoff_median"]) for k, v in L["censored_lower_bounds"].items() if "random" not in k]
ax.plot(*zip(*lb), "^", color="#7a9e4b", mfc="none", ms=9, label="F4 dense, censored (lower bound)")
ax.set_xscale("log", base=2)
ax.set_yscale("log")
ax.set_xlabel("|V| = S")
ax.set_ylabel("field_ops to find all relations (planted, median)")
ax.set_title("m = 2: cost of one target")
ax.grid(alpha=0.3, which="both")
ax.legend(fontsize=7)
AM = os.path.join(HERE, "results_amortised.json")
am = [c for c in json.load(open(AM))["cells"] if c["vkind"] == "coset"] if os.path.exists(AM) else []
if am:
    ax.plot(*zip(*sorted({c["S"]: c["marginal"] for c in am}.items())), "o--", color="#c08bd6", ms=3,
            label="resultant, V-only work paid once (Amendment 1)")
    ax.plot(*zip(*sorted({c["S"]: c["enum_marginal"] for c in am}.items())), "d--", color="#999999", ms=3,
            label="enumeration, V lifted once (Amendment 1)")
    ax.legend(fontsize=6.5)
ratio = sorted({c["S"]: c["res_over_enum"] for c in cells}.items())
bx.plot(*zip(*ratio), "o-", color="#7a3e9d", ms=4, label="resultant ÷ enumeration")
if am:
    bx.plot(*zip(*sorted({c["S"]: c["marginal_over_enum_marginal"] for c in am}.items())), "o--", color="#c08bd6",
            ms=3, label="same, both with V-only work paid once")
s0, r0 = ratio[-1]
bx.plot([s for s, _ in ratio], [r0 * (math.log2(s) / math.log2(s0)) ** 2 for s, _ in ratio], "--", color="#999",
        label="∝ log² S (reference)")
bx.axhline(1, color="#555", lw=0.8)
bx.set_xscale("log", base=2)
bx.set_yscale("log")
bx.set_xlabel("|V| = S")
bx.set_ylabel("ratio of counted field_ops")
bx.set_title("gap to enumeration grows polylogarithmically")
bx.grid(alpha=0.3, which="both")
bx.legend(fontsize=7)
fig.suptitle("A quasi-linear algebraic oracle closes most of F4's gap, but not enumeration's", fontsize=9)
for ext in ("svg", "png"):
    fig.savefig(os.path.join(HERE, "figures", f"cost.{ext}"), dpi=160, bbox_inches="tight")
print("ok")
