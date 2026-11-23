#!/usr/bin/env python3
"""Render full_jacobian.json (2026-11-14 rerun with r >= 0.4 p; adapted from
research/full_jacobian_ic_20261113/render.py): full_jacobian.svg and tables.md (tables for RESULTS.md).

Usage: python3 render.py [full_jacobian.json]
Every figure is [pcc:unverified].
"""
import json
import math
import sys

path = sys.argv[1] if len(sys.argv) > 1 else "research/full_jacobian_ic_rp_20261114/full_jacobian.json"
d = json.load(open(path))
rows = [r for r in d["rows"] if "arms" in r]
out_dir = path.rsplit("/", 1)[0]


def fmt(x, nd=0):
    if x is None:
        return "-"
    if abs(x) >= 1e6:
        return f"{x:.3g}"
    return f"{x:.{nd}f}"


# ---------- tables ----------
t = []
t.append("### Group operations and totals per prime (means; native priced at w = 0.1 GAE)\n")
t.append("| p | r | #J | rho E (neg) ops | rho E total w=0.1 | A ops | A total w=0.1 | B ops | B total w=0.1 | A_S ops | B_S ops | rho J(r) ops |")
t.append("|---|---|---|---|---|---|---|---|---|---|---|---|")
for r in rows:
    a = r["arms"]
    t.append(
        f"| {r['p']} | {r['r']} | {r['jac_order']} | {fmt(r['rho_E']['negation']['total_ops'],1)} | {fmt(r['rho_E']['band_negation'][1],1)} | "
        f"{fmt(a['A']['group_ops'])} | {fmt(a['A']['total_band'][1])} | {fmt(a['B']['group_ops'])} | {fmt(a['B']['total_band'][1])} | "
        f"{fmt(a['A_S']['group_ops'])} | {fmt(a['B_S']['group_ops'])} | {fmt(a['rho_J_negation']['group_ops'],1)} |"
    )
t.append("")
t.append("### Ratio to the matched negation rho on E (band w = 0, 0.1, 1) and calibrated reading (0, w_cal, 10 w_cal)\n")
t.append("| p | arm | ratio w=0 | ratio w=0.1 | ratio w=1 | cal 0 | cal w_cal | cal 10 w_cal | class (band) | class (cal) |")
t.append("|---|---|---|---|---|---|---|---|---|---|")
for r in rows:
    for arm in ["A", "B", "A_S", "B_S"]:
        e = r["arms"][arm]
        rb, rc = e["ratio_neg_band"], e["ratio_neg_cal"]
        t.append(
            f"| {r['p']} | {arm} | {rb[0]:.1f} | {rb[1]:.1f} | {rb[2]:.1f} | {rc[0]:.1f} | {rc[1]:.1f} | {rc[2]:.1f} | {e['class_neg_band']} | {e['class_neg_cal']} |"
        )
t.append("")
t.append("### Arm detail (means over runs)\n")
t.append("| p | arm | runs | certified | exhausted | columns | trials | relations | core cols | attempts | rel mul | la mul | smooth rate | wall s |")
t.append("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
for r in rows:
    for arm in ["A", "B", "A_S", "B_S"]:
        e = r["arms"][arm]
        sr = e.get("smooth_rate_deg2")
        sr = "-" if sr is None or (isinstance(sr, float) and math.isnan(sr)) else f"{sr:.3f}"
        t.append(
            f"| {r['p']} | {arm} | {e['runs']} | {e['certified']} | {e['exhausted']} | {fmt(e['columns'])} | {fmt(e['trials'])} | {fmt(e['relations'])} | {fmt(e['core_cols'])} | {e['attempts']:.2f} | "
            f"{fmt(e['rel_mul'])} | {fmt(e['la_mul'])} | {sr} | {e['wall_s']:.2f} |"
        )
t.append("")
t.append("### Selection, order data and calibration\n")
t.append("| p (requested) | candidate | skipped (by reason) | n1 = #E1 | n2 = #E2 | h = #J/r | r^2 divides #J | ord(Z)/#J (3 inst.) | t_add E (ns) | t_add J (ns) | w_cal mul_p (J) | w_cal mul_r (J) |")
t.append("|---|---|---|---|---|---|---|---|---|---|---|---|")
for r in rows:
    ct = r["class_times_ns"]
    ta = r["t_add_ns"]
    t.append(
        f"| {r['p']} ({r.get('p_requested', r['p'])}) | {r['candidate']} | {json.dumps(r['skipped_candidates'])} | {r['e1_order']} | {r['e2_order']} | {r['h']} | {r['r_squared_divides_jac_order']} | "
        f"{', '.join(f'{z:.2f}' for z in r['z_order_over_jac_order'])} | {ta['E1']:.0f} | {ta['J_word']:.0f} | {ct['mul_p']/ta['J_word']:.4f} | {ct['mul_r']/ta['J_word']:.4f} |"
    )
t.append("")
for key in ("fits_stage1", "fits_all"):
    f = d[key]
    t.append(f"### Fitted exponents, {f['label']} (OLS slope of ln y on ln p, 95% t interval)\n")
    t.append("| quantity | slope | 95% interval | n |")
    t.append("|---|---|---|---|")
    for k, v in f.items():
        if k == "label" or v is None:
            continue
        t.append(f"| {k} | {v['slope']:.3f} | [{v['ci95'][0]:.3f}, {v['ci95'][1]:.3f}] | {v['n']} |")
    t.append("")

def ols(xs, ys):
    n = len(xs)
    lx = [math.log(v) for v in xs]
    ly = [math.log(v) for v in ys]
    mx, my = sum(lx) / n, sum(ly) / n
    sxx = sum((a - mx) ** 2 for a in lx)
    return sum((a - mx) * (b - my) for a, b in zip(lx, ly)) / sxx

t.append("### Secondary, not frozen: dependence on r (r >= 0.4 p by selection; r/p per prime listed)\n")
t.append("- r/p per prime: " + ", ".join(f"{r['p']}: {r['r']/r['p']:.3f}" for r in rows))
rs_ = [r["r"] for r in rows]
t.append(f"- slope of ln(rho E negation ops) on ln r: {ols(rs_, [r['rho_E']['negation']['total_ops'] for r in rows]):.3f}")
for arm in ("A", "B"):
    adj = [r["arms"][arm]["group_ops"] / math.sqrt(r["r"]) for r in rows]
    t.append(f"- {arm}: group ops / sqrt(r) per prime: " + ", ".join(f"{r['p']}: {v:.1f}" for r, v in zip(rows, adj)))
    t.append(f"- {arm}: slope of ln(group ops / sqrt(r)) on ln p: {ols([r['p'] for r in rows], adj):.3f}")
t.append("")
# ---- Amendment 1 (2026-11-15): field unit (primary), lower-bound labels ----
wi = d.get("inv_weight", 10.0)
t.append(f"### Field unit (primary, Amendment 1): one F_p multiplication or squaring; inversion = {wi:g}; native priced at w = 0.1\n")
t.append("IC arms A, B are lower bounds (`lower_bound: true`); A_S, B_S carry no Cantor tally (their base is rel_mul + la_mul only).\n")
t.append("| p | rho E (neg) mul+sqr | rho E inv | rho E total w=0.1 | A mul+sqr | A inv | A total w=0.1 | A/rho w=0.1 | A class | B mul+sqr | B inv | B total w=0.1 | B/rho w=0.1 | B class |")
t.append("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
for r in rows:
    a = r["arms"]
    f = r["rho_E"]["field"]
    t.append(
        f"| {r['p']} | {fmt(f['negation']['field_mul_sqr'],1)} | {fmt(f['negation']['field_inv'],1)} | {fmt(f['band_negation'][1])} | "
        + " | ".join(
            f"{fmt(a[x]['field_mul_sqr'])} | {fmt(a[x]['field_inv'],1)} | {fmt(a[x]['field_unit']['total_band'][1])} | "
            f"{fmt(a[x]['field_unit']['ratio_neg_band'][1],1)} | {a[x]['field_unit']['class_neg_band']}"
            for x in ("A", "B")
        )
        + " |"
    )
t.append("")
t.append("### Cantor step over E1 addition, in field operations (per group operation)\n")
t.append("| p | J mul+sqr / op | J inv / op | E1 mul+sqr / op | E1 inv / op | ratio at inv weight " + f"{wi:g}" + " | ratio w=0 | ratio w=30 |")
t.append("|---|---|---|---|---|---|---|---|")
for r in rows:
    A = r["arms"]["A"]
    fe = r["rho_E"]["field"]["negation"]
    go_e = r["rho_E"]["negation"]["total_ops"]
    jm, ji = A["field_mul_sqr"] / A["group_ops"], A["field_inv"] / A["group_ops"]
    em, ei = fe["field_mul_sqr"] / go_e, fe["field_inv"] / go_e
    q = lambda w: (jm + w * ji) / (em + w * ei)
    t.append(f"| {r['p']} | {jm:.1f} | {ji:.2f} | {em:.2f} | {ei:.2f} | {q(wi):.2f} | {q(0):.2f} | {q(30):.2f} |")
t.append("")
t.append("### Sensitivity to the inversion weight (base only, native work at 0; A and B over negation rho on E1)\n")
t.append("| p | A w=0 | A w=10 | A w=30 | B w=0 | B w=10 | B w=30 |")
t.append("|---|---|---|---|---|---|---|")
for r in rows:
    fe = r["rho_E"]["field"]["negation"]
    cells = []
    for x in ("A", "B"):
        e = r["arms"][x]
        for w in (0, 10, 30):
            num = e["field_mul_sqr"] + w * e["field_inv"] + e["rel_mul"] + e["la_mul"]
            cells.append(f"{num / (fe['field_mul_sqr'] + w * fe['field_inv']):.1f}")
    t.append(f"| {r['p']} | " + " | ".join(cells) + " |")
t.append("")
t.append("### Verdicts\n")
t.append(f"- primary (field unit): {d['verdict']['verdict']}")
t.append(f"- secondary (group operations): {d['verdict_group_ops']['verdict']}")
for k in ("A_field_ratio_to_rho_E_w0.1", "B_field_ratio_to_rho_E_w0.1", "A_ratio_to_rho_E_w0.1", "B_ratio_to_rho_E_w0.1"):
    v = d["fits_stage1"].get(k)
    if v:
        t.append(f"- slope {k}: {v.get('slope')} (95% CI {v.get('ci95')})")
t.append("")
open(f"{out_dir}/tables.md", "w").write("\n".join(t) + "\n")

# ---------- svg ----------
W, H = 880, 560
L, R, T, B = 80, 200, 50, 60
series = [
    ("rho E (negation)", "#1b7837", lambda r: r["rho_E"]["band_negation"][1]),
    ("rho J(r) (negation, group ops)", "#7fbf7b", lambda r: r["arms"]["rho_J_negation"]["total_band"][1]),
    ("A whole-J, full base", "#b2182b", lambda r: r["arms"]["A"]["total_band"][1]),
    ("B whole-J, double LP", "#ef8a62", lambda r: r["arms"]["B"]["total_band"][1]),
    ("A_S subgroup control", "#2166ac", lambda r: r["arms"]["A_S"]["total_band"][1]),
    ("B_S subgroup control", "#67a9cf", lambda r: r["arms"]["B_S"]["total_band"][1]),
]
xs = [r["p"] for r in rows]
allv = [s[2](r) for s in series for r in rows]
lx0, lx1 = math.log10(min(xs)) - 0.1, math.log10(max(xs)) + 0.1
ly0, ly1 = math.floor(math.log10(min(allv))), math.ceil(math.log10(max(allv)))


def X(p):
    return L + (math.log10(p) - lx0) / (lx1 - lx0) * (W - L - R)


def Y(v):
    return H - B - (math.log10(v) - ly0) / (ly1 - ly0) * (H - T - B)


s = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}" font-family="sans-serif" font-size="12">']
s.append(f'<rect width="{W}" height="{H}" fill="#ffffff"/>')
s.append(f'<text x="{L}" y="24" font-size="15" font-weight="bold">O3 rerun (r r &gt;= p/8gt;= 0.4 p): whole-J index calculus against the matched negation rho on E</text>')
s.append(f'<text x="{L}" y="40" fill="#555">totals in GAE, native work priced at 0.1 GAE per operation; means over instances and seeds; [pcc:unverified]</text>')
for e in range(ly0, ly1 + 1):
    y = Y(10.0**e)
    s.append(f'<line x1="{L}" x2="{W-R}" y1="{y:.1f}" y2="{y:.1f}" stroke="#ddd"/>')
    s.append(f'<text x="{L-8}" y="{y+4:.1f}" text-anchor="end">1e{e}</text>')
for p in xs:
    x = X(p)
    s.append(f'<line x1="{x:.1f}" x2="{x:.1f}" y1="{T}" y2="{H-B}" stroke="#eee"/>')
    s.append(f'<text x="{x:.1f}" y="{H-B+16}" text-anchor="middle">{p}</text>')
s.append(f'<text x="{(L+W-R)/2}" y="{H-14}" text-anchor="middle">prime p (log scale)</text>')
s.append(f'<text transform="translate(18,{(T+H-B)/2}) rotate(-90)" text-anchor="middle">total cost, GAE (log scale)</text>')
# reference slopes anchored at the first rho point
v0 = series[0][2](rows[0])
for sl, lab in ((0.5, "p^0.5"), (1.0, "p^1")):
    pts = " ".join(f"{X(p):.1f},{Y(v0*(p/xs[0])**sl):.1f}" for p in xs)
    s.append(f'<polyline points="{pts}" fill="none" stroke="#999" stroke-dasharray="4 4"/>')
    s.append(f'<text x="{X(xs[-1])+4:.1f}" y="{Y(v0*(xs[-1]/xs[0])**sl):.1f}" fill="#777">{lab}</text>')
for i, (name, col, fn) in enumerate(series):
    pts = " ".join(f"{X(r['p']):.1f},{Y(fn(r)):.1f}" for r in rows)
    s.append(f'<polyline points="{pts}" fill="none" stroke="{col}" stroke-width="2"/>')
    for r in rows:
        s.append(f'<circle cx="{X(r["p"]):.1f}" cy="{Y(fn(r)):.1f}" r="3" fill="{col}"/>')
    ly = T + 12 + 18 * i
    s.append(f'<line x1="{W-R+30}" x2="{W-R+52}" y1="{ly}" y2="{ly}" stroke="{col}" stroke-width="2"/>')
    s.append(f'<text x="{W-R+57}" y="{ly+4}" font-size="11">{name}</text>')
f = d["fits_stage1"]
y0 = T + 12 + 18 * len(series) + 18
s.append(f'<text x="{W-R+30}" y="{y0}" font-size="11" font-weight="bold">slopes of ln(total) on ln p</text>')
for j, (lab, key) in enumerate((("rho E", "rho_E_negation_total_w0.1"), ("A", "A_total_w0.1"), ("B", "B_total_w0.1"),
                                ("A/rho E", "A_ratio_to_rho_E_w0.1"), ("B/rho E", "B_ratio_to_rho_E_w0.1"))):
    v = f.get(key)
    if v:
        s.append(f'<text x="{W-R+30}" y="{y0+16*(j+1)}" font-size="11">{lab}: {v["slope"]:.2f} [{v["ci95"][0]:.2f}, {v["ci95"][1]:.2f}]</text>')
s.append("</svg>")
open(f"{out_dir}/full_jacobian.svg", "w").write("\n".join(s))
print("wrote tables.md and full_jacobian.svg")
