#!/usr/bin/env python3
"""S2: PROJECTED whole-run ablation table from (i) R2W lab A/A records (per-target whole Ir and number of solver calls) and
(ii) per-system lab cost under each ablation from the paired probe (same 150 systems, outcomes identical to the baseline).
A_t = whole_t + calls_t * (mean_abl - mean_base)   [per-system means over the 150 probe systems]; median over the 8 targets.
This is a projection, not a counted ecbench session; ecbench sessions replace it where they exist.
usage: an_proj.py <r2w_research_dir> <abl_probe_dir> [out.json]"""
import json, glob, sys, statistics as st
R2W, Q = sys.argv[1], sys.argv[2]
def ir(p):
    for l in open(p):
        if "instructions retired" in l: return int(l.split()[0])
def lab(sess):
    s = f'{R2W}/runs/R2W_{sess}'
    recs = [json.loads(x) for x in open(s + '/records.jsonl')]; tms = sorted(glob.glob(s + '_tm/*.time'))
    return [(ir(tms[i]) - 3.51e7, r['solver']['calls']) for i, r in enumerate(recs) if 'inherited' in r['method']['params'].get('oracle', '')]
CELL = {'k1n11': ('W_k1n11', 5, 3.85e9, 1.36e9), 'k0n13': ('W_k0n13', 6, 5.77e9, 1.23e9), 'k0n15': ('W_k0n15', 7, 1.34e10, 6.61e9)}
ABL = ['memo0', 'nolin', 'noinh', 'splitlow', 'chainlayout', 'occmult', 'alloff']
def mean(f): return sum(json.loads(l)['lab_ir'] for l in open(f)) / sum(1 for _ in open(f))
out = {}
print('cell  abl  A_whole(proj)  A/L0  frac of way to WDSat(a) whole  | frac to solver-only ref')
for c, (sess, l, Wa, Ws) in CELL.items():
    L = lab(sess); L0 = st.median(w for w, _ in L); base = mean(f'{Q}/{c}.l{l}.base.jsonl')
    print(f'{c}: L0={L0:.3e} W_a(whole best)={Wa:.3e} ratio {Wa/L0:.2f}; solver-only ref {Ws:.3e} ratio {Ws/L0:.2f}')
    for a in ABL:
        d = mean(f'{Q}/{c}.l{l}.{a}.jsonl') - base
        A = st.median(w + k * d for w, k in L)
        out[f'{c}.{a}'] = dict(L0=L0, A=A, Wa=Wa, Ws=Ws)
        print(f'  {a:11s} A={A:.3e} A/L0={A/L0:7.2f} frac_whole={(A-L0)/(Wa-L0):7.2f} frac_solveronly={(A-L0)/(Ws-L0):7.2f}')
if len(sys.argv) > 3: json.dump(out, open(sys.argv[3], 'w'), indent=1)
