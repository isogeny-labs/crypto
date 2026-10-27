# Layout only: draws raw/summary-ext.json (written by
# examples/gaudry_quartic_d4_exponent_fit_ext.rs), with raw/summary.json (the frozen
# 72-job fit) for comparison. Computes nothing that is reported; the fitted lines are
# drawn from the summaries' own coefficients.
import json
import math
import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = Path(__file__).resolve().parent
E = json.loads((HERE.parent / "raw/summary-ext.json").read_text())
S72 = json.loads((HERE.parent / "raw/summary.json").read_text())
C = {"rel": "#2a78d6", "la": "#eb6834", "gauss": "#6250d6", "rho": "#1baf7a", "lp2": "#eb6834", "lp1": "#6250d6", "ink": "#0b0b0b", "muted": "#52514e", "grid": "#e4e3df"}
plt.rcParams.update({"font.size": 9, "axes.edgecolor": C["muted"], "axes.labelcolor": C["ink"], "xtick.color": C["muted"], "ytick.color": C["muted"], "axes.spines.top": False, "axes.spines.right": False, "svg.fonttype": "none"})


def save(fig, name):
    fig.tight_layout()
    fig.savefig(HERE / f"{name}.svg")
    fig.savefig(HERE / f"{name}.png", dpi=150)
    plt.close(fig)


def grid(ax):
    ax.grid(True, which="major", color=C["grid"], linewidth=0.6)
    ax.set_axisbelow(True)


def line(ax, fit, ps, color, style="-"):
    xs = [ps[0] * (ps[1] / ps[0]) ** (i / 50) for i in range(51)]
    ax.plot(xs, [math.exp(fit["const_ln"] + fit["slope"] * math.log(x)) for x in xs], style, color=color, linewidth=1.2)


def fmt_ci(f):
    a, b = f["slope_ci95_union"]
    return f"β = {f['slope']:.2f} [{a:.2f}, {b:.2f}]"


w = E["arms"]["ic-d4-w"]
pr = w["per_rung"]
ps = [r["p"] for r in pr]
rho = E["rho"]

# 1. Phases and rho against p.
fig, ax = plt.subplots(figsize=(7.2, 5.4))
for key, col, mk, lab in (("relations", C["rel"], "o", "relations (D₄ solves)"), ("linear_algebra", C["la"], "s", "linear algebra, Wiedemann (priced)"), ("shadow_gauss", C["gauss"], "^", "dense elimination, same rows (derived, not priced)")):
    for r in pr:
        ax.scatter([r["p"]] * len(r[f"{key}_values"]), r[f"{key}_values"], s=8, color=col, alpha=0.3, linewidths=0)
    ax.scatter(ps, [r[key] for r in pr], s=26, color=col, marker=mk, label=f"{lab}: {fmt_ci(w['fits'][key])}", zorder=3)
    line(ax, w["fits"][key], (ps[0], ps[-1]), col)
rps = [r["p"] for r in rho["per_rung"]]
ax.scatter(rps, [r["gae"] for r in rho["per_rung"]], s=26, color=C["rho"], marker="D", zorder=3, label=f"rho (negation), {rho['records']} distinct runs: β = {rho['fit_gae']['slope']:.2f}")
line(ax, rho["fit_gae"], (rps[0], rps[-1]), C["rho"], "--")
ax.set_xscale("log")
ax.set_yscale("log")
ax.set_xlabel("p  (curves over F_{p⁴}; p ≤ 509: three curves per rung, p = 701, 1009: one)")
ax.set_ylabel("cost (GAE, group-addition equivalents)")
ax.set_title("Extended design: cost by phase and rho against p (rung = mean over curves; dots: runs)", fontsize=9, color=C["ink"])
grid(ax)
ax.legend(fontsize=7, frameon=False, loc="upper center", bbox_to_anchor=(0.5, -0.14), ncol=2)
save(fig, "ext_phases_vs_p")

# 2. Crossover.
fig, ax = plt.subplots(figsize=(7.2, 4.4))
pmax = 2 ** 40
xs = [29 * (pmax / 29) ** (i / 200) for i in range(201)]
f = w["fits"]
tot = [sum(math.exp(f[k]["const_ln"] + f[k]["slope"] * math.log(x)) for k in ("relations", "linear_algebra", "other")) for x in xs]
gtot = [sum(math.exp(f[k]["const_ln"] + f[k]["slope"] * math.log(x)) for k in ("relations", "shadow_gauss", "other")) for x in xs]
ax.plot(xs, tot, color=C["rel"], linewidth=1.6, label="IC, sum of fitted phases (Wiedemann)")
ax.plot(xs, gtot, color=C["gauss"], linewidth=1.2, linestyle=":", label="IC with the dense elimination priced instead")
for h, st in ((4, "--"), (16, "-.")):
    ax.plot(xs, [rho["s_rho_mean"] * math.sqrt(x ** 4 / h) for x in xs], st, color=C["rho"], linewidth=1.2, label=f"rho, S̄ = {rho['s_rho_mean']:.2f}, h = {h}")
ax.axvspan(29, 1009, color=C["grid"], alpha=0.6, linewidth=0)
ax.text(math.sqrt(29 * 1009), 1e6, "measured", ha="center", fontsize=8, color=C["muted"])
cx = [c for c in w["crossovers"] if c["variant"] == "wiedemann" and c["rho"] == "h=4"][0]
ax.axvline(2 ** (cx["log2_group_star"] / 4), color=C["muted"], linewidth=0.8)
lo, hi = cx["log2_group_star_boot95"]
ax.axvspan(2 ** (lo / 4), 2 ** (hi / 4), color=C["rel"], alpha=0.12, linewidth=0)
ax.text(2 ** (cx["log2_group_star"] / 4), 1e5, f" extended fit, h = 4:\n group 2^{cx['log2_group_star']:.1f}\n (cluster bootstrap 2^{lo:.0f}–2^{hi:.0f})", fontsize=7, color=C["ink"])
sens = [c["log2_group_star"] for c in w["crossover_relations_slope_sensitivity"] if c["h"] == 4.0 and c["log2_group_star"]]
if len(sens) == 2:
    ax.axvspan(2 ** (min(sens) / 4), 2 ** (max(sens) / 4), facecolor="none", alpha=0.5, linewidth=0, hatch="////", edgecolor=C["rel"])
    ax.text(2 ** (min(sens) / 4), 1e20, f" relations slope at its t-interval ends:\n group 2^{min(sens):.0f}…2^{max(sens):.0f}", fontsize=7, color=C["muted"])
c72 = [c for c in S72["arms"]["ic-d4-w"]["crossovers"] if c["variant"] == "wiedemann" and c["rho"] == "h=4"][0]
ax.axvline(2 ** (c72["log2_group_star"] / 4), color=C["la"], linewidth=0.9, linestyle=":")
ax.text(2 ** (c72["log2_group_star"] / 4), 3e2, f"72-job fit: 2^{c72['log2_group_star']:.1f} ", ha="right", fontsize=7, color=C["la"])
m = S72["model_11_16_from_pr1863"]["log2_group_star"]["h=4"]
ax.axvspan(2 ** (m[0] / 4), 2 ** (m[1] / 4), color=C["la"], alpha=0.25, linewidth=0)
ax.text(2 ** (m[1] / 4), 2e1, f" §11.16 model, h = 4:\n group 2^{m[0]:.1f}…2^{m[1]:.1f}", fontsize=7, color=C["ink"])
ax.set_xscale("log", base=2)
ax.set_yscale("log")
ax.set_xlabel("p (log₂ scale)")
ax.set_ylabel("cost (GAE)")
ax.set_title("Extended fit: crossover with rho (extrapolated beyond the shaded range)", fontsize=9, color=C["ink"])
grid(ax)
ax.legend(fontsize=7, frameon=False, loc="upper left")
save(fig, "ext_crossover")

# 3. Large primes.
fig, axs = plt.subplots(1, 2, figsize=(7.4, 3.6))
for arm, col, mk, lab in (("ic-d4-w", C["rel"], "o", "no large prime"), ("ic-d4-lp2", C["lp2"], "s", "two large primes"), ("ic-d4-lp1", C["lp1"], "^", "one large prime")):
    a = E["arms"][arm]
    q = [r["p"] for r in a["per_rung"]]
    for ax, key in zip(axs, ("linear_algebra", "total")):
        ax.scatter(q, [r[key] for r in a["per_rung"]], s=22, color=col, marker=mk, zorder=3, label=f"{lab}: {fmt_ci(a['fits'][key])}")
        line(ax, a["fits"][key], (q[0], q[-1]), col)
for ax, t in zip(axs, ("linear algebra (Wiedemann)", "total cost")):
    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_xlabel("p")
    ax.set_ylabel("GAE")
    ax.set_title(t, fontsize=9, color=C["ink"])
    grid(ax)
    ax.legend(fontsize=6.5, frameon=False, loc="upper left")
save(fig, "ext_large_primes")

# 4. Slopes: the 72-job fit and the extended fit side by side.
rows = []
for arm, lab in (("ic-d4-w", "plain"), ("ic-d4-lp2", "LP2"), ("ic-d4-lp1", "LP1")):
    for key, kl in (("relations", "relations"), ("linear_algebra", "lin. alg. (Wiedemann)"), ("shadow_gauss", "dense elimination")):
        rows.append((f"{lab}: {kl}", S72["arms"][arm]["fits"][key], E["arms"][arm]["fits"][key]))
rows.append(("rho", None, None))
fig, ax = plt.subplots(figsize=(7.0, 0.42 * len(rows) + 1.2))
for i, (lab, a, b) in enumerate(rows):
    if lab == "rho":
        for off, d, col, mk in ((-0.12, S72["rho"]["fit_gae"], C["muted"], "o"), (0.12, E["rho"]["fit_gae"], C["rel"], "o")):
            lo = min(d["slope_t95"][0], d["slope_boot95"][0])
            hi = max(d["slope_t95"][1], d["slope_boot95"][1])
            ax.plot([lo, hi], [i + off, i + off], color=col, linewidth=1.2)
            ax.scatter([d["slope"]], [i + off], s=18, color=col, zorder=3, facecolors="white" if col == C["muted"] else col)
        continue
    for off, d, col in ((-0.12, a, C["muted"]), (0.12, b, C["rel"])):
        lo, hi = d["slope_ci95_union"]
        ax.plot([lo, hi], [i + off, i + off], color=col, linewidth=1.2)
        ax.scatter([d["slope"]], [i + off], s=18, color=col, zorder=3, facecolors="white" if col == C["muted"] else col)
for v in (1, 1.5, 2, 3):
    ax.axvline(v, color=C["grid"], linewidth=0.8, zorder=0)
ax.set_yticks(range(len(rows)))
ax.set_yticklabels([r[0] for r in rows], fontsize=7.5)
ax.invert_yaxis()
ax.set_xlabel("fitted exponent β in cost ∝ p^β (95 %: union of OLS-t and bootstrap)")
ax.scatter([], [], s=18, color=C["muted"], facecolors="white", label="72-job fit (one curve per rung)")
ax.scatter([], [], s=18, color=C["rel"], label="extended fit (all curves and targets)")
ax.legend(fontsize=7, frameon=False, loc="lower right")
save(fig, "ext_slopes")

# 5. The scatter between curves, and the slope by chain.
fig, axs = plt.subplots(1, 2, figsize=(7.6, 3.6), gridspec_kw={"width_ratios": [1.5, 1]})
ax = axs[0]
for row in w["curve_table"]:
    ax.scatter([row["p"]] * len(row["ln_trials_per_relation"]), row["ln_trials_per_relation"], s=16, color=C["rel"], alpha=0.8, linewidths=0, zorder=3)
ax.set_xscale("log")
ax.set_xlabel("p")
ax.set_ylabel("ln(trials per relation), one point per curve")
ax.set_title(f"between-curve scatter (pooled SD {w['trial_rate_sd_pooled']:.2f})", fontsize=9, color=C["ink"])
grid(ax)
ax = axs[1]
chains = w["relations_slope_by_chain"]
for i, (ch, d) in enumerate(sorted(chains.items())):
    lo, hi = d["slope_t95"]
    ax.plot([lo, hi], [i, i], color=C["muted"], linewidth=1.2)
    ax.scatter([d["slope"]], [i], s=22, color=C["rel"], zorder=3)
    ax.text(hi + 0.03, i, f"{d['slope']:.2f}", va="center", fontsize=7, color=C["ink"])
fit = w["fits"]["relations"]
ax.axvline(fit["slope"], color=C["rel"], linewidth=0.8, linestyle="--")
ax.set_yticks(range(len(chains)))
ax.set_yticklabels([f"chain {c} ({chains[c]['rungs']} rungs)" for c in sorted(chains)], fontsize=7.5)
ax.invert_yaxis()
ax.set_xlabel("relations slope β (95 % t-interval)")
ax.set_title("each curve chain alone; dashed: pooled", fontsize=9, color=C["ink"])
save(fig, "ext_curves")
print("ok", file=sys.stderr)

# 6. Run report of the extension: S of its index-calculus runs among all IC runs.
import glob

RUN3 = HERE.parent / "reports/run-20261020T0605Z"
RUN3.mkdir(parents=True, exist_ok=True)
ext_jobs = [l.strip() for l in (HERE.parent / "jobs_ext2.txt").read_text().split() if l.strip()]


def ic_points(job_dirs):
    pts = []
    for d in job_dirs:
        for line in (d / "records.jsonl").read_text().splitlines():
            r = json.loads(line)
            if r["arm"].startswith("ic-d4"):
                pts.append((r["arm"], r["workload"]["curve_spec"]["p"], float(r["cost"]["s"])))
    return pts


all_dirs = [Path(d) for d in sorted(glob.glob(str(HERE.parent / "sessions/*/"))) if (Path(d) / "records.jsonl").exists()]
old_dirs = [d for d in all_dirs if d.name not in ext_jobs]
new_dirs = [d for d in all_dirs if d.name in ext_jobs]
fig, ax = plt.subplots(figsize=(7.2, 4.2))
op = ic_points(old_dirs)
ax.scatter([p for _, p, _ in op], [s for _, _, s in op], s=14, color=C["grid"], edgecolors=C["muted"], linewidths=0.5, label=f"the first 72 jobs ({len(op)} IC runs)", zorder=2)
for arm, col, mk, lab in (("ic-d4-w", C["rel"], "o", "ic-d4-w"), ("ic-d4-lp2", C["lp2"], "s", "ic-d4-lp2"), ("ic-d4-lp1", C["lp1"], "^", "ic-d4-lp1")):
    q = [(p, s) for a, p, s in ic_points(new_dirs) if a == arm]
    ax.scatter([p for p, _ in q], [s for _, s in q], s=30, color=col, marker=mk, label=f"the extension: {lab} ({len(q)})", zorder=3)
ax.set_xscale("log")
ax.set_yscale("log")
ax.set_xlabel("p")
ax.set_ylabel("S (GAE / √r), counted")
ax.set_title("The extension's index-calculus runs (coloured) among all IC runs", fontsize=9, color=C["ink"])
grid(ax)
ax.legend(fontsize=7, frameon=False, loc="upper right")
fig.tight_layout()
fig.savefig(RUN3 / "s_extension_runs.svg")
fig.savefig(RUN3 / "s_extension_runs.png", dpi=150)
plt.close(fig)
