#!/usr/bin/env python3
"""Layout only: analysis.json -> tables.md, fig_la_mul_exponent.svg. Every number is read from analysis.json. [pcc:unverified]"""
import json, math, sys
A = json.load(open("analysis.json")); PS = A["primes"]; C = A["cells"]; X = A["exponents"]; SC = A["scorecard"]
NAMES = {"unmerged": "unmerged", "plain": "plain SGE(8,3)", "ex0.3": "ex(0.3,0.01) (R4)", "bd2": "bd(8) cover 2 (spec)", "bd1": "bd(8) cover 1"}
ORDER = ["unmerged", "plain", "ex0.3", "bd2", "bd1"]
def sci(x): return "%.3g" % x if abs(x) < 1e5 else "%.3e" % x
def ci(d): return "%.3f [%.3f, %.3f]" % (d["est"], d["lo"], d["hi"])
o = ["# Tables (every figure `[pcc:unverified]`; 3 curves x 2 seeds = 6 runs per arm per prime; cell mean = mean over curves of the mean over seeds)\n",
     "FPM3 = cantor_mul + 10 cantor_inv + rel_mul + 3 la_mul; relation phase = cantor_mul + 10 cantor_inv + rel_mul. Weights: mean row weight of the peeled core (before the step), of the core the step hands on (after the step), and of the merged system SGE hands to Wiedemann (nnz_out / rows_out). Ratios to plain: ratio of cell means, 95% hierarchical bootstrap (same curve and seed resample for every arm).\n"]
for p in PS:
    o.append("\n## p = %d\n" % p)
    o.append("| arm | runs | relations | core rows x cols (peeled) | rows x cols after step | w before | w after step | rows x cols after SGE | nnz after SGE | w after SGE | nnz / col | la_mul | relation phase | FPM3 | FPM3 / plain [95%] | la_mul / plain [95%] | probes | E | capped |\n|---|--:|--:|--|--|--:|--:|--|--:|--:|--:|--:|--:|--:|--|--|--:|--:|--:|")
    for n in ORDER:
        c = C.get("%d|%s" % (p, n))
        if not c: continue
        nr = A["n_runs"]["%d|%s" % (p, n)]
        if n == "unmerged":
            o.append("| %s | %d | %s | %s x %s | - | - | - | - | - | - | - | %s | %s | %s | - | - | - | - | - |" % (NAMES[n], nr, sci(c["relations"]), sci(c["core_rows"]), sci(c["core_cols"]), sci(c["la_mul"]), sci(c["relphase"]), sci(c["fpm3"])))
            continue
        step = n != "plain"
        r1 = ci(A["ratio_to_plain"]["%d|%s|fpm3" % (p, n)]); r2 = ci(A["ratio_to_plain"]["%d|%s|la_mul" % (p, n)])
        o.append("| %s | %d | %s | %s x %s | %s | %s | %s | %s x %s | %s | %.2f | %.2f | %s | %s | %s | %s | %s | %s | %s | %s |" % (
            NAMES[n], nr, sci(c["relations"]), sci(c["rows_pre"]), sci(c["cols_pre"]),
            ("%s x %s" % (sci(c["core_rows"]), sci(c["core_cols"]))) if step else "-",
            ("%.2f" % c["w_pre"]) if step else "%.2f" % c["w_in"], ("%.2f" % c["w_sel"]) if step else "-",
            sci(c["rows_out"]), sci(c["cols_out"]), sci(c["nnz_out"]), c["w_out"], c["nnz_per_col_out"], sci(c["la_mul"]), sci(c["relphase"]), sci(c["fpm3"]),
            r1, r2, ("%.1f" % c["bd_probes"]) if n.startswith("bd") else "-", ("%.2f" % c["bd_extra"]) if n.startswith("bd") else "-", ("%.2f" % c["bd_capped"]) if n.startswith("bd") else "-"))
    o.append("\n(The `w before` of the plain row is the weight of the cores handed to SGE, nnz_in / rows_in. la_mul includes graph, peel, selection (zero multiplications), SGE and Wiedemann; the step's own additions are `la_sel_add`: %s)\n" % ", ".join("%s %s" % (NAMES[n], sci(C["%d|%s" % (p, n)]["la_sel_add"])) for n in ("ex0.3", "bd2", "bd1") if "%d|%s" % (p, n) in C))
o.append("\n## Exponents in p over %s (least squares of log cell mean on log p; 95%% bootstrap over curves and seeds)\n" % ", ".join(map(str, PS)))
o.append("| arm | la_mul | FPM3 | relation phase | FPM1 | FPM9 |\n|---|--|--|--|--|--|")
for n in ORDER:
    if n in X: o.append("| %s | %s | %s | %s | %s | %s |" % (NAMES[n], ci(X[n]["la_mul"]), ci(X[n]["fpm3"]), ci(X[n]["relphase"]), ci(X[n]["fpm1"]), ci(X[n]["fpm9"])))
o.append("\nPaired difference of exponents against plain (same resamples):\n\n| arm | la_mul | FPM3 |\n|---|--|--|")
for n in ("ex0.3", "bd2", "bd1"):
    if n in X: o.append("| %s | %s | %s |" % (NAMES[n], ci(A["exp_diff_vs_plain"]["%s|la_mul" % n]), ci(A["exp_diff_vs_plain"]["%s|fpm3" % n])))
o.append("\n## Mean row weight after SGE across the primes (B2)\n\n| arm | " + " | ".join(str(p) for p in PS) + " | max / min [95%] |\n|---|" + "--:|" * len(PS) + "--|")
for n in ORDER:
    if n == "unmerged" or n not in A["w_spread"]: continue
    o.append("| %s | %s | %s |" % (NAMES[n], " | ".join("%.2f" % C["%d|%s" % (p, n)]["w_out"] for p in PS), ci(A["w_spread"][n])))
o.append("\n## Scorecard\n")
o.append("B1 (plain rows equal the earlier studies; all runs certified): **%s** (all certified: %s; rows compared %d; mismatches %d; instance lines equal: %s)\n" % ("holds" if SC["B1_holds"] else "FAILS", A["all_certified"], A["B1"]["compared"], len(A["B1"]["mismatch"]), A["B1"]["instances_equal"]))
o.append("| arm | B2 w spread max/min (<= 1.10) | B3 la_mul exponent (<= 1.42, upper < 1.47) | B4(i) FPM3/plain at largest p (<= 1) | B4(ii) FPM3 exponent vs plain |\n|---|--|--|--|--|")
for n in ("bd2", "bd1"):
    if n not in SC: continue
    s = SC[n]
    o.append("| %s | %.3f: %s | %.3f (upper %.3f): %s | %.3f: %s | %.3f vs %.3f: %s |" % (NAMES[n], s["B2_w_spread_max_over_min"], "holds" if s["B2_holds"] else "FAILS", s["B3_la_exp"], s["B3_hi"], "holds" if s["B3_holds"] else "FAILS", s["B4_fpm3_over_plain_262147"], "holds" if s["B4_i_holds"] else "FAILS", s["B4_ii_fpm3_exp"], s["B4_ii_plain_fpm3_exp"], "holds" if s["B4_ii_holds"] else "FAILS"))
open("tables.md", "w").write("\n".join(o) + "\n")
# ---- SVG: log-log la_mul against p ----
SURF, INK, INK2, GRID = "#fcfcfb", "#0b0b0b", "#52514e", "#e4e3df"
COL = {"plain": "#2a78d6", "ex0.3": "#eb6834", "bd1": "#1baf7a", "bd2": "#eda100"}
W, H = 900, 520; L, R, T, B = 90, 330, 60, 70
arms = [n for n in ("plain", "ex0.3", "bd2", "bd1") if n in X]
vals = [C["%d|%s" % (p, n)]["la_mul"] for p in PS for n in arms]
lo, hi = math.log10(min(vals)) - 0.08, math.log10(max(vals)) + 0.08
xl, xh = math.log(PS[0]) - 0.15, math.log(PS[-1]) + 0.15
px = lambda p: L + (math.log(p) - xl) / (xh - xl) * (W - L - R)
py = lambda v: H - B - (math.log10(v) - lo) / (hi - lo) * (H - T - B)
s = ['<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 %d %d" font-family="sans-serif" font-size="12"><rect width="%d" height="%d" fill="%s"/>' % (W, H, W, H, SURF)]
s.append('<text x="%d" y="28" font-size="15" font-weight="bold" fill="%s">Linear algebra la_mul against p, genus 3 (log-log), fitted slopes</text>' % (L, INK))
d0, d1 = math.floor(lo), math.ceil(hi)
for k in range(d0, d1 + 1):
    for m in (1, 2, 5):
        v = m * 10.0 ** k
        if lo < math.log10(v) < hi:
            y = py(v); s.append('<line x1="%d" x2="%d" y1="%.1f" y2="%.1f" stroke="%s"/><text x="%d" y="%.1f" text-anchor="end" fill="%s">%.0e</text>' % (L, W - R, y, y, GRID, L - 6, y + 4, INK2, v))
for p in PS:
    x = px(p); s.append('<line x1="%.1f" x2="%.1f" y1="%d" y2="%d" stroke="%s"/><text x="%.1f" y="%d" text-anchor="middle" fill="%s">%d</text>' % (x, x, T, H - B, GRID, x, H - B + 18, INK2, p))
s.append('<text x="%d" y="%d" text-anchor="middle" fill="%s">p (log scale)</text>' % ((L + W - R) // 2, H - 24, INK2))
for n in arms:
    pts = [(px(p), py(C["%d|%s" % (p, n)]["la_mul"])) for p in PS]
    ex = X[n]["la_mul"]["est"]; lm = sum(math.log(C["%d|%s" % (p, n)]["la_mul"]) for p in PS) / len(PS); lpm = sum(math.log(p) for p in PS) / len(PS)
    f = lambda p: math.exp(lm + ex * (math.log(p) - lpm))
    s.append('<line x1="%.1f" y1="%.1f" x2="%.1f" y2="%.1f" stroke="%s" stroke-width="1.5" stroke-dasharray="6 4"/>' % (px(PS[0]), py(f(PS[0])), px(PS[-1]), py(f(PS[-1])), COL[n]))
    s.append('<polyline fill="none" stroke="%s" stroke-width="2" points="%s"/>' % (COL[n], " ".join("%.1f,%.1f" % q for q in pts)))
    for q in pts: s.append('<circle cx="%.1f" cy="%.1f" r="4" fill="%s"/>' % (q[0], q[1], COL[n]))
ly = T + 10
s.append('<text x="%d" y="%d" font-weight="bold" fill="%s">arm: fitted slope [95%% interval]</text>' % (W - R + 20, ly, INK))
for j, n in enumerate(arms):
    y = ly + 22 + j * 24; d = X[n]["la_mul"]
    s.append('<rect x="%d" y="%d" width="12" height="12" fill="%s"/><text x="%d" y="%d" fill="%s">%s: %.3f [%.3f, %.3f]</text>' % (W - R + 20, y - 10, COL[n], W - R + 38, y, INK, NAMES[n], d["est"], d["lo"], d["hi"]))
s.append('<text x="%d" y="%d" fill="%s" font-size="11">solid: cell means (3 curves x 2 seeds);</text><text x="%d" y="%d" fill="%s" font-size="11">dashed: least-squares fit in log-log.</text>' % (W - R + 20, ly + 22 + len(arms) * 24 + 10, INK2, W - R + 20, ly + 22 + len(arms) * 24 + 26, INK2))
s.append('<text x="%d" y="%d" fill="%s" font-size="10">[pcc:unverified] genus 3, p = %s; shared 4-vCPU host, counts only</text></svg>' % (L, H - 6, INK2, ", ".join(map(str, PS))))
open("fig_la_mul_exponent.svg", "w").write("\n".join(s))
