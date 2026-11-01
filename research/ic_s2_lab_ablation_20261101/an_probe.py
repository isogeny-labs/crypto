#!/usr/bin/env python3
"""S2: analyse s2_split_probe output.  usage: an_probe.py <probes_dir> [abl_dir]
Part 1 (probes_dir/<cell>.l<l>.{laba,b}.jsonl): per-system cost of lab, WDSat (a), WDSat (b), emitter; SAT/UNSAT split (SAT = WDSat (a) returns a model).
Part 2 (abl_dir/<cell>.l<l>.<abl>.jsonl): lab-only per-system cost under each ablation (switches set by env in the probe process)."""
import json, sys, statistics as st, os
def load(f):
    out = []
    if not os.path.exists(f): return out
    for l in open(f):
        try: out.append(json.loads(l))
        except Exception: pass
    return out
def m(v): return sum(v) / len(v) if v else float('nan')
cells = [('k1n11', [5, 8]), ('k0n13', [6, 8]), ('k0n15', [7, 10])]
P = sys.argv[1]
for c, ls in cells:
    for l in ls:
        A = load(f'{P}/{c}.l{l}.laba.jsonl'); B = {r['idx']: r for r in load(f'{P}/{c}.l{l}.b.jsonl')}
        rows = []
        for r in A:
            b = B.get(r['idx'])
            rows.append(dict(sat=r.get('wda_sat'), lab=r.get('lab_ir'), labf=r.get('lab_found'), exh=r.get('lab_exhausted'), a=r.get('wda_ir'),
                             b=b.get('wdb_ir') if b else None, bsat=b.get('wdb_sat') if b else None, emit=r.get('emit_ir'), bad=r.get('wda_bad', 0) + (b.get('wdb_bad', 0) if b else 0)))
        print(f'== {c} l={l}: systems {len(rows)} (b: {len(B)}); UNSAT {sum(1 for x in rows if x["sat"] is False)}')
        for cls, name in [(True, 'SAT'), (False, 'UNSAT'), (None, 'ALL')]:
            S = [x for x in rows if cls is None or x['sat'] == cls]
            if not S: continue
            g = lambda k: [x[k] for x in S if x[k] is not None]
            tot = lambda k: sum(x[k] for x in rows if x[k] is not None)
            s = f' {name:5s} n={len(S):3d} mean Ir: lab {m(g("lab")):.2e}  wd(a) {m(g("a")):.2e}  wd(b) {m(g("b")):.2e} (n={len(g("b"))})  emitter {m(g("emit")):.2e}'
            if cls is not None:
                s += f' | UNSAT/SAT share of total: lab {sum(g("lab"))/tot("lab"):.2f}, a {sum(g("a"))/tot("a"):.2f}'
                if g("b"): s += f', b {sum(g("b"))/sum(x["b"] for x in rows if x["b"] is not None):.2f}'
            print(s)
        print(f' lab found != WDSat(a) SAT: {sum(1 for x in rows if x["labf"] != x["sat"])}; lab exhausted {sum(1 for x in rows if x["exh"])}; b SAT != a SAT: {sum(1 for x in rows if x["bsat"] is not None and x["bsat"] != x["sat"])}; invalid models {sum(x["bad"] for x in rows)}')
if len(sys.argv) > 2:
    Q = sys.argv[2]
    for c, l in [('k1n11', 5), ('k0n13', 6), ('k0n15', 7)]:
        base = load(f'{Q}/{c}.l{l}.base.jsonl')
        bm = m([r['lab_ir'] for r in base])
        print(f'== ablations {c} l={l}: base mean per-system Ir {bm:.3e} (n={len(base)})')
        bf = {r['idx']: r['lab_found'] for r in base}
        for x in ['base', 'memo0', 'nolin', 'noinh', 'splitlow', 'chainlayout', 'occmult', 'alloff']:
            R = load(f'{Q}/{c}.l{l}.{x}.jsonl')
            if not R: print(f'  {x}: no data'); continue
            mean = m([r['lab_ir'] for r in R]); med = st.median(r['lab_ir'] for r in R)
            sat = [r['lab_ir'] for r in R if r['lab_found']]; uns = [r['lab_ir'] for r in R if not r['lab_found']]
            print(f'  {x:11s} n={len(R):3d} mean {mean:.3e} (x{mean/bm:.2f})  median {med:.2e}  found-mean {m(sat):.2e} none-mean {m(uns):.2e}  exhausted {sum(1 for r in R if r["lab_exhausted"])}  outcome!=base {sum(1 for r in R if bf.get(r["idx"]) != r["lab_found"])}')
