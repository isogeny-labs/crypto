#!/usr/bin/env python3
"""Lay out paired.json (paired.py) and analysis.json (genus_ladder_analyze) as tables.md
and fig_sge.svg. Layout only: every number is read from those two files.

Usage: python3 render.py paired.json analysis.json
"""
import json
import sys

P = json.load(open(sys.argv[1] if len(sys.argv) > 1 else "paired.json"))
A = json.load(open(sys.argv[2] if len(sys.argv) > 2 else "analysis.json"))
SURF, INK, INK2, GRID = "#fcfcfb", "#0b0b0b", "#52514e", "#e4e3df"
SET_COL = {"sge(2,3)": "#2a78d6", "sge(4,3)": "#eb6834", "sge(8,3)": "#1baf7a"}
SETTINGS = sorted({x["setting"] for x in P["by_genus"]}, key=lambda s: int(s[4:].split(",")[0]))


def sci(x):
    if x == 0:
        return "0"
    if abs(x) >= 1e5:
        return f"{x:.2e}"
    if abs(x) >= 100:
        return f"{x:,.0f}"
    return f"{x:.3g}"


def ci(d):
    return f"{d['mean']:.3f} [{d['lo']:.3f}, {d['hi']:.3f}]"


out = []
w = out.append
w("# Tables: SGE before Wiedemann (genus ladder follow-up 2)\n")
w("Every number from `paired.json` (paired.py) or `analysis.json` (genus_ladder_analyze). "
  "`[pcc:unverified]`.\n")
w(f"Pairs: {P['pairs']}; relation phase identical: {P['relation_identical']}; "
  f"both certified: {P['certified_both']}; both recovered the planted t: {P['same_t']}; "
  f"unpaired: {P['unpaired']}. Best setting (lowest pooled la_mul ratio, genus 4 and 5): "
  f"`{P.get('best_setting')}`.\n")
w("## T1. Paired ratios per genus (mean of per-pair ratios, hierarchical bootstrap 95%)\n")
w("| setting | g | pairs | la_mul ratio | Wiedemann mul ratio | FPM_mid ratio |")
w("|---|--:|--:|--:|--:|--:|")
for x in P["by_genus"]:
    w(f"| {x['setting']} | {x['g']} | {x['la_mul_ratio']['pairs']} | {ci(x['la_mul_ratio'])} | "
      f"{ci(x['la_wied_mul_ratio'])} | {ci(x['fpm_mid_ratio'])} |")
w("\n## T2. Paired ratios per cell\n")
w("| setting | g | p | pairs | la_mul ratio | FPM_mid ratio |")
w("|---|--:|--:|--:|--:|--:|")
for x in P["by_cell"]:
    w(f"| {x['setting']} | {x['g']} | {x['p']} | {x['la_mul_ratio']['pairs']} | "
      f"{ci(x['la_mul_ratio'])} | {ci(x['fpm_mid_ratio'])} |")
w("\n## T3. What the merge did (cell means)\n")
w("| setting | g | p | rows in → out | cols in → out | nnz in → out | non-±1 in → out | merged | wt-1 | dup | empty | refused | merge mul | merge add | Wiedemann mul (base → sge) |")
w("|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|")
for x in P["structure"]:
    f = lambda k: x["mean_" + k]
    w(f"| {x['setting']} | {x['g']} | {x['p']} | {f('sge_rows_in'):.1f} → {f('sge_rows_out'):.1f} | "
      f"{f('sge_cols_in'):.1f} → {f('sge_cols_out'):.1f} | {f('sge_nnz_in'):.0f} → {f('sge_nnz_out'):.0f} | "
      f"{f('sge_generic_in'):.0f} → {f('sge_generic_out'):.0f} | {f('sge_merged'):.1f} | {f('sge_peeled'):.1f} | "
      f"{f('sge_duplicates'):.1f} | {f('sge_empty'):.1f} | {f('sge_refused'):.1f} | {sci(f('la_sge_mul'))} | "
      f"{sci(f('la_sge_add'))} | {sci(f('base_la_wied_mul'))} → {sci(f('la_wied_mul'))} |")
w("\n## T4. Cost shares of the unmerged arm and absolute FPM_mid (cell means)\n")
w("| g | p | base FPM_mid | g²·la_mul share of base FPM_mid | " +
  " | ".join(f"FPM_mid {s}" for s in SETTINGS) + " |")
w("|--:|--:|--:|--:|" + "--:|" * len(SETTINGS))
cells = {}
for x in P["structure"]:
    cells.setdefault((x["g"], x["p"]), {})[x["setting"]] = x
for (g, p), d in sorted(cells.items()):
    any_ = next(iter(d.values()))
    w(f"| {g} | {p} | {sci(any_['mean_base_fpm_mid'])} | {any_['mean_base_la_share_fpm']:.3f} | " +
      " | ".join(sci(d[s]["mean_fpm_mid"]) if s in d else "-" for s in SETTINGS) + " |")
w("\n## T5. genus_ladder_analyze cell means (FPM_mid, 95% interval)\n")
w("| g | p | arm | curves | FPM_mid mean [95%] | g²·mean la_mul / mean FPM_mid |")
w("|--:|--:|---|--:|--:|--:|")
for c in sorted(A["cells"], key=lambda c: (c["g"], c["p"], len(c["arm"]), c["arm"])):
    fm = c["fpm_mid"]
    share = c["g"] ** 2 * c["mean_la_mul"] / fm["mean"]
    w(f"| {c['g']} | {c['p']} | {c['arm']} | {c.get('curves', '-')} | {sci(fm['mean'])} "
      f"[{sci(fm['lo'])}, {sci(fm['hi'])}] | {share:.3f} |")
open("tables.md", "w").write("\n".join(out) + "\n")

# ── figure: per genus, per setting, la_mul and FPM_mid ratios with intervals ──
W, H = 760, 400
ml, mr, mt, mb = 64, 20, 48, 84
panels = [("la_mul_ratio", "linear algebra multiplications (merge included)"),
          ("fpm_mid_ratio", "whole logarithm, FPM_mid")]
pw = (W - ml - mr - 40) / 2
ph = H - mt - mb
svg = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" '
       f'font-family="Helvetica, Arial, sans-serif">',
       f'<rect width="{W}" height="{H}" fill="{SURF}"/>',
       f'<text x="{ml}" y="22" font-size="14" fill="{INK}">SGE before Wiedemann: with / without, '
       f'paired on identical relations (arm C_double_lp+pre)</text>']
genera = sorted({x["g"] for x in P["by_genus"]})
for k, (key, title) in enumerate(panels):
    x0 = ml + k * (pw + 40)
    y = lambda v: mt + ph * (1 - v / 1.1)
    svg.append(f'<text x="{x0}" y="{mt - 8}" font-size="12" fill="{INK2}">{title}</text>')
    for v in [0, 0.25, 0.5, 0.75, 1.0]:
        svg.append(f'<line x1="{x0}" x2="{x0 + pw}" y1="{y(v):.1f}" y2="{y(v):.1f}" '
                   f'stroke="{INK if v == 1.0 else GRID}" stroke-width="{1.2 if v == 1.0 else 1}"/>')
        svg.append(f'<text x="{x0 - 6}" y="{y(v) + 4:.1f}" font-size="11" fill="{INK2}" '
                   f'text-anchor="end">{v:.2f}</text>')
    gw = pw / len(genera)
    bw = gw * 0.7 / len(SETTINGS)
    for gi, g in enumerate(genera):
        gx = x0 + gi * gw + gw * 0.15
        svg.append(f'<text x="{x0 + gi * gw + gw / 2:.1f}" y="{mt + ph + 18}" font-size="12" '
                   f'fill="{INK}" text-anchor="middle">g = {g}</text>')
        for si, s in enumerate(SETTINGS):
            rec = next((x for x in P["by_genus"] if x["g"] == g and x["setting"] == s), None)
            if rec is None:
                continue
            d = rec[key]
            bx = gx + si * bw
            svg.append(f'<rect x="{bx:.1f}" y="{y(d["mean"]):.1f}" width="{bw - 2:.1f}" '
                       f'height="{y(0) - y(d["mean"]):.1f}" fill="{SET_COL.get(s, INK2)}"/>')
            cx = bx + (bw - 2) / 2
            svg.append(f'<line x1="{cx:.1f}" x2="{cx:.1f}" y1="{y(d["lo"]):.1f}" y2="{y(d["hi"]):.1f}" '
                       f'stroke="{INK}" stroke-width="1.2"/>')
    svg.append(f'<text x="{x0 - 44}" y="{mt + ph / 2}" font-size="11" fill="{INK2}" '
               f'transform="rotate(-90 {x0 - 44} {mt + ph / 2})" text-anchor="middle">ratio with / without</text>')
lx = ml
for s in SETTINGS:
    svg.append(f'<rect x="{lx}" y="{H - 46}" width="12" height="12" fill="{SET_COL.get(s, INK2)}"/>')
    svg.append(f'<text x="{lx + 16}" y="{H - 36}" font-size="12" fill="{INK}">w_max = '
               f'{s[4:-1].split(",")[0]}, fill_max = {s[4:-1].split(",")[1]}</text>')
    lx += 190
svg.append(f'<text x="{ml}" y="{H - 12}" font-size="11" fill="{INK2}">'
           f'bars: mean of paired ratios; whiskers: 95% bootstrap; [pcc:unverified]</text>')
svg.append("</svg>")
open("fig_sge.svg", "w").write("\n".join(svg) + "\n")
print("wrote tables.md, fig_sge.svg")
