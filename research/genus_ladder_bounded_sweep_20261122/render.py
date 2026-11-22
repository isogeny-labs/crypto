#!/usr/bin/env python3
"""Layout only: analysis.json -> tables.md, fig_fpm3_vs_w.svg. Numbers are read from analysis.json. [pcc:unverified]"""
import json, math
A = json.load(open("analysis.json")); PS = A["primes"]; C = A["cells"]; WS = A["ws"]; SC = A["scorecard"]; R = A["ratios"]
X = A["exponents"]; DD = A["exp_diff_vs_plain"]; SEL = A["selection"]
def sci(x): return "%.3g" % x if abs(x) < 1e5 else "%.3e" % x
def ci(d, f="%.3f"): return (f + " [" + f + ", " + f + "]") % (d["est"], d["lo"], d["hi"])
wn = lambda w: "w%g" % w
o = ["# Tables (every figure `[pcc:unverified]`; 3 curves x 2 seeds = 6 runs per arm per prime; cell mean = mean over curves of the mean over seeds)\n",
     "FPM3 = cantor_mul + 10 cantor_inv + rel_mul + 3 la_mul; relation phase = cantor_mul + 10 cantor_inv + rel_mul. E = mean `bd_extra` (relation excess reached). Ratios to plain SGE(8,3): ratio of cell means, 95% paired hierarchical bootstrap. W = 8 rows are R11's (same binary and flags), shown for reference only.\n"]
for p in PS:
    o.append("\n## p = %d\n" % p)
    o.append("| arm | runs | E | probes | w peeled | w after step | w after SGE | rows x cols after SGE | la_mul | relation phase | FPM3 | FPM3 / plain [95%] | la_mul / plain [95%] | relphase / plain [95%] |\n|---|--:|--:|--:|--:|--:|--:|--|--:|--:|--:|--|--|--|")
    pl = C["%d|plain" % p]
    o.append("| plain SGE(8,3) | %d | 0 | - | %s | - | %.2f | %s x %s | %s | %s | %s | 1 | 1 | 1 |" % (A["n_runs"]["%d|plain" % p], "-", pl["w_out"], sci(pl["rows_out"]), sci(pl["cols_out"]), sci(pl["la_mul"]), sci(pl["relphase"]), sci(pl["fpm3"])))
    for n, lab in [("r11w8", "W = 8 (R11)")] + [(wn(w), "W = %g" % w) for w in WS]:
        c = C.get("%d|%s" % (p, n))
        if not c: continue
        rr = lambda m: ci(R["%d|%s|%s" % (p, n, m)]) if n != "r11w8" else "-"
        r3 = rr("fpm3"); r2 = rr("la_mul"); r1 = rr("relphase")
        if n == "r11w8": r3 = "%.3f" % (c["fpm3"] / pl["fpm3"]); r2 = "%.3f" % (c["la_mul"] / pl["la_mul"]); r1 = "%.3f" % (c["relphase"] / pl["relphase"])
        mark = " (best)" if SEL[str(p)]["best_w"] == n else ""
        mark += " (const-E)" if SEL[str(p)]["const_excess_w"] == n else ""
        o.append("| %s%s | %d | %.2f | %.1f | %.2f | %.2f | %.2f | %s x %s | %s | %s | %s | %s | %s | %s |" % (lab, mark, A["n_runs"]["%d|%s" % (p, n)], c["bd_extra"], c["bd_probes"], c["w_pre"], c["w_sel"], c["w_out"], sci(c["rows_out"]), sci(c["cols_out"]), sci(c["la_mul"]), sci(c["relphase"]), sci(c["fpm3"]), r3, r2, r1))
    o.append("\ncapped runs (E hit EMAX), mean fraction: " + ", ".join("W=%g: %.2f" % (w, C["%d|%s" % (p, wn(w))]["bd_capped"]) for w in WS) + "; selection additions `la_sel_add`: " + ", ".join("W=%g: %s" % (w, sci(C["%d|%s" % (p, wn(w))]["la_sel_add"])) for w in WS))
o.append("\n## Selection per prime\n\n| p | best W (lowest FPM3) | FPM3 / plain [95%] | min over W of FPM3 / plain [95%] | const-excess W (E nearest 0.2) | E | FPM3 / plain [95%] | bootstrap share of draws choosing the same best W |\n|--:|--|--|--|--|--:|--|--:|")
for p in PS:
    s = SEL[str(p)]; fr = A["selection_freq"]["%d|best" % p]; tot = sum(fr.values())
    o.append("| %d | %s | %s | %s | %s | %.2f | %s | %.2f |" % (p, s["best_w"][1:], ci(R["%d|best|fpm3" % p]), ci(R["%d|best|minratio" % p]), s["const_excess_w"][1:], C["%d|%s" % (p, s["const_excess_w"])]["bd_extra"], ci(R["%d|ce|fpm3" % p]), fr.get(s["best_w"], 0) / tot))
o.append("\n## Exponents in p over %s (least squares of log cell mean on log p; 95%% bootstrap; `re` = W re-selected inside every draw, `fx` = selection fixed)\n" % ", ".join(map(str, PS)))
o.append("| arm | FPM3 (re) | FPM3 (fx) | relation phase (re) | la_mul (re) | FPM3 exponent minus plain's (re) | same (fx) |\n|---|--|--|--|--|--|--|")
rows = [("plain", "plain SGE(8,3)"), ("unmerged", "unmerged")] + [(wn(w), "W = %g (fixed over p)" % w) for w in WS] + [("env", "best-W envelope (4 W)"), ("envp", "envelope incl. plain (secondary)"), ("ce", "constant-excess arm")]
for a, lab in rows:
    dre = ci(DD["re|%s|fpm3" % a]) if a not in ("plain", "unmerged") else "-"
    dfx = ci(DD["fx|%s|fpm3" % a]) if a not in ("plain", "unmerged") else "-"
    o.append("| %s | %s | %s | %s | %s | %s | %s |" % (lab, ci(X["re|%s|fpm3" % a]), ci(X["fx|%s|fpm3" % a]), ci(X["re|%s|relphase" % a]), ci(X["re|%s|la_mul" % a]), dre, dfx))
o.append("\n## Scorecard\n")
o.append("- S1 (plain and unmerged rows equal R11's; all certified): **%s** (all certified %s; rows compared %d; mismatches %d; instance lines equal %s)" % ("holds" if SC["S1_holds"] else "FAILS", A["all_certified"], A["S1"]["compared"], len(A["S1"]["mismatch"]), A["S1"]["instances_equal"]))
o.append("- S2 (some W with FPM3 <= 0.99 of plain at every p): **%s**; min ratio per p: %s" % ("holds" if SC["S2_holds"] else "FAILS", "; ".join("%s: %s" % (p, ci(SC["S2_min_ratio"][str(p)])) for p in PS)))
o.append("- S3 (envelope FPM3 exponent <= plain - 0.02 and paired upper < 0): **%s**; envelope %s vs plain %.3f (limit %.3f); paired difference %s" % ("holds" if SC["S3_holds"] else "FAILS", ci(SC["S3_env_exp"]), SC["S3_plain_exp"], SC["S3_plain_exp"] - 0.02, ci(SC["S3_diff"])))
o.append("- S4 (constant-excess arm: relation-phase exponent <= 1.32 and LA exponent <= 1.47): **%s**; relation phase %s (%s), LA %s (%s)" % ("holds" if SC["S4_holds"] else "FAILS", ci(SC["S4_rel_exp"]), "ok" if SC["S4_rel_ok"] else "FAILS", ci(SC["S4_la_exp"]), "ok" if SC["S4_la_ok"] else "FAILS"))
open("tables.md", "w").write("\n".join(o) + "\n")
# ---- SVG: FPM3 / plain against W per p ----
SURF, INK, INK2, GRID = "#fcfcfb", "#0b0b0b", "#52514e", "#e4e3df"
COLS = ["#2a78d6", "#eb6834", "#1baf7a", "#eda100"]
W, H = 900, 520; L, Rr, T, B = 80, 220, 60, 70
xs = [8.0] + WS
ys = []
for p in PS:
    ys.append(C["%d|r11w8" % p]["fpm3"] / C["%d|plain" % p]["fpm3"])
    for w in WS: ys.append(C["%d|%s" % (p, wn(w))]["fpm3"] / C["%d|plain" % p]["fpm3"])
lo, hi = min(min(ys), 0.98) - 0.02, max(max(ys), 1.0) + 0.02
px = lambda w: L + (w - 7.8) / (10.2 - 7.8) * (W - L - Rr)
py = lambda v: H - B - (v - lo) / (hi - lo) * (H - T - B)
s = ['<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 %d %d" font-family="sans-serif" font-size="12"><rect width="%d" height="%d" fill="%s"/>' % (W, H, W, H, SURF)]
s.append('<text x="%d" y="28" font-size="15" font-weight="bold" fill="%s">FPM3 of the bounded-core arm over plain SGE(8,3), genus 3, against the weight target W</text>' % (L, INK))
t = math.ceil(lo * 20) / 20
while t < hi:
    y = py(t); s.append('<line x1="%d" x2="%d" y1="%.1f" y2="%.1f" stroke="%s"/><text x="%d" y="%.1f" text-anchor="end" fill="%s">%.2f</text>' % (L, W - Rr, y, y, GRID, L - 6, y + 4, INK2, t)); t += 0.05
for w in xs:
    x = px(w); s.append('<line x1="%.1f" x2="%.1f" y1="%d" y2="%d" stroke="%s"/><text x="%.1f" y="%d" text-anchor="middle" fill="%s">%g</text>' % (x, x, T, H - B, GRID, x, H - B + 18, INK2, w))
s.append('<line x1="%d" x2="%d" y1="%.1f" y2="%.1f" stroke="%s" stroke-width="1.5"/>' % (L, W - Rr, py(1.0), py(1.0), INK2))
s.append('<text x="%d" y="%.1f" fill="%s" font-size="11">plain = 1</text>' % (L + 6, py(1.0) - 5, INK2))
s.append('<text x="%d" y="%d" text-anchor="middle" fill="%s">weight target W (W = 8 from R11)</text>' % ((L + W - Rr) // 2, H - 36, INK2))
for j, p in enumerate(PS):
    pts = [(px(8.0), C["%d|r11w8" % p]["fpm3"] / C["%d|plain" % p]["fpm3"])] + [(px(w), None) for w in WS]
    vals = [(8.0, C["%d|r11w8" % p]["fpm3"] / C["%d|plain" % p]["fpm3"], None)]
    for w in WS:
        r = R["%d|%s|fpm3" % (p, wn(w))]; vals.append((w, r["est"], r))
    s.append('<polyline fill="none" stroke="%s" stroke-width="2" points="%s"/>' % (COLS[j], " ".join("%.1f,%.1f" % (px(w), py(v)) for w, v, _ in vals)))
    for w, v, r in vals:
        if r: s.append('<line x1="%.1f" x2="%.1f" y1="%.1f" y2="%.1f" stroke="%s" stroke-width="1.5"/>' % (px(w), px(w), py(r["lo"]), py(r["hi"]), COLS[j]))
        best = SEL[str(p)]["best_w"] == wn(w)
        s.append('<circle cx="%.1f" cy="%.1f" r="%d" fill="%s"%s/>' % (px(w), py(v), 6 if best else 4, COLS[j], ' stroke="%s" stroke-width="2"' % INK if best else ""))
    y = T + 24 + j * 24
    s.append('<rect x="%d" y="%d" width="12" height="12" fill="%s"/><text x="%d" y="%d" fill="%s">p = %d (best W = %s)</text>' % (W - Rr + 20, y - 10, COLS[j], W - Rr + 38, y, INK, p, SEL[str(p)]["best_w"][1:]))
s.append('<text x="%d" y="%d" fill="%s" font-weight="bold">arm: p</text>' % (W - Rr + 20, T + 4, INK))
s.append('<text x="%d" y="%d" fill="%s" font-size="11">bars: 95%% paired bootstrap;</text><text x="%d" y="%d" fill="%s" font-size="11">ringed dot: best W at that p</text>' % (W - Rr + 20, T + 24 + 4 * 24 + 8, INK2, W - Rr + 20, T + 24 + 4 * 24 + 24, INK2))
s.append('<text x="%d" y="%d" fill="%s" font-size="10">[pcc:unverified] 3 curves x 2 seeds per cell, counts only, shared 4-vCPU host</text></svg>' % (L, H - 8, INK2))
open("fig_fpm3_vs_w.svg", "w").write("\n".join(s))
