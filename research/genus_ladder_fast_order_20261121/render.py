#!/usr/bin/env python3
"""Layout of analysis.json (genus_ladder_analyze) and timing.jsonl as tables.md and fig_order.svg. Numbers are read, not computed."""
import json, math
A = json.load(open("analysis.json")); T = [json.loads(l) for l in open("timing.jsonl")]
SURF, INK, INK2, GRID = "#fcfcfb", "#0b0b0b", "#52514e", "#e4e3df"
cells = [c for c in A["cells"] if c["g"] == 3 and c["family"] == "hyperelliptic"]
def ser(arm):
    return sorted((c["p"], c["fpm_mid_over_rho"]["ratio"], c["fpm_mid_over_rho"]["lo"], c["fpm_mid_over_rho"]["hi"]) for c in cells if c["arm"] == arm and "fpm_mid_over_rho" in c)
S1, S2 = ser("C_double_lp+pre"), ser("C_double_lp+pre+sge(8,3)")
out = ["# Tables (`[pcc:unverified]`, read from analysis.json and timing.jsonl)\n",
       "## FPM (primary) over matched rho, genus 3\n", "| p | 2LP+pre | 2LP+pre+SGE(8,3) | SGE band (FPM) |", "|--:|--:|--:|---|"]
for (p, a, al, ah), (_, b, bl, bh) in zip(S1, S2):
    band = [c for c in cells if c["p"] == p and c["arm"] == "C_double_lp+pre+sge(8,3)"][0]["band_fpm"]
    out.append(f"| {p} | {a:.3f} [{al:.3f}, {ah:.3f}] | {b:.3f} [{bl:.3f}, {bh:.3f}] | {band} |")
out += ["", "## Order search, wall per order computation (single thread, `genus_ladder run --threads 1`)\n", "| p | g | method | orders tried | wall per order (s) |", "|--:|--:|---|--:|--:|"]
for t in T: out.append(f"| {t['p']} | {t['g']} | {t['method']} | {t['tried']} | {t['wall_per_order_s']} |")
open("tables.md", "w").write("\n".join(out) + "\n")
W, H = 860, 420
s = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" font-family="sans-serif" font-size="12"><rect width="{W}" height="{H}" fill="{SURF}"/>']
def panel(x0, title, xs, series, ylog, ymin, ymax, ylab):
    w, h, y0 = 340, 270, 70
    lx = lambda v: x0 + (math.log2(v) - math.log2(xs[0])) / (math.log2(xs[-1]) - math.log2(xs[0])) * w
    ly = (lambda v: y0 + h - (math.log10(v) - math.log10(ymin)) / (math.log10(ymax) - math.log10(ymin)) * h) if ylog else (lambda v: y0 + h - (v - ymin) / (ymax - ymin) * h)
    s.append(f'<text x="{x0}" y="40" fill="{INK}" font-size="14" font-weight="600">{title}</text>')
    s.append(f'<rect x="{x0}" y="{y0}" width="{w}" height="{h}" fill="none" stroke="{GRID}"/>')
    for p in xs: s.append(f'<text x="{lx(p):.0f}" y="{y0+h+16}" fill="{INK2}" text-anchor="middle">{p}</text>')
    s.append(f'<text x="{x0}" y="{y0-8}" fill="{INK2}">{ylab}</text>')
    for lab, col, pts in series:
        d = " ".join(f"{lx(p):.1f},{ly(v):.1f}" for p, v in pts)
        s.append(f'<polyline points="{d}" fill="none" stroke="{col}" stroke-width="2"/>')
        for p, v in pts: s.append(f'<circle cx="{lx(p):.1f}" cy="{ly(v):.1f}" r="3.5" fill="{col}"/>')
    for i, (lab, col, pts) in enumerate(series):
        s.append(f'<text x="{x0+8}" y="{y0+h-10-16*i}" fill="{col}">{lab}</text>')
xs = [p for p, *_ in S1]
panel(40, "2LP+pre(+SGE) over rho, genus 3", xs, [("2LP+pre", "#eb6834", [(p, a) for p, a, *_ in S1]), ("2LP+pre+SGE(8,3)", "#2a78d6", [(p, a) for p, a, *_ in S2])], False, 0, 1.0, "FPM ratio to rho")
g3 = lambda m: [(t["p"], t["wall_per_order_s"]) for t in T if t["g"] == 3 and t["method"] == m]
panel(460, "Order search wall, genus 3 (s/order)", [8209, 1048583], [("enumerate", "#eb6834", g3("enumerate")), ("bsgs1", "#1baf7a", g3("bsgs1"))], True, 0.1, 100, "log wall")
s.append("</svg>"); open("fig_order.svg", "w").write("\n".join(s))
