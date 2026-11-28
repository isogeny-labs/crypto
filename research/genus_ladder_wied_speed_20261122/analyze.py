#!/usr/bin/env python3
"""Reads raw/{before,after}_<p>.jsonl; writes analysis.json, tables.md, fig_wied.svg. [pcc:unverified]
Medians; kappa = ratio of medians; A/A = even vs odd trials (split-half)."""
import json, statistics as st, os, math
PS = [65537, 131101, 262147]
SURF, INK, INK2, GRID = "#fcfcfb", "#0b0b0b", "#52514e", "#e4e3df"
BLUE, ORG, GRN, YEL = "#2a78d6", "#eb6834", "#1baf7a", "#eda100"
here = os.path.dirname(os.path.abspath(__file__))
HP = os.path.join(here, "..", "genus_ladder_g3_highp_20261121")
def load(s, p):
    meta, tr = None, []
    for l in open(f"{here}/raw/{s}_{p}.jsonl"):
        d = json.loads(l)
        if d["kind"] == "meta": meta = d
        else: tr.append(d)
    return meta, tr
med = st.median
def aa(x): return abs(med(x[0::2]) - med(x[1::2])) / med(x)
COUNTS = ["trials_ic","la_mul","la_add","la_wied_mul","la_graph_mul","la_peel_mul","la_sge_mul","cantor_mul","cantor_inv",
          "rel_mul","core_rows","core_cols","nnz_after_sge","r"]
A = {}
for p in PS:
    mb, tb = load("before", p); ma, ta = load("after", p)
    c = {"counts_identical": all(mb[k] == ma[k] for k in COUNTS), "counts": {k: mb[k] for k in COUNTS}, "n": [len(tb), len(ta)]}
    for s, tr in (("before", tb), ("after", ta)):
        r = {}
        for k in ("tR_situ_ns", "tF_situ_ns", "t_la_ns", "tLA_per_add_ns"):
            v = [t[k] for t in tr]
            r[k] = {"median": med(v), "min": min(v), "max": max(v), "aa": aa(v)}
        r["load_max"] = max(max(t["load_before"], t["load_after"]) for t in tr)
        r["load_min"] = min(min(t["load_before"], t["load_after"]) for t in tr)
        r["load_ge_1.5"] = sum(1 for t in tr if max(t["load_before"], t["load_after"]) >= 1.5)
        ks = [t["tR_situ_ns"] / t["tF_situ_ns"] for t in tr]
        r["kappa"] = r["tR_situ_ns"]["median"] / r["tF_situ_ns"]["median"]
        r["kappa_range"] = [min(ks), max(ks)]; r["kappa_aa"] = aa(ks)
        c[s] = r
    c["speedup_tR"] = c["before"]["tR_situ_ns"]["median"] / c["after"]["tR_situ_ns"]["median"]
    c["speedup_range"] = [c["before"]["tR_situ_ns"]["min"] / c["after"]["tR_situ_ns"]["max"],
                          c["before"]["tR_situ_ns"]["max"] / c["after"]["tR_situ_ns"]["min"]]
    A[p] = c
# W3 re-pricing (same as the kappa study): ratio of means over curves of per-curve mean FPM, SGE(8,3) arm over negation rho
SGE, RHO = "C_double_lp+pre+sge(8,3)", "R_rho_neg"
def ratio(p, kap, arm=SGE):
    rs = [json.loads(l) for l in open(f"{HP}/run_{p}.jsonl")]
    rs = [d for d in rs if d["kind"] == "run"]
    f = lambda d: d["cantor_mul"] + 10*d["cantor_inv"] + d.get("rel_mul", 0) + kap*d.get("la_mul", 0)
    def m(a):
        c = {}
        for d in rs:
            if d["arm"] == a: c.setdefault(d["curve_seed"], []).append(f(d))
        return sum(sum(v)/len(v) for v in c.values())/len(c)
    return m(arm)/m(RHO)
for p in PS:
    A[p]["reprice"] = {"kappa9": ratio(p, 9), "old_kappa_in_situ": ratio(p, A[p]["before"]["kappa"]),
                       "new_kappa_in_situ": ratio(p, A[p]["after"]["kappa"]),
                       "kappa_old_study": {65537: 17.9, 131101: 17.0, 262147: 15.8}[p],
                       "ratio_at_old_study_kappa": ratio(p, {65537: 17.9, 131101: 17.0, 262147: 15.8}[p])}
json.dump(A, open(f"{here}/analysis.json", "w"), indent=1)
g = lambda x, d=4: f"{x:.{d}g}"
o = ["# Tables (faster Wiedemann) [pcc:unverified]\n",
     "One VM, single thread, wall clock, no PMU. Medians of 15 trials after 2 warm-ups; A/A = split-half (even vs odd trials).\n",
     "## Before / after per p\n",
     "| p | r bits | t_R in situ before ns/la_mul (min-max, A/A) | after (min-max, A/A) | speedup (median ratio; extreme range) | t_F in situ before / after ns | kappa in situ before | kappa after (trial range; A/A) | LA wall before / after s |",
     "|---|---|---|---|---|---|---|---|---|"]
for p, c in A.items():
    b, a = c["before"], c["after"]
    o.append(f"| {p} | {c['counts']['r'].bit_length()} | {g(b['tR_situ_ns']['median'])} ({g(b['tR_situ_ns']['min'])}-{g(b['tR_situ_ns']['max'])}, {100*b['tR_situ_ns']['aa']:.1f}%) | "
             f"{g(a['tR_situ_ns']['median'])} ({g(a['tR_situ_ns']['min'])}-{g(a['tR_situ_ns']['max'])}, {100*a['tR_situ_ns']['aa']:.1f}%) | "
             f"{g(c['speedup_tR'],3)}x ({g(c['speedup_range'][0],3)}-{g(c['speedup_range'][1],3)}) | {g(b['tF_situ_ns']['median'])} / {g(a['tF_situ_ns']['median'])} | "
             f"{g(b['kappa'],3)} | {g(a['kappa'],3)} ({g(a['kappa_range'][0],3)}-{g(a['kappa_range'][1],3)}; {100*a['kappa_aa']:.1f}%) | "
             f"{g(b['t_la_ns']['median']/1e9,3)} / {g(a['t_la_ns']['median']/1e9,3)} |")
o.append("\n## Load average (1 min), trials with load >= 1.5\n\n| p | before min-max | after min-max | trials >= 1.5 |\n|---|---|---|---|")
for p, c in A.items():
    o.append(f"| {p} | {c['before']['load_min']:.2f}-{c['before']['load_max']:.2f} | {c['after']['load_min']:.2f}-{c['after']['load_max']:.2f} | {c['before']['load_ge_1.5']} / {c['after']['load_ge_1.5']} |")
o.append("\n## Count identity on the timing instances (meta of before vs after)\n\n| p | all counted fields equal | la_mul | la_add | la_wied_mul | la_sge_mul | rel_mul | cantor_mul | trials |\n|---|---|---|---|---|---|---|---|---|")
for p, c in A.items():
    k = c["counts"]
    o.append(f"| {p} | {c['counts_identical']} | {k['la_mul']:,} | {k['la_add']:,} | {k['la_wied_mul']:,} | {k['la_sge_mul']} | {k['rel_mul']:,} | {k['cantor_mul']:,} | {k['trials_ic']:,} |")
o.append("\n## Re-priced frontier row: R2 SGE(8,3) arm over negation rho (FPM = cantor_mul + 10 cantor_inv + rel_mul + kappa la_mul)\n\n| p | kappa = 9 | old study kappa (value) | new in-situ kappa (value) |\n|---|---|---|---|")
for p, c in A.items():
    r = c["reprice"]
    o.append(f"| {p} | {r['kappa9']:.3f} | {r['kappa_old_study']} ({r['ratio_at_old_study_kappa']:.3f}) | {c['after']['kappa']:.2f} ({r['new_kappa_in_situ']:.3f}) |")
open(f"{here}/tables.md", "w").write("\n".join(o) + "\n")

# SVG: grouped bars of ns per la_mul, before/after
W, H, L, R, T, B = 760, 420, 70, 20, 56, 64
ymax = 140
y = lambda v: T + (H-T-B) * (1 - v/ymax)
s = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" font-family="system-ui,sans-serif" font-size="12">',
     f'<rect width="{W}" height="{H}" fill="{SURF}"/>',
     f'<text x="{L}" y="24" fill="{INK}" font-size="15" font-weight="600">Wiedemann wall time per counted la_mul, in situ (ns)</text>',
     f'<text x="{L}" y="42" fill="{INK2}">median of 15 trials, whiskers min-max; one VM, wall clock; [pcc:unverified]</text>']
for v in range(0, ymax+1, 20):
    s.append(f'<line x1="{L}" x2="{W-R}" y1="{y(v)}" y2="{y(v)}" stroke="{GRID}"/><text x="{L-8}" y="{y(v)+4}" text-anchor="end" fill="{INK2}">{v}</text>')
gw = (W-L-R)/3
for i, (p, c) in enumerate(A.items()):
    for j, (ser, col) in enumerate((("before", ORG), ("after", BLUE))):
        r = c[ser]["tR_situ_ns"]; x0 = L + i*gw + gw*0.2 + j*gw*0.3; bw = gw*0.26
        s.append(f'<rect x="{x0:.1f}" y="{y(r["median"]):.1f}" width="{bw:.1f}" height="{y(0)-y(r["median"]):.1f}" fill="{col}"/>')
        xm = x0 + bw/2
        s.append(f'<line x1="{xm:.1f}" x2="{xm:.1f}" y1="{y(r["min"]):.1f}" y2="{y(r["max"]):.1f}" stroke="{INK}"/>')
        s.append(f'<text x="{xm:.1f}" y="{y(r["max"])-6:.1f}" text-anchor="middle" fill="{INK}">{r["median"]:.0f}</text>')
    s.append(f'<text x="{L+i*gw+gw/2:.1f}" y="{H-B+20}" text-anchor="middle" fill="{INK}">p = {p}  ({c["speedup_tR"]:.2f}x)</text>')
s.append(f'<rect x="{L}" y="{H-24}" width="12" height="12" fill="{ORG}"/><text x="{L+18}" y="{H-14}" fill="{INK}">before (unmodified)</text>')
s.append(f'<rect x="{L+170}" y="{H-24}" width="12" height="12" fill="{BLUE}"/><text x="{L+188}" y="{H-14}" fill="{INK}">after (fast Wiedemann)</text>')
s.append("</svg>")
open(f"{here}/fig_wied.svg", "w").write("\n".join(s))
