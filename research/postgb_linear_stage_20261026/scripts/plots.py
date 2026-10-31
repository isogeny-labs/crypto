"""Lay out analysis/summary.json as SVG + 150 dpi PNG. Plotting only: every
number comes from summary.json (examples/postgb_la_summary.rs over the run JSONs)."""
import json
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

here = Path(__file__).resolve().parent.parent
d = json.loads((here / "analysis" / "summary.json").read_text())
out = here / "figures"
out.mkdir(exist_ok=True)
FAM = ["jv-nagao", "gaudry-s4", "gaudry-d4", "gaudry-s5", "dense"]
COL = {"jv-nagao": "#4f81bd", "gaudry-s4": "#9bbb59", "gaudry-d4": "#e46c0a",
       "gaudry-s5": "#8064a2", "dense": "#7f7f7f"}
NOTE = ("Median wall time of 7 interleaved rounds per system, single thread; family bars: geometric mean "
        "with bootstrap 95% interval over systems. Host B unless marked. Every arm agreed on every system.")


def save(fig, name):
    fig.tight_layout(rect=(0, 0.03, 1, 1))
    fig.savefig(out / f"{name}.svg")
    fig.savefig(out / f"{name}.png", dpi=150)
    plt.close(fig)


# 1. Per-family speed-up of fast over the incumbent sparse (and dense), host B and host A.
fig, ax = plt.subplots(figsize=(7.4, 4.0))
series = [("sparse_over_fast", "sparse / fast (host B)", 0.0, "o"),
          ("host_a_sparse_over_fast", "sparse / fast (host A, L1-L4)", 0.18, "s"),
          ("ablation_sparse_over_portable_fast", "sparse / portable fast (no SIMD, host B)", 0.36, "^"),
          ("dense_over_fast", "dense / fast (host B)", -0.18, "D")]
for key, lab, dx, mk in series:
    t = d.get(key, {})
    xs, ys, lo, hi = [], [], [], []
    for i, f in enumerate(FAM):
        if f in t:
            v = t[f]
            xs.append(i + dx)
            ys.append(v["geomean"])
            lo.append(v["geomean"] - v["ci95"][0])
            hi.append(v["ci95"][1] - v["geomean"])
    ax.errorbar(xs, ys, yerr=[lo, hi], fmt=mk, ms=5, capsize=2, label=lab)
ax.axhline(1, color="black", lw=0.8)
ax.axhline(1.5, color="#c0504d", lw=0.8, ls="--", label="P1 bar (1.5)")
ax.set_yscale("log")
ax.set_xticks(range(len(FAM)))
ax.set_xticklabels([f"{f}\n({d['sparse_over_fast'][f]['systems']} sys, δ {d['sparse_over_fast'][f]['delta'][0]}"
                    f"–{d['sparse_over_fast'][f]['delta'][1]})" if f in d["sparse_over_fast"] else f
                    for f in FAM], fontsize=7)
ax.set_ylabel("wall-time ratio (higher: fast is faster)")
ax.set_title("Linear stage after F4: speed-up of fast_stage per Gröbner-basis family", fontsize=9)
ax.legend(fontsize=6.5, loc="upper left")
ax.grid(alpha=0.3, which="both", axis="y")
fig.text(0.01, 0.005, NOTE, fontsize=5)
save(fig, "family_speedup")

# 2. Per-system wall time, sparse against fast, log-log.
fig, ax = plt.subplots(figsize=(5.6, 4.6))
for f in FAM:
    rs = [s for s in d["systems"] if s["family"] == f]
    ax.scatter([s["ms"]["fast"] for s in rs], [s["ms"]["sparse"] for s in rs], s=10, color=COL[f],
               label=f, alpha=0.8)
lo = min(s["ms"]["fast"] for s in d["systems"])
hi = max(s["ms"]["sparse"] for s in d["systems"])
for k, ls in ((1, "-"), (10, "--"), (100, ":")):
    ax.plot([lo, hi / k], [lo * k, hi], color="black", lw=0.6, ls=ls)
    ax.text(hi / k, hi, f"{k}×", fontsize=6)
ax.set_xscale("log")
ax.set_yscale("log")
ax.set_xlabel("fast median wall time (ms)")
ax.set_ylabel("sparse (incumbent) median wall time (ms)")
ax.set_title(f"Every system of the corpus ({len(d['systems'])}), host B", fontsize=9)
ax.legend(fontsize=6.5)
ax.grid(alpha=0.3, which="both")
fig.text(0.01, 0.005, "Above the 1× line: fast is faster. The gaudry-d4 point near the 1× line is "
         "p97-4 (x_n not separating; fast fell back to dense).", fontsize=5)
save(fig, "per_system")

# 3. dense family: speed-up against δ (P2).
fig, ax = plt.subplots(figsize=(5.6, 3.8))
rs = [s for s in d["systems"] if s["family"] == "dense"]
for n in sorted({s["n"] for s in rs}):
    q = [s for s in rs if s["n"] == n]
    ax.scatter([s["delta"] for s in q], [s["ms"]["sparse"] / s["ms"]["fast"] for s in q], s=12, label=f"n = {n}")
ax.set_xscale("log", base=2)
ax.set_yscale("log")
ax.set_xlabel("δ (dimension of the quotient)")
ax.set_ylabel("sparse / fast")
ax.set_title(f"dense control family: Spearman ρ(δ, ratio) = {d['predictions']['P2']['spearman_rho']:.2f}",
             fontsize=9)
ax.legend(fontsize=6, ncol=2)
ax.grid(alpha=0.3, which="both")
save(fig, "dense_vs_delta")

# 4. Where the fast arm spends its time, per family (share of its median wall time).
PH = ["nf_ms", "krylov_ms", "bm_ms", "roots_ms", "readout_ms"]
fig, ax = plt.subplots(figsize=(6.4, 3.6))
bottoms = [0.0] * len(FAM)
for ph in PH:
    vals = []
    for f in FAM:
        rs = [s for s in d["systems"] if s["family"] == f and s["fast_phases_ms"]]
        tot = sum(sum(s["fast_phases_ms"].values()) for s in rs) or 1
        vals.append(sum(s["fast_phases_ms"][ph] for s in rs) / tot)
    ax.bar(range(len(FAM)), vals, bottom=bottoms, label=ph.replace("_ms", ""))
    bottoms = [b + v for b, v in zip(bottoms, vals)]
ax.set_xticks(range(len(FAM)))
ax.set_xticklabels(FAM, fontsize=7)
ax.set_ylabel("share of fast_stage phase time")
ax.set_title("fast_stage phase breakdown (summed over each family's systems)", fontsize=9)
ax.legend(fontsize=6.5, ncol=5, loc="upper center", bbox_to_anchor=(0.5, -0.12))
save(fig, "fast_phases")

# 5. jv_cover DLP end to end: linear-stage wall time and S/rho, sparse against fast.
dl = d["dlp"]
if dl:
    fig, axes = plt.subplots(1, 2, figsize=(8.0, 3.4))
    lab = [f"p={r['p']} s={r['seed']}" for r in dl]
    x = range(len(dl))
    axes[0].bar([i - 0.2 for i in x], [r["lin_ms"][0] for r in dl], 0.4, label="JV_LIN=sparse")
    axes[0].bar([i + 0.2 for i in x], [r["lin_ms"][1] for r in dl], 0.4, label="JV_LIN=fast")
    axes[0].set_yscale("log")
    axes[0].set_ylabel("linear-stage wall time (ms, summed)")
    axes[1].bar([i - 0.2 for i in x], [r["s_over_rho_neg"][0] for r in dl], 0.4, label="sparse")
    axes[1].bar([i + 0.2 for i in x], [r["s_over_rho_neg"][1] for r in dl], 0.4, label="fast")
    axes[1].axhline(1, color="black", lw=0.8)
    axes[1].set_yscale("log")
    axes[1].set_ylabel("S / negation-map rho (distance remaining)")
    for a in axes:
        a.set_xticks(list(x))
        a.set_xticklabels(lab, fontsize=6.5)
        a.legend(fontsize=6.5)
        a.grid(alpha=0.3, axis="y", which="both")
    fig.suptitle("jv_cover DLP inside the IC run, end to end (every logarithm verified)", fontsize=9)
    save(fig, "jv_cover_dlp")
