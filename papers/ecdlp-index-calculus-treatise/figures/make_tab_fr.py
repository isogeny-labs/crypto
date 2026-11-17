"""LaTeX tables for the fr data pass, generated from the data-fr-*.json files only.

    python3 make_tab_fr.py   # writes tab-fr-*.tex (tabular bodies, \\input by the sections)

Numbers are copied from the JSON and only formatted (rounded for display); the verdict-class
counts of tab-fr-julsep and the row counts of tab-fr-benchcount are tallied here.
"""
import collections
import json
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
def load(n): return json.load(open(os.path.join(HERE, n)))
def write(name, lines):
    open(os.path.join(HERE, name), "w").write("\n".join(lines) + "\n")
    print("wrote", name)
def f(x, d=2): return "" if x is None else f"{x:.{d}f}"
def ci(c, d=2): return "" if not c else f"[{c[0]:.{d}f},{c[1]:.{d}f}]"
def short(slug): return slug.split("-")[1]   # icv1-fp10-t5-... -> fp10
def tt(s): return "\\texttt{" + s.replace("_", "\\_") + "}"

LB = load("data-fr-leaderboard.json")
rows = LB["rows"]

# ---- Table B ----
L = ["\\begin{tabular}{@{}lrrrrrrrrr@{}}", "\\toprule",
     "row & $\\log_2 r$ & $S_{\\rm IC}$ & $S_\\rho$ & IC/$\\rho$ & IC/frozen $\\rho$ & $w=0$ & $w=0.1$ & $w=1$ & $w=1$ ext. \\\\",
     "\\midrule"]
for reg in ("prime", "char2"):
    for r in sorted([r for r in rows if r["family"] == "B" and r["regime"] == reg], key=lambda r: r["log2_r"]):
        w = r["band_ratio_w"]
        L.append(f"{short(r['slug'])} & {f(r['log2_r'])} & {f(r['s_ic'])} & {f(r['s_ref'],3)} & {f(r['ratio_rho'])} & "
                 f"{f(r['ratio_vs_frozen_rho'])} & {f(w[0])} & {f(w[1])} & {f(w[2])} & {f(r['band_ratio_extended_w'][2])} \\\\")
    if reg == "prime": L.append("\\addlinespace")
L += ["\\bottomrule", "\\end{tabular}"]
write("tab-fr-boardB.tex", L)

# ---- Table A ----
L = ["\\begin{tabular}{@{}lrrrlrlrr@{}}", "\\toprule",
     "row & $\\log_2 r$ & $S_{\\rm IC}$ & $S_\\rho$ & IC/$\\rho$ [95\\%] & IC/$\\rho$, \\S23 & online speed-up [95\\%] & break-even & canon.\\ step \\\\",
     "\\midrule"]
for r in sorted([r for r in rows if r["family"] == "A"], key=lambda r: r["log2_r"]):
    L.append(f"{short(r['slug'])} & {f(r['log2_r'])} & {f(r['s_ic'])} & {f(r['s_ref'],3)} & {f(r['ratio_rho'])} {ci(r['ratio_rho_ci'])} & "
             f"{f(r['ratio_rho_section23'])} & {f(r['online_speedup'],1)} {ci(r['online_speedup_ci'],1)} & {f(r['break_even_targets'])} & {f(r['canonical_step_units'])} \\\\")
L += ["\\bottomrule", "\\end{tabular}"]
write("tab-fr-boardA.tex", L)

# ---- Table C ----
L = ["\\begin{tabular}{@{}lrrrlrlr@{}}", "\\toprule",
     "row & $\\log_2 r$ & $S_{\\rm IC}$ & $S_{\\rm ref}$ & IC/ref [95\\%] & IC/ref before & speed-up [95\\%] & base \\\\",
     "\\midrule"]
for r in sorted([r for r in rows if r["family"] == "C"], key=lambda r: r["log2_r"]):
    L.append(f"{short(r['slug'])} & {f(r['log2_r'])} & {f(r['s_ic'],3)} & {f(r['s_ref'],4)} & {f(r['ratio_rho'])} {ci(r['ratio_rho_ci'])} & "
             f"{f(r['ratio_rho_before'])} & {f(r['speedup'],3)} {ci(r['speedup_ci'],3)} & {r['base_points']} \\\\")
L += ["\\bottomrule", "\\end{tabular}"]
write("tab-fr-boardC.tex", L)

# ---- re-pricing (audit) ----
L = ["\\begin{tabular}{@{}llrrrrrr@{}}", "\\toprule",
     "report & instance & $\\log_2 r$ & $S_{\\rm IC}$ & frozen $S_\\rho$ & matched $S_\\rho$ & IC/frozen & IC/matched \\\\",
     "\\midrule"]
names = {"ic-boundary-ledger-round5-2026-09-22": "ladder", "ic-boundary-ledger-round5-headline-2026-09-22": "headline",
         "ic-boundary-ledger-round5-holdout-2026-09-22": "holdout"}
for k, v in LB["reprice_boundary"].items():
    for x in v:
        L.append(f"{names[k]} & {tt(x['instance'])} & {f(x['log2_r'])} & {f(x['best_s'])} & {f(x['rho_frozen'])} & {f(x['rho_matched'],3)} & "
                 f"{f(x['best_vs_rho_frozen'])} & {f(x['best_vs_rho_matched'])} \\\\")
    L.append("\\addlinespace")
L = L[:-1] + ["\\bottomrule", "\\end{tabular}"]
write("tab-fr-reprice.tex", L)

# ---- rule v3..v8c ----
R = load("data-fr-rule.json")
vers = ["v3", "v4", "v6", "v7", "v8", "v8c"]
cells = collections.OrderedDict()
for c in R["cells"]:
    cells.setdefault(c["n"], {})[c["version"]] = c
order = sorted(cells, key=lambda n: cells[n]["v3"]["log2_r"])
for key, cik, name, d in (("cold_ratio", "cold_ratio_ci95", "tab-fr-rule.tex", 2), ("canonical_gap", "canonical_gap_ci95", "tab-fr-rulecanon.tex", 2)):
    L = ["\\begin{tabular}{@{}lr" + "r" * len(vers) + "@{}}", "\\toprule",
         "cell & $\\log_2 r$ & " + " & ".join(vers) + " \\\\", "\\midrule"]
    for n in order:
        c3 = cells[n]["v3"]
        L.append(f"$n={n}$, $a={c3['a']}$ & {f(c3['log2_r'])} & " + " & ".join(f(cells[n][v][key], d) for v in vers) + " \\\\")
        if any(cells[n][v][cik] for v in vers):
            L.append("\\multicolumn{2}{@{}r}{\\scriptsize 95\\%} & " + " & ".join(
                ("{\\scriptsize " + ci(cells[n][v][cik], 2) + "}") if cells[n][v][cik] else "" for v in vers) + " \\\\")
    if key == "cold_ratio":
        sl = R["cold_ratio_slope"]
        L.append("\\midrule")
        L.append("slope in $\\ln r$ & & " + " & ".join(f(sl[v]["slope"], 3) if v in sl else "" for v in vers) + " \\\\")
        L.append("\\multicolumn{2}{@{}r}{\\scriptsize 95\\%} & " + " & ".join(
            ("{\\scriptsize " + ci(sl[v]["ci95"], 3) + "}") if v in sl else "" for v in vers) + " \\\\")
    L += ["\\bottomrule", "\\end{tabular}"]
    write(name, L)

# derived ratios quoted in the text (printed, not tabulated)
for a_, b_, k_ in (("v8", "v8c", "cold_ratio"), ("v8", "v7", "cold_ratio"), ("v8", "v7", "canonical_gap"), ("v8", "v8c", "canonical_gap"), ("v3", "v7", "cold_ratio")):
    rs = [cells[n][a_][k_] / cells[n][b_][k_] for n in order]
    print(f"derived {a_}/{b_} {k_}: " + ", ".join(f"{x:.3f}" for x in rs) + f"  range {min(rs):.2f}-{max(rs):.2f}")
bw = [r for r in rows if r["family"] == "B"]
d0 = [r["band_ratio_w"][0] / r["ratio_rho"] for r in bw]
print(f"derived B band w=0 / board ratio: {min(d0):.3f}-{max(d0):.3f}")
print("derived B min/max over band and extended:", min(min(r["band_ratio_w"] + r["band_ratio_extended_w"]) for r in bw),
      max(max(r["band_ratio_w"] + r["band_ratio_extended_w"]) for r in bw))

# ---- matched rho ladder ----
D = load("data-fr-rholadder.json")
L = ["\\begin{tabular}{@{}llrrrrrrrr@{}}", "\\toprule",
     "regime & instance & $\\log_2 r$ & $J$ & frozen & tuned plain & negation & neg.\\ median & neg.\\ walk/floor & plain/neg.\\ walk \\\\",
     "\\midrule"]
for reg in ("prime", "char2"):
    for x in sorted([x for x in D["ladder"] if x["regime"] == reg], key=lambda x: x["log2_r"]):
        L.append(f"{reg} & {tt(x['instance'])} & {f(x['log2_r'])} & {x['jumps']} & {f(x['frozen_s'])} & {f(x['tuned_plain_s'],3)} & "
                 f"{f(x['negation_s'],3)} & {f(x['negation_median_s'],3)} & {f(x['negation_walk_over_own_floor'],3)} & {f(x['plain_over_negation_walk'],3)} \\\\")
    if reg == "prime": L.append("\\addlinespace")
L += ["\\bottomrule", "\\end{tabular}"]
write("tab-fr-rholadder.tex", L)

# ---- negation audit ----
C = load("data-fr-collision.json")
band = {(r["curve"], r["arm"]): r for r in C["band"]["rows"]}
pair = {c["slug"]: c for c in C["curves_paired"]}
L = ["\\begin{tabular}{@{}lrlrrrrrl@{}}", "\\toprule",
     "curve & $\\log_2 r$ & arm & mean $S$ [95\\%] & $S$/theory & wall ms & $w=0$ & $w=0.1$ & $w=1$ \\\\",
     "\\midrule"]
curves = sorted({t["curve"] for t in C["table"]}, key=lambda cv: next(t["log2_r"] for t in C["table"] if t["curve"] == cv))
for cv in curves:
    for arm in ("rho-neg", "rho-plain"):
        t = next(t for t in C["table"] if t["curve"] == cv and t["arm"] == arm)
        b = band[(cv, arm)]["mean_s"]
        L.append(f"{short(cv) if arm == 'rho-neg' else ''} & {f(t['log2_r']) if arm == 'rho-neg' else ''} & {arm} & {f(t['mean_s'],3)} {ci(t['ci95'],3)} & "
                 f"{f(t['s_over_theory'],3)} & {f(t['median_wall_ms'],1)} & {f(b[0],3)} & {f(b[1],3)} & {f(b[2],3)} \\\\")
    p = pair[cv]
    L.append(f" & & plain/neg. & {f(p['ratio_b_over_a'],3)} {ci(p['ci95'],3)} & & & & & \\\\")
    L.append("\\addlinespace")
po = C["pooled_ops"]
L.append(f"pooled & & plain/neg. & {f(po['ratio_b_over_a'],3)} {ci(po['ci95'],3)} & & & & & \\\\")
L += ["\\bottomrule", "\\end{tabular}"]
write("tab-fr-negaudit.tex", L)

# ---- Jul-Sep verdicts ----
S = load("data-fr-julsep.json")["summary"]
OUT = ("support", "support_scoped", "weaken", "reject_scoped", "inconclusive", "supersede", "pause", "replicate")
def cls(v):
    if v in OUT: return v
    if v.startswith("approve"): return "approve*"
    if v.startswith("revise") or v in ("amend", "refine"): return "revise*, amend, refine"
    if v == "expand": return "expand"
    return "other"
months = ["2026-07", "2026-08", "2026-09", "2026-10"]
cnt = collections.defaultdict(collections.Counter)
for k, v in S["verdicts"].items():
    m, vv = k.split("|", 1)
    cnt[cls(vv)][m] += v
order = ["approve*", "revise*, amend, refine", "expand"] + list(OUT) + ["other"]
L = ["\\begin{tabular}{@{}lrrrrr@{}}", "\\toprule",
     "verdict class & Jul & Aug & Sep & Jul--Sep & Oct-dated \\\\", "\\midrule"]
tot = collections.Counter()
for o in order:
    vals = [cnt[o][m] for m in months]
    for m, x in zip(months, vals): tot[m] += x
    name = o.replace("_", "\\_")
    L.append(f"{name} & {vals[0]} & {vals[1]} & {vals[2]} & {sum(vals[:3])} & {vals[3]} \\\\")
    if o == "expand" or o == "replicate": L.append("\\addlinespace")
L.append("\\midrule")
L.append(f"all decisions & {tot[months[0]]} & {tot[months[1]]} & {tot[months[2]]} & {sum(tot[m] for m in months[:3])} & {tot[months[3]]} \\\\")
de = S["distinct_experiments_by_month"]; ru = S["runs_by_month"]; dm = S["decisions_by_month"]
L.append(f"\\quad check: \\texttt{{decisions\\_by\\_month}} & {dm['2026-07']} & {dm['2026-08']} & {dm['2026-09']} & {dm['2026-07']+dm['2026-08']+dm['2026-09']} & {dm['2026-10']} \\\\")
L.append(f"distinct experiments & {de['2026-07']} & {de['2026-08']} & {de['2026-09']} & & {de['2026-10']} \\\\")
L.append(f"run directories & {ru['2026-07']} & {ru['2026-08']} & {ru['2026-09']} & {ru['2026-07']+ru['2026-08']+ru['2026-09']} & {ru['2026-10']} \\\\")
L += ["\\bottomrule", "\\end{tabular}"]
write("tab-fr-julsep.tex", L)
print({o: sum(cnt[o][m] for m in months[:3]) for o in order})

# ---- benchmarks ----
B = load("data-fr-bench.json")
def standing(r):
    if r["validation_linked"]: return "VALIDATION.json linked"
    if r["pre_gate"]: return "unvalidated (pre-gate)"
    return "admitted, no file linked in row"
sc = collections.defaultdict(collections.Counter)
for r in B["rows"]:
    sc[r["date"][:7]][standing(r)] += 1
cols = ["VALIDATION.json linked", "unvalidated (pre-gate)", "admitted, no file linked in row"]
L = ["\\begin{tabular}{@{}lrrrr@{}}", "\\toprule",
     "row date & \\texttt{VALIDATION.json} linked & unvalidated (pre-gate) & neither & all \\\\", "\\midrule"]
for g in sorted(sc):
    L.append(f"{g} & " + " & ".join(str(sc[g][c]) for c in cols) + f" & {sum(sc[g].values())} \\\\")
L.append("\\midrule")
L.append("all & " + " & ".join(str(sum(sc[g][c] for g in sc)) for c in cols) + f" & {len(B['rows'])} \\\\")
L += ["\\bottomrule", "\\end{tabular}"]
write("tab-fr-benchcount.tex", L)

# excerpts: copied from each row's headline column; every digit group is checked against it
EX = [
    ("20261102-e39-pivot-rules", "$k=4$ GLS/$V_4$ chain solve, pivot rules (E39)", "rho ahead everywhere, closest row F 5.18·10³× (0 GAE) and 1.11·10⁵× (1 GAE) rho"),
    ("20261102-e40-macaulay-entries", "same, Macaulay build stores rows from the lead column (E40)", "rho ahead everywhere, closest row 6.00·10³× / 1.58·10⁴× / 6.17·10⁴×"),
    ("20261031-e32-handover-ladder", "same, handover ladder $p=101$--$3089$, 33 curves (E32)", "rho ahead on all 33 curves at every weight; median distance 2.07·10⁴× (p = 101) to 1.54·10³× (p = 3089) at 0 GAE; best curve 1,143×"),
    ("20261030-e27-walk-karatsuba", "matched walk with Karatsuba products (E27)", "distance remaining 6.39·10³–4.06·10⁵× at 0 GAE, rho ahead across the §5a band"),
    ("20261024-e31-chain-solve", "chain solve, solvers A and B (E31)", "best S/rho 4.42·10³ (A) at 0 GAE, rho ahead at every weight"),
    ("20261029-jv-cover-hilbert-stop", "Joux--Vitse cover, F4 Hilbert stop", "log–log slope of S/rho −2.27; IC is a lower bound"),
    ("20261029-ic-aimed-gather", "Koblitz $n=53$ aimed gather, collect phase", "the board's rule v7 row (IC 8.146 vs rho 1.159 S, 7.03×) predates the change"),
    ("20261029-ic-packed-filter", "prime pair-table packed filter, 32 bits", "dgbf / rho-neg 31.47 / 31.50 / 31.63"),
    ("20261029-prime-fb-size-20bit", "prime base size $N=32\\to45$, 20 bits", "distance to rho 9.47 → 7.75 (w = 0), 12.88 → 9.35 (w = 1)"),
    ("20261028-d4-char2-frob-roots", "$k=4$ characteristic two, Frobenius roots", "IC/rho at m = 14: 4.27 → 3.48 (w = 0), 2.13 → 1.74 (w = 1)"),
    ("20261027-postgb-linear-stage", "post-Gröbner linear stage, \\texttt{jv\\_cover}", "S over the negation-map rho 257–1310 (pooled 497 at w = 0, 64 at w = 1)"),
    ("20261017-isogeny-volcano-cost-v2", "IC yield across volcano levels", "IC/rho 17–170× at w = 0"),
    ("20261018-blindspots-fp2-gaudry-k2", "Gaudry $k=2$ over $\\FF_{p^2}$, 24--40 bits", "best arms ic-b and ic-c2 50x and 52x rho at 40 bits (w=0), 22x and 14x (w=1)"),
    ("20261018-blindspots-prime-m4-cells", "prime arity $m=4$, 16--28 bits", "6.5x to 35.6x across w = 0, 0.1, 1"),
    ("20261018-blindspots-genus3-la-exponent", "genus-3 linear algebra, $b=20$", "below rho at w=0 (0.38x) and above it at w=0.1 (1.6x to 2.6x) and w=1 (3.7x to 6.5x)"),
]
byid = {r["entry"]: r for r in B["rows"]}
print("linked rows not in EX:", sorted(r["entry"] for r in B["rows"] if r["validation_linked"] and r["entry"] not in {e[0] for e in EX}))
def latexify(s):
    s = s.replace("_", "\\_").replace("§", "\\S")
    s = re.sub(r"(\d+(?:\.\d+)?)·10([⁰¹²³⁴⁵⁶⁷⁸⁹]+)", lambda m: "$" + m.group(1) + "\\cdot10^{" + m.group(2).translate(str.maketrans("⁰¹²³⁴⁵⁶⁷⁸⁹", "0123456789")) + "}$", s)
    s = s.replace("×", "$\\times$").replace("→", "$\\to$").replace("−", "$-$").replace("–", "--")
    return s
L = ["\\begin{tabularx}{\\linewidth}{@{}l>{\\raggedright}p{4.2cm}>{\\raggedright\\arraybackslash}X@{}}", "\\toprule",
     "date & entry and what it measures & distance to rho, as the row states it \\\\", "\\midrule"]
for eid, what, ex in EX:
    h = byid[eid]["headline"]
    for tok in re.findall(r"\d[\d.,]*", ex):
        assert tok.rstrip(".,") in h, (eid, tok)
    assert eid in byid and byid[eid]["validation_linked"], eid
    L.append(f"{byid[eid]['date'][5:]} & \\art{{{eid}}}: {what} & {latexify(ex)} \\\\")
L += ["\\bottomrule", "\\end{tabularx}"]
write("tab-fr-bench.tex", L)
