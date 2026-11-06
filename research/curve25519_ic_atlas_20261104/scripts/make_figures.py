"""Plot recorded values only (no computation beyond the ratios the report states)."""
import json, pathlib
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import Rectangle, Patch

ROOT = pathlib.Path(__file__).resolve().parents[2]
OUT = pathlib.Path(__file__).resolve().parents[1] / "figures"
OUT.mkdir(exist_ok=True)
band = json.load(open(ROOT / "curve25519_t10_winner_ecbench_20261104/ecbench/band_pooled.json"))
S = {r["arm"]: r["mean_s"] for r in band["rows"]}

def save(fig, name):
    fig.savefig(OUT / f"{name}.svg", bbox_inches="tight")
    fig.savefig(OUT / f"{name}.png", dpi=150, bbox_inches="tight")
    plt.close(fig)

REF, BASE, PKM, WIN = "#555555", "#2a7ab0", "#d9822b", "#b03060"
# ---- Fig 1
arms = [("rho-neg", "rho-neg\n(reference)", REF), ("proj-a", "proj-a\n(baseline)", BASE),
        ("pkm-iso-t6-g4", "pkm-iso t6 G4\nseed 0", PKM), ("pkm-iso-t6-g4-s1308", "pkm-iso t6 G4\nseed 1308", PKM),
        ("pkm-iso-t5-g4-s1307", "pkm-iso t5 G4 seed 1307\n(T10 winner)", WIN)]
ws = ["0", "0.1", "1"]
fig, axs = plt.subplots(1, 3, figsize=(13, 5.4), sharey=True)
for i, ax in enumerate(axs):
    for j, (k, lab, c) in enumerate(arms):
        v = S[k][i]
        ax.bar(j, v, color=c, hatch="//" if k == "rho-neg" else None, edgecolor="white" if k != "rho-neg" else "black")
        ax.text(j, v * 1.25, f"{v:.4g}" if v < 1e3 else f"{v:.3e}", ha="center", fontsize=8)
    ax.set_yscale("log"); ax.set_ylim(0.5, 1e9)
    ax.set_xticks(range(5)); ax.set_xticklabels([a[1] for a in arms], rotation=60, ha="right", fontsize=8)
    ax.set_title(f"w = {ws[i]} GAE per native op"); ax.grid(axis="y", alpha=.3, which="major")
    if i == 1:
        ax.annotate("winner is 6.06e4x\nrho-neg at w = 0.1", xy=(4, S["pkm-iso-t5-g4-s1307"][1]*2.6), xytext=(2.6, 5e7),
                    arrowprops=dict(arrowstyle="->"), fontsize=9, ha="center")
axs[0].set_ylabel("S = (counted GAE + w x native ops) / sqrt(r)")
fig.suptitle("Curve25519 family k = 20 (p = 1,048,573, r = 131,203): T12 session, 16 targets, mean S per full solve", fontsize=11)
fig.legend(handles=[Patch(facecolor=REF, hatch="//", edgecolor="black", label="reference (rho-neg)"),
                    Patch(color=BASE, label="generic baseline (proj-a)"), Patch(color=PKM, label="pkm-iso seeds 0 / 1308"),
                    Patch(color=WIN, label="T10 winner")], loc="lower center", ncol=4, bbox_to_anchor=(.5, -.02))
fig.text(.01, -.07, "Source: research/curve25519_t10_winner_ecbench_20261104/ecbench/band_pooled.json (T12, 2 rounds, 32 verified runs per arm); REPORT.md section 6.",
         fontsize=7, color="#444")
fig.tight_layout(rect=(0, .03, 1, .95)); save(fig, "ledger_k20")

# ---- Fig 2
pts = [("2026-11-02", "algebraic ecbench\n(best arm)", 9.55e5, "curve25519_algebraic_ecbench_20261102"),
       ("2026-11-03", "T5 seed 1093\n(its own 6 targets)", 6.9e5, "T5 ECBS1h405f20219036"),
       ("2026-11-03", "T9 seed 1308", 4.73e5, "T9, wave 3"),
       ("2026-11-04", "T12 winner\nt5 G4 seed 1307", 6.06e4, "T12, 16 targets"),
       ("2026-11-04", "winner + trace\nreplay (L12)", 2.63e4, "L12, 8 targets"),
       ("2026-11-04", "t4 G3 seed 1193\n+ trace (T13)", 1.83e4, "T13, 16 targets")]
fig, ax = plt.subplots(figsize=(9, 5))
xs = range(len(pts)); ys = [p[2] for p in pts]
ax.plot(xs, ys, "-o", color=WIN, lw=2)
for x, p in zip(xs, pts):
    ax.annotate(f"{p[2]:.3g}x\n{p[1]}\n{p[0]}", (x, p[2]), textcoords="offset points", xytext=(0, 12), ha="center", fontsize=8)
ax.set_yscale("log"); ax.set_ylim(8e3, 5e6); ax.set_xlim(-.5, len(pts) - .5)
ax.set_xticks(list(xs)); ax.set_xticklabels([p[3] for p in pts], fontsize=7, rotation=10)
ax.set_ylabel("best algebraic arm / rho-neg, S at w = 0.1"); ax.grid(alpha=.3)
ax.set_title("Algebraic track: distance remaining to rho-neg at w = 0.1 (rho-neg = 1x)")
ax.text(.5, -.3, "Target sets differ between points (6, 16, 16, 8 and 16 targets, different sessions); not a paired series.", transform=ax.transAxes, ha="center", fontsize=8)
fig.text(.01, -.06, "Source: REPORT.md sections 6 and 11; pooled_compare.jsonl (T12, L12, T13), curve25519_isogeny_seed_20261103/ecbench/band.json (T5).", fontsize=7, color="#444")
fig.tight_layout(); save(fig, "algebraic_progress")

# ---- Fig 3
C = {"closed": "#c9c9c9", "done": "#8fcf9a", "running": "#f2d16b", "design": "#a9c4e8"}
leads = [("coordinates\n#3359", "closed", "selection is\nthe lever"), ("T1", "closed", "1.00-1.12x"), ("T2", "closed", "degree\npenalty"),
 ("T3", "closed", "m=3 too dear"), ("T4/T5", "done", "0.641\n[0.379, 1.084]"), ("T6", "closed", "real, useless\nat best cells"),
 ("T7", "closed", ""), ("T8", "design", "deprioritised\n(degree)"), ("T9", "done", "0.5605\n[0.421, 0.747]"),
 ("T10", "done", "t5 G4 seed 1307\nmodelled 18.65x"), ("T11", "done", "0.437\n[0.300, 0.637]"),
 ("T12", "done", "0.0578\n[0.047, 0.072]"), ("L10", "done", "r^1.03;\noptimal t = 4"), ("L11", "design", "wave 4\ndesign only"),
 ("L12", "done", "trace 0.474\nend to end"), ("L13", "done", "F4 42x above\nits grid"), ("L14", "done", "greedy digits\n2.1-2.2x coverage"),
 ("L15", "done", "0.48x / 0.44x\nper relation"), ("T13", "done", "t4 + trace\n0.284x winner")]
fig, ax = plt.subplots(figsize=(11, 6.2)); cols = 6
for i, (n, s, t) in enumerate(leads):
    r, c = divmod(i, cols)
    ax.add_patch(Rectangle((c, -r), .94, .9, color=C[s], ec="black", lw=.6))
    ax.text(c + .47, -r + .68, n, ha="center", va="center", fontsize=10, weight="bold")
    ax.text(c + .47, -r + .28, t, ha="center", va="center", fontsize=8)
ax.set_xlim(-.05, cols); ax.set_ylim(-3.7, 1.0); ax.axis("off")
ax.legend(handles=[Patch(color=C["closed"], label="closed"), Patch(color=C["done"], label="done"),
                   Patch(color=C["running"], label="running"), Patch(color=C["design"], label="design-only / deprioritised")],
          loc="lower center", ncol=4, bbox_to_anchor=(.5, -.02), frameon=False)
ax.set_title("Curve25519 IC atlas: lead status (REPORT.md section 9). Cell text is the headline number as the report gives it.", fontsize=10)
fig.text(.01, .01, "Source: REPORT.md section 9 (status 2026-11-04).", fontsize=7, color="#444")
save(fig, "leads_map")
