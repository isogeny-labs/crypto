#!/usr/bin/env python3
"""Layout of analysis.json and amend1.json as tables.md and fig_cc.svg. No statistic computed here."""
import json, math
A = json.load(open("analysis.json")); B = json.load(open("amend1.json"))
CCS = ["1.0", "1.5", "2.0", "3.0", "4.0"]; SGE = "C_double_lp+pre+sge(8,3)"; UNM = "C_double_lp+pre"
def s(x): return f"{x:.3g}" if abs(x) < 1e5 else f"{x:.3e}"
L = ["# Tables [pcc:unverified]", "", "## Per-cell sweep (arm 2LP+pre+SGE(8,3); means over 6 runs; ratio to rho = FPM_mid over matched rho with 95% interval)", "",
     "| g | p | cc | trials | relations | fb_cols | core_cols | rel_mul | la_mul | FPM low | FPM mid | FPM high | mid/rho [lo, hi] | low/rho | high/rho | paired mid vs cc 2.0 [lo, hi] |", "|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|---|--:|--:|---|"]
for c in A["cells"]:
    for cc in CCS:
        e = c["by_cc"][cc]; r = e["ratio_mid"]; q = e["paired_vs_2.0"]
        L.append(f"| {c['g']} | {c['p']} | {cc} | {s(e['trials'])} | {s(e['relations'])} | {s(e['fb_cols'])} | {s(e['core_cols'])} | {s(e['rel_mul'])} | {s(e['la_mul'])} | {s(e['fpm_low'])} | {s(e['fpm_mid'])} | {s(e['fpm_high'])} | {r['ratio']:.3f} [{r['lo']:.3f}, {r['hi']:.3f}] | {e['ratio_low']['ratio']:.3f} | {e['ratio_high']['ratio']:.3f} | {q['mean']:.3f} [{q['lo']:.3f}, {q['hi']:.3f}] |")
L += ["", "## Best cc and ranking", "", "| g | p | best low | best mid | best high | gain of best (mid) vs 2.0 | ranking low | ranking mid | ranking high |", "|--:|--:|--:|--:|--:|--:|---|---|---|"]
for c in A["cells"]:
    L.append(f"| {c['g']} | {c['p']} | {c['best_cc']['low']} | {c['best_cc']['mid']} | {c['best_cc']['high']} | {100*c['gain_best_vs_2.0']:.1f}% | {' < '.join(c['rank']['low'])} | {' < '.join(c['rank']['mid'])} | {' < '.join(c['rank']['high'])} |")
L += ["", "## kappa-sensitivity (Amendment 1): la_mul / rel_mul per cc", "", "| g | p | arm | cc 1.0 | 1.5 | 2.0 | 3.0 | 4.0 |", "|--:|--:|---|--:|--:|--:|--:|--:|"]
for c in B:
    g = 3 if c["family"] == "g3" else 4
    for arm in (SGE, UNM):
        L.append(f"| {g} | {c['p']} | {arm} | " + " | ".join(f"{c['la_over_rel'][arm+'|'+cc]:.3f}" for cc in CCS) + " |")
L += ["", "## Best cc against the LA price kappa (I = 10; gain over cc 2.0 in brackets)", "", "| g | p | arm | k=1 | 2 | 3 | 4 | 6 | 9 |", "|--:|--:|---|--:|--:|--:|--:|--:|--:|"]
for c in B:
    g = 3 if c["family"] == "g3" else 4
    for arm in (SGE, UNM):
        L.append(f"| {g} | {c['p']} | {arm} | " + " | ".join(f"{c['best_cc_by_kappa'][arm+'|'+str(k)]['best']} ({100*c['best_cc_by_kappa'][arm+'|'+str(k)]['gain_vs_2.0']:.1f}%)" for k in (1, 2, 3, 4, 6, 9)) + " |")
L += ["", "## SGE effect at cc = 2.0 (merged over unmerged, same runs)", "", "| g | p | la_mul ratio | FPM mid ratio | unmerged mid/rho | merged mid/rho |", "|--:|--:|--:|--:|--:|--:|"]
for c in A["cells"]:
    e = c["by_cc"]["2.0"]; x = c["sge_effect_2.0"]
    L.append(f"| {c['g']} | {c['p']} | {x['la_ratio']:.3f} | {x['fpm_mid_ratio']:.3f} | {e['ratio_mid_unmerged']['ratio']:.4f} | {e['ratio_mid']['ratio']:.4f} |")
open("tables.md", "w").write("\n".join(L) + "\n")
# SVG
COL = ["#2a78d6", "#eb6834", "#1baf7a", "#eda100"]; W, H = 760, 460; X0, X1, Y0, Y1 = 70, 730, 40, 390
allr = [c["by_cc"][cc]["ratio_mid"]["ratio"] for c in A["cells"] for cc in CCS]
lo, hi = math.log10(min(allr)) - 0.15, math.log10(max(allr)) + 0.15
xs = {cc: X0 + (X1 - X0) * i / 4 + 0 for i, cc in enumerate(CCS)}
ys = lambda v: Y1 - (Y1 - Y0) * (math.log10(v) - lo) / (hi - lo)
o = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" font-family="sans-serif" font-size="12"><rect width="{W}" height="{H}" fill="#fcfcfb"/>',
     '<text x="70" y="22" font-size="14" fill="#0b0b0b">FPM_mid over matched rho against cc, 2LP+pre+SGE(8,3) [pcc:unverified]</text>']
for t in (0.02, 0.03, 0.05, 0.1, 0.2, 0.3, 0.5):
    if lo < math.log10(t) < hi:
        o.append(f'<line x1="{X0}" x2="{X1}" y1="{ys(t):.1f}" y2="{ys(t):.1f}" stroke="#e4e3df"/><text x="{X0-6}" y="{ys(t)+4:.1f}" text-anchor="end" fill="#52514e">{t}</text>')
for cc in CCS:
    o.append(f'<text x="{xs[cc]}" y="{Y1+18}" text-anchor="middle" fill="#52514e">{cc}</text>')
o.append(f'<text x="{(X0+X1)/2}" y="{Y1+38}" text-anchor="middle" fill="#52514e">cc (base = ceil(cc p^(1-1/g)))</text>')
for i, c in enumerate(A["cells"]):
    pts = " ".join(f"{xs[cc]:.1f},{ys(c['by_cc'][cc]['ratio_mid']['ratio']):.1f}" for cc in CCS)
    o.append(f'<polyline points="{pts}" fill="none" stroke="{COL[i]}" stroke-width="2"/>')
    for cc in CCS:
        e = c["by_cc"][cc]["ratio_mid"]
        o.append(f'<line x1="{xs[cc]:.1f}" x2="{xs[cc]:.1f}" y1="{ys(e["lo"]):.1f}" y2="{ys(e["hi"]):.1f}" stroke="{COL[i]}" stroke-opacity="0.35"/><circle cx="{xs[cc]:.1f}" cy="{ys(e["ratio"]):.1f}" r="3.5" fill="{COL[i]}"/>')
    o.append(f'<rect x="{80+i*170}" y="{H-24}" width="10" height="10" fill="{COL[i]}"/><text x="{95+i*170}" y="{H-14}" fill="#0b0b0b">g={c["g"]}, p={c["p"]}</text>')
o.append("</svg>")
open("fig_cc.svg", "w").write("\n".join(o))
