#!/usr/bin/env python3
"""render.py: analysis.json -> tables.md, scorecard.json, fig_light_rows.svg. Numbers are read from analysis.json."""
import json, math
A = json.load(open("analysis.json"))
PS = [32771, 65537, 131101]
UNM, PLAIN = "C_double_lp+pre", "C_double_lp+pre+sge(8,3)"
SETS = ["+ex(0.2,0.85)", "+ex(0.5,0.7)", "+ex(1,0.5)"]
ARMS = [UNM, PLAIN] + [PLAIN + s for s in SETS]
LAB = {UNM: "unmerged", PLAIN: "SGE(8,3)", PLAIN + SETS[0]: "ex(0.2,0.85)", PLAIN + SETS[1]: "ex(0.5,0.7)", PLAIN + SETS[2]: "ex(1.0,0.5)"}
COL = {UNM: "#9a9993", PLAIN: "#2a78d6", PLAIN + SETS[0]: "#eb6834", PLAIN + SETS[1]: "#1baf7a", PLAIN + SETS[2]: "#eda100"}
C = {(c["p"], c["arm"]): c for c in A["cells"]}
P = {(q["p"], q["setting"], q["stat"]): q for q in A["paired"]}
import os
XS = ["+ex(0.2,0.01)", "+ex(0.5,0.01)"]  # post-hoc, not preregistered (analysis_explore.json)
AE = json.load(open("analysis_explore.json")) if os.path.exists("analysis_explore.json") else None
if AE:
    for c in AE["cells"]:
        if c["arm"].endswith(tuple(XS)): C[(c["p"], c["arm"])] = c
    for q in AE["paired"]: P[(q["p"], q["setting"], q["stat"])] = q
    for x, lab, col in zip(XS, ["post-hoc ex(0.2,min)", "post-hoc ex(0.5,min)"], ["#e8a58c", "#7fd1b4"]):
        LAB[PLAIN + x] = lab; COL[PLAIN + x] = col
sci = lambda x: f"{x:.3g}"
out = ["# Tables (`[pcc:unverified]`, read from analysis.json)\n"]
for p in PS:
    out += [f"## p = {p} (genus 3, 3 curves x 2 seeds = 6 runs per arm)\n",
            "| arm | certified | relations collected | rows pre-selection / kept | cols | mean weight before -> after | nnz before SGE -> after SGE | la_mul | la_sel (mul, add) | rel phase (mul) | FPM_mid | FPM_mid / rho [95%] |",
            "|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|---|"]
    for a in ARMS:
        c = C[(p, a)]
        if a == UNM:
            out.append(f"| {LAB[a]} | {c['certified']}/{c['runs']} | {sci(c['mean_relations'])} | {sci(c['mean_core_rows'])} | {sci(c['mean_core_cols'])} | - | - | {sci(c['mean_la_mul'])} | - | {sci(c['mean_relcost'])} | {sci(c['mean_fpm'])} | {c['fpm_over_rho']['ratio']:.3f} [{c['fpm_over_rho']['lo']:.3f}, {c['fpm_over_rho']['hi']:.3f}] |")
            continue
        ex = "mean_core_rows_pre" in c
        rows_pre = c["mean_core_rows_pre"] if ex else c["mean_core_rows"]
        sel = f"{sci(c['mean_la_sel_mul'])}, {sci(c['mean_la_sel_add'])}" if ex else "-"
        cols = c["mean_core_cols"]
        out.append(f"| {LAB[a]} | {c['certified']}/{c['runs']} | {sci(c['mean_excess_rows_collected'] if ex else c['mean_relations'])} | {sci(rows_pre)} / {sci(c['mean_rows_kept'])} | {sci(cols)} | {c['mean_weight_before']:.2f} -> {c['mean_weight_after']:.2f} | {sci(c['mean_core_nnz_before_sge'])} -> {sci(c['mean_core_nnz_after_sge'])} | {sci(c['mean_la_mul'])} | {sel} | {sci(c['mean_relcost'])} | {sci(c['mean_fpm'])} | {c['fpm_over_rho']['ratio']:.3f} [{c['fpm_over_rho']['lo']:.3f}, {c['fpm_over_rho']['hi']:.3f}] |")
    out.append("")
out += ["## Paired ratios to the plain SGE(8,3) arm (mean of per-pair ratios, hierarchical bootstrap 95%)\n",
        "| p | setting | la_mul | relation phase (cantor_mul + 10 cantor_inv + rel_mul) | FPM_mid |", "|--:|---|---|---|---|"]
f = lambda q: f"{q['mean']:.3f} [{q['lo']:.3f}, {q['hi']:.3f}]"
for p in PS:
    for s in SETS:
        out.append(f"| {p} | {LAB[PLAIN + s]} | {f(P[(p, s, 'la_mul')])} | {f(P[(p, s, 'relcost')])} | {f(P[(p, s, 'fpm')])} |")
out += ["", "## LA share of FPM_mid (9 la_mul / FPM_mid, mean over runs)\n", "| p | " + " | ".join(LAB[a] for a in ARMS) + " |", "|--:|" + "--:|" * len(ARMS)]
for p in PS:
    out.append(f"| {p} | " + " | ".join(f"{C[(p, a)]['la_share_fpm']:.3f}" for a in ARMS) + " |")
if AE:
    out += ["", "## Post-hoc exploration (not preregistered, not in the scorecard): keep = 0.01, i.e. the lightest `cols + margin` rows", "",
            "Same instances, command of `PROTOCOL.md` with `--excess 0.2,0.01 --excess 0.5,0.01` (`explore.jsonl`); its plain arm again equals the earlier studies (`analysis_explore.json`, X1 block).", "",
            "| p | setting | relations | core rows before / kept | cols | mean weight before -> after | nnz before SGE -> after SGE | la_mul | la_sel (mul, add) | fallbacks | la_mul / plain | relation phase / plain | FPM_mid / plain | FPM_mid / rho |", "|--:|---|--:|--:|--:|--:|--:|--:|--:|--:|---|---|---|---|"]
    for p in PS:
        for x in XS:
            c = C[(p, PLAIN + x)]
            out.append(f"| {p} | {x[1:]} | {sci(c['mean_excess_rows_collected'])} | {sci(c['mean_core_rows_pre'])} / {sci(c['mean_rows_kept'])} | {sci(c['mean_core_cols'])} | {c['mean_weight_before']:.2f} -> {c['mean_weight_after']:.2f} | {sci(c['mean_core_nnz_before_sge'])} -> {sci(c['mean_core_nnz_after_sge'])} | {sci(c['mean_la_mul'])} | 0, {sci(c['mean_la_sel_add'])} | {c['mean_ex_fallback']:.1f} | {f(P[(p, x, 'la_mul')])} | {f(P[(p, x, 'relcost')])} | {f(P[(p, x, 'fpm')])} | {c['fpm_over_rho']['ratio']:.3f} [{c['fpm_over_rho']['lo']:.3f}, {c['fpm_over_rho']['hi']:.3f}] |")
open("tables.md", "w").write("\n".join(out) + "\n")

# scorecard
sc = {}
sc["X1"] = {"held": all(v["field_mismatches"] == 0 and not v["instance_mismatches"] and v["all_certified"] for v in A["x1"].values()), "detail": A["x1"]}
wa = {p: {s: C[(p, PLAIN + s)]["mean_weight_after"] for s in SETS} for p in PS}
mono = {p: wa[p][SETS[0]] > wa[p][SETS[1]] > wa[p][SETS[2]] and wa[p][SETS[0]] < C[(p, PLAIN)]["mean_weight_before"] for p in PS}
r07 = P[(131101, SETS[1], "la_mul")]["mean"]
sc["X2"] = {"weight_after": wa, "plain_weight": {p: C[(p, PLAIN)]["mean_weight_before"] for p in PS}, "monotone": mono, "la_mul_ratio_0.5_0.7_at_131101": r07,
            "held": all(mono.values()) and r07 <= 0.7}
best = min(SETS, key=lambda s: P[(131101, s, "fpm")]["mean"])
rb = {p: P[(p, best, "fpm")]["mean"] for p in PS}
sc["X3"] = {"best_setting_at_131101": best, "fpm_ratio_to_plain": rb, "held_part1": rb[131101] <= 0.85, "held_part2": rb[131101] < rb[32771], "held": rb[131101] <= 0.85 and rb[131101] < rb[32771]}
rel10 = {p: P[(p, SETS[2], "relcost")]["mean"] for p in PS}
n10 = {p: P[(p, SETS[2], "fpm")]["mean"] for p in PS}
n05 = {p: P[(p, SETS[1], "fpm")]["mean"] for p in PS}
sc["X4"] = {"relation_phase_ratio_1.0_0.5": rel10, "fpm_ratio_1.0_0.5": n10, "fpm_ratio_0.5_0.7": n05,
            "held_part1": all(v <= 2.0 for v in rel10.values()),
            "held_part2": n10[32771] > n05[32771] and n10[131101] < n05[131101]}
sc["X4"]["held"] = sc["X4"]["held_part1"] and sc["X4"]["held_part2"]
json.dump(sc, open("scorecard.json", "w"), indent=1)

# figure
SURF, INK, INK2, GRID = "#fcfcfb", "#0b0b0b", "#52514e", "#e4e3df"
W, H = 900, 430
s = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" font-family="sans-serif" font-size="12"><rect width="{W}" height="{H}" fill="{SURF}"/>']
def panel(x0, title, series, ymin, ymax, ylab, ref=None):
    w, h, y0 = 360, 270, 70
    lx = lambda p: x0 + (math.log2(p) - math.log2(PS[0])) / (math.log2(PS[-1]) - math.log2(PS[0])) * w
    ly = lambda v: y0 + h - (v - ymin) / (ymax - ymin) * h
    s.append(f'<text x="{x0}" y="36" fill="{INK}" font-size="14" font-weight="600">{title}</text>')
    s.append(f'<rect x="{x0}" y="{y0}" width="{w}" height="{h}" fill="none" stroke="{GRID}"/>')
    for k in range(5):
        v = ymin + (ymax - ymin) * k / 4
        s.append(f'<line x1="{x0}" x2="{x0+w}" y1="{ly(v):.1f}" y2="{ly(v):.1f}" stroke="{GRID}"/><text x="{x0-6}" y="{ly(v)+4:.1f}" fill="{INK2}" text-anchor="end">{v:.2f}</text>')
    for p in PS: s.append(f'<text x="{lx(p):.0f}" y="{y0+h+16}" fill="{INK2}" text-anchor="middle">{p}</text>')
    s.append(f'<text x="{x0}" y="{y0-8}" fill="{INK2}">{ylab}</text>')
    if ref is not None:
        s.append(f'<line x1="{x0}" x2="{x0+w}" y1="{ly(ref):.1f}" y2="{ly(ref):.1f}" stroke="{INK2}" stroke-dasharray="4 3"/>')
    for i, (lab, col, pts) in enumerate(series):
        d = " ".join(f"{lx(p):.1f},{ly(v):.1f}" for p, v in pts)
        s.append(f'<polyline points="{d}" fill="none" stroke="{col}" stroke-width="2"/>')
        for p, v in pts: s.append(f'<circle cx="{lx(p):.1f}" cy="{ly(v):.1f}" r="3.5" fill="{col}"/>')
        s.append(f'<text x="{x0+w-6}" y="{y0+16+15*i}" fill="{col}" text-anchor="end">{lab}</text>')
FIGARMS = ARMS + ([PLAIN + x for x in XS] if AE else [])
ser = [(LAB[a], COL[a], [(p, C[(p, a)]["fpm_over_rho"]["ratio"]) for p in PS]) for a in FIGARMS]
ymax = max(v for _, _, pts in ser for _, v in pts) * 1.1
panel(60, "FPM_mid over matched rho, genus 3", ser, 0, math.ceil(ymax * 10) / 10, "ratio to rho (mean of 6 runs)")
ser2 = [(LAB[PLAIN + st], COL[PLAIN + st], [(p, P[(p, st, "fpm")]["mean"]) for p in PS]) for st in SETS + (XS if AE else [])]
lo = min(v for _, _, pts in ser2 for _, v in pts); hi = max(v for _, _, pts in ser2 for _, v in pts)
panel(500, "FPM_mid of excess arms / plain SGE(8,3)", ser2, math.floor(min(lo, 0.5) * 10) / 10 if lo < 0.5 else 0.5, math.ceil(max(hi, 1.0) * 10) / 10, "paired ratio (1 = plain)", ref=1.0)
s.append("</svg>")
open("fig_light_rows.svg", "w").write("\n".join(s))
