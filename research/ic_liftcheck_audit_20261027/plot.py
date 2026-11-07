"""Layout only: draws the pipeline diagram and the share graph from costs.json.
No value is computed here beyond reading and placing recorded or hand-derived
numbers; every derivation is shown in REPORT.typ."""
import json, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch, FancyArrowPatch

d = json.load(open("costs.json"))
BLUE, ORANGE, GREY, RED = "#2a6fb0", "#d9822b", "#8a8f98", "#b23a48"

def box(ax, x, y, w, h, text, fc, tc="white", fs=7.4, ec="none"):
    ax.add_patch(FancyBboxPatch((x, y), w, h, boxstyle="round,pad=0,rounding_size=0.012",
                                fc=fc, ec=ec, lw=1.2))
    ax.text(x + w / 2, y + h / 2, text, ha="center", va="center", color=tc, fontsize=fs, linespacing=1.25)

def arrow(ax, x0, x1, y):
    ax.add_patch(FancyArrowPatch((x0, y), (x1, y), arrowstyle="-|>", mutation_scale=9, lw=1.0, color="#444"))

# ---------------- figure 1: pipelines ----------------
fig, ax = plt.subplots(figsize=(11.2, 7.6))
ax.set_xlim(0, 1); ax.set_ylim(0, 1); ax.axis("off")
ax.text(0.0, 0.985, "What happens after a root or decomposition is found, per production path",
        fontsize=11.5, weight="bold", va="top")
ax.text(0.0, 0.955, "bitop/v1 units at the stated n (M, S, A, I weights in REPORT.typ section 4); blue = needed, "
        "orange = check that is dead or removable by the theorem or the code path, grey = search itself",
        fontsize=7.6, va="top", color="#333")

# lane P1
y = 0.72
ax.text(0.0, 0.915, "P1  incumbent Koblitz rows (Tables A, C): PairTable m=3, aimed collection, n=47, per relation  "
        "[koblitz_index_calculus.rs:13111-13160, 6836-7024, 6575, 6785-6803; ic_price 13821, 13271]",
        fontsize=7.6, weight="bold", va="top")
ws = [0.17, 0.17, 0.17, 0.17, 0.17]
xs = [0.0, 0.205, 0.41, 0.615, 0.82]
labs = [
    ("scan R - P_k\n7,204 summands\n18.5k each = 133.4M\n97.3%", GREY),
    ("fused key + filter\n(x only, no y, no root,\nno lift): not a post-step", GREY),
    ("hit: recover_pair_tagged\n94 members, 1.76M\n1.29%  (summand id)", BLUE),
    ("fast re-add check\n2 adds = 63k\n0.046%", BLUE),
    ("independent re-verify\n2 adds + [a]G, ~1.8M\n1.32% bitop; push lap =\n17% of collect+push\n(upper bound)", BLUE),
]
for (t, c), x, w in zip(labs, xs, ws):
    box(ax, x, y, w, 0.15, t, c)
for i in range(4):
    arrow(ax, xs[i] + ws[i], xs[i + 1], y + 0.075)
ax.text(0.0, y - 0.02, "theorem-redundant steps on this path: none. No root, y-recovery or rationality test exists here; the pair-table hit fixes the summands.",
        fontsize=7.6, color=RED, va="top")

# lane P3
y = 0.40
ax.text(0.0, 0.615, "P3  n=83 orbit DLP (leaderboard n83 rows): x-only S3 roots, per candidate pair  "
        "[examples/koblitz_orbit_dlp_fast.rs:339-367, 406-440, 1013-1090, 975-1010]",
        fontsize=7.6, weight="bold", va="top")
labs = [
    ("prepare + batch inverse\n+ finish: 11 M = 153k\n(77k per probe)", GREY),
    ("trace test d&mask\nparity: ~166 bit-ops\n0.108%; inputs are\nE-abscissae: never fires", ORANGE),
    ("half-trace, 2 roots,\n2 canonical keys,\n2 table probes", GREY),
    ("lift on a hit: by_x,\n<=28 adds + 14 inversions\n= ~2.6M per relation\n0.005%", BLUE),
    ("relation_check (rec.)\n0.019% of target time\n(384 rows)", BLUE),
]
for (t, c), x, w in zip(labs, xs, ws):
    box(ax, x, y, w, 0.15, t, c)
for i in range(4):
    arrow(ax, xs[i] + ws[i], xs[i + 1], y + 0.075)
ax.text(0.0, y - 0.02, "the only theorem-adjacent redundancy: the trace test, dead by group closure alone; <0.2% of a probe.",
        fontsize=7.6, color=RED, va="top")

# lane P4
y = 0.08
ax.text(0.0, 0.295, "P4  Semaev-system roots (SAT, Groebner, crossbred, chained S3) -> lift_candidate, per root, n=47  "
        "[koblitz_index_calculus.rs:7301-7360, 8864-8905, 9062, 9328]",
        fontsize=7.6, weight="bold", va="top")
labs = [
    ("solver: SAT/F4 words\nper root >> everything\nbelow (GAE-priced)", GREY),
    ("points_with_x per x_i\n~56.2k with generic trace\npass 10.6k = 18.9% of\ny-recovery (rev2)", ORANGE),
    ("index_of (hash) then\nsign DFS, <=14 adds\n<=444k for m=3", BLUE),
    ("acc == target\n(2 A)", BLUE),
    ("rejected root: blocking\nclause + re-solve\n(ic_boundary oracle:\ndirect 580 vs 583 adds)", RED),
]
for (t, c), x, w in zip(labs, xs, ws):
    box(ax, x, y, w, 0.15, t, c)
for i in range(4):
    arrow(ax, xs[i] + ws[i], xs[i + 1], y + 0.075)
ax.text(0.0, y - 0.02, "T2: roots with x_i in V that lift to the twist are degenerate only. The trace pass is the cost of rejecting them; it is a candidate saving, not shown negligible.",
        fontsize=7.6, color=RED, va="top")
fig.savefig("fig1_pipeline.svg", bbox_inches="tight")
fig.savefig("fig1_pipeline.png", dpi=150, bbox_inches="tight")
plt.close(fig)

# ---------------- figure 2: shares on a log axis ----------------
rows = []
for s in d["P1_incumbent_pair_table_n47"]["stages_bitop_per_relation"][1:]:
    rows.append(("P1 " + {"hit":"hit summand recovery (recover_pair_tagged), bitop","independent re-verification":"independent re-verification, bitop (n=47)","fast re-add check":"fast re-add check, bitop","finish_lazy":"finish_lazy y of the hit, bitop"}[s["stage"].split(" (")[0].split(":")[0]], s["share_pct"], BLUE))
rows.append(("P1 relation check, online wall (6 sizes, max of medians)", max(d["P1_recorded_wall_shares_rule_v7_median_over_rows"]["online_target_relation_check_pct_of_online_wall"]), BLUE))
rows.append(("P1 push lap (verify+dedup+LogSystem::push), WALL % of cold setup, upper bound", 7.3, BLUE))
rows.append(("P2 frob-fold confirm add, pooled mean (1 of 14,416 adds; median row ~1%)", 100 / 14417, BLUE))
p3 = d["P3_n83_s3_root_path"]
rows.append(("P3 trace test (bitop est.)", p3["trace_test_share_pct"], ORANGE))
rows.append(("P3 lift: sign search + re-add per relation", p3["lift_share_pct"], BLUE))
rows.append(("P3 relation check, recorded wall (aggregate)", p3["relation_check_share_aggregate_pct"], BLUE))
rows.append(("P4 trace pass inside points_with_x, % of y-recovery (n=47, derived)", d["P4_semaev_roots_lift_candidate_n47"]["trace_pass_share_pct"]["of_y_recovery_with_trace_pass"], ORANGE))
rows.append(("P5 all group work in the relation phase, n=23 direct, % of phase GAE (upper bound, not a step)", d["P5_structured_s4_recorded"]["n23_direct_group_work_pct_of_phase_gae_upper_bound"], BLUE))
rows.sort(key=lambda r: r[1])
fig, ax = plt.subplots(figsize=(10.6, 5.6))
ys = range(len(rows))
ax.barh(list(ys), [r[1] for r in rows], color=[r[2] for r in rows], height=0.62)
ax.set_xscale("log"); ax.set_xlim(1e-4, 40)
ax.axvline(1.0, color=RED, lw=1.4, ls="--")
ax.text(1.05, -0.45, "1%", color=RED, fontsize=9, va="center")
ax.set_yticks(list(ys)); ax.set_yticklabels([r[0] for r in rows], fontsize=7.6)
for i, r in enumerate(rows):
    ax.text(r[1] * 1.12, i, f"{r[1]:.3g}%", va="center", fontsize=7.2)
ax.set_xlabel("share, % (log scale). Denominators differ per bar and are named in each label; the 1% line is a visual aid, not a common-unit gate", fontsize=8.5)
ax.set_title("Post-root steps against the 1% line (mixed denominators, see labels)\n"
             "blue = needed (sign, summand identity, certificate); orange = check dead or removable; P4 orange bar is ABOVE 1%", fontsize=9.5)
ax.grid(axis="x", which="major", lw=0.4, alpha=0.5)
fig.text(0.01, 0.005, "Sources: costs.json (derived from src at HEAD fbccdb89e; recorded rows in research/ic_tool_program/rule/v7, "
         "n83_parallel_target_cancel_20261017, ic_algebraic_crossover_20261023). Derived, not measured, unless marked wall/recorded.",
         fontsize=6.4, color="#444")
fig.savefig("fig2_shares.svg", bbox_inches="tight")
fig.savefig("fig2_shares.png", dpi=150, bbox_inches="tight")
print("ok")
