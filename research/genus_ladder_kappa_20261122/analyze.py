#!/usr/bin/env python3
"""Reads raw/kappa_<p>.jsonl; writes analysis.json, tables.md, fig_kappa.svg. [pcc:unverified]
Layout and medians only; kappa = ratio of medians. A/A = even trials vs odd trials."""
import json, statistics as st, sys, os
PS = [65537, 131101, 262147]
SURF, INK, INK2, GRID = "#fcfcfb", "#0b0b0b", "#52514e", "#e4e3df"
BLUE, ORG, GRN, YEL = "#2a78d6", "#eb6834", "#1baf7a", "#eda100"
here = os.path.dirname(os.path.abspath(__file__))

def load(p):
    meta, tr = None, []
    for l in open(f"{here}/raw/kappa_{p}.jsonl"):
        d = json.loads(l)
        if d["kind"] == "meta": meta = d
        else: tr.append(d)
    return meta, tr

def med(x): return st.median(x)
def aa(x):
    a, b = med(x[0::2]), med(x[1::2])
    return abs(a - b) / med(x)

KEYS = ["tF_situ_ns", "tF_bare_lat_ns", "tF_bare_thr_ns", "tR_situ_ns", "tR_bare_madd_ns",
        "tR_bare_lat_ns", "tR_bare_mul_ns", "tAdd_p_ns", "tAdd_r_ns", "tLA_per_add_ns", "t_la_ns"]
A = {}
for p in PS:
    if not os.path.exists(f"{here}/raw/kappa_{p}.jsonl"): continue
    meta, tr = load(p)
    if not tr: continue
    c = {"meta": meta, "n": len(tr)}
    for k in KEYS:
        v = [t[k] for t in tr]
        c[k] = {"median": med(v), "min": min(v), "max": max(v), "aa": aa(v)}
    c["load_max"] = max(max(t["load_before"], t["load_after"]) for t in tr)
    c["load_min"] = min(min(t["load_before"], t["load_after"]) for t in tr)
    m = lambda k: c[k]["median"]
    c["kappa_situ"] = m("tR_situ_ns") / m("tF_situ_ns")
    # kappa per trial for a spread of the ratio itself
    ks = [t["tR_situ_ns"] / t["tF_situ_ns"] for t in tr]
    c["kappa_situ_range"] = [min(ks), max(ks)]
    c["kappa_situ_aa"] = aa(ks)
    c["kappa_bare_matched"] = m("tR_bare_madd_ns") / m("tF_bare_thr_ns")
    c["kappa_bare_lat"] = m("tR_bare_lat_ns") / m("tF_bare_lat_ns")
    c["kappa_bare_mulonly_thr"] = m("tR_bare_mul_ns") / m("tF_bare_thr_ns")
    c["kappa_situ_over_bareF"] = m("tR_situ_ns") / m("tF_bare_thr_ns")
    c["kappa_add"] = m("tAdd_r_ns") / m("tAdd_p_ns")
    A[p] = c
json.dump(A, open(f"{here}/analysis.json", "w"), indent=1)

def f(x, d=3): return f"{x:.{d}g}"
o = ["# Tables (kappa at genus 3) [pcc:unverified]\n",
     "Single host, single thread, wall clock, no PMU. Medians of the trials; A/A = |median(even) - median(odd)| / median.\n"]
for p, c in A.items():
    M = c["meta"]
    o.append(f"\n## p = {p}  (r = {M['r']}, {M['r_bits']} bits, mulmod branch: {M['mulmod_branch']}; {c['n']} trials)\n")
    o.append(f"IC run: trials {M['trials_ic']}, la_mul {M['la_mul']:,} (wiedemann {M['la_wied_mul']:,}), la_add {M['la_add']:,}, "
             f"core {M['core_rows']}x{M['core_cols']}, nnz after SGE {M['nnz_after_sge']:,}; "
             f"cantor_mul {M['cantor_mul']:,}, cantor_inv {M['cantor_inv']:,}. Load average (1 min) over trials: {c['load_min']:.2f} to {c['load_max']:.2f}.\n")
    o.append("| quantity | median ns | min | max | A/A spread |\n|---|---|---|---|---|")
    for k, lab in [("tF_situ_ns", "t_F in situ (per fmul+10 finv, Cantor)"), ("tF_bare_thr_ns", "t_F bare, throughput"),
                   ("tF_bare_lat_ns", "t_F bare, dependent chain"), ("tR_situ_ns", "t_R in situ (LA wall / la_mul)"),
                   ("tR_bare_madd_ns", "t_R bare mulmod+addm"), ("tR_bare_mul_ns", "t_R bare mulmod, throughput"),
                   ("tR_bare_lat_ns", "t_R bare mulmod, dependent chain"), ("tLA_per_add_ns", "LA wall / la_add (info)"),
                   ("tAdd_p_ns", "bare F_p add"), ("tAdd_r_ns", "bare mod-r add"), ("t_la_ns", "LA wall total (ns)")]:
        x = c[k]
        o.append(f"| {lab} | {f(x['median'],4)} | {f(x['min'],4)} | {f(x['max'],4)} | {100*x['aa']:.1f}% |")
    o.append("\n| kappa | value |\n|---|---|")
    o.append(f"| in situ: t_R / t_F | {f(c['kappa_situ'])} (per-trial {f(c['kappa_situ_range'][0])} to {f(c['kappa_situ_range'][1])}; A/A {100*c['kappa_situ_aa']:.1f}%) |")
    o.append(f"| bare matched: (mulmod+addm) / F_p mul throughput | {f(c['kappa_bare_matched'])} |")
    o.append(f"| bare mulmod throughput / F_p mul throughput | {f(c['kappa_bare_mulonly_thr'])} |")
    o.append(f"| bare dependent chains | {f(c['kappa_bare_lat'])} |")
    o.append(f"| in-situ t_R over bare F_p mul throughput | {f(c['kappa_situ_over_bareF'])} |")
    o.append(f"| add: mod-r / F_p | {f(c['kappa_add'])} |")
open(f"{here}/tables.md", "w").write("\n".join(o) + "\n")

# SVG: log-scale kappa, three series per p, thresholds 2, 2.5, 4.1
import math
W, H, L, R, T, B = 760, 440, 70, 20, 50, 70
lo, hi = 0.3, 60
def y(v): return T + (H - T - B) * (1 - (math.log(v) - math.log(lo)) / (math.log(hi) - math.log(lo)))
S = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" font-family="sans-serif" font-size="12">',
     f'<rect width="{W}" height="{H}" fill="{SURF}"/>',
     f'<text x="{L}" y="24" fill="{INK}" font-size="15" font-weight="bold">kappa = t(mod-r mul) / t(F_p mul), genus 3 [pcc:unverified]</text>',
     f'<text x="{L}" y="40" fill="{INK2}">wall clock, one host, no PMU; log scale; bars are medians</text>']
for v in [0.5, 1, 2, 5, 10, 20, 50]:
    S.append(f'<line x1="{L}" x2="{W-R}" y1="{y(v)}" y2="{y(v)}" stroke="{GRID}"/><text x="{L-8}" y="{y(v)+4}" text-anchor="end" fill="{INK2}">{v}</text>')
for v, lab in [(2.5, "2.5"), (4.1, "4.1")]:
    S.append(f'<line x1="{L}" x2="{W-R}" y1="{y(v)}" y2="{y(v)}" stroke="{INK}" stroke-dasharray="5 4"/><text x="{W-R-2}" y="{y(v)-4}" text-anchor="end" fill="{INK}">threshold {lab}</text>')
S.append(f'<line x1="{L}" x2="{W-R}" y1="{y(2)}" y2="{y(2)}" stroke="{INK2}" stroke-dasharray="2 3"/><text x="{L+4}" y="{y(2)-4}" fill="{INK2}">main PR 4852: ~2</text>')
ser = [("kappa_situ", BLUE, "in situ"), ("kappa_bare_matched", ORG, "bare (mulmod+addm / F_p mul)"), ("kappa_bare_lat", GRN, "bare dependent chains")]
gw = (W - L - R) / max(1, len(A))
for i, (p, c) in enumerate(A.items()):
    x0 = L + i * gw + gw * 0.12
    bw = gw * 0.22
    for j, (k, col, lab) in enumerate(ser):
        v = c[k]; x = x0 + j * (bw + 6)
        S.append(f'<rect x="{x}" y="{y(v)}" width="{bw}" height="{y(lo)-y(v)}" fill="{col}"/>')
        S.append(f'<text x="{x+bw/2}" y="{y(v)-4}" text-anchor="middle" fill="{INK}">{v:.2f}</text>')
    S.append(f'<text x="{L+i*gw+gw/2}" y="{H-B+18}" text-anchor="middle" fill="{INK}">p = {p}</text>')
for j, (k, col, lab) in enumerate(ser):
    S.append(f'<rect x="{L+j*230}" y="{H-24}" width="12" height="12" fill="{col}"/><text x="{L+j*230+18}" y="{H-14}" fill="{INK}">{lab}</text>')
S.append("</svg>")
open(f"{here}/fig_kappa.svg", "w").write("\n".join(S))
print(json.dumps({p: {k: c[k] for k in ["kappa_situ", "kappa_bare_matched", "kappa_bare_lat", "kappa_add"]} for p, c in A.items()}, indent=1))
