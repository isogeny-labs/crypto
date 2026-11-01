import os
"""Independent recomputation of weighted S ratios from records.jsonl (no decide.py, no ecbench).
priced_gae(w) = G - C + w*(U + n_C); S = priced_gae / sqrt(r)  (band.rs module doc, AGENTS.md 5a)."""
import json,math,sys,collections
CALIB=json.load(open(os.path.join(os.path.dirname(os.path.abspath(__file__)),'calibration-pinned-4df337e.json')))['instances']
PRICED_NATIVE=[("sqrt_solves","ns_per_sqrt"),("as_solves","ns_per_as_solve"),("s4_pairs","ns_per_s4_pair"),("lookups","ns_per_lookup"),("row_ops","ns_per_row_op"),("legendre_symbols","ns_per_legendre"),("inversions","ns_per_inversion"),("frobenius_maps","ns_per_frobenius"),("canonicalisations","ns_per_canon"),("target_guard_probes","ns_per_lookup")]
BITS={'fp10':'prime/bench-10bit','fp12':'prime/bench-12bit','fp14':'prime/bench-14bit','fp16':'prime/bench-16bit'}
def curve_key(r):
    s=r['workload']['curve']['slug']; return s.split('-')[1]  # icv1-fp10-...
def load(p): return [json.loads(l) for l in open(p)]
def counts(r): return (not r['warmup']) and r['outcome']['status']=='verified'
def unpriced(r):
    tot=0; missing=False
    for name in r['cost']['unpriced']:
        base=name[:-len('_uncharged')] if name.endswith('_uncharged') else name
        if name in r['counters']: tot+=r['counters'][name]; continue
        found=False; t=0
        for p in r['phases']:
            n=p['native']
            if name in n: t+=n[name]; found=True
            elif base in n: t+=n[base]; found=True
        if not found and base.startswith('solver_') and r.get('solver'): tot+=r['solver']['ops']; continue
        if found: tot+=t
        else: missing=True
    return tot,missing
def calibrated(r):
    pinned=r['detail'].get('calibration_pinned_units') or []
    if not pinned: return 0.0,0
    row=CALIB[BITS[curve_key(r)]]
    g=0.0;ops=0
    for p in r['phases']:
        for c,u in PRICED_NATIVE:
            n=p['native'].get(c,0)
            if n>0 and u in pinned:
                g+=n*row[u]; ops+=n
    s=r.get('solver')
    if s and s.get('ops',0)>0 and 'word XOR' in s.get('op_unit','') and 'ns_per_word_xor' in pinned and not any(x.startswith('solver_') for x in r['cost']['unpriced']):
        g+=s['ops']*row['ns_per_word_xor']; ops+=s['ops']
    return g,ops
def priced_s(r,w):
    G=r['cost']['total_gae']
    U,miss=unpriced(r)
    if miss and w>0: return None
    C,nC=calibrated(r)
    return (G-C+w*(U+nC))/math.sqrt(r['workload']['curve']['r'])
def pairs(recs,A,B,curve):
    a={};b={}
    for r in recs:
        if curve_key(r)!=curve or not counts(r): continue
        k=(r['workload']['workload_id'],r['round'])
        if r['arm']==A: a[k]=r
        if r['arm']==B: b[k]=r
    ks=sorted(set(a)&set(b)); return [(a[k],b[k]) for k in ks]
def ratio(recs,A,B,curve,w):
    ps=pairs(recs,A,B,curve)
    sa=[priced_s(x,w) for x,_ in ps]; sb=[priced_s(y,w) for _,y in ps]
    if any(v is None for v in sa+sb) or not ps: return None,len(ps)
    return sum(sb)/sum(sa),len(ps)
if __name__=='__main__':
    C1014=load('/dev/shm/restore/snapshot-1730/sessions/C-10-14/records.jsonl')
    C16=load('/dev/shm/restore/v2/sessions/C-16/records.jsonl')
    B=load('/dev/shm/restore/snapshot-1730/sessions/B/records.jsonl')
    W=[0.0,0.1,1.0]
    print('Track C: ratio sum S(B)/sum S(A)')
    for name,recs,curves in [('C-10-14',C1014,['fp10','fp12','fp14']),('C-16',C16,['fp16'])]:
        for c in curves:
            for (A,Bm) in [('rho-neg','tensor-m2'),('ic-mitm','tensor-m2'),('rho-neg','ic-mitm'),('rho-neg','rho-neg-aa')]:
                out=[ratio(recs,A,Bm,c,w) for w in W]
                print(f"{c} {Bm}/{A}: "+' / '.join(f"{o[0]:.4g}" if o[0] is not None else 'NA' for o in out)+f"  pairs={out[0][1]}")
    print('Track B (rho-neg reference)')
    for c in ['fp10','fp12','fp14','fp16']:
        for A,Bm in [('rho-neg','native-selectors'),('rho-neg','native-partial'),('rho-neg','native-source'),('rho-neg','native-width'),('rho-neg','ic-mitm'),('rho-neg','rho-neg-v2'),('native-source','native-selectors'),('rho-neg','rho-neg-aa'),('native-selectors','native-selectors-aa')]:
            out=[ratio(B,A,Bm,c,w) for w in W]
            print(f"{c} {Bm}/{A}: "+' / '.join(f"{o[0]:.4g}" if o[0] is not None else 'NA' for o in out)+f"  pairs={out[0][1]}")
