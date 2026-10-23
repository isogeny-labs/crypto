#!/usr/bin/env python3
"""Plots recorded values only (AGENTS.md "Implementation language: no Python").

Inputs, all produced by ecbench or jq from the sessions' records:
  analysis/table/*.md           ecbench table (mean S, 95% interval, S / floor)
  analysis/summary/*.json       jq over records.jsonl (arm_summary.jq)
  analysis/pair_check.json      jq over records.jsonl (pair_check.jq)
  analysis/compare/SUMMARY.tsv  ecbench compare --json (compare_table.sh)
Outputs: figures/*.svg and figures/*.data.json (the exact numbers drawn).
Run: python3 analysis/plot.py   (matplotlib only; no number is computed here beyond dividing a
recorded interval by the recorded floor, labelled in each figure)."""
import json, re, csv, sys, pathlib
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

matplotlib.rcParams.update({"svg.fonttype": "path", "font.size": 9, "axes.spines.top": False,
                            "axes.spines.right": False, "axes.titlesize": 10, "axes.titleweight": "bold"})
D = pathlib.Path(__file__).resolve().parent
ROOT = D.parent
FIG = ROOT / "figures"
FIG.mkdir(exist_ok=True)
FROZEN, V2, CTRL, OLD = "#8c8c8c", "#0072B2", "#D55E00", "#CC79A7"  # Okabe-Ito based

def table(name):
    rows = {}
    for line in (D / "table" / f"{name}.md").read_text().splitlines()[2:]:
        c = [x.strip() for x in line.strip().strip("|").split("|")]
        if len(c) < 15:
            continue
        lo, hi = [float(x) for x in re.findall(r"[-\d.]+", c[7])]
        rows[c[1]] = dict(session=c[0], method=c[2], curve=c[3], log2r=float(c[4]), verified=c[5],
                          mean_s=float(c[6]), lo=lo, hi=hi, s_over_floor=float(c[10]))
    return rows

def summ(name):
    return {r["arm"]: r for r in json.load(open(D / "summary" / f"{name}.json"))}

def save(fig, name, data):
    fig.savefig(FIG / f"{name}.svg", bbox_inches="tight")
    fig.savefig(FIG / f"{name}.png", dpi=150, bbox_inches="tight")
    (FIG / f"{name}.data.json").write_text(json.dumps(data, indent=1))
    plt.close(fig)

# ---- Figure 1: j = 0, S / floor -------------------------------------------------------------
t = table("j0-v2"); tm = table("j0-frozen-main"); sj = summ("j0-v2")
floor_v2 = sj["rho-cubic-v2"]["floor_s"]            # recorded boundaries.floor_s, A = 6
floor_old = summ("j0-frozen-main")["rho-cubic"]["floor_s"]   # recorded by the main binary, A = 2
smm = summ("j0-frozen-main")
items = [  # label, table row, floor used, exact mean S/floor from the records, colour
    ("rho.plain (A=1 walk)", t["rho-plain"], floor_v2, sj["rho-plain"]["mean_ratio_to_floor"], FROZEN),
    ("rho.negation_v2", t["rho-neg-v2"], floor_v2, sj["rho-neg-v2"]["mean_ratio_to_floor"], V2),
    ("rho.cubic, old floor (A=2), main binary", tm["rho-cubic"], floor_old, smm["rho-cubic"]["mean_ratio_to_floor"], OLD),
    ("rho.cubic, floor v2 (A=6)", t["rho-cubic"], floor_v2, sj["rho-cubic"]["mean_ratio_to_floor"], FROZEN),
    ("rho.cubic_v2, floor v2 (A=6)", t["rho-cubic-v2"], floor_v2, sj["rho-cubic-v2"]["mean_ratio_to_floor"], V2),
]
fig, ax = plt.subplots(figsize=(7.2, 3.1))
data = []
for i, (lab, r, fl, ratio, col) in enumerate(items):
    y = len(items) - 1 - i
    m, lo, hi = ratio, r["lo"] / fl, r["hi"] / fl
    ax.errorbar(m, y, xerr=[[m - lo], [hi - m]], fmt="o", color=col, capsize=3)
    ax.text(hi + 0.05, y, f"{m:.3f}", va="center", fontsize=8)
    data.append(dict(label=lab, mean_s=r["mean_s"], s_lo=r["lo"], s_hi=r["hi"], floor_s=fl,
                     s_over_floor=m, lo=lo, hi=hi, session=r["session"]))
ax.axvline(1.0, color="black", lw=0.8, ls="--")
ax.set_yticks(range(len(items))); ax.set_yticklabels([x[0] for x in items][::-1])
ax.set_xlabel("mean S / floor")
ax.set_xlim(0, 4.4)
ax.set_title("j = 0, r = 1 048 627: below the floor only under the old A = 2", loc="left")
fig.text(0.01, -0.12, f"24 runs per arm (6 targets x 4 rounds). Point: mean of the records' ratio_to_floor; whiskers: ecbench table 95% interval of S\ndivided by the recorded floor_s (derived). Sessions: j0-v2 {t['rho-cubic']['session']}, j0-frozen-main {tm['rho-cubic']['session']}.\nfloor_s = {floor_v2:.4f} (A=6, floor_rule automorphisms/v2), {floor_old:.4f} (A=2, first rule).", fontsize=7, va="top")
save(fig, "fig1_j0_floor", data)

# ---- Figure 2: strong reference, frozen vs v2, three sizes ---------------------------------------
sizes = [("n = 13", "claim-n13"), ("n = 19", "claim-n19"), ("n = 37", "strong-n37")]
fig, (a1, a2) = plt.subplots(1, 2, figsize=(7.6, 3.4))
data = []
for k, (lab, s) in enumerate(sizes):
    tt, ss = table(s), summ(s)
    for j, (arm, col, nm) in enumerate([("rho-strong-frozen", FROZEN, "frozen rho.signed_frobenius_strong"),
                                         ("rho-strong-v2", V2, "rho.signed_frobenius_strong_v2")]):
        r = tt[arm]; x = k + (j - 0.5) * 0.34
        a1.bar(x, r["mean_s"], 0.3, color=col, label=nm if k == 0 else None)
        a1.errorbar(x, r["mean_s"], yerr=[[r["mean_s"] - r["lo"]], [r["hi"] - r["mean_s"]]], color="black", capsize=2, lw=0.8)
        a1.text(x, r["hi"] * 1.12, f"{r['mean_s']:.3g}", ha="center", fontsize=7)
        sh = ss[arm]["setup_gae_sum_over_total_sum"] * 100
        a2.bar(x, sh, 0.3, color=col)
        a2.text(x, sh + 1.5, f"{sh:.1f}%", ha="center", fontsize=7)
        data.append(dict(size=lab, session=s, arm=arm, method=r["method"], mean_s=r["mean_s"], s_lo=r["lo"], s_hi=r["hi"],
                         setup_share_pct=sh, setup_share_mean_of_runs_pct=ss[arm]["setup_share_mean_of_runs"] * 100,
                         runs=r["verified"], log2r=r["log2r"]))
a1.set_yscale("log"); a1.set_ylim(0.1, 40)
a1.set_xticks(range(3)); a1.set_xticklabels([x[0] for x in sizes]); a2.set_xticks(range(3)); a2.set_xticklabels([x[0] for x in sizes])
a1.set_ylabel("mean S (GAE / sqrt(r)), log scale"); a1.set_title("Mean S")
a2.set_ylabel("set-up share of total GAE (%)"); a2.set_title("Fixed set-up share"); a2.set_ylim(0, 100)
a1.legend(frameon=False, fontsize=7, loc="upper right")
fig.text(0.01, -0.04, "Koblitz a=0; 24 runs/arm (n=13, 19: 8 targets x 3 rounds) and 32 (n=37: 8 x 4). Bars: mean over runs, whiskers: ecbench table 95% interval.\n"
         "Share = sum(setup + automorphism_setup phase GAE) / sum(total GAE) over the measured runs, from the records' phases (derived, same formula for both ids).", fontsize=7)
save(fig, "fig2_strong_frozen_vs_v2", data)

# ---- Figure 3: the verification difference is exactly the verification phase ----------------------
pc = json.load(open(D / "pair_check.json"))
fig, ax = plt.subplots(figsize=(4.6, 4.2))
cols = {"bsgs": "#0072B2", "bsgs-il": "#56B4E9", "bsgs-neg": "#009E73", "kangaroo": "#E69F00", "claw": "#D55E00", "lp": "#CC79A7", "bsgs-neg (Koblitz)": "#7F7F00"}
mk = {"bsgs": "o", "bsgs-il": "s", "bsgs-neg": "^", "kangaroo": "D", "claw": "v", "lp": "P"}
data = []
for p in pc:
    if p["mode"] == "aa":
        continue
    a = p["pair"].split(" -> ")[0]
    lab = a + (" (Koblitz)" if p["frozen_records"] == 40 and a == "bsgs-neg" and p["mean_frozen_total"] < 400 else "")
    xs = [r["v2_verification_gae"] - r["frozen_verification_gae"] for r in p["rows"]]
    ys = [r["delta"] for r in p["rows"]]
    ck = "bsgs-neg (Koblitz)" if lab.endswith("(Koblitz)") else a
    ax.scatter(xs, ys, s=26, alpha=0.7, color=cols[ck], marker=mk[a], label=f"{lab} (n={len(xs)})")
    data.append(dict(pair=p["pair"], n=len(xs), max_abs_diff=max(abs(x - y) for x, y in zip(xs, ys))))
lim = [16, 32]; ax.plot(lim, lim, color="black", lw=0.8, ls="--", label="y = x")
ax.set_xlim(*lim); ax.set_ylim(*lim); ax.set_aspect("equal")
ax.set_xlabel("v2 verification phase GAE minus frozen verification phase GAE")
ax.set_ylabel("total GAE: v2 minus frozen (same workload, round, seed)")
ax.legend(frameon=False, fontsize=7); ax.set_title("Cost added by the v2 id = its verification phase")
save(fig, "fig3_verification_exact", data)

# ---- Figure 4: paired ratio v2 / frozen for every pair -------------------------------------------
rows = list(csv.DictReader(open(D / "compare" / "SUMMARY.tsv"), delimiter="\t"))
want = [("claim-n13__rho-strong-frozen__rho-strong-v2", "strong, n=13", "rho"),
        ("claim-n19__rho-strong-frozen__rho-strong-v2", "strong, n=19", "rho"),
        ("pairs-koblitz__rho-strong__rho-strong-v2", "strong, n=17 (a=1)", "rho"),
        ("strong-n37__rho-strong-frozen__rho-strong-v2", "strong, n=37", "rho"),
        ("strong-n37__rho-frob__rho-frob-v2", "signed_frobenius, n=37", "rho"),
        ("pairs-koblitz__rho-frob__rho-frob-v2", "signed_frobenius, n=17", "rho"),
        ("pairs-prime__rho-neg__rho-neg-v2", "negation, 2^20 prime", "rho"),
        ("j0-v2__rho-cubic__rho-cubic-v2", "cubic, j=0", "rho"),
        ("pairs-prime__bsgs__bsgs-v2", "bsgs.textbook", "ver"),
        ("pairs-prime__bsgs-il__bsgs-il-v2", "bsgs.interleaved", "ver"),
        ("pairs-prime__bsgs-neg__bsgs-neg-v2", "bsgs.negation, prime", "ver"),
        ("pairs-koblitz__bsgs-neg__bsgs-neg-v2", "bsgs.negation, n=17", "ver"),
        ("pairs-prime__kangaroo__kangaroo-v2", "kangaroo.vow", "ver"),
        ("pairs-koblitz__claw__claw-v2", "claw.pair_table", "ver"),
        ("pairs-large-prime__lp__lp-v2", "ic.large_prime", "ver"),
        ("pairs-prime__rho-neg-v2__rho-neg-v2-aa", "A/A control, negation_v2", "aa"),
        ("strong-n37__rho-strong-v2__rho-strong-v2-aa", "A/A control, strong v2", "aa")]
byname = {r["comparison"]: r for r in rows}
fig, ax = plt.subplots(figsize=(7.6, 4.6))
data = []
for i, (key, lab, kind) in enumerate(want):
    r = byname[key]; y = len(want) - 1 - i
    m, lo, hi = float(r["ratio_B_over_A"]), float(r["ci95_lo"]), float(r["ci95_hi"])
    col = {"rho": V2, "ver": FROZEN, "aa": CTRL}[kind]
    ax.errorbar(m, y, xerr=[[m - lo], [hi - m]], fmt="o", color=col, capsize=3)
    ax.annotate(f"{m:.3f}  [{lo:.3f}, {hi:.3f}]  n={r['pairs']}", xy=(1.02, y), xycoords=("axes fraction", "data"), va="center", fontsize=7)
    data.append(dict(comparison=key, label=lab, ratio=m, lo=lo, hi=hi, pairs=int(r["pairs"])))
ax.axvline(1.0, color="black", lw=0.8, ls="--")
ax.set_xscale("log"); ax.set_xlim(0.12, 3.2)
ax.set_xticks([0.15, 0.2, 0.3, 0.5, 0.7, 1, 1.5, 2, 3]); ax.set_xticklabels(["0.15", "0.2", "0.3", "0.5", "0.7", "1", "1.5", "2", "3"]); ax.minorticks_off()
ax.set_yticks(range(len(want))); ax.set_yticklabels([x[1] for x in want][::-1])
ax.set_xlabel("sum S (v2 id) / sum S (frozen id), matched (workload, round) pairs; 95% cluster interval")
ax.set_title("v2 against its frozen id on the same workloads and seeds")
fig.text(0.01, -0.03, "Blue: the walk or reference changed. Grey: only the final verification (and kangaroo restarts' phase) changed. Orange: A/A control, identical method.", fontsize=7)
save(fig, "fig4_paired_ratios", data)

# ---- Run-report figure: mean S per arm in the two pairs sessions ----------------------------------
fig, axs = plt.subplots(1, 2, figsize=(8.2, 3.6), gridspec_kw={"width_ratios": [1.2, 1]})
data = []
for ax, s, ttl in zip(axs, ["pairs-prime", "pairs-koblitz"], ["pairs-prime (2^20 prime, 8 targets x 4 rounds)", "pairs-koblitz (a=1, n=17, 8 x 4)"]):
    tt = table(s)
    names = list(tt)[::-1]
    for y, n in enumerate(names):
        r = tt[n]; col = V2 if n.endswith("v2") else (CTRL if n.endswith("aa") else FROZEN)
        ax.errorbar(r["mean_s"], y, xerr=[[r["mean_s"] - r["lo"]], [r["hi"] - r["mean_s"]]], fmt="o", color=col, capsize=2)
        data.append(dict(session=s, arm=n, method=r["method"], mean_s=r["mean_s"], lo=r["lo"], hi=r["hi"], verified=r["verified"]))
    ax.set_yticks(range(len(names))); ax.set_yticklabels(names, fontsize=7); ax.set_title(ttl, fontsize=8)
    ax.set_xlabel("mean S (GAE / sqrt(r)), 95% interval")
    ax.set_xscale("log"); ax.minorticks_off()
    ax.set_xticks([0.7, 1, 1.5, 2, 4]); ax.set_xticklabels(["0.7", "1", "1.5", "2", "4"])
save(fig, "run_mean_s_per_arm", data)
print("ok")
