import json,glob,collections,random,math,sys
root=sys.argv[1] if len(sys.argv)>1 else 'res_b8'
CAP=int(sys.argv[2]) if len(sys.argv)>2 else 150000
rows=[]
for f in glob.glob(f'{root}/*/seed*/runs.jsonl'):
    var=f.split('/')[1]; seed=int(f.split('seed')[1].split('/')[0])
    for l in open(f):
        r=json.loads(l); r['var']=var; r['seed']=seed; rows.append(r)
# instance key
for r in rows: r['key']=(r['seed'],r['l'],r['target_kind'],r['target_idx'])
def split(r): return 'SAT' if r['oracle_sat'] else 'UNSAT'
bad=[r for r in rows if r['verdict'].startswith('FAIL')]
print('rows',len(rows),'FAIL verdicts',len(bad))
rng=random.Random(0)
def lg(c): return math.log2(max(c,1))
def boot_mean(v,B=2000):
    if not v: return (float('nan'),)*3
    m=sum(v)/len(v); bs=[]
    for _ in range(B):
        s=[v[rng.randrange(len(v))] for _ in range(len(v))]; bs.append(sum(s)/len(s))
    bs.sort(); return m,bs[int(.025*B)],bs[int(.975*B)]
variants=['ref','nosym','hi','lo','rand']
# table: per variant,l,split: n, censored, median conflicts, mean log2
print('\n== cells: var l split n cens median_conf meanlog2 [CI]')
for v in variants:
  for l in (2,3,4):
    for sp in ('SAT','UNSAT','ALL'):
      rs=[r for r in rows if r['var']==v and r['l']==l and (sp=='ALL' or split(r)==sp)]
      if not rs: continue
      cs=sorted(r['conflicts'] for r in rs); cens=sum(1 for r in rs if r['status']=='UNKNOWN')
      m,lo,hi=boot_mean([lg(r['conflicts']) for r in rs])
      print(f'{v:6s} l={l} {sp:5s} n={len(rs):3d} cens={cens:3d} med={cs[len(cs)//2]:7d} meanlog2={m:.2f} [{lo:.2f},{hi:.2f}]')
# slope e: per-instance bootstrap: regress mean log2 conflicts on l by variant; resample instances within l
def slope(rs, ls):
    pts=[(l,[lg(r['conflicts']) for r in rs if r['l']==l]) for l in ls]
    pts=[(l,v) for l,v in pts if v]
    if len(pts)<2: return None
    xs=[l for l,v in pts]; ys=[sum(v)/len(v) for l,v in pts]
    mx=sum(xs)/len(xs); my=sum(ys)/len(ys)
    return sum((x-mx)*(y-my) for x,y in zip(xs,ys))/sum((x-mx)**2 for x in xs)
def boot_slope(rs, ls, B=2000):
    by={l:[r for r in rs if r['l']==l] for l in ls}
    out=[]
    for _ in range(B):
        sm=[]
        for l in ls:
            if by[l]: sm+= [by[l][rng.randrange(len(by[l]))] for _ in range(len(by[l]))]
        s=slope(sm,ls)
        if s is not None: out.append(s)
    out.sort(); return out[int(.025*len(out))],out[int(.975*len(out))]
print('\n== slope e = d log2(conflicts)/d l (lower bound where censored); ls=(2,3,4) and (2,3)')
for sp in ('UNSAT','SAT','ALL'):
  for ls in ((2,3,4),(2,3),(3,4)):
    for v in variants:
      rs=[r for r in rows if r['var']==v and (sp=='ALL' or split(r)==sp)]
      s=slope(rs,ls)
      if s is None: continue
      lo,hi=boot_slope(rs,ls)
      print(f'{sp:5s} ls={ls} {v:6s} e={s:.2f} [{lo:.2f},{hi:.2f}]')
# paired comparison vs ref per l: log2 ratio of conflicts, bootstrap CI
print('\n== paired log2(conf_variant/conf_ref) per l and split (instances present in both)')
ref={r['key']:r for r in rows if r['var']=='ref'}
for v in variants[1:]:
  for l in (2,3,4):
    for sp in ('UNSAT','SAT'):
      d=[lg(r['conflicts'])-lg(ref[r['key']]['conflicts']) for r in rows if r['var']==v and r['l']==l and r['key'] in ref and split(r)==sp]
      if d:
        m,lo,hi=boot_mean(d); print(f'{v:6s} l={l} {sp:5s} n={len(d):3d} mean={m:+.2f} [{lo:+.2f},{hi:+.2f}]')
print('\nfactor-base sizes per (seed,l):', sorted({(r['seed'],r['l'],r['fb_xs'],r['t_enum']) for r in rows if r['var']=='ref'}))
