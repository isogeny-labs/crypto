"""Layout only: plots recorded values from ../results/*.json. Computes no result (AGENTS.md, no Python)."""
import json, math, os, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.join(HERE, "..", "results")
load = lambda f: json.load(open(os.path.join(RES, f)))
plt.rcParams.update({"font.size": 9, "svg.fonttype": "none"})

def save(fig, name):
    # SVG is canonical (embedded in REPORT.pdf); the PNG preview stays under the 64 KiB
    # artifact-store threshold for data files (tools/artifacts/artifacts.py) so it can live in Git.
    fig.savefig(os.path.join(HERE, f"{name}.svg"), bbox_inches="tight")
    fig.savefig(os.path.join(HERE, f"{name}.png"), dpi=90, bbox_inches="tight")

# ---- Fig 1: R2 spectral maximum relative to sqrt(q) ----
r2 = load("r2_v2.json")["rows"]
insts = list(dict.fromkeys(r["instance"] for r in r2))
cats = {
    "algebraic character (chi(x), chi(y), chi(x+1))": lambda r: r["field"] == "prime" and r["predicate"].startswith("chi"),
    "prime-field bit (lsb, middle bit)": lambda r: r["field"] == "prime" and not r["predicate"].startswith("chi"),
    "binary-field bit / trace functional": lambda r: r["field"] == "binary" and "R1" not in r["predicate"],
}
colors = ["#1f77b4", "#d62728", "#2ca02c"]
fig, ax = plt.subplots(figsize=(7.2, 3.6))
for (lab, pred), col in zip(cats.items(), colors):
    xs, ys = [], []
    for r in r2:
        if r["role"] == "test" and pred(r):
            xs.append(insts.index(r["instance"]))
            ys.append(r["M_over_sqrt_q"])
    ax.scatter(xs, ys, s=14, color=col, label=lab, alpha=0.8)
pcx = [insts.index(r["instance"]) for r in r2 if r["predicate"] == "planted_spectral"]
pcy = [r["M_over_sqrt_q"] for r in r2 if r["predicate"] == "planted_spectral"]
ax.scatter(pcx, pcy, s=14, marker="x", color="k", label="positive control (planted)")
# random-function reference: recorded M/sqrt(q) divided by recorded M/sqrt(n ln n) gives sqrt(n ln n)/sqrt(q)
ref = {}
for r in r2:
    if r["M_over_sqrt_nlnn"] and r["M_over_sqrt_nlnn"] > 0 and r["instance"] not in ref:
        ref[r["instance"]] = r["M_over_sqrt_q"] / r["M_over_sqrt_nlnn"]
ax.plot(range(len(insts)), [ref[i] for i in insts], "--", color="gray", lw=1, label="sqrt(n ln n)/sqrt(q) (random-function scale)")
ax.axhline(2.0, color="#1f77b4", lw=0.8, ls=":", label="2 sqrt(q) (Weil, rank-2 sheaf)")
ax.set_yscale("log")
ax.set_xticks(range(len(insts)))
ax.set_xticklabels([i.split("_n")[0].replace("_A1", "") for i in insts], rotation=60, ha="right", fontsize=7)
ax.set_ylabel("max_a |S_f(a)| / sqrt(q)")
ax.set_title("R2: largest group-character coefficient of predicate f(kG), exact over all k (n up to 2^20)")
ax.legend(fontsize=6.5, loc="upper center", bbox_to_anchor=(0.5, -0.42), ncol=2)
save(fig, "fig1_r2_spectrum")

# ---- Fig 2: R3 lifts and WHT ----
r3 = load("r3.json")
fig, (a1, a2) = plt.subplots(1, 2, figsize=(7.4, 3.2), gridspec_kw={"width_ratios": [1.4, 1]})
rows = [r for r in r3["rows"] if "lift" in r and r.get("lift") is not None]
ins3 = list(dict.fromkeys(r["instance"] for r in rows))
for r in rows:
    x = ins3.index(r["instance"])
    if r["role"] == "positive_control":
        a1.scatter(x, r["lift"], marker="x", color="k", s=16)
    else:
        a1.scatter(x, r["lift"], color="#d62728" if r["field"] == "prime" else "#2ca02c", s=10, alpha=0.7)
a1.axhline(1.10, color="gray", ls="--", lw=0.8)
a1.text(0, 1.12, "preregistered usefulness floor 1.10", fontsize=6.5, color="gray")
a1.set_yscale("log")
a1.set_xticks(range(len(ins3)))
a1.set_xticklabels([i.split("_n")[0].replace("_A1", "") for i in ins3], rotation=60, ha="right", fontsize=6.5)
a1.set_ylabel("lift = P(decomposes | f) / P(decomposes | not f)")
a1.set_title("R3: residue predicates as pre-filters (x = planted control)", fontsize=8)
wr = [r for r in r3["rows"] if r["predicate"] == "WHT_max_all_linear_functionals"]
wp = {r["instance"]: r for r in r3["rows"] if r["predicate"] == "planted_codim2_subspace"}
for i, r in enumerate(wr):
    a2.bar(i - 0.2, r["z_max"], width=0.4, color="#2ca02c")
    a2.bar(i + 0.2, wp[r["instance"]]["z_max"], width=0.4, color="k")
a2.set_yscale("log")
a2.set_xticks(range(len(wr)))
a2.set_xticklabels([r["instance"].split("_n")[0] for r in wr], rotation=60, ha="right", fontsize=6.5)
a2.set_ylabel("max |z| over 2^m linear functionals")
a2.set_title("R3 binary WHT: green = Semaev D, black = planted", fontsize=8)
save(fig, "fig2_r3_prefilter")

# ---- Fig 3: R4 eigenvalue orders vs bound ----
r4 = load("r4.json")["rows"]
fig, (b1, b2) = plt.subplots(1, 2, figsize=(7.6, 3.0))
fig.subplots_adjust(wspace=0.38)
fam = {"D=-7 norm-2": ("#9467bd", "o"), "Koblitz Frobenius (norm 2, ~free)": ("#2ca02c", "s")}
for r in r4:
    if r["family"] in fam:
        c, mk = fam[r["family"]]
        b1.scatter(r["s_min_bound"], r["s"], color=c, marker=mk, s=14)
lo, hi = 8, 30000
b1.plot([lo, hi], [lo, hi], "k--", lw=0.8)
b1.set_xscale("log"); b1.set_yscale("log")
b1.set_xlim(8, 25)
b1.set_xlabel("theorem bound s_min (norm d = 2)"); b1.set_ylabel("exact eigenvalue order s on <G>")
b1.set_title("R4: purple D=-7 (20 curves), green Koblitz tau", fontsize=8)
costs = [0, 0.01, 0.1, 0.5, 1.0]
for r in r4:
    if r["family"] in fam:
        c, _ = fam[r["family"]]
        b2.plot(costs[1:], r["net_factor_at_c"][1:], color=c, alpha=0.4, lw=0.8)
for r in r4:
    if r["family"].startswith("j="):
        b2.plot(costs[1:], r["net_factor_at_c"][1:], color="#ff7f0e", alpha=0.4, lw=0.8)
        break
b2.axhline(1, color="k", lw=0.8)
b2.set_xscale("log"); b2.set_yscale("log")
b2.set_xlabel("cost c of one endomorphism evaluation (group ops)")
b2.set_ylabel("sqrt(s) / (1 + s c)  (R4-cost model)")
b2.set_title("net rho factor; orange = j=0/1728 automorphism", fontsize=8)
save(fig, "fig3_r4_classes")
print("ok")
