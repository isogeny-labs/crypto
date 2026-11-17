#!/usr/bin/env python3
"""Layout only (research-visuals skill): plots values recorded in data/*.tsv and data/tables/*.md.
No statistic is computed here, except counting rows of the recorded tables (fig3).
Usage: plot.py  (from this directory); writes figures/*.svg and figures/*.png (150 dpi)."""
import csv, glob, re, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
D, F = os.path.join(here, "data"), os.path.join(here, "figures")
os.makedirs(F, exist_ok=True)
C_OLD, C_NEW, C_BOOT = "#8a6d3b", "#1f5fa8", "#7a7a7a"
SESS = "yield sweep koblitz = ecbench_yield_sweep_20261004/sessions/koblitz (ECBS1h0d1ce1d04e54); followup3 = ecbench_all_candidates_20261003/sessions/koblitz-followup-3; ghs = ghs_curve_comparison_20261009 panel-01; sat-pilot = ecbench_sat_pdp_20261009 pilot"

def save(fig, name):
    for ext, kw in (("svg", {}), ("png", {"dpi": 150})):
        fig.savefig(os.path.join(F, f"{name}.{ext}"), bbox_inches="tight", **kw)
    plt.close(fig)

def fl(x):
    return None if x in ("NA", "") else float(x)

rows = list(csv.DictReader(open(os.path.join(D, "comparisons.tsv")), delimiter="\t"))
rows = [r for r in rows if not (r["b"].endswith("-aa") or r["b"] == "rho-aa")]

# ---- Fig 1: ops ratio, old v1 interval vs new v2 interval
fig, ax = plt.subplots(figsize=(9, 6.2))
n = len(rows)
for i, r in enumerate(rows):
    y = n - i
    o, ol, oh = fl(r["old_ratio"]), fl(r["old_lo"]), fl(r["old_hi"])
    nr, nl, nh = fl(r["new_ratio"]), fl(r["new_lo"]), fl(r["new_hi"])
    bl, bh = fl(r["boot_lo"]), fl(r["boot_hi"])
    if o is not None:
        ax.plot([ol, oh], [y + 0.17] * 2, color=C_OLD, lw=3, solid_capstyle="butt")
        ax.plot(o, y + 0.17, "o", color=C_OLD, ms=6)
    if nr and nl and nh and nr > 0:
        ax.plot([nl, nh], [y - 0.17] * 2, color=C_NEW, lw=3, solid_capstyle="butt")
        marker = "D" if r["bound"] == "lower" else "o"
        ax.plot(nr, y - 0.17, marker, color=C_NEW, ms=7, mfc=C_NEW if r["bound"] != "lower" else "white", mew=1.6)
    if r["bound"] == "lower" and nr:
        ax.annotate("lower bound", (nr or o, y - 0.17), xytext=(6, -3), textcoords="offset points", fontsize=7, color=C_NEW)
    if nr == 0:
        ax.annotate("censored: both runs timed out; bound 0 (no spent cost recorded)", (1.2, y - 0.17), fontsize=7, color=C_NEW, va="center")
        ax.annotate("no verified pair: no ratio", (1.2, y + 0.17), fontsize=7, color=C_OLD, va="center")
ax.set_yticks([n - i for i in range(n)])
ax.set_yticklabels([f"{r['tag']}\n{r['b']} / {r['a']}  (K={r['new_dof'].split('.')[0]!s}+1 workloads)" if False else f"{r['tag']}: {r['b']} / {r['a']}" for r in rows], fontsize=7.5)
ax.set_xscale("log")
ax.set_xlim(1, 2000)
ax.set_xlabel("ops ratio B/A, mean S over matched pairs (ecbench.gae), log scale; 95% interval")
ax.axvline(1, color="#bbb", lw=0.8)
ax.plot([], [], color=C_OLD, lw=3, label="old (v1, pre-#2442): percentile cluster bootstrap, verified pairs only")
ax.plot([], [], color=C_NEW, lw=3, label="new (v2): linearised t(K-1); open diamond = censored lower bound")
ax.legend(loc="upper right", fontsize=7.5, frameon=False)
ax.set_title("Ops-ratio intervals before and after #2442, same recorded sessions", fontsize=10)
fig.text(0.01, -0.02, "Source: data/comparisons.tsv (ecbench compare --json at 7edc35a9fb^1 and at the study base). " + "Sessions: " + SESS, fontsize=5.5, wrap=True)
save(fig, "fig1_ops_intervals")

# ---- Fig 2: coverage and width vs K (simulation, stats::tests::coverage_table_for_report)
cov = list(csv.DictReader(open(os.path.join(D, "coverage.tsv")), delimiter="\t"))
fig, (a1, a2) = plt.subplots(1, 2, figsize=(9, 3.6))
for m, c, lab in (("linearised_t_log", C_NEW, "linearised t(K-1), log scale (v2)"), ("cluster_bootstrap_percentile", C_OLD, "percentile cluster bootstrap (v1)")):
    xs = [int(r["K"]) for r in cov if r["method"] == m]
    a1.plot(xs, [float(r["coverage"]) for r in cov if r["method"] == m], "o-", color=c, label=lab)
    a2.plot(xs, [float(r["median_width_ratio_scale"]) for r in cov if r["method"] == m], "o-", color=c, label=lab)
a1.axhline(0.95, color="#999", ls="--", lw=0.8)
a1.set_xlabel("workloads K (3 rounds each)"); a1.set_ylabel("coverage of the true ratio (target 0.95)")
a1.set_ylim(0.82, 1.0); a1.legend(fontsize=7, frameon=False, loc="lower right")
a2.set_xlabel("workloads K"); a2.set_ylabel("median interval width hi - lo (ratio scale)"); a2.set_yscale("log")
for a in (a1, a2):
    a.set_xticks([2, 3, 4, 8])
fig.suptitle("Monte-Carlo coverage, Rayleigh-vs-uniform cluster model (2000 replicates per K; MC s.e. about 0.005)", fontsize=9)
fig.text(0.01, -0.09, "Source: data/coverage.tsv from `cargo test --release --lib stats::tests::coverage_table_for_report -- --ignored` (data/stats_tests.log); bootstrap uses 400 resamples.", fontsize=6)
save(fig, "fig2_coverage")

# ---- Fig 3: heavy-tail flag rate against the number of verified runs behind a table row
cnt = {}
for f in glob.glob(os.path.join(D, "tables", "new.*.md")):
    for line in open(f):
        if not line.startswith("| ECBS"):
            continue
        c = [x.strip() for x in line.split("|")]
        v = c[6].split("/")[0]
        cnt.setdefault(int(v), [0, 0])
        cnt[int(v)][0] += 1
        cnt[int(v)][1] += "heavy tail" in c[7]
ks = sorted(k for k in cnt if k > 0)
fig, ax = plt.subplots(figsize=(6.5, 3.4))
ax.bar([str(k) for k in ks], [cnt[k][1] / cnt[k][0] for k in ks], color=C_NEW)
for i, k in enumerate(ks):
    ax.text(i, cnt[k][1] / cnt[k][0] + 0.02, f"{cnt[k][1]}/{cnt[k][0]}", ha="center", fontsize=8)
ax.set_ylim(0, 1.15)
ax.set_xlabel("verified runs behind the (arm, curve) row")
ax.set_ylabel("fraction of rows flagged heavy tail")
ax.set_title("`ecbench table` heavy-tail flag vs runs per row (5 sessions)", fontsize=9)
fig.text(0.01, -0.06, "Source: data/tables/new.*.md. The flag is: largest run > 25% of the row's total S; with n runs that share is at least 1/n.", fontsize=6)
save(fig, "fig3_tail_flag_vs_n")

# ---- Fig 4: wall: v1 median per-pair wall ratio vs v2 ns/GAE ratio and total-wall ratio
w = [r for r in rows if fl(r["old_wall_median_ratio"]) and fl(r["new_wall_ratio"])]
fig, ax = plt.subplots(figsize=(9, 4.8))
n = len(w)
for i, r in enumerate(w):
    y = n - i
    ax.plot(fl(r["old_wall_median_ratio"]), y + 0.25, "s", color=C_OLD, ms=6)
    lo, hi = fl(r["old_wall_lo"]), fl(r["old_wall_hi"])
    if lo and hi:
        ax.plot([lo, hi], [y + 0.25] * 2, color=C_OLD, lw=2)
    if fl(r["total_wall_ratio"]):
        ax.plot(fl(r["total_wall_ratio"]), y, "^", color=C_BOOT, ms=6)
    nl, nh = fl(r["new_wall_lo"]), fl(r["new_wall_hi"])
    ax.plot(fl(r["new_wall_ratio"]), y - 0.25, "o", color=C_NEW, ms=6)
    if nl and nh:
        ax.plot([nl, nh], [y - 0.25] * 2, color=C_NEW, lw=2)
ax.set_yticks([n - i for i in range(n)])
ax.set_yticklabels([f"{r['tag']}: {r['b']} / {r['a']}" for r in w], fontsize=7.5)
ax.set_xscale("log"); ax.axvline(1, color="#bbb", lw=0.8)
ax.set_xlabel("wall ratio B/A (log scale)")
ax.plot([], [], "s-", color=C_OLD, label="v1 headline: median per-pair solve-wall ratio (interval = bootstrap)")
ax.plot([], [], "^", color=C_BOOT, label="v2 total_wall_ratio (sum of solve wall; point)")
ax.plot([], [], "o-", color=C_NEW, label="v2 headline: ns per GAE ratio (linearised t over pairs)")
ax.legend(fontsize=7, frameon=False, loc="upper left", bbox_to_anchor=(0, -0.12))
ax.set_title("Wall-time statistic before and after #2442 (different estimands; gate-admitted pairs only)", fontsize=9.5)
save(fig, "fig4_wall_statistic")
print("ok")
