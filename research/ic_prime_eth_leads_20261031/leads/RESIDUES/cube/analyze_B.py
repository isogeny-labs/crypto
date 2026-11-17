#!/usr/local/bin/python3.13
import json,glob,math,random,statistics,collections,os
os.chdir(os.path.dirname(os.path.abspath(__file__)))
random.seed(11)
CAP=100000
SEEDS=[3,4,5,8,10,13,16,21,24,25]
direct={}
for s in SEEDS:
    for l in open(f'raw/sweep/b09-s{s}/runs.jsonl') if os.path.exists(f'raw/sweep/b09-s{s}/runs.jsonl') else []:
        d=json.loads(l); direct[(s,d['run'])]=d
cubes=collections.defaultdict(list)
seen=set()
for fn in sorted(glob.glob('raw/cube_results*.jsonl')):
  for l in open(fn):
    r=json.loads(l); k=(r['seed'],r['run'],r['cls'],r['cube'])
    if k in seen: continue
    seen.add(k); cubes[(r['seed'],r['run'],r['cls'])].append(r)
inst=[]  # per (seed,run,cls)
for (s,run,cls),cs in cubes.items():
    if len(cs)<8 or (s,run) not in direct: continue
    d=direct[(s,run)]
    tc=sum(c['conflicts'] for c in cs); tp=sum(c['propagations'] for c in cs)
    mc=max(c['conflicts'] for c in cs)
    cens=any(c['status']=='UNKNOWN' for c in cs); dcens=d['status']=='UNKNOWN'
    inst.append(dict(seed=s,run=run,cls=cls,kind=d['status'],tc=tc,tp=tp,mc=mc,dc=d['conflicts'],dp=d['propagations'],cens=cens,dcens=dcens,
        nsatcubes=sum(c['status']=='SAT' for c in cs)))
def bootmean(groups,B=4000):
    # groups: dict seed->list of values; stat = mean over curves of per-curve median
    seeds=list(groups); 
    def est(ix): return statistics.mean(statistics.median(groups[seeds[i]]) for i in ix)
    pt=est(range(len(seeds))); bs=sorted(est([random.randrange(len(seeds)) for _ in seeds]) for _ in range(B))
    return pt,bs[int(.025*B)],bs[int(.975*B)]
out=[]
out.append('## Cube classes at b=9 (m=3, s=3), 8 cubes per class, cap 100000 per solve. log2 ratios = log2(CnC total / direct); per-curve median, mean over curves, bootstrap over curves (4000).')
out.append('class | curves | instances | CnC-cens inst | direct-cens inst | median direct conf | median CnC total conf | log2 conf ratio [CI] | median direct props(M) | median CnC props(M) | log2 prop ratio [CI] | log2 maxcube/direct conf [CI] | verdict')
for cls in ('hi','lo','one','rnd'):
    I=[x for x in inst if x['cls']==cls]
    if not I: continue
    for sel,name in ((lambda x:True,cls),):
        pass
    g1=collections.defaultdict(list); g2=collections.defaultdict(list); g3=collections.defaultdict(list)
    for x in I:
        g1[x['seed']].append(math.log2(x['tc']/x['dc'])); g2[x['seed']].append(math.log2(x['tp']/x['dp'])); g3[x['seed']].append(math.log2(x['mc']/x['dc']))
    a=bootmean(g1); b=bootmean(g2); c=bootmean(g3)
    helps = a[2]<0 and b[2]<0
    out.append(f"{cls} | {len(g1)} | {len(I)} | {sum(x['cens'] for x in I)} | {sum(x['dcens'] for x in I)} | {statistics.median(x['dc'] for x in I):.0f} | {statistics.median(x['tc'] for x in I):.0f} | {a[0]:+.2f} [{a[1]:+.2f},{a[2]:+.2f}] | {statistics.median(x['dp'] for x in I)/1e6:.2f} | {statistics.median(x['tp'] for x in I)/1e6:.2f} | {b[0]:+.2f} [{b[1]:+.2f},{b[2]:+.2f}] | {c[0]:+.2f} [{c[1]:+.2f},{c[2]:+.2f}] | {'helps (total work)' if helps else 'no total-work gain'}")
out.append('')
out.append('by direct status (SAT/UNSAT), log2 conflict ratio per class (mean over curves of per-curve median; n=instances):')
for cls in ('hi','lo','one','rnd'):
    for k in ('SAT','UNSAT'):
        I=[x for x in inst if x['cls']==cls and x['kind']==k]
        if not I: continue
        g=collections.defaultdict(list)
        for x in I: g[x['seed']].append(math.log2(x['tc']/x['dc']))
        a=bootmean(g); out.append(f"  {cls} {k}: n={len(I)} curves={len(g)} {a[0]:+.2f} [{a[1]:+.2f},{a[2]:+.2f}]")
open('raw/B_table.txt','w').write('\n'.join(out)+'\n'); print('\n'.join(out))
