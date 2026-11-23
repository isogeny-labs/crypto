#!/usr/bin/env python3
"""Layout only: analysis.json -> tables.md, fig_light_floor_g45.svg. Every number is read from analysis.json."""
import json
A = json.load(open("analysis.json")); L = A["scorecard"]
KEYS = ["g4_2053", "g4_4099", "g5_1031"]; NAME = {"g4_2053": "genus 4, p = 2053", "g4_4099": "genus 4, p = 4099", "g5_1031": "genus 5, p = 1031"}
ES = ["0.1", "0.2", "0.3"]; PLAIN = "C_double_lp+pre+sge(8,3)"; UNM = "C_double_lp+pre"
cell = {(c["key"], c["arm"]): c for c in A["cells"]}
def sci(x): return "%.3g" % x if abs(x) < 1e5 else "%.2e" % x
def iv(d, k="mean"): return "%.3f [%.3f, %.3f]" % (d[k], d["lo"], d["hi"])
def ivr(d): return "%.4f [%.4f, %.4f]" % (d["ratio"], d["lo"], d["hi"]) if d else "-"
o = ["# Tables (every figure `[pcc:unverified]`; means over the runs of each cell: 6 at genus 4, 9 at genus 5)\n",
     "FPM_mid = cantor_mul + 10 cantor_inv + rel_mul + g^2 la_mul (g^2 = 16 / 25); relation phase = cantor_mul + 10 cantor_inv + rel_mul. Paired ratio = mean of per-(curve, seed) ratios to the plain arm, hierarchical bootstrap 95% (3 curves: coarse). "
     "Ratio to rho = mean FPM of the arm over the mean of the matching rho runs on the same curves (g5: curves 39 and 42 only).\n"]
for k in KEYS:
    o.append("\n## %s (rho FPM_mid mean %s, %d runs on curves %s; plain LA share %.3f)\n" % (NAME[k], sci(A["rho"][k]["fpm_mean"]), A["rho"][k]["runs"], A["rho"][k]["curves_with_rho"], L["la_share_plain"][k]))
    o.append("| arm | rows collected | rows kept | weight before -> after | nnz before -> after SGE | la_mul | relation phase | FPM_mid | FPM / plain [95%] | la_mul / plain [95%] | FPM / rho [95%] |\n|---|--:|--:|--|--|--:|--:|--:|--|--|--|")
    for name, arm in [("unmerged", UNM), ("plain SGE(8,3)", PLAIN)] + [("ex(%s,0.01)" % e, PLAIN + "+ex(%s,0.01)" % e) for e in ES]:
        c = cell[(k, arm)]
        if "ex(" in arm:
            rc, rk = sci(c["mean_excess_rows_collected"]), sci(c["mean_rows_kept"]); wt = "%.2f -> %.2f" % (c["mean_wt_before"], c["mean_wt_after"])
            nz = "%s -> %s" % (sci(c["mean_core_nnz_before_sge"]), sci(c["mean_core_nnz_after_sge"]))
            pr = A["paired"]["%s|%s" % (k, arm.split("ex(")[1].split(",")[0])]; r1, r2 = iv(pr["fpm"]), iv(pr["la_mul"])
        elif "sge" in arm:
            rc, rk = sci(c["mean_relations"]), sci(c["mean_core_rows"]); wt = "%.2f" % c["mean_wt_before"]; nz = "%s -> %s" % (sci(c["mean_sge_nnz_in"]), sci(c["mean_sge_nnz_out"])); r1 = r2 = "1"
        else:
            rc, rk = sci(c["mean_relations"]), sci(c["mean_core_rows"]); wt = "-"; nz = "-"; r1 = r2 = "-"
        o.append("| %s | %s | %s | %s | %s | %s | %s | %s | %s | %s | %s |" % (name, rc, rk, wt, nz, sci(c["mean_la_mul"]), sci(c["mean_relcost"]), sci(c["mean_fpm"]), r1, r2, ivr(c["fpm_over_rho"])))
o.append("\n(`rows kept` of the plain/unmerged arms is the peeled core's row count; core columns: %s. The excess arms keep cols + margin rows, which is about the plain core at genus 5.)\n" % ", ".join("%s %.0f" % (NAME[k], cell[(k, PLAIN)]["mean_core_cols"]) for k in KEYS))
o.append("\n## Relation-phase ratio to plain (paired), weight ratio, LA share\n\n| cell | E | relation phase / plain | mean weight after / plain | LA share of the arm |\n|---|--:|--|--:|--:|")
for k in KEYS:
    for e in ES:
        d = A["paired"]["%s|%s" % (k, e)]; o.append("| %s | %s | %s | %.3f | %.3f |" % (NAME[k], e, iv(d["relcost"]), d["wt_after_over_plain"], cell[(k, PLAIN + "+ex(%s,0.01)" % e)]["la_share"]))
o.append("\n## Extrapolation to E > 0.3 (a model fitted to the three measured E, not a run): FPM_mid over plain\n\n| cell | slope of ln(la_mul) per unit E | E = 0.1 | 0.2 | 0.3 | 0.4 | 0.5 | 0.7 | 1.0 |\n|---|--:|--:|--:|--:|--:|--:|--:|--:|")
for k in KEYS:
    x = A["extrap"][k]; o.append("| %s | %.3f | %s |" % (NAME[k], x["la_slope_per_E"], " | ".join("%.3f" % x["model"][e]["fpm_over_plain"] for e in ("0.1", "0.2", "0.3", "0.4", "0.5", "0.7", "1.0"))))
o.append("\n## Scorecard inputs\n\nBest E per cell (lowest paired FPM ratio): %s. Best arm over rho at g4 p = 4099: %s, %s (plain %s)." % (L["best_E"], L["G5"]["best_arm"], ivr(L["G5"]["ratio"]), ivr(L["G5"]["plain"])))
open("tables.md", "w").write("\n".join(o) + "\n")
SURF, INK, INK2, GRID = "#fcfcfb", "#0b0b0b", "#52514e", "#e4e3df"; COL = {"g4_2053": "#2a78d6", "g4_4099": "#eb6834", "g5_1031": "#1baf7a"}
W, H = 900, 400; s = ['<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 %d %d" font-family="sans-serif" font-size="12"><rect width="%d" height="%d" fill="%s"/>' % (W, H, W, H, SURF)]
def panel(x0, title, ylo, yhi, ref):
    s.append('<text x="%d" y="28" font-size="14" font-weight="bold" fill="%s">%s</text>' % (x0, INK, title))
    for i in range(5):
        v = ylo + (yhi - ylo) * i / 4; y = 330 - (v - ylo) / (yhi - ylo) * 260
        s.append('<line x1="%d" x2="%d" y1="%.1f" y2="%.1f" stroke="%s"/><text x="%d" y="%.1f" text-anchor="end" fill="%s">%.2f</text>' % (x0, x0 + 360, y, y, GRID, x0 - 6, y + 4, INK2, v))
    yy = 330 - (ref - ylo) / (yhi - ylo) * 260
    s.append('<line x1="%d" x2="%d" y1="%.1f" y2="%.1f" stroke="%s" stroke-dasharray="2 3"/>' % (x0, x0 + 360, yy, yy, INK2))
    s.append('<text x="%d" y="362" fill="%s" text-anchor="middle">relation excess E</text>' % (x0 + 180, INK2))
X = lambda x0, i: x0 + 60 + i * 120
def lines(x0, stat, ylo, yhi):
    for k in KEYS:
        pts = [(X(x0, i), 330 - (A["paired"]["%s|%s" % (k, e)][stat]["mean"] - ylo) / (yhi - ylo) * 260) for i, e in enumerate(ES)]
        s.append('<polyline fill="none" stroke="%s" stroke-width="2" points="%s"/>' % (COL[k], " ".join("%.1f,%.1f" % q for q in pts)))
        for i, e in enumerate(ES):
            d = A["paired"]["%s|%s" % (k, e)][stat]; yl = 330 - (d["lo"] - ylo) / (yhi - ylo) * 260; yh = 330 - (d["hi"] - ylo) / (yhi - ylo) * 260
            s.append('<line x1="%.1f" x2="%.1f" y1="%.1f" y2="%.1f" stroke="%s"/><circle cx="%.1f" cy="%.1f" r="3.5" fill="%s"/>' % (pts[i][0], pts[i][0], yl, yh, COL[k], pts[i][0], pts[i][1], COL[k]))
    for i, e in enumerate(ES): s.append('<text x="%.1f" y="348" text-anchor="middle" fill="%s">%s</text>' % (X(x0, i), INK2, e))
panel(60, "FPM_mid of the floor arm over plain", 0.8, 1.2, 1.0); lines(60, "fpm", 0.8, 1.2)
panel(490, "la_mul of the floor arm over plain", 0.5, 1.1, 1.0); lines(490, "la_mul", 0.5, 1.1)
for j, k in enumerate(KEYS): s.append('<rect x="%d" y="48" width="12" height="12" fill="%s"/><text x="%d" y="59" fill="%s">%s (LA share %.2f)</text>' % (70 + j * 190, COL[k], 88 + j * 190, INK, NAME[k], L["la_share_plain"][k]))
s.append('<text x="60" y="388" fill="%s" font-size="10">3 curves per cell (6 runs genus 4, 9 genus 5), 95%% hierarchical bootstrap bars; dotted line = plain SGE(8,3) = 1; lines connect E only to guide the eye; [pcc:unverified]</text></svg>' % INK2)
open("fig_light_floor_g45.svg", "w").write("\n".join(s))
