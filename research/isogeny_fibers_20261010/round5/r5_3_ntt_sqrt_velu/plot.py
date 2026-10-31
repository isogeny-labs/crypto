"""Plot recorded values only (threshold.json, rational*.json, generic_*.json,
analysis.json written by examples/r5_3_ntt_sqrt_velu.rs). No computation
beyond log2 of recorded numbers for axes and drawing recorded fits."""
import json
import math
import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

D = Path(sys.argv[1] if len(sys.argv) > 1 else ".")


def rows(*names):
    out = []
    for n in names:
        p = D / n
        if p.exists():
            out += json.loads(p.read_text())["rows"]
    return out


rat = rows("rational.json", "rational_hi.json")
gk = rows("generic_karatsuba.json")
gn = rows("generic_ntt.json", "generic_ntt_hi.json")
thr = json.loads((D / "threshold.json").read_text())
ana = json.loads((D / "analysis.json").read_text())

C = ["#2a78d6", "#eb6834", "#1baf7a", "#eda100", "#e87ba4", "#008300"]
INK, INK2 = "#0b0b0b", "#52514e"
plt.rcParams.update({"font.size": 9, "axes.edgecolor": INK2, "axes.labelcolor": INK,
                     "xtick.color": INK2, "ytick.color": INK2, "svg.fonttype": "none"})
lg = math.log2


def fit(series, rng):
    for f in ana["fits"]:
        if f["series"].startswith(series) and f["range"] == rng:
            return f
    raise KeyError((series, rng))


def line(ax, f, x0, x1, color):
    xs = [x0, x1]
    ax.plot(xs, [f["log2_constant"] + f["exponent"] * x for x in xs], color=color,
            lw=1, ls="--", alpha=0.8)


# Figure 1: sqrt-elu counted cost, Karatsuba vs NTT, plus Velu.
fig, ax = plt.subplots(figsize=(7.2, 4.6))
s0 = [r for r in rat if r["seed"] == 0]
ax.plot([lg(r["ell"]) for r in s0 if r["velu_1pt"]], [lg(r["velu_1pt"]["Meq"]) for r in s0 if r["velu_1pt"]],
        "s", ms=3.5, color=C[1], label="Vélu, codomain + 1 image")
ax.plot([lg(r["ell"]) for r in s0], [lg(r["sqrtvelu_karatsuba_1pt"]["Meq"]) for r in s0], "^", ms=3.5,
        color=C[0], label="√élu, Karatsuba (round-4 arithmetic)")
ax.plot([lg(r["ell"]) for r in s0], [lg(r["sqrtvelu_ntt_1pt"]["Meq"]) for r in s0], "o", ms=3.5,
        color=C[2], label="√élu, NTT above the threshold (W counted as 1 M)")
top = max(lg(r["ell"]) for r in rat)
rng = f"2^16 <= ell <= 2^{top:.1f}"
fk, fn = fit("sqrtvelu_karatsuba_1pt", rng), fit("sqrtvelu_ntt_1pt", rng)
line(ax, fk, 16, top, C[0])
line(ax, fn, 16, top, C[2])
line(ax, fit("velu_1pt", "ell >= 2^10"), 10, 24, C[1])
ax.set_xlabel("log₂ ℓ")
ax.set_ylabel("log₂ cost (Meq = M + W + 91 I)")
ax.set_title(f"√élu exponent ℓ ≥ 2^16: Karatsuba {fk['exponent']:.3f}, NTT {fn['exponent']:.3f} (measured, 61-bit p)",
             fontsize=9)
ax.legend(fontsize=8, frameon=False)
ax.grid(alpha=0.25)
fig.tight_layout()
fig.savefig(D / "fig1_sqrtvelu_cost.svg")
fig.savefig(D / "fig1_sqrtvelu_cost.png", dpi=160)

# Figure 2: local exponents over 4-octave windows.
fig, ax = plt.subplots(figsize=(7.2, 3.6))
loc = ana["local_exponents"]
mid = [(w["log2_ell_from"] + w["log2_ell_to"]) / 2 for w in loc]
for key, col, name in [("karatsuba", C[0], "Karatsuba"), ("ntt", C[2], "NTT")]:
    ys = [w[key][0] for w in loc]
    ax.errorbar(mid, ys, yerr=[[w[key][0] - w[key][1] for w in loc], [w[key][2] - w[key][0] for w in loc]],
                fmt="o-", ms=3, color=col, lw=1, capsize=2, label=name)
ax.axhline(0.82, color=C[0], ls=":", lw=0.8)
ax.text(mid[0], 0.825, "round 4: 0.82", fontsize=7, color=C[0])
ax.axhline(0.5, color=INK2, ls=":", lw=0.8)
ax.text(mid[0], 0.505, "½ (theory, without log factors)", fontsize=7, color=INK2)
ax.set_xlabel("window centre, log₂ ℓ (window = 4 octaves)")
ax.set_ylabel("local exponent [95 % CI]")
ax.legend(fontsize=8, frameon=False)
ax.grid(alpha=0.25)
fig.tight_layout()
fig.savefig(D / "fig2_local_exponent.svg")
fig.savefig(D / "fig2_local_exponent.png", dpi=160)

# Figure 3: generic kernel finding per attempt and F_p^k product cost.
fig, (a1, a2) = plt.subplots(1, 2, figsize=(7.2, 3.4))
for rs, col, name in [(gk, C[0], "Karatsuba"), (gn, C[2], "NTT (every product)")]:
    if not rs:
        continue
    a1.plot([lg(r["ell"]) for r in rs], [lg(r["ladder_per_attempt_Meq"]) for r in rs], "o", ms=3,
            color=col, label=name)
    a2.plot([lg(r["k"]) for r in rs], [lg(r["ext_mul"]["Meq"]) for r in rs], "o", ms=3, color=col,
            label=name)
a1.set_xlabel("log₂ ℓ")
a1.set_ylabel("log₂ Meq, ladder in F_{p^k} per attempt")
a2.set_xlabel("log₂ k")
a2.set_ylabel("log₂ Meq, one F_{p^k} product")
for a in (a1, a2):
    a.grid(alpha=0.25)
    a.legend(fontsize=7, frameon=False)
fig.tight_layout()
fig.savefig(D / "fig3_generic.svg")
fig.savefig(D / "fig3_generic.png", dpi=160)

# Figure 4: threshold measurement (count ratio and contended wall ratio).
fig, ax = plt.subplots(figsize=(7.2, 3.4))
tr = [r for r in thr["rows"] if "la" in r and r["p"] == 2305843009213693951]
for unb, mk in [(False, "o-"), (True, "s--")]:
    sub = [r for r in tr if (r["la"] != r["lb"]) == unb]
    tag = "8n×n" if unb else "n×n"
    ax.plot([lg(r["lb"]) for r in sub], [r["count_ratio_ntt_over_karatsuba"] for r in sub], mk, ms=3,
            color=C[2], label=f"counted Meq ratio NTT/Karatsuba ({tag})")
    ax.plot([lg(r["lb"]) for r in sub], [r["wall_ratio_ntt_over_karatsuba"] for r in sub], mk, ms=3,
            color=C[3], label=f"wall ratio, contended ({tag})")
ax.axhline(1, color=INK2, lw=0.8)
ax.axvline(lg(thr["library_NTT_THRESHOLD"]), color=INK2, lw=0.8, ls=":")
ax.set_yscale("log")
ax.set_xlabel("log₂ n (shorter operand), p = 2^61 − 1")
ax.set_ylabel("NTT / Karatsuba")
ax.legend(fontsize=7, frameon=False)
ax.grid(alpha=0.25)
fig.tight_layout()
fig.savefig(D / "fig4_threshold.svg")
fig.savefig(D / "fig4_threshold.png", dpi=160)

# Figure 5: crossover table (g = 6).
fig, ax = plt.subplots(figsize=(7.2, 4.0))
tabs = [t for t in ana["crossover"] if t["g_mults_per_group_op"] == 6]
models = [r["model"] for r in tabs[0]["rows"]]
w = 0.8 / len(tabs)
for i, t in enumerate(tabs):
    ys = [r["log2_ell_star"] if r["log2_ell_star"] is not None else 0 for r in t["rows"]]
    ax.barh([j + i * w for j in range(len(models))], ys, height=w, color=C[i],
            label=f"r = 2^{int(t['log2_r'])} (ℓ_max = 2^{int(t['log2_ell_max'])})")
    ax.axvline(t["log2_ell_max"], color=C[i], lw=0.8, ls=":")
ax.set_yticks([j + w for j in range(len(models))])
ax.set_yticklabels(models, fontsize=6.5)
ax.invert_yaxis()
ax.set_xlabel("log₂ ℓ* (DERIVED, extrapolated; dotted: ℓ_max = 2√p)")
ax.legend(fontsize=7, frameon=False, loc="lower right")
fig.tight_layout()
fig.savefig(D / "fig5_crossover.svg")
fig.savefig(D / "fig5_crossover.png", dpi=160)
print("ok")
