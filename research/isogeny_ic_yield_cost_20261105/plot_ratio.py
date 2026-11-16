# Graph for Follow-up 3: best IC cost / matched rho, from results_cost2/*.jsonl (same rules as analyze_cost2.py).
import json, glob, statistics as st
from collections import defaultdict
import matplotlib; matplotlib.use("Agg"); import matplotlib.pyplot as plt
IC, RHO = [], {}
for f in sorted(glob.glob("results_cost2/b*_m*_c*.jsonl")):
    for l in open(f):
        r = json.loads(l); (RHO.__setitem__((r["bits"], r["curve"]), r) if r["kind"] == "rho" else IC.append(r))
m = st.mean
dp1 = lambda r: m(r["dp_single_ops"]) + r["dp_single_setup"]
dpD = lambda r: m(r["dp_multi_ops"]) + r["dp_multi_setup"] / r["n"]
bits = sorted({b for b, _ in RHO}); ncur = defaultdict(int)
for r in IC: ncur[r["bits"]] = max(ncur[r["bits"]], r["curve"] + 1)
g = defaultdict(list)
for r in IC: g[(r["strat"], r["m"], r["c"], r["coset"])].append(r)
def best(b, D, keep=lambda k: True):
    c = [m(r["precomp_ops"] / D + r["desc_ops_mean"] for r in rb) for k, rs in g.items() if keep(k)
         for rb in [[r for r in rs if r["bits"] == b]] if len({r["curve"] for r in rb}) == ncur[b] and rb]
    return min(c) if c else None
fig, ax = plt.subplots(figsize=(7, 4.2))
r1 = [best(b, 1) / m(dp1(RHO[k]) for k in RHO if k[0] == b) for b in bits]
rD = [best(b, 100) / m(dpD(RHO[k]) for k in RHO if k[0] == b) for b in bits]
ax.plot(bits, r1, "o-", label="best IC, 1 DLP / DP rho with negation")
ax.plot(bits, rD, "s-", label="best IC, D=100 / multi-target DP rho")
for mm, mk in ((3, "^"), (4, "v"), (5, "d")):
    ys = [best(b, 1, lambda k: k[0] == "MITM" and k[1] == mm) for b in bits]
    ax.plot([b for b, y in zip(bits, ys) if y], [y / m(dp1(RHO[k]) for k in RHO if k[0] == b) for b, y in zip(bits, ys) if y],
            mk + ":", alpha=.7, label=f"best MITM m={mm}, 1 DLP")
ax.axhline(1, color="k", lw=.8); ax.set_yscale("log"); ax.set_xlabel("field bits (3 curves per size, 100 scalars each)")
ax.set_ylabel("IC cost / matched rho cost (group additions)"); ax.set_title("Semaev IC vs Pollard rho: the gap grows with size")
ax.legend(fontsize=7); fig.tight_layout()
for ext in ("svg", "png", "pdf"): fig.savefig(f"results_cost2/ic_vs_rho.{ext}", dpi=150)
print([round(x, 2) for x in r1], [round(x, 2) for x in rD])
