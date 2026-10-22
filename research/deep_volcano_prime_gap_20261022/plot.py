"""Lay out the recorded values of research/deep_volcano_prime_gap_20261022.

No computation beyond arranging numbers that volcano-stats and volcano-gap
wrote (AGENTS.md "Implementation language: no Python"): analysis*.json,
run/pair.json and the curves.jsonl sidecars.

usage: python plot.py STUDY_DIR
"""
import json
import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402
from matplotlib.patches import FancyArrowPatch, Polygon  # noqa: E402

study = Path(sys.argv[1])
sys.path.insert(0, str(study.resolve().parents[1] / "tools" / "volcano-diagram"))
from volcano_diagram import render  # noqa: E402

run = study / "run"
figs = study / "figures"
figs.mkdir(exist_ok=True)
A = json.loads((study / "analysis.json").read_text())
pair = json.loads((run / "pair.json").read_text())
plants = {
    "S2 × 1.05 (planted)": json.loads((study / "analysis-plant-up.json").read_text()),
    "S2 × 0.95 (planted)": json.loads((study / "analysis-plant-down.json").read_text()),
}
cl = pair["class"]
N, depth, p = cl["gap_prime"], cl["two_depth"], cl["p"]
SIDE = {0: "S1 (conductor 2^a)", 1: f"S2 (conductor {N}·2^a)"}
COL = {0: "#2a6fdb", 1: "#e07b00"}
sessions = {
    s: json.loads((run / "sessions" / s / "session.json").read_text()).get("session_id", s)
    for s in ["spec", "spec-aa", "spec-bsgs"]
}


def metric(doc, arm):
    return next(m for m in doc["metrics"] if m["arm"] == arm and m["metric"] == "S")


def save(fig, name):
    fig.savefig(figs / f"{name}.svg", bbox_inches="tight")
    fig.savefig(figs / f"{name}.png", dpi=150, bbox_inches="tight")
    plt.close(fig)


rho = metric(A, "rho-neg")
g = rho["gap"]

# 1. The two volcanoes, in the standard diagram, coloured by mean S per curve.
for c, side in [(0, "s1"), (1, "s2")]:
    lines = [json.loads(x) for x in (run / f"export-{side}" / "curves.jsonl").read_text().splitlines()]
    model0 = [x for x in lines if x["model"] == 0]
    means = {m["vertex"]: m for m in g["curve_means"] if m["class"] == c}
    spec = {
        "schema": "volcano-diagram/v2",
        "ell": 2,
        "depth": depth,
        "title": f"{SIDE[c]} · 2-volcano over F_p, p = {p} · rho.negation, "
        f"{means[0]['runs']} paired runs per curve (2 models × 6 targets)",
        "subtitle": f"session {sessions['spec']} · gap prime N = {N}",
        "footnote": "curves per level " + ", ".join(
            f"L{lv}: {sum(1 for x in model0 if x['level'] == lv)}" for lv in range(depth + 1)
        ) + " · every curve measured · root ringed",
        "metrics": [
            {"key": "s", "title": "mean S = GAE/√r", "label": "S per solve"},
            {"key": "ms", "title": "mean solve wall time (L0, descriptive)", "label": "ms per solve"},
        ],
        "vertices": [
            {
                "id": x["vertex"],
                "label": f"{'a' if c == 0 else 'b'}{x['vertex']}",
                "level": x["level"],
                "nbrs": x["nbrs"],
                "values": {"s": means[x["vertex"]]["mean_s"], "ms": means[x["vertex"]]["mean_wall_ms"]},
                **({"highlight": "root"} if x["vertex"] == 0 else {}),
            }
            for x in model0
        ],
    }
    (figs / f"volcano-{side}.diagram.json").write_text(json.dumps(spec, indent=1))
    render(spec, str(figs / f"volcano-{side}"))

# 2. Per-level means on both sides, and the per-level ratio with the margin.
fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(12, 4.6))
lm = {(m["class"], m["level"]): m for m in g["level_means"]}
for c in (0, 1):
    xs = [lv + (-0.12 if c == 0 else 0.12) for lv in range(depth + 1)]
    ys = [lm[(c, lv)]["mean_s"] for lv in range(depth + 1)]
    es = [lm[(c, lv)]["ci95_half"] for lv in range(depth + 1)]
    ax1.errorbar(xs, ys, yerr=es, fmt="o", color=COL[c], capsize=3, label=SIDE[c])
ax1.axhline(1.0, color="#888", lw=0.8, ls=":")
ax1.set_xticks(range(depth + 1))
ax1.set_xticklabels([f"L{lv}\n{lm[(0, lv)]['curves']}+{lm[(1, lv)]['curves']} curves" for lv in range(depth + 1)], fontsize=8)
ax1.set_ylabel("mean S = GAE/√r (95% CI, runs)")
ax1.set_title("rho.negation, mean S per 2-level, each side", fontsize=10)
ax1.legend(fontsize=8)
pl = g["per_level"]
ax2.axhspan(1 - g["margin"], 1 + g["margin"], color="#cfe8cf", alpha=0.6, label="±5% equivalence margin")
for row in pl:
    lv = row["level"]
    ax2.plot([lv, lv], row["ci95"], color="#444", lw=1.2)
    ax2.plot([lv], [row["b_over_a"]], "o", color="#b03a8c")
ax2.axhline(1.0, color="#888", lw=0.8)
ax2.errorbar([depth + 1.2], [g["b_over_a"]], yerr=[[g["b_over_a"] - g["ci90"][0]], [g["ci90"][1] - g["b_over_a"]]],
             fmt="D", color="#000", capsize=4, label=f"pooled G = {g['b_over_a']:.4f}, 90% CI")
ax2.set_xticks(list(range(depth + 1)) + [depth + 1.2])
ax2.set_xticklabels([f"L{lv}" for lv in range(depth + 1)] + ["all"])
ax2.set_ylabel("S2 / S1 mean S (95% curve bootstrap)")
ax2.set_title(f"Across the gap N = {N}: ratio by level", fontsize=10)
ax2.legend(fontsize=8, loc="upper right")
fig.suptitle(f"Session {sessions['spec']} · {g['runs']} runs · p ≈ 2^52, n ≈ 2^40.8", fontsize=9)
save(fig, "gap-by-level")

# 3. The controls: the primary G beside the A/A arm, BSGS and the planted shifts.
rows = [("rho-neg (primary)", g)]
rows.append(("rho-neg-aa (A/A arm)", metric(A, "rho-neg-aa")["gap"]))
rows.append(("bsgs-neg (exactness)", metric(A, "bsgs-neg")["gap"]))
for name, doc in plants.items():
    rows.append((name, metric(doc, "rho-neg")["gap"]))
fig, ax = plt.subplots(figsize=(8, 3.6))
ax.axvspan(0.95, 1.05, color="#cfe8cf", alpha=0.6, label="±5% margin")
for i, (name, gg) in enumerate(rows):
    y = len(rows) - 1 - i
    ax.plot(gg["ci95"], [y, y], color="#999", lw=4, solid_capstyle="butt")
    ax.plot(gg["ci90"], [y, y], color="#333", lw=2)
    ax.plot([gg["b_over_a"]], [y], "o", color="#b03a8c")
    ax.text(1.115, y, f"{gg['b_over_a']:.4f}  p={gg['perm_p']:.4f}", va="center", fontsize=8)
ax.axvline(1.0, color="#888", lw=0.8)
ax.set_yticks(range(len(rows)))
ax.set_yticklabels([r[0] for r in reversed(rows)], fontsize=8)
ax.set_xlim(0.88, 1.20)
ax.set_xlabel("S2 / S1 mean S (thin: 90% CI, thick grey: 95% CI)")
ax.set_title("Primary contrast and controls", fontsize=10)
ax.legend(fontsize=8, loc="lower right")
save(fig, "controls")

# 4. Explanatory: the class, its two sides and the maps that join them.
fig, ax = plt.subplots(figsize=(10, 4.8))
ax.set_xlim(0, 10)
ax.set_ylim(-0.6, 7.4)
ax.axis("off")
for c, x0 in [(0, 2.2), (1, 7.8)]:
    ax.add_patch(Polygon([[x0, 6.6], [x0 - 1.9, 0.4], [x0 + 1.9, 0.4]], closed=True, fc=COL[c], alpha=0.15, ec=COL[c]))
    for lv in range(depth + 1):
        y = 6.6 - lv * (6.2 / depth)
        cnt = 1 if lv == 0 else 3 * 2 ** (lv - 1)
        cond = f"2^{lv}" if c == 0 else (f"N·2^{lv}" if lv else "N")
        ax.text(x0, y, f"L{lv}: {cnt} · c = {cond}", ha="center", va="center", fontsize=7.5)
    ax.text(x0, 7.15, SIDE[c] + ("  (one volcano, j = −32768 on top)" if c == 0 else f"  ({N - 1} volcanoes like this one)"),
            ha="center", fontsize=8.5, color=COL[c], weight="bold")
ax.add_patch(FancyArrowPatch((3.3, 6.4), (6.7, 6.4), arrowstyle="-|>", mutation_scale=14, color="#333"))
ax.text(5, 6.65, f"φ: degree N = {N}, descending (Vélu, twist-rational kernel)", ha="center", fontsize=8)
ax.add_patch(FancyArrowPatch((6.7, 5.9), (3.3, 5.9), arrowstyle="-|>", mutation_scale=14, color="#777"))
ax.text(5, 5.55, "ψ: the dual, ascending; ψ∘φ = ±[N] checked", ha="center", fontsize=8, color="#555")
iso = pair["isogeny"]
ax.text(5, 3.4, f"build φ: {iso['construct_field_mults']:.3g} F_p mults · evaluate: {iso['eval_field_mults_per_point']:.3g} per point",
        ha="center", fontsize=8)
ax.text(5, 2.9, "2-isogenies (Vélu) carry the instance to all 190 curves of each side", ha="center", fontsize=8)
ax.text(5, 0.0, f"t² − 4p = −11·(2^6·N)², t ≡ −2 (mod N), #E = {cl['cofactor']}·n, n = {cl['n']}", ha="center", fontsize=8)
ax.set_title(f"One ECDLP instance, two deep 2-volcanoes across a prime conductor gap (p = {p})", fontsize=10)
save(fig, "class-diagram")
print("figures ->", figs)
