# F2 figures. This script draws only from results/run.json (rows, null, plant, aa_checks, summary, fatal); it computes
# no result, only re-formats recorded values (ratios c/N, differences L-(r-1), counts sums for the summary).
# Usage: python plot.py <path-to-results/run.json> <out-dir>
# Run F2-RUN-02, study p2_lift_linear_complexity_20261110. matplotlib only. Also writes table_<set>.md/.typ, summary.txt.
import json, sys, math
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.lines import Line2D

run = json.load(open(sys.argv[1])); out = sys.argv[2]
hdr = run["header"]; curves = hdr["curves"]; rows = run["rows"]
nulls = {n["curve"]: n for n in run["null"]}
aa = {a["curve"]: a for a in run["aa_checks"]}
summ = {s["curve"]: s for s in run["summary"]}
SETS = ["H", "R12", "R14", "R16", "S"]
OBS = ["O1", "O6", "O2", "O3"]
COL = {"O1": "#1f5fa8", "O6": "#c45a00", "O2": "#2a9d4b", "O3": "#8a4fbf"}
MK = {"O1": "o", "O6": "s", "O2": "^", "O3": "v"}
OFF = {"O1": -0.24, "O6": -0.08, "O2": 0.08, "O3": 0.24}
LAB = {"O1": "O1 f0", "O6": "O6 torsion_u", "O2": "O2 f1", "O3": "O3 f1-f0"}
names = [c["name"] for c in curves]
cinfo = {c["name"]: c for c in curves}
pos = {}; i = 0; prev = None
for c in curves:
    if prev is not None and c["set"] != prev: i += 1.0
    pos[c["name"]] = i; i += 1; prev = c["set"]
# multiplier-1 rows: (curve, obs) -> row
r1 = {(r["curve"], r["observable"]): r for r in rows if r["multiplier"] == 1}

# ---------------- Figure 1: c/N and L-(r-1) per observable, null band, S threshold ----------------
fig, (axa, axb) = plt.subplots(1, 2, figsize=(14, 5.6), gridspec_kw=dict(width_ratios=[1.15, 1]))
axa.axhline(1.0, color="#999999", lw=0.8)
for n in names:
    nu = nulls[n]; N = cinfo[n]["N"]
    axa.fill_between([pos[n] - 0.4, pos[n] + 0.4], nu["c_min"] / N, nu["c_max"] / N, color="#2a9d4b", alpha=0.25, lw=0)
    if cinfo[n]["set"] == "S":
        thr = ((cinfo[n]["r"] - 1) / 2 - 3) / N
        axa.plot([pos[n] - 0.4, pos[n] + 0.4], [thr, thr], color="#c62828", lw=2.0)
    for o in OBS:
        rr = r1[(n, o)]
        axa.plot(pos[n] + OFF[o], rr["c"] / rr["N"], MK[o], color=COL[o], ms=6, mec="white", mew=0.5, zorder=3)
axa.set_xticks([pos[n] for n in names]); axa.set_xticklabels(names, rotation=70, fontsize=7.5)
axa.set_ylim(0.9975, 1.0015)
axa.set_ylabel("c / N,  N = (r-1)/m  (multiplier 1)")
axa.grid(alpha=0.25, axis="y")
axa.legend(handles=[Line2D([], [], marker=MK[o], color=COL[o], ls="", ms=6, label=LAB[o]) for o in OBS] +
           [matplotlib.patches.Patch(color="#2a9d4b", alpha=0.35, label="null c_min..c_max /N (200 random seqs)"),
            Line2D([], [], color="#c62828", lw=2, label="rule 2 on S: ((r-1)/2-3)/N")],
           fontsize=7.5, loc="lower center", ncol=2, frameon=True)
axa.set_title("Support of the DFT: c/N is exactly 1 for every observable, null band is a point", fontsize=10)

axb.axhline(0, color="#999999", lw=0.8); axb.axhline(1, color="#999999", lw=0.8, ls=":")
for n in names:
    for o in OBS:
        rr = r1[(n, o)]
        axb.plot(pos[n] + OFF[o], rr["L"] - (rr["r"] - 1), MK[o], color=COL[o], ms=6, mec="white", mew=0.5, zorder=3)
axb.set_xticks([pos[n] for n in names]); axb.set_xticklabels(names, rotation=70, fontsize=7.5)
axb.set_ylim(-0.5, 1.5); axb.set_yticks([0, 1])
axb.set_ylabel("L - (r-1)   (L = [mean != 0] + m c)")
axb.grid(alpha=0.25, axis="y")
axb.set_title("Linear complexity L in {r-1, r}: O6 has mean != 0 (L = r), others mean 0 (L = r-1)", fontsize=10)
axb.legend(handles=[Line2D([], [], marker=MK[o], color=COL[o], ls="", ms=6, label=LAB[o]) for o in OBS], fontsize=8, loc="center right")
fig.suptitle("F2-RUN-02 (p2_lift_linear_complexity_20261110): DFT support c and linear complexity L of lift-derived sequences; 21 curves, local host", fontsize=11)
fig.text(0.01, 0.005, "Source: results/run.json (rows, null). Markers for the 4 multipliers {1,2,3,5} coincide (A/A passes on 21/21); multiplier 1 drawn. [pcc:unverified]", fontsize=7.5)
fig.tight_layout(rect=(0, 0.025, 1, 0.95))
for ext in ("svg", "png"): fig.savefig(f"{out}/support_L.{ext}", dpi=150)
plt.close(fig)

# ---------------- Figure 2: log10 P_vanish per curve vs the 1e-3 gate ----------------
fig, ax = plt.subplots(figsize=(10, 4.8))
ax.axhline(-3, color="#c62828", lw=1.6)
ax.text(len(names) + 0.9, -3, " rule 2 gate: log10 = -3 (P_vanish < 1e-3) ", color="#c62828", fontsize=8, va="top", ha="right", bbox=dict(fc="white", ec="#c62828", lw=0.6), zorder=5)
for n in names:
    v = r1[(n, "O1")]["log10_P_vanish"]
    ax.bar(pos[n], v, width=0.7, color="#1f5fa8" if cinfo[n]["set"] != "S" else "#c45a00")
ax.set_yscale("symlog", linthresh=10)
ax.set_xticks([pos[n] for n in names]); ax.set_xticklabels(names, rotation=70, fontsize=7.5)
ax.set_ylabel("log10 P_vanish  (union bound N p^-m; symlog axis)")
ax.set_ylim(-1e6, 0.5)
ax.grid(alpha=0.25, axis="y")
ax.set_title("Null probability that any degree-m component vanishes: below 1e-3 on all 21 curves\n(closest to the gate: S_0, S_1 with log10 = %.2f, %.2f)" %
             (r1[("S_0", "O1")]["log10_P_vanish"], r1[("S_1", "O1")]["log10_P_vanish"]), fontsize=10)
fig.text(0.01, 0.005, "Source: results/run.json rows[].log10_P_vanish (P_vanish itself underflows to 0 for r >= 4093 except on S). [pcc:unverified]", fontsize=7.5)
fig.tight_layout(rect=(0, 0.03, 1, 1))
for ext in ("svg", "png"): fig.savefig(f"{out}/pvanish.{ext}", dpi=150)
plt.close(fig)

# ---------------- table fragments ----------------
th = ["curve", "p", "r", "m", "N=(r-1)/m", "O1 L/c", "O6 L/c", "O2 L/c", "O3 L/c", "c/N (all 4)", "A/A", "null c range (200)", "log10 P_vanish"]
for S in SETS:
    lines = []
    for c in [c for c in curves if c["set"] == S]:
        n = c["name"]; cells = [n, str(c["p"]), str(c["r"]), str(c["m"]), str(c["N"])]
        for o in OBS:
            rr = r1[(n, o)]; cells.append(f"{rr['L']}/{rr['c']}")
        cells.append("1.0" if all(r1[(n, o)]["c"] == c["N"] for o in OBS) else "NOT 1")
        cells.append("pass" if aa[n]["pass"] else "FAIL")
        nu = nulls[n]; cells.append(f"{nu['c_min']}..{nu['c_max']}")
        cells.append(f"{r1[(n,'O1')]['log10_P_vanish']:.2f}")
        lines.append(cells)
    with open(f"{out}/table_{S}.md", "w") as fh:
        fh.write("| " + " | ".join(th) + " |\n|" + "---|" * len(th) + "\n")
        for l in lines: fh.write("| " + " | ".join(l) + " |\n")
    with open(f"{out}/table_{S}.typ", "w") as fh:
        fh.write(f"#table(columns: {len(th)}, align: (left,) + (right,) * {len(th)-1}, stroke: 0.4pt, inset: 2.5pt,\n  text(size: 6.5pt)[*" +
                 "*], text(size: 6.5pt)[*".join(th) + "*],\n")
        for l in lines: fh.write("  " + ", ".join(f"text(size: 6.5pt)[{x}]" for x in l) + ",\n")
        fh.write(")\n")

# ---------------- summary.txt (numbers, not verdicts) ----------------
gated = [c for c in curves]
def allc(o): return all(r["c"] == r["N"] for r in rows if r["observable"] == o)
Lok = all(r["L"] in (r["r"] - 1, r["r"]) for r in rows)
gen = {}
for r in rows: gen[(r["curve"], r["multiplier"])] = r["counts_generation_shared_by_curve_multiplier"]
def gsum(k): return sum(v[k] for v in gen.values())
def bsum(k): return sum(r["counts"][k] for r in rows)
def rowsum(k): return sum(r["counts"][k] for r in rows)
s = []
s.append("summary (numbers, not verdicts):")
s.append(f"  rows: {len(rows)} ({len(curves)} curves x {len(hdr['multipliers'])} multipliers x {len(OBS)} observables); fatal: {run['fatal']}")
s.append(f"  self-checks all true on every row: {all(all(r['checks'].values()) for r in rows)}")
for o in OBS:
    s.append(f"  c == N on all 84 rows of {o}: {allc(o)}; rows with c != N: {sum(1 for r in rows if r['observable']==o and r['c']!=r['N'])}")
s.append(f"  L in {{r-1, r}} on all rows: {Lok}; mean_nonzero: O1,O2,O3 False on all 252 rows, O6 True on all 84 rows: "
         f"{all((not r['mean_nonzero']) == (r['observable']!='O6') for r in rows)}")
s.append(f"  A/A pass: {sum(1 for a in run['aa_checks'] if a['pass'])}/{len(run['aa_checks'])} curves")
s.append(f"  plant O5 (set S): " + "; ".join(f"{p['curve']}: L={p['L']}, c={p['c']}, pass={p['pass']}" for p in run["plant"]))
s.append(f"  empirical null: sequences with c < N, all curves: {sum(n['n_with_c_below_N'] for n in run['null'])} of {sum(n['n_sequences'] for n in run['null'])}; "
         f"curves with c_min != N or c_max != N: {sum(1 for n in run['null'] if n['c_min']!=n['N'] or n['c_max']!=n['N'])}")
for n in ("S_0", "S_1"):
    c = cinfo[n]; rr = r1[(n, "O1")]
    s.append(f"  {n}: r={c['r']}, c(O1)={rr['c']}, (r-1)/2={(c['r']-1)//2}, rule-2 threshold (r-1)/2-3={(c['r']-1)//2-3}, c(O1)-threshold={rr['c']-((c['r']-1)//2-3)}")
lp = [r1[(n, "O1")]["log10_P_vanish"] for n in names]
s.append(f"  log10 P_vanish: min {min(lp):.2f}, max {max(lp):.2f} (S_0 {r1[('S_0','O1')]['log10_P_vanish']:.2f}, S_1 {r1[('S_1','O1')]['log10_P_vanish']:.2f}); curves with P_vanish < 1e-3: {sum(1 for v in lp if v < -3)} of {len(lp)}")
s.append("  cost counts (generation counted once per (curve,multiplier), 84 pairs; BM counted per row):")
for k in ("hensel_lifts", "inversions_mod_p2", "inversions_mod_p", "affine_mod_p_adds", "lifted_adds", "lifted_doublings", "kernel_mixed_adds", "formal_extractions"):
    s.append(f"    {k}: generation {gsum(k)}; naive sum over 336 rows {rowsum(k)} (= {rowsum(k)/gsum(k):.1f} x generation)")
s.append(f"    bm_multiplications: sum over 336 rows {bsum('bm_multiplications')} (generation part 0)")
s.append(f"    bm_inversions: sum over 336 rows {bsum('bm_inversions')}")
s.append(f"    plant O5 bm_multiplications: {sum(p['bm_multiplications'] for p in run['plant'])}")
open(f"{out}/summary.txt", "w").write("\n".join(s) + "\n")
