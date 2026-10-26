"""Lay out recorded values only (AGENTS.md: Python never computes a result).
Reads results/*.jsonl, writes figures/*.svg and *.png (150 dpi)."""
import json, math, os, sys
from collections import defaultdict
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = os.path.dirname(os.path.abspath(__file__))
R = os.path.join(HERE, "results")
F = os.path.join(HERE, "figures")
os.makedirs(F, exist_ok=True)
plt.rcParams.update({"font.size": 9, "axes.grid": True, "grid.alpha": 0.3})
BLUE, ORANGE, GREY, GREEN = "#2a6fb0", "#d9822b", "#7a7a7a", "#3a9a5b"

def load(name):
    p = os.path.join(R, name)
    return [json.loads(l) for l in open(p)] if os.path.exists(p) else []

def save(fig, name):
    fig.tight_layout()
    fig.savefig(os.path.join(F, name + ".svg"))
    fig.savefig(os.path.join(F, name + ".png"), dpi=150)
    plt.close(fig)

att = [d for d in load("attack.jsonl") if d.get("solved")]
by = defaultdict(list)
for d in att:
    by[(d["arm"], d["p"])].append(d)
ps = sorted({d["p"] for d in att})

def rho_exp(d):
    # derived: expected rho steps times the measured seconds per step of that run
    return d["rho_expected_steps"] * d["rho_seconds"] / d["rho_steps"]

def med(v):
    v = sorted(v); n = len(v)
    return v[n // 2] if n % 2 else 0.5 * (v[n // 2 - 1] + v[n // 2])

# 1. wall time IC (both arms) vs rho, per p (points = runs, line = median)
fig, ax = plt.subplots(figsize=(6.4, 4.0))
for arm, col, lab in [("full", ORANGE, "IC on C, full factor base"), ("iota", BLUE, "IC on C, iota-halved factor base")]:
    xs = [d["p"] for d in att if d["arm"] == arm]
    ax.scatter(xs, [d["t_ic_total"] for d in att if d["arm"] == arm], s=10, color=col, alpha=0.5)
    ax.plot(ps, [med([d["t_ic_total"] for d in by[(arm, p)]]) for p in ps], color=col, label=lab + " (median)")
rho = [d for d in att if d["arm"] == "full"]
ax.scatter([d["p"] for d in rho], [d["rho_seconds"] for d in rho], s=10, color=GREY, alpha=0.5)
ax.plot(ps, [med([d["rho_seconds"] for d in by[("full", p)]]) for p in ps], color=GREY, label="Pollard rho on E(F_{q^4})[r] (median of measured runs)")
ax.plot(ps, [med([rho_exp(d) for d in by[("full", p)]]) for p in ps], color="black", ls=":", label="rho: sqrt(pi r/2) x measured s/step (median)")
ax.set_xscale("log"); ax.set_yscale("log")
ax.set_xlabel("q (prime)"); ax.set_ylabel("wall time per DLP [s]")
ax.set_title("Genus-5 cover IC vs matched rho, toy fields (n = %d solved runs)" % len(att))
ax.legend(fontsize=7)
save(fig, "ic_vs_rho_time")

# 2. ratio IC/rho
fig, ax = plt.subplots(figsize=(6.4, 3.6))
for arm, col in [("full", ORANGE), ("iota", BLUE)]:
    ax.scatter([d["p"] for d in att if d["arm"] == arm], [d["ic_over_rho_time"] for d in att if d["arm"] == arm], s=10, color=col, alpha=0.5)
    ax.plot(ps, [med([d["ic_over_rho_time"] for d in by[(arm, p)]]) for p in ps], color=col, label=arm)
for arm, col in [("full", ORANGE), ("iota", BLUE)]:
    ax.plot(ps, [med([d["t_ic_total"] / rho_exp(d) for d in by[(arm, p)]]) for p in ps], color=col, ls=":", label=arm + " vs expected-steps rho")
ax.axhline(1.0, color="black", lw=0.8, ls="--")
ax.set_xscale("log"); ax.set_yscale("log")
ax.set_xlabel("q (prime)"); ax.set_ylabel("IC time / rho time (same instance)")
ax.set_title("Distance remaining to matched rho (>1: rho faster)")
ax.legend(fontsize=7)
save(fig, "ic_over_rho_ratio")

# 3. phase breakdown (iota arm, medians)
fig, ax = plt.subplots(figsize=(6.4, 3.6))
bottom = [0.0] * len(ps)
for key, col, lab in [("t_relations", BLUE, "relations"), ("t_linear_algebra", GREEN, "linear algebra"), ("t_descent", ORANGE, "individual-log descent (2 targets)")]:
    vals = [med([d[key] for d in by[("iota", p)]]) for p in ps]
    ax.bar([str(p) for p in ps], vals, bottom=bottom, color=col, label=lab)
    bottom = [b + v for b, v in zip(bottom, vals)]
ax.set_xlabel("q"); ax.set_ylabel("median wall time [s]")
ax.set_title("Phase breakdown, iota arm")
ax.legend(fontsize=7)
save(fig, "phase_breakdown")

# 4. unknowns and relations, full vs iota
fig, ax = plt.subplots(figsize=(6.4, 3.6))
for arm, col in [("full", ORANGE), ("iota", BLUE)]:
    ax.plot(ps, [med([d["unknowns"] for d in by[(arm, p)]]) for p in ps], color=col, marker="o", label=arm + ": unknowns")
    ax.plot(ps, [med([d["rel_candidates"] + d["quad_candidates"] for d in by[(arm, p)]]) / 100 for p in ps], color=col, ls="--", marker="x", label=arm + ": relation candidates / 100")
ax.set_xscale("log"); ax.set_yscale("log"); ax.set_xlabel("q"); ax.set_title("Linear system size and relation search, medians")
ax.legend(fontsize=7)
save(fig, "iota_halving")

# 5. classifier: genus vs excess for d = 4, g0 = 1
cls = load("classify.jsonl")
rows = [r for r in cls if r["d"] == 4 and r["g0"] == 1 and r["conjugates_distinct"]]
fig, ax = plt.subplots(figsize=(6.4, 3.6))
cnt = defaultdict(int)
for r in rows:
    cnt[(r["m"], r["e"])] += 1
for (m, e), c in cnt.items():
    ax.scatter(e, m, s=12 + 4 * c, color=BLUE, alpha=0.6)
    ax.annotate(str(c), (e, m), fontsize=6, ha="center", va="center")
t = [r for r in rows if r["target_family"]][0]
ax.scatter([t["e"]], [t["m"]], s=200, facecolors="none", edgecolors="red", label="quartic / genus-5 family (m=4, S=5, e=1)")
ax.set_xlabel("excess genus e = g - 4"); ax.set_ylabel("rank m of the Frobenius module")
ax.set_title("Binary-pattern classifier, d = 4, source genus 1 (%d patterns)" % len(rows))
ax.legend(fontsize=7)
save(fig, "classifier_d4_g1")
print("figures written:", sorted(os.listdir(F)))

# 6. structure diagram of the construction (no data)
fig, ax = plt.subplots(figsize=(6.8, 4.2))
ax.axis("off"); ax.set_xlim(0, 10); ax.set_ylim(0, 6.4)
def box(x, y, txt, col):
    ax.text(x, y, txt, ha="center", va="center", fontsize=8, bbox=dict(boxstyle="round,pad=0.4", fc="white", ec=col, lw=1.4))
def arrow(x0, y0, x1, y1, txt, col="black"):
    ax.annotate("", xy=(x1, y1), xytext=(x0, y0), arrowprops=dict(arrowstyle="->", color=col, lw=1.1))
    ax.text((x0 + x1) / 2, (y0 + y1) / 2 + 0.18, txt, fontsize=7, ha="center", color=col)
box(5, 5.5, "C / F_q : twisted Humbert curve, genus 5\nw_i^2 = eta^(q^i) (x - a^(q^i)),  i = 0..3\nPhi: (x, w_0..w_3) -> (x^q, w_3^q, w_0^q, w_1^q, w_2^q)", BLUE)
box(1.6, 2.9, "E / F_{q^4}\ny^2 = eta (x-a)(x-a^q)(x-a^{q^2})\ntarget subgroup of order r", ORANGE)
box(8.4, 2.9, "D / F_q  (extra factor, e = 1)\ny^2 = N(eta) m_a(x)\n#D(F_q) ~ q, r not | #D", GREEN)
box(5, 0.7, "P^1_x over F_q\nbranch points a, a^q, a^{q^2}, a^{q^3}, oo  (S = 5)", GREY)
arrow(4.0, 4.85, 2.2, 3.55, "", ORANGE)
ax.text(3.75, 4.3, "pi_0 = (x, w0 w1 w2 / mu)\ndeg 8", fontsize=7, color=ORANGE, ha="left")
arrow(6.0, 4.85, 7.8, 3.55, "", GREEN)
ax.text(7.05, 4.4, "C -> C/iota, deg 2", fontsize=7, color=GREEN, ha="left")
ax.annotate("", xy=(5, 1.25), xytext=(5, 4.85), arrowprops=dict(arrowstyle="->", color=GREY, lw=1.1))
ax.text(5.12, 4.15, "x, deg 16 = |(Z/2)^4|", fontsize=7, ha="left", color=GREY)
ax.annotate("", xy=(2.3, 3.55), xytext=(3.5, 4.85), arrowprops=dict(arrowstyle="->", color=BLUE, lw=1.1, ls="--", connectionstyle="arc3,rad=-0.3"))
ax.text(2.25, 4.35, "T = Tr_Phi o pi_0^*\npi_0* T = [8]", fontsize=7, color=BLUE, ha="right")
ax.text(5, 2.3, "Jac C ~ Res_{F_{q^4}/F_q} E  x  D\niota = (w -> -w): -1 on Res E, +1 on D", fontsize=7.5, ha="center", bbox=dict(boxstyle="round,pad=0.3", fc="#f4f4f4", ec="none"))
save(fig, "construction_diagram")
