#!/usr/bin/env python3
"""Layout and scorecard from paired.json (paired.py) and analysis.json (genus_ladder_analyze).
Every number is read from those files or computed from recorded numbers (means, differences,
ratios). Writes tables.md, scorecard.json, fig_depth.svg.

Usage: python3 render.py paired.json analysis.json
"""
import json
import sys

P = json.load(open(sys.argv[1]))
A = json.load(open(sys.argv[2]))
SURF, INK, INK2, GRID = "#fcfcfb", "#0b0b0b", "#52514e", "#e4e3df"
COLS = ["#2a78d6", "#eb6834", "#1baf7a", "#eda100"]
BASE = "C_double_lp+pre"


def wf(s):
    a, b = s[4:-1].split(",")
    return int(a), float(b)


SETTINGS = sorted({x["setting"] for x in P["by_cell"]}, key=wf)
CELLS = sorted({(x["g"], x["p"]) for x in P["by_cell"]})
ST = {(x["setting"], x["g"], x["p"]): x for x in P["structure"]}
BY = {(x["setting"], x["g"], x["p"]): x for x in P["by_cell"]}
AC = {(c["arm"], c["g"], c["p"]): c for c in A["cells"]}


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


def rho_ratio(arm, g, p):
    c = AC.get((arm, g, p))
    r = c.get("fpm_mid_over_rho") if c else None
    return f"{r['ratio']:.3f} [{r['lo']:.3f}, {r['hi']:.3f}]" if r else "-"


out = []
w = out.append
w("# Tables: SGE merge depth (genus ladder, lead C1)\n")
w("Every number from `paired.json` (paired.py) or `analysis.json` (genus_ladder_analyze). "
  "`[pcc:unverified]`.\n")
w(f"Pairs: {P['pairs']}; relation phase identical: {P['relation_identical']}; both certified: "
  f"{P['certified_both']}; planted t recovered: {P['same_t']}; unpaired: {P['unpaired']}. "
  f"Best setting (lowest la ratio at genus 3, p = 32771): `{P.get('best_setting')}`.\n")
w("## T1. Per cell: la_mul (cell mean, merge included) and ratio to the unmerged arm\n")
w("| g | p | setting | mean la_mul | la_mul ratio (paired, 95%) | FPM_mid ratio to unmerged (paired, 95%) | FPM_mid / rho (95%) |")
w("|--:|--:|---|--:|--:|--:|--:|")
for g, p in CELLS:
    s0 = ST[(SETTINGS[0], g, p)]
    w(f"| {g} | {p} | unmerged | {sci(s0['mean_base_la_mul'])} | 1 | 1 | {rho_ratio(BASE, g, p)} |")
    for s in SETTINGS:
        k = (s, g, p)
        w(f"| {g} | {p} | {s} | {sci(ST[k]['mean_la_mul'])} | {ci(BY[k]['la_mul_ratio'])} | "
          f"{ci(BY[k]['fpm_mid_ratio'])} | {rho_ratio(BASE + '+' + s, g, p)} |")
w("\n## T2. What the merge did (cell means)\n")
w("| g | p | setting | rows in → out | cols in → out | nnz in → out (growth) | non-±1 in → out | merged | wt-1 | dup | empty | refused | merge mul | merge add | Wiedemann mul (base → merged) |")
w("|--:|--:|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|")
for g, p in CELLS:
    for s in SETTINGS:
        x = ST[(s, g, p)]
        f = lambda k: x["mean_" + k]
        w(f"| {g} | {p} | {s} | {f('sge_rows_in'):.0f} → {f('sge_rows_out'):.0f} | "
          f"{f('sge_cols_in'):.0f} → {f('sge_cols_out'):.0f} | {f('sge_nnz_in'):.0f} → "
          f"{f('sge_nnz_out'):.0f} ({f('sge_nnz_out') / f('sge_nnz_in'):.2f}x) | "
          f"{f('sge_generic_in'):.0f} → {f('sge_generic_out'):.0f} | {f('sge_merged'):.1f} | "
          f"{f('sge_peeled'):.1f} | {f('sge_duplicates'):.1f} | {f('sge_empty'):.1f} | "
          f"{f('sge_refused'):.1f} | {sci(f('la_sge_mul'))} | {sci(f('la_sge_add'))} | "
          f"{sci(f('base_la_wied_mul'))} → {sci(f('la_wied_mul'))} |")
w("\n## T3. Absolute FPM_mid (cell means) and la share of the unmerged arm\n")
w("| g | p | unmerged FPM_mid | g²·la share | " + " | ".join(SETTINGS) + " |")
w("|--:|--:|--:|--:|" + "--:|" * len(SETTINGS))
for g, p in CELLS:
    s0 = ST[(SETTINGS[0], g, p)]
    w(f"| {g} | {p} | {sci(s0['mean_base_fpm_mid'])} | {s0['mean_base_la_share_fpm']:.3f} | " +
      " | ".join(sci(ST[(s, g, p)]["mean_fpm_mid"]) for s in SETTINGS) + " |")
open("tables.md", "w").write("\n".join(out) + "\n")

# ── scorecard ──
sc = {}
sc["D1"] = {"pairs": P["pairs"], "relation_identical": P["relation_identical"],
            "certified_both": P["certified_both"], "same_t": P["same_t"],
            "held": P["pairs"] == P["relation_identical"] == P["certified_both"] == P["same_t"]
            and P["unpaired"] == 0}
mono, viol = True, []
for g, p in CELLS:
    la = lambda s: ST[(s, g, p)]["mean_la_mul"]
    for a, b, tag in [("sge(8,3)", "sge(16,3)", "8>16 F3"), ("sge(16,3)", "sge(32,3)", "16>32 F3"),
                      ("sge(16,10)", "sge(32,10)", "16>32 F10")]:
        ok = la(a) > la(b)
        sc.setdefault("D2_steps", []).append({"g": g, "p": p, "step": tag, "from": la(a),
                                              "to": la(b), "falls": ok})
        if not ok:
            mono = False
            viol.append(f"g{g} p{p} {tag}")
best = P.get("best_setting")
b32 = BY[(best, 3, 32771)]["la_mul_ratio"]["mean"] if best and (best, 3, 32771) in BY else None
sc["D2"] = {"monotone": mono, "violations": viol, "best": best, "best_la_ratio_g3_32771": b32,
            "held": mono and b32 is not None and b32 <= 0.25}
if best and (best, 3, 8209) in BY and b32 is not None:
    d = abs(b32 - BY[(best, 3, 8209)]["la_mul_ratio"]["mean"])
    sc["D3"] = {"best": best, "ratio_8209": BY[(best, 3, 8209)]["la_mul_ratio"]["mean"],
                "ratio_32771": b32, "abs_diff": d, "held": d <= 0.03}
    per = {s: BY[(s, 3, 32771)]["la_mul_ratio"]["mean"] - BY[(s, 3, 8209)]["la_mul_ratio"]["mean"]
           for s in SETTINGS}
    sc["D3_all_settings_drift_32771_minus_8209"] = per
d4 = {}
if best and (BASE + "+" + best, 3, 32771) in AC:
    r = AC[(BASE + "+" + best, 3, 32771)]["fpm_mid_over_rho"]["ratio"]
    d4["g3_32771_fpm_over_rho"] = r
    d4["g3_ok"] = r <= 0.22
if (best, 4, 2053) in BY:
    r = BY[(best, 4, 2053)]["fpm_mid_ratio"]["mean"]
    d4["g4_2053_best_over_unmerged_fpm"] = r
    d4["g4_ok"] = r <= 0.45
d4["held"] = bool(d4.get("g3_ok")) and bool(d4.get("g4_ok"))
sc["D4"] = d4
d5 = []
for g, p in CELLS:
    for W_ in (16, 32):
        a, b = ST[(f"sge({W_},3)", g, p)], ST[(f"sge({W_},10)", g, p)]
        d5.append({"g": g, "p": p, "w_max": W_, "la_F3": a["mean_la_mul"], "la_F10": b["mean_la_mul"],
                   "rel_change": b["mean_la_mul"] / a["mean_la_mul"] - 1,
                   "merged_F3": a["mean_sge_merged"], "merged_F10": b["mean_sge_merged"],
                   "refused_F3": a["mean_sge_refused"], "refused_F10": b["mean_sge_refused"]})
sc["D5_cells_w16_w32"] = d5
sc["D5_note"] = "w_max 8 has only fill_max 3 in the grid (no (8,10) arm); see PROTOCOL deviation"
json.dump(sc, open("scorecard.json", "w"), indent=1, sort_keys=True)

# ── figure ──
W, H = 780, 420
ml, mr, mt, mb = 60, 20, 50, 92
pw = (W - ml - mr - 50) / 2
ph = H - mt - mb
LINES = [(3, 8209), (3, 32771), (4, 2053), (5, 1031)]
LINES = [c for c in LINES if c in CELLS]
xs = {8: 0, 16: 1, 32: 2}
svg = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" '
       f'font-family="Helvetica, Arial, sans-serif">', f'<rect width="{W}" height="{H}" fill="{SURF}"/>',
       f'<text x="{ml}" y="22" font-size="14" fill="{INK}">SGE merge depth: ratio to the unmerged arm '
       f'against w_max (identical relations)</text>']
for k, (key, title, top) in enumerate([("la_mul_ratio", "linear algebra multiplications", 1.0),
                                       ("fpm_mid_ratio", "whole logarithm, FPM_mid", 1.0)]):
    x0 = ml + k * (pw + 50)
    X = lambda wm: x0 + 20 + (pw - 40) * xs[wm] / 2
    Y = lambda v: mt + ph * (1 - v / top)
    svg.append(f'<text x="{x0}" y="{mt - 10}" font-size="12" fill="{INK2}">{title}</text>')
    for v in [0, 0.25, 0.5, 0.75, 1.0]:
        svg.append(f'<line x1="{x0}" x2="{x0 + pw}" y1="{Y(v):.1f}" y2="{Y(v):.1f}" stroke="{GRID}"/>')
        svg.append(f'<text x="{x0 - 6}" y="{Y(v) + 4:.1f}" font-size="11" fill="{INK2}" text-anchor="end">{v:.2f}</text>')
    for wm in (8, 16, 32):
        svg.append(f'<text x="{X(wm):.1f}" y="{mt + ph + 16}" font-size="11" fill="{INK}" text-anchor="middle">{wm}</text>')
    svg.append(f'<text x="{x0 + pw / 2}" y="{mt + ph + 32}" font-size="11" fill="{INK2}" text-anchor="middle">w_max</text>')
    for ci_, (g, p) in enumerate(LINES):
        col = COLS[ci_]
        pts = [(wm, BY[(f"sge({wm},3)", g, p)][key]) for wm in (8, 16, 32) if (f"sge({wm},3)", g, p) in BY]
        svg.append('<polyline fill="none" stroke="%s" stroke-width="2" points="%s"/>' %
                   (col, " ".join(f"{X(a):.1f},{Y(d['mean']):.1f}" for a, d in pts)))
        for a, d in pts:
            svg.append(f'<line x1="{X(a):.1f}" x2="{X(a):.1f}" y1="{Y(min(d["hi"], top)):.1f}" y2="{Y(d["lo"]):.1f}" stroke="{col}" stroke-width="1"/>')
            svg.append(f'<circle cx="{X(a):.1f}" cy="{Y(d["mean"]):.1f}" r="3.5" fill="{col}"/>')
        for wm in (16, 32):
            kk = (f"sge({wm},10)", g, p)
            if kk in BY:
                svg.append(f'<circle cx="{X(wm) + 5:.1f}" cy="{Y(BY[kk][key]["mean"]):.1f}" r="3.5" fill="{SURF}" stroke="{col}" stroke-width="1.5"/>')
lx = ml
for ci_, (g, p) in enumerate(LINES):
    svg.append(f'<rect x="{lx}" y="{H - 52}" width="12" height="12" fill="{COLS[ci_]}"/>')
    svg.append(f'<text x="{lx + 16}" y="{H - 42}" font-size="12" fill="{INK}">genus {g}, p = {p}</text>')
    lx += 170
svg.append(f'<text x="{ml}" y="{H - 22}" font-size="11" fill="{INK2}">filled: fill_max 3; hollow (offset right): fill_max 10; whiskers: 95% bootstrap</text>')
svg.append(f'<text x="{ml}" y="{H - 8}" font-size="11" fill="{INK2}">[pcc:unverified]</text>')
svg.append("</svg>")
open("fig_depth.svg", "w").write("\n".join(svg) + "\n")
print("wrote tables.md scorecard.json fig_depth.svg")
