"""Graphs and summary.json for E31 (RESEARCH_GLV_INVARIANT_FACTOR_BASES.md §9.22-§9.23).

Lays out recorded values only.  Inputs:

  experiments/31_solve_cost/stage_a.json             Stage A, per call, three solvers
  experiments/31_solve_cost/e2e_{chain,chain-sparsest}_p*s*.json   E31 end to end
  experiments/30_gls_v4/p*s*.json                    E25 end to end (the baseline)

Derived quantities are ratios of recorded fields, computed exactly as
examples/e31_tables.rs computes them for experiments/31_solve_cost/tables.md:

  S(w)   = total_muls       + w * fp_muls_per_add * macaulay_entries
  rho(w) = rho_matched_muls + w * fp_muls_per_add * rho_matched_steps

with w in {0, 0.1, 1} GAE per unpriced native operation (AGENTS.md §5a).
No number is typed in by hand.

    python3 research/e31_chain_solve_20261024/figures/plot.py
"""

import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
STUDY = os.path.dirname(HERE)
ROOT = os.path.abspath(os.path.join(STUDY, "..", ".."))
E31 = os.path.join(ROOT, "experiments/31_solve_cost")
E25 = os.path.join(ROOT, "experiments/30_gls_v4")
PS = [101, 149, 197, 269, 397]
SEEDS = [1, 2]
WEIGHTS = [0.0, 0.1, 1.0]
SOLVERS = ["macaulay", "chain", "chain-sparsest"]
LABEL = {"macaulay": "existing (E25 solver)", "chain": "chain (A)", "chain-sparsest": "chain, sparsest pivots (B)"}
COLOR = {"macaulay": "#7f7f7f", "chain": "#1f77b4", "chain-sparsest": "#d62728"}
ARM_LABEL = {"A": "arm A (V4 fold)", "B": "arm B (<V4, psi> fold)"}
plt.rcParams.update({"font.size": 8, "svg.fonttype": "none"})


def load(path):
    with open(path) as fh:
        return json.load(fh)


def rel(path):
    return os.path.relpath(path, ROOT)


curves = [(p, s) for p in PS for s in SEEDS]
tick = [f"{p}/{s}" for p, s in curves]

# ── Stage A ──
stage_path = os.path.join(E31, "stage_a.json")
stage = {(r["p"], r["seed"]): r for r in load(stage_path)}
summ = {k: {x["solver"]: x for x in r["summaries"]} for k, r in stage.items()}

# ── End to end ──
e25 = {c: {r["arm"]: r for r in load(os.path.join(E25, f"p{c[0]}s{c[1]}.json"))} for c in curves}
e31 = {
    sv: {c: {r["arm"]: r for r in load(os.path.join(E31, f"e2e_{sv}_p{c[0]}s{c[1]}.json"))} for c in curves}
    for sv in ["chain", "chain-sparsest"]
}


def s_at(r, w):
    return r["total_muls"] + w * r["fp_muls_per_add"] * r["macaulay_entries"]


def rho_at(r, w):
    return r["rho_matched_muls"] + w * r["fp_muls_per_add"] * r["rho_matched_steps"]


# Fields compared between each E31 record and its E25 record (a subset of
# e31_tables.rs's full diff, recorded here so the summary states what it checked).
REPLAY_FIELDS = ["n", "residuals", "unsolved", "reruns", "decompositions", "lifted", "off_base",
                 "check_failures", "rational_solutions", "base_muls", "generation_muls",
                 "weil_muls", "lift_muls", "check_muls", "la_fp_muls", "rho_matched_muls",
                 "rho_matched_steps", "folded", "rho"]


def save(fig, name):
    fig.tight_layout()
    fig.savefig(os.path.join(HERE, name + ".svg"))
    fig.savefig(os.path.join(HERE, name + ".png"), dpi=150)
    plt.close(fig)


# 1. Per-call solve cost C, Stage A: median with p10-p90 is not recorded per
#    summary, so median (marker) and mean (x) per solver per curve.
fig, ax = plt.subplots(figsize=(6.4, 3.6))
x = list(range(len(curves)))
for j, sv in enumerate(SOLVERS):
    off = (j - 1) * 0.22
    ax.bar([i + off for i in x], [summ[c][sv]["call_median"] for c in curves], width=0.22,
           color=COLOR[sv], label=LABEL[sv] + ", median")
    ax.scatter([i + off for i in x], [summ[c][sv]["call_mean"] for c in curves], marker="x",
               color="k", s=12, zorder=3, label="mean (fallbacks charged)" if j == 0 else None)
ax.set_xticks(x)
ax.set_xticklabels(tick)
ax.set_xlabel("p / seed (E25's GLS curves)")
ax.set_ylabel("C per call (F_p multiplications)")
ax.set_title("E31 Stage A: one k = 4 decomposition, same 1,000 calls per curve, three solvers")
ax.legend(fontsize=6.5, loc="upper right", ncol=2)
ax.set_ylim(0, 1.25 * max(summ[c]["macaulay"]["call_mean"] for c in curves))
save(fig, "per_call_cost")

# 2. Where a call's multiplications go (phase means, all calls, fallbacks charged).
PHASES = [("echelon_mean", "echelon"), ("normal_forms_mean", "normal forms"), ("charpoly_mean", "charpoly"),
          ("roots_mean", "roots"), ("eigenvectors_mean", "eigenvectors"), ("other_mean", "other")]
PCOL = ["#4c72b0", "#dd8452", "#55a868", "#c44e52", "#8172b3", "#937860"]
fig, ax = plt.subplots(figsize=(6.4, 3.6))
for j, sv in enumerate(SOLVERS):
    off = (j - 1) * 0.27
    bottom = [0.0] * len(curves)
    for (key, name), col in zip(PHASES, PCOL):
        vals = [summ[c][sv][key] for c in curves]
        ax.bar([i + off for i in x], vals, width=0.25, bottom=bottom, color=col,
               label=name if j == 0 else None, edgecolor="white", linewidth=0.3)
        bottom = [b + v for b, v in zip(bottom, vals)]
ax.set_xticks(x)
ax.set_xticklabels(tick)
ax.set_xlabel("p / seed; bars left to right: existing, chain (A), chain sparsest (B)")
ax.set_ylabel("mean F_p multiplications per call")
ax.set_title("E31 Stage A: phase means per call")
ax.legend(fontsize=6.5, ncol=3, loc="upper right")
ax.set_ylim(0, 1.3 * max(summ[c]["macaulay"]["call_mean"] for c in curves))
save(fig, "phase_breakdown")

# 3. S per curve: E25 against E31 A and B, both arms.
fig, axes = plt.subplots(1, 2, figsize=(7.2, 3.6), sharey=True)
for ax, arm in zip(axes, ["A", "B"]):
    for j, (name, src) in enumerate([("macaulay", e25), ("chain", e31["chain"]),
                                     ("chain-sparsest", e31["chain-sparsest"])]):
        off = (j - 1) * 0.22
        ax.bar([i + off for i in x], [src[c][arm]["total_muls"] for c in curves], width=0.22,
               color=COLOR[name], label="E25 (existing solver)" if name == "macaulay" else "E31 " + LABEL[name])
    ax.set_yscale("log")
    ax.set_xticks(x)
    ax.set_xticklabels(tick, rotation=45)
    ax.set_xlabel("p / seed")
    ax.set_title(ARM_LABEL[arm])
axes[0].set_ylabel("S, whole DLP (F_p multiplications)")
axes[0].legend(fontsize=6.5, loc="upper right")
fig.suptitle("E31 end to end: S on E25's curves and target streams (0 GAE)", fontsize=9)
save(fig, "s_by_curve")

# 4. Distance remaining: S / rho(<±psi>) at 0, 0.1, 1 GAE, both arms.
MARK = {0.0: "o", 0.1: "^", 1.0: "s"}
fig, axes = plt.subplots(1, 2, figsize=(7.2, 4.3), sharey=True)
for ax, arm in zip(axes, ["A", "B"]):
    for j, (name, src) in enumerate([("macaulay", e25), ("chain", e31["chain"]),
                                     ("chain-sparsest", e31["chain-sparsest"])]):
        off = (j - 1) * 0.24
        for i, c in enumerate(curves):
            r = src[c][arm]
            v = [s_at(r, w) / rho_at(r, w) for w in WEIGHTS]
            ax.plot([i + off] * 3, v, color=COLOR[name], lw=1)
            for w, val in zip(WEIGHTS, v):
                ax.scatter([i + off], [val], marker=MARK[w], color=COLOR[name], s=12, zorder=3)
    ax.axhline(1, color="k", ls="--", lw=0.8)
    ax.text(0, 1.4, "parity with the matched <±psi> rho (S / rho = 1)", fontsize=6)
    ax.set_yscale("log")
    ax.set_ylim(0.5, 2e7)
    ax.set_xticks(x)
    ax.set_xticklabels(tick, rotation=45)
    ax.set_xlabel("p / seed")
    ax.set_title(ARM_LABEL[arm])
axes[0].set_ylabel("S_IC / S_rho  (distance remaining)")
handles = [plt.Line2D([], [], color=COLOR[n], lw=1.5, label=("E25 " if n == "macaulay" else "E31 ") + LABEL[n])
           for n in SOLVERS]
handles += [plt.Line2D([], [], color="k", marker=MARK[w], ls="", label=f"{w:g} GAE") for w in WEIGHTS]
fig.suptitle("Distance to the matched <±psi> rho, priced at 0 / 0.1 / 1 GAE per unpriced native operation",
             fontsize=9)
fig.tight_layout(rect=(0, 0.08, 1, 1))
fig.legend(handles=handles, fontsize=6.5, loc="lower center", ncol=6, frameon=False)
fig.savefig(os.path.join(HERE, "distance_to_rho.svg"))
fig.savefig(os.path.join(HERE, "distance_to_rho.png"), dpi=150)
plt.close(fig)

# ── summary.json ──
stage_rows = []
for c in curves:
    m, a, b = (summ[c][sv] for sv in SOLVERS)
    r = stage[c]
    stage_rows.append({
        "p": c[0], "seed": c[1], "n": r["n"], "calls": r["calls"],
        "agree_A_B": r["agree"], "quotient_agree_A_B": r["quotient_agree"],
        "quotient_checked_A_B": r["quotient_checked"],
        "c_median": {sv: summ[c][sv]["call_median"] for sv in SOLVERS},
        "c_mean": {sv: summ[c][sv]["call_mean"] for sv in SOLVERS},
        "existing_over_A": m["call_median"] / a["call_median"],
        "existing_over_B": m["call_median"] / b["call_median"],
        "nf_existing_over_A": m["normal_forms_mean"] / a["normal_forms_mean"],
        "charpoly_existing_over_A": m["charpoly_mean"] / a["charpoly_mean"],
        "eigvec_existing_over_A": m["eigenvectors_mean"] / a["eigenvectors_mean"],
        "echelon_A_over_B": a["echelon_mean"] / b["echelon_mean"],
        "fallbacks_A_B": [a["fallbacks"], b["fallbacks"]],
        "wall_us_median": {sv: summ[c][sv]["wall_us_median"] for sv in SOLVERS},
        "wall_existing_over_A": m["wall_us_median"] / a["wall_us_median"],
        "wall_existing_over_B": m["wall_us_median"] / b["wall_us_median"],
    })

e2e_rows = []
for c in curves:
    for arm in ["A", "B"]:
        old = e25[c][arm]
        row = {"p": c[0], "seed": c[1], "arm": arm, "log2_n": old["bits"],
               "S_E25": old["total_muls"], "rho_pm_psi": old["rho_matched_muls"],
               "S_over_rho_E25": [s_at(old, w) / rho_at(old, w) for w in WEIGHTS]}
        for sv, tag in [("chain", "A"), ("chain-sparsest", "B")]:
            new = e31[sv][c][arm]
            row[f"S_{tag}"] = new["total_muls"]
            row[f"E25_over_{tag}"] = [s_at(old, w) / s_at(new, w) for w in WEIGHTS]
            row[f"S_over_rho_{tag}"] = [s_at(new, w) / rho_at(new, w) for w in WEIGHTS]
            row[f"fallbacks_{tag}"] = new["solver_fallbacks"]
            row[f"fallbacks_eq_reruns_plus_unsolved_{tag}"] = (
                new["solver_fallbacks"] == old["reruns"] + old["unsolved"])
            row[f"solve_share_{tag}"] = (new["weil_muls"] + new["solve_muls"]) / new["total_muls"]
            row[f"replay_fields_identical_{tag}"] = all(old[f] == new[f] for f in REPLAY_FIELDS)
            row[f"correct_{tag}"] = bool(new["folded"]["correct"] and all(w["correct"] for w in new["rho"]))
            row[f"c_median_{tag}"] = new["call"]["median"]
        row["c_median_E25"] = old["call"]["median"]
        e2e_rows.append(row)


def span(vals):
    return [min(vals), max(vals)]


# Secondary statements of §9.23, re-read from the records.  Stage A records no
# per-call fallback flag; a call is taken as a fallback when the chain arm's
# normal-form count exceeds FB_NF (clean chain calls record ~6.6e4, a charged
# rerun of the existing solver carries its ~8.5e5).  The summary states how
# many of the recorded fallbacks that rule finds.
FB_NF = 3e5


def cross_checks():
    fb_found = fb_recorded = 0
    exist_fb, extra, clean_nf_a, clean_nf_b, fb_share_b = [], [], [], [], []
    for c in curves:
        arms = {a["solver"]: a for a in stage[c]["arms"]}
        m, a, b = arms["macaulay"], arms["chain"], arms["chain-sparsest"]
        fb = [i for i, v in enumerate(a["normal_forms"]) if v > FB_NF]
        clean = [i for i in range(len(a["call"])) if a["normal_forms"][i] <= FB_NF]
        fb_found += len(fb)
        fb_recorded += a["fallbacks"]
        exist_fb += [m["call"][i] for i in fb]
        extra += [a["call"][i] / m["call"][i] - 1 for i in fb]
        clean_nf_a.append(sum(a["normal_forms"][i] for i in clean) / len(clean))
        clean_nf_b.append(sum(b["normal_forms"][i] for i in clean) / len(clean))
        fb_share_b.append(sum(b["call"][i] for i in fb) / sum(b["call"]))
    e2e_rates = [e31[sv][c][arm]["solver_fallbacks"] / e31[sv][c][arm]["call"]["calls"]
                 for sv in e31 for c in curves for arm in "AB"]
    a_share = [summ[c]["chain"]["echelon_mean"] / summ[c]["chain"]["call_mean"] for c in curves]
    a_cp = [summ[c]["chain"]["charpoly_mean"] / summ[c]["chain"]["call_mean"] for c in curves]
    return {
        "stage_a_fallback_rule": f"chain normal_forms > {FB_NF:g}",
        "stage_a_fallbacks_found_of_recorded": [fb_found, fb_recorded],
        "stage_a_fallback_call_existing_cost_range": span(exist_fb),
        "stage_a_fallback_chain_extra_over_existing_mean": sum(extra) / len(extra),
        "stage_a_clean_call_nf_mean_A_range": span(clean_nf_a),
        "stage_a_clean_call_nf_mean_B_range": span(clean_nf_b),
        "stage_a_fallback_share_of_B_multiplications_range": span(fb_share_b),
        "stage_a_A_echelon_share_of_mean_C_range": span(a_share),
        "stage_a_A_charpoly_share_of_mean_C_range": span(a_cp),
        "e2e_fallback_rate_range_all_40_records": span(e2e_rates),
    }


def best(tag):
    key = "S_over_rho_E25" if tag == "E25" else f"S_over_rho_{tag}"
    r = min(e2e_rows, key=lambda r: r[key][0])
    return {"p": r["p"], "seed": r["seed"], "arm": r["arm"], "S_over_rho": r[key][0]}


summary = {
    "experiment": "E31",
    "class": "engineering (phase speed-up)",
    "what": "chain solve for the k = 4 V4-line decomposition (A: chain; B: chain with sparsest pivots) "
            "against E25's Macaulay solver, same outputs, counted F_p multiplications",
    "note": "research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md §9.22 (pre-registration "
            "G2.85-G2.94), §9.23 (results)",
    "inputs": [rel(stage_path)] + [rel(os.path.join(E31, f"e2e_{sv}_p{p}s{s}.json"))
                                   for sv in ["chain", "chain-sparsest"] for p, s in curves]
              + [rel(os.path.join(E25, f"p{p}s{s}.json")) for p, s in curves],
    "binary_commit": open(os.path.join(E31, "binary_commit.txt")).read().strip(),
    "gae_fp_muls": sorted({r["fp_muls_per_add"] for sv in e31 for c in curves for r in e31[sv][c].values()}),
    "weights_gae": WEIGHTS,
    "replay_fields_checked": REPLAY_FIELDS,
    "stage_a": {
        "curves": len(stage_rows),
        "calls_total": sum(r["calls"] for r in stage_rows),
        "agree_total_A_B": [sum(r["agree_A_B"][i] for r in stage_rows) for i in (0, 1)],
        "quotient_agree_total_A_B": [sum(r["quotient_agree_A_B"][i] for r in stage_rows) for i in (0, 1)],
        "quotient_checked_total_A_B": [sum(r["quotient_checked_A_B"][i] for r in stage_rows) for i in (0, 1)],
        "c_median_range": {sv: span([r["c_median"][sv] for r in stage_rows]) for sv in SOLVERS},
        "existing_over_A_range": span([r["existing_over_A"] for r in stage_rows]),
        "existing_over_B_range": span([r["existing_over_B"] for r in stage_rows]),
        "nf_existing_over_A_range": span([r["nf_existing_over_A"] for r in stage_rows]),
        "charpoly_existing_over_A_range": span([r["charpoly_existing_over_A"] for r in stage_rows]),
        "eigvec_existing_over_A_range": span([r["eigvec_existing_over_A"] for r in stage_rows]),
        "echelon_A_over_B_range": span([r["echelon_A_over_B"] for r in stage_rows]),
        "fallback_rate_A_range": span([r["fallbacks_A_B"][0] / r["calls"] for r in stage_rows]),
        "wall_existing_over_A_range": span([r["wall_existing_over_A"] for r in stage_rows]),
        "rows": stage_rows,
    },
    "end_to_end": {
        "records": len(e2e_rows) * 2,
        "all_replay_fields_identical": all(r[f"replay_fields_identical_{t}"] for r in e2e_rows for t in "AB"),
        "all_fallbacks_eq_reruns_plus_unsolved": all(
            r[f"fallbacks_eq_reruns_plus_unsolved_{t}"] for r in e2e_rows for t in "AB"),
        "all_correct": all(r[f"correct_{t}"] for r in e2e_rows for t in "AB"),
        "E25_over_A_range_by_weight": [span([r["E25_over_A"][i] for r in e2e_rows]) for i in range(3)],
        "E25_over_B_range_by_weight": [span([r["E25_over_B"][i] for r in e2e_rows]) for i in range(3)],
        "S_over_rho_E25_range_by_weight": [span([r["S_over_rho_E25"][i] for r in e2e_rows]) for i in range(3)],
        "S_over_rho_A_range_by_weight": [span([r["S_over_rho_A"][i] for r in e2e_rows]) for i in range(3)],
        "S_over_rho_B_range_by_weight": [span([r["S_over_rho_B"][i] for r in e2e_rows]) for i in range(3)],
        "solve_share_A_range": span([r["solve_share_A"] for r in e2e_rows]),
        "solve_share_B_range": span([r["solve_share_B"] for r in e2e_rows]),
        "below_E25_every_row_every_weight": all(
            min(r[f"E25_over_{t}"]) > 1 for r in e2e_rows for t in "AB"),
        "rho_ahead_every_row_every_weight": all(
            min(r[f"S_over_rho_{t}"]) > 1 for r in e2e_rows for t in "AB"),
        "best_S_over_rho_0gae": {t: best(t) for t in ["E25", "A", "B"]},
        "rows": e2e_rows,
    },
    "cross_checks": cross_checks(),
    "figures": [f"figures/{n}.svg" for n in ["per_call_cost", "phase_breakdown", "s_by_curve", "distance_to_rho"]],
}
with open(os.path.join(STUDY, "summary.json"), "w") as fh:
    json.dump(summary, fh, indent=1, sort_keys=False)
    fh.write("\n")
print("wrote", rel(os.path.join(STUDY, "summary.json")))
