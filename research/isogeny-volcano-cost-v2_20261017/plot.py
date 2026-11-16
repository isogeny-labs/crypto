"""Draw the recorded results of PROTOCOL v2 / v2.1 (analysis/*.json from
volcano-stats, band/*.json from ecbench band, classes/*/curves.jsonl): no
computation beyond arranging recorded values (AGENTS.md "Implementation
language").

usage: python plot.py STUDY_DIR
"""
import json
import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

study = Path(sys.argv[1])
sys.path.insert(0, str(Path(__file__).resolve().parents[2] / "tools" / "volcano-diagram"))
from volcano_diagram import render  # noqa: E402

out = study / "figures"
out.mkdir(exist_ok=True)
A = lambda name: json.loads((study / "analysis" / f"{name}.json").read_text())  # noqa: E731


def metric(a, arm, met):
    return next(m for m in a["metrics"] if m["arm"] == arm and m["metric"] == met)


def save(fig, name):
    fig.savefig(out / f"{name}.svg", bbox_inches="tight")
    fig.savefig(out / f"{name}.png", dpi=150, bbox_inches="tight")
    plt.close(fig)


classes = sorted(p.name for p in (study / "classes").iterdir() if p.is_dir())

# 1. Volcano heat maps per class, from that class's own analysis: rho S and IC yield, % from the class mean.
for name in classes:
    a = A(f"class-{name}")
    rows = [json.loads(l) for l in open(study / "classes" / name / "curves.jsonl")]
    m0 = [r for r in rows if r["model"] == 0]
    ell = 2 if name.startswith("l2") else 3
    val = {}
    for key, (arm, met) in (("rho", ("rho-neg", "S")), ("ic", ("ic-mitm", "relation_yield"))):
        for c in metric(a, arm, met).get("curves", []):
            val.setdefault(c["vertex"], {})[key] = 100 * (c["rel_mean"] - 1)
    spec = {
        "schema": "volcano-diagram/v2",
        "ell": ell,
        "depth": max(r["level"] for r in m0),
        "title": f"{name}: {ell}-isogeny volcano over F_{m0[0]['p']} · crater curves × 16 models, lower levels × 4, 8 paired targets",
        "subtitle": "PROTOCOL v2.1 (research/isogeny-volcano-cost-v2_20261017) · % from the class mean",
        "metrics": [
            {"key": "rho", "title": "rho.negation · mean S", "label": "% from class mean S"},
            {"key": "ic", "title": "ic.pipeline mitm · relation yield", "label": "% from class mean yield"},
        ],
        "vertices": [
            {"id": r["vertex"], "label": f"v{r['vertex']}", "level": r["level"], "nbrs": r["nbrs"],
             "values": val.get(r["vertex"], {}), "highlight": "root" if r["vertex"] == 0 else None}
            for r in m0
        ],
        "footnote": "Colours span each panel's observed range. Read them against the per-curve CIs and the "
                    "horizontal test in analysis/class-<name>.json before reading a pattern.",
    }
    (out / f"volcano-{name}.diagram.json").write_text(json.dumps(spec, indent=1))
    render(spec, str(out / f"volcano-{name}"))

# 2. Hypothesis forest: H1 (ℓ = 2 IC yield), H2 (ℓ = 3 IC yield), H3 (rho, all), v2 and v2.1 side by side.
rows = [
    ("H1  ℓ=2 IC relation yield (one-sided)", "H1-l2", "ic-mitm", "relation_yield"),
    ("H2  ℓ=3 IC relation yield", "H2-l3", "ic-mitm", "relation_yield"),
    ("H3  rho S, all classes", "H3H4-all", "rho-neg", "S"),
    ("H5  ℓ=2 IC S (descriptive)", "H1-l2", "ic-mitm", "S"),
    ("H5  ℓ=3 IC S (descriptive)", "H2-l3", "ic-mitm", "S"),
    ("    ℓ=2 rho S (exploratory)", "H1-l2", "rho-neg", "S"),
    ("    ℓ=3 rho S (exploratory)", "H2-l3", "rho-neg", "S"),
]
fig, ax = plt.subplots(figsize=(8, 0.62 * len(rows) + 1.6))
ax.axvspan(0.95, 1.05, color="#dfe8f5", label="equivalence margin ±5%")
ax.axvline(1, color="#888", lw=0.8)
for i, (lab, suf, arm, met) in enumerate(rows):
    y = len(rows) - 1 - i
    for dy, ver, col in ((0.17, "v2", "#8a8f99"), (-0.17, "v2.1", "#d1495b")):
        v = metric(A(f"{ver}-{suf}"), arm, met)["vertical"]
        ax.plot(v["ci95"], [y + dy] * 2, color=col, lw=1)
        ax.plot(v["ci90"], [y + dy] * 2, color=col, lw=3.2)
        ax.plot(v["floor_over_crater"], y + dy, "o", color=col, ms=4.5, label=ver if i == 0 else None)
        if ver == "v2.1":
            txt = f"{v['floor_over_crater']:.3f}" + (f"  p={v['perm_p']:.2g}" if lab.startswith("H") and "H5" not in lab else "")
            ax.annotate(txt, (v["ci95"][1], y + dy),
                        xytext=(6, -3), textcoords="offset points", fontsize=7)
ax.set_yticks(range(len(rows)), [r[0] for r in reversed(rows)], fontsize=8)
ax.set_xlabel("floor / crater, class-stratified (thick 90%, thin 95% CI)")
ax.set_title("PROTOCOL v2 (8 classes, grey) and v2.1 (28 classes, red)", fontsize=9)
ax.legend(fontsize=7, loc="upper center", bbox_to_anchor=(0.5, -0.12), ncol=3)
save(fig, "hypotheses_forest")

# 3. Per-class IC-yield floor/crater, with the pre-specified mechanism contrasts marked.
mech = {"l3-s1011": "Z/n×Z/3 above cyclic floor: deficit predicted",
        "l3-s1003": "Z/n×Z/2 at every level: none predicted",
        "l3-s1010": "Z/n×Z/2 at every level: none predicted"}
fig, ax = plt.subplots(figsize=(8, 0.26 * len(classes) + 1.4))
ax.axvline(1, color="#888", lw=0.8)
for i, name in enumerate(classes):
    v = metric(A(f"class-{name}"), "ic-mitm", "relation_yield")["vertical"]
    y = len(classes) - 1 - i
    col = "#2b6cb0" if name.startswith("l2") else "#2f855a"
    if name in mech:
        col = "#d1495b"
        ax.annotate(mech[name], (v["ci95"][1], y), xytext=(6, -3), textcoords="offset points", fontsize=6.5, color=col)
    ax.plot(v["ci95"], [y, y], color=col, lw=1.2)
    ax.plot(v["floor_over_crater"], y, "o", color=col, ms=4)
ax.set_yticks(range(len(classes)), list(reversed(classes)), fontsize=7)
ax.set_xlabel("IC relation yield, floor / crater (95% CI); blue ℓ = 2, green ℓ = 3, red pre-specified contrasts")
ax.set_title("Per-class contrasts (reported, not tested)", fontsize=9)
save(fig, "per_class_yield")

# 4. Heterogeneity, v2.1 all classes: horizontal vs coordinate τ (H4).
a = A("v2.1-H3H4-all")
hr = [("rho-neg", "S"), ("rho-neg-aa", "S"), ("ic-mitm", "relation_yield"), ("ic-mitm", "S")]
fig, axs = plt.subplots(1, 2, figsize=(9, 0.55 * len(hr) + 1.4), sharey=True)
for ax, key, title in [(axs[0], "horizontal", "between curves of one level (horizontal)"),
                       (axs[1], "model", "between isomorphic models of one curve")]:
    for i, (arm, met) in enumerate(hr):
        h = metric(a, arm, met)[key]
        y = len(hr) - 1 - i
        ax.barh(y, 100 * (h["tau_null_p95"] or 0), color="#e2e2e2", height=0.6)
        ax.plot(100 * h["tau_rel"], y, "o", color="#d1495b")
        ax.annotate(f"Q p={h['perm_p']:.2g}, max|z| p={h['max_z_perm_p']:.2g}", (100 * (h["tau_null_p95"] or 0), y),
                    xytext=(4, -3), textcoords="offset points", fontsize=6.5)
    ax.set_title(title, fontsize=8)
    ax.set_xlabel("τ, % of mean (dot); grey: null 95th percentile")
axs[0].set_yticks(range(len(hr)), [f"{x} · {m}" for x, m in reversed(hr)], fontsize=7)
save(fig, "heterogeneity")

# 5. §5a band: IC S over rho S per curve-model, every class, at weights 0, 0.1, 1.
ratios = {0: [], 1: [], 2: []}
for f in sorted((study / "band").glob("*.json")):
    b = json.loads(f.read_text())
    by = {}
    for r in b["rows"]:
        by.setdefault(r["curve"], {})[r["arm"]] = r["mean_s"]
    for c, d in by.items():
        if "ic-mitm" in d and "rho-neg" in d:
            for k in ratios:
                ratios[k].append(d["ic-mitm"][k] / d["rho-neg"][k])
fig, ax = plt.subplots(figsize=(7, 3))
ax.boxplot([ratios[k] for k in ratios], orientation="horizontal", tick_labels=["w = 0", "w = 0.1", "w = 1"], showfliers=True)
ax.axvline(1, color="#d1495b", lw=1, label="parity with rho")
ax.set_xscale("log")
ax.set_xlabel(f"IC S / rho S per curve-model ({len(ratios[0])} curve-models, 28 classes); GAE per native op = w")
ax.set_title("Distance remaining to matched rho across the §5a band", fontsize=9)
ax.legend(fontsize=7)
save(fig, "band_gap")
print("wrote", out, "min ratios", [round(min(v), 1) for v in ratios.values()])
