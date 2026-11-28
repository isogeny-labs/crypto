#!/usr/bin/env python3
"""Layout only: analysis.json -> tables.md, fig_light_floor.svg. Every number is read from analysis.json."""
import json
A = json.load(open("analysis.json")); L = A["scorecard"]
PS = [65537, 131101, 262147]; ES = ["0.1", "0.2", "0.3"]
PLAIN = "C_double_lp+pre+sge(8,3)"; UNM = "C_double_lp+pre"
cell = {(c["p"], c["arm"]): c for c in A["cells"]}
def sci(x): return "%.3g" % x if abs(x) < 1e5 else "%.2e" % x
def iv(d, k="mean"): return "%.3f [%.3f, %.3f]" % (d[k], d["lo"], d["hi"])
o = ["# Tables (every figure `[pcc:unverified]`; 6 fresh curves x 2 seeds = 12 runs per arm per prime; means over the 12 runs)\n",
     "FPM_mid = cantor_mul + 10 cantor_inv + rel_mul + 9 la_mul; relation phase = cantor_mul + 10 cantor_inv + rel_mul. Paired ratio = mean of per-(curve, seed) ratios to the plain arm, hierarchical bootstrap 95%. Ratio to rho = mean FPM of the arm over the mean of R2's 12 rho runs on the same curves.\n"]
for p in PS:
    o.append("\n## p = %d (rho FPM_mid mean %s, %d runs)\n" % (p, sci(A["rho"][str(p)]["fpm_mean"]), A["rho"][str(p)]["runs"]))
    o.append("| arm | rows collected | rows kept | weight before -> after | nnz before -> after SGE | la_mul | relation phase | FPM_mid | FPM / plain [95%] | la_mul / plain [95%] | FPM / rho [95%] |\n|---|--:|--:|--|--|--:|--:|--:|--|--|--|")
    for name, arm in [("unmerged", UNM), ("plain SGE(8,3)", PLAIN)] + [("ex(%s,0.01)" % e, PLAIN + "+ex(%s,0.01)" % e) for e in ES]:
        c = cell[(p, arm)]; ex = "ex(" in arm
        if ex:
            rc, rk = sci(c["mean_excess_rows_collected"]), sci(c["mean_rows_kept"])
            wt = "%.2f -> %.2f" % (c["mean_wt_before"], c["mean_wt_after"]); nz = "%s -> %s" % (sci(c["mean_core_nnz_before_sge"]), sci(c["mean_core_nnz_after_sge"]))
            pr = A["paired"]["%d|%s" % (p, arm.split("ex(")[1].split(",")[0])]; r1, r2 = iv(pr["fpm"]), iv(pr["la_mul"])
        elif "sge" in arm:
            rc, rk = sci(c["mean_relations"]), sci(c["mean_core_rows"]); wt = "%.2f" % c["mean_wt_before"]; nz = "%s -> %s" % (sci(c["mean_sge_nnz_in"]), sci(c["mean_sge_nnz_out"])); r1 = r2 = "1"
        else:
            rc, rk = sci(c["mean_relations"]), sci(c["mean_core_rows"]); wt = "-"; nz = "-"; r1 = r2 = "-"
        o.append("| %s | %s | %s | %s | %s | %s | %s | %s | %s | %s | %s |" % (name, rc, rk, wt, nz, sci(c["mean_la_mul"]), sci(c["mean_relcost"]), sci(c["mean_fpm"]), r1, r2, iv(c["fpm_over_rho"], "ratio")))
o.append("\n(For the excess arms `nnz before` is the non-zeros of the core after the lightest-row selection, handed to SGE; `weight before` is the mean row weight of the peeled core before selection. LA share of FPM_mid, plain / ex(0.2) / ex(0.3) at 262147: %.3f / %.3f / %.3f.)\n" % (cell[(262147, PLAIN)]["la_share"], cell[(262147, PLAIN + "+ex(0.2,0.01)")]["la_share"], cell[(262147, PLAIN + "+ex(0.3,0.01)")]["la_share"]))
o.append("\n## Relation-phase ratio to plain (paired) and weight ratio\n\n| p | E | relation phase / plain | mean weight after / plain |\n|--:|--:|--|--:|")
for p in PS:
    for e in ES:
        d = A["paired"]["%d|%s" % (p, e)]; o.append("| %d | %s | %s | %.3f |" % (p, e, iv(d["relcost"]), d["wt_after_over_plain"]))
ex = L["exponents"]
o.append("\n## Exponent of mean la_mul in p (three primes, descriptive)\n\n| arm | exponent |\n|---|--:|\n| plain SGE(8,3) | %.3f |\n| ex(0.1,0.01) | %.3f |\n| ex(0.2,0.01) | %.3f |\n| ex(0.3,0.01) (best E) | %.3f |\n\nFPM_mid exponent: plain %.3f, ex(0.3) %.3f.\n" % (ex["plain_la_mul"], ex["E0.1_la_mul"], ex["E0.2_la_mul"], ex["E0.3_la_mul"], ex["plain_fpm"], ex["best_fpm"]))
open("tables.md", "w").write("\n".join(o) + "\n")
# ---- SVG ----
SURF, INK, INK2, GRID = "#fcfcfb", "#0b0b0b", "#52514e", "#e4e3df"; COL = {"0.1": "#2a78d6", "0.2": "#eb6834", "0.3": "#1baf7a"}
W, H = 900, 380; s = ['<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 %d %d" font-family="sans-serif" font-size="12"><rect width="%d" height="%d" fill="%s"/>' % (W, H, W, H, SURF)]
def panel(x0, title, ylo, yhi, ylabel):
    s.append('<text x="%d" y="28" font-size="14" font-weight="bold" fill="%s">%s</text>' % (x0, INK, title))
    for i in range(5):
        v = ylo + (yhi - ylo) * i / 4; y = 320 - (v - ylo) / (yhi - ylo) * 260
        s.append('<line x1="%d" x2="%d" y1="%.1f" y2="%.1f" stroke="%s"/><text x="%d" y="%.1f" text-anchor="end" fill="%s">%.2f</text>' % (x0, x0 + 360, y, y, GRID, x0 - 6, y + 4, INK2, v))
    s.append('<text x="%d" y="352" fill="%s" text-anchor="middle">p = 65537, 131101, 262147 (log spacing)</text>' % (x0 + 180, INK2))
    s.append('<text x="%d" y="48" fill="%s">%s</text>' % (x0, INK2, ylabel))
X = lambda x0, i: x0 + 60 + i * 120
def Y(v, ylo, yhi): return 320 - (v - ylo) / (yhi - ylo) * 260
panel(60, "FPM_mid and la_mul of the floor arm over plain", 0.5, 1.0, "paired ratio (solid: FPM_mid, dashed: la_mul); 95% bars")
for e in ES:
    for stat, dash in (("fpm", ""), ("la_mul", ' stroke-dasharray="5 4"')):
        pts = [(X(60, i), Y(A["paired"]["%d|%s" % (p, e)][stat]["mean"], 0.5, 1.0)) for i, p in enumerate(PS)]
        s.append('<polyline fill="none" stroke="%s" stroke-width="2"%s points="%s"/>' % (COL[e], dash, " ".join("%.1f,%.1f" % q for q in pts)))
        for i, p in enumerate(PS):
            d = A["paired"]["%d|%s" % (p, e)][stat]
            s.append('<line x1="%.1f" x2="%.1f" y1="%.1f" y2="%.1f" stroke="%s"/><circle cx="%.1f" cy="%.1f" r="3.5" fill="%s"/>' % (pts[i][0], pts[i][0], Y(d["lo"], .5, 1), Y(d["hi"], .5, 1), COL[e], pts[i][0], pts[i][1], COL[e]))
for i, p in enumerate(PS): s.append('<text x="%.1f" y="338" text-anchor="middle" fill="%s">%d</text>' % (X(60, i), INK2, p))
for j, e in enumerate(ES): s.append('<rect x="%d" y="%d" width="12" height="12" fill="%s"/><text x="%d" y="%d" fill="%s">E = %s</text>' % (80 + j * 90, 56, COL[e], 98 + j * 90, 67, INK, e))
panel(490, "FPM_mid over matched rho (R2 rho rows, same curves)", 0.2, 0.8, "ratio of means; 95% bars")
for k, (nm, arm, col) in enumerate([("plain SGE(8,3)", PLAIN, "#52514e"), ("ex(0.2,0.01)", PLAIN + "+ex(0.2,0.01)", COL["0.2"]), ("ex(0.3,0.01)", PLAIN + "+ex(0.3,0.01)", COL["0.3"])]):
    pts = [(X(490, i) + (k - 1) * 12, Y(cell[(p, arm)]["fpm_over_rho"]["ratio"], .2, .8)) for i, p in enumerate(PS)]
    s.append('<polyline fill="none" stroke="%s" stroke-width="2" points="%s"/>' % (col, " ".join("%.1f,%.1f" % q for q in pts)))
    for i, p in enumerate(PS):
        d = cell[(p, arm)]["fpm_over_rho"]
        s.append('<line x1="%.1f" x2="%.1f" y1="%.1f" y2="%.1f" stroke="%s"/><circle cx="%.1f" cy="%.1f" r="3.5" fill="%s"/>' % (pts[i][0], pts[i][0], Y(d["lo"], .2, .8), Y(d["hi"], .2, .8), col, pts[i][0], pts[i][1], col))
    s.append('<rect x="%d" y="56" width="12" height="12" fill="%s"/><text x="%d" y="67" fill="%s">%s</text>' % (500 + k * 120, col, 518 + k * 120, INK, nm))
for i, p in enumerate(PS): s.append('<text x="%.1f" y="338" text-anchor="middle" fill="%s">%d</text>' % (X(490, i), INK2, p))
yy = Y(0.33, .2, .8); s.append('<line x1="490" x2="850" y1="%.1f" y2="%.1f" stroke="%s" stroke-dasharray="2 3"/><text x="852" y="%.1f" fill="%s" font-size="10">L5 0.33</text>' % (yy, yy, INK2, yy + 3, INK2))
s.append('<text x="60" y="372" fill="%s" font-size="10">genus 3, 6 fresh curves x 2 seeds per prime, [pcc:unverified]; lines connect primes only to guide the eye</text></svg>' % INK2)
open("fig_light_floor.svg", "w").write("\n".join(s))
