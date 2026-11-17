import json, pathlib
import matplotlib; matplotlib.use("Agg"); import matplotlib.ticker; import matplotlib.pyplot as plt
D = pathlib.Path(__file__).resolve().parents[1]
t = json.load(open(D / "data/table.json")); d2 = json.load(open(D / "data/d2_coset.json"))
rows = [x for x in t['rows'] if x['design'] != 'screen' or True]
fig, (ax, bx) = plt.subplots(1, 2, figsize=(13, 6.2), gridspec_kw=dict(width_ratios=[1.6, 1]))
lab, val, col, mk = [], [], [], []
for x in rows:
    g = x['gap']['0.1'] if '0.1' in x['gap'] else x['gap'][0.1]
    lab.append(f"{x['design']} {x['pricing']}: {x['variant'][:58]}" + (" [capped 1e9]" if g > 1e9 else "")); val.append(min(g, 1e9))
    col.append("#b23a48" if x['cls'].startswith("algebraic") else "#2a6f97"); mk.append(x['pricing'])
y = range(len(val))
ax.barh(list(y), val, color=col, alpha=[0.45 if m == "P_N" else 0.95 for m in mk] if False else None)
for i, (v, m) in enumerate(zip(val, mk)):
    if m == "P_N": ax.barh(i, v, color="white", edgecolor=col[i], hatch="//")
ax.set_xscale("log"); ax.axvline(1, color="k", lw=1); ax.axvline(10, color="k", lw=1, ls="--")
ax.text(10.5, len(val) - 0.6, "10x", fontsize=8); ax.text(1.05, len(val) - 0.6, "parity", fontsize=8)
ax.set_yticks(list(y)); ax.set_yticklabels(lab, fontsize=6.5); ax.invert_yaxis()
ax.set_xlabel("gap to rho-neg at w = 0.1  (S ÷ 1.716; setup and linear algebra charged)")
ax.set_title("Relation-oracle designs, k = 20 (blue: generic, red: algebraic; hatched: P_N pricing)", fontsize=9)
# panel 2: decision cost against base size
pc = [(x['d'], x['c_poly_last']) for x in d2['poly_check']] + [(x['d'], x['c_poly']) for x in d2['poly_cost']]
pp = [(x['d'], x['c_points']) for x in d2['rows'] if x['g'] == 5]
bx.loglog(*zip(*pc), "o-", color="#b23a48", label=f"norm/resultant + gcd in F_p[X]/(X^d - c)  (~d^{t['poly_cost_exponent_in_d']:.2f})")
bx.loglog(*zip(*pp), "s-", color="#2a6f97", label="per-point additions (generic), 7 mults per base point")
bx.axhline(14, color="k", ls="--", lw=1); bx.text(130, 17, "parity budget c ≈ 14 at w = 0.1 (screen)", fontsize=7.5)
bx.xaxis.set_minor_formatter(matplotlib.ticker.NullFormatter()); bx.set_xlabel("coset size d"); bx.set_ylabel("counted F_p multiplications per trial")
bx.set_title("Measured decision cost, coset base g·μ_d (k = 20)", fontsize=9); bx.legend(fontsize=7); bx.grid(alpha=.3, which="both")
fig.text(.01, .005, "Sources: data/table.json (scripts/model.py), data/d2_coset.json (scripts/d2_coset.py). Model values from measured counts [pcc:unverified].", fontsize=7, color="#444")
fig.tight_layout(rect=(0, .02, 1, 1))
fig.savefig(D / "figures/oracle_search.svg", bbox_inches="tight"); fig.savefig(D / "figures/oracle_search.png", dpi=150, bbox_inches="tight")
