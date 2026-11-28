import json,math,sys
from collections import defaultdict
D=defaultdict(dict)
for l in open(sys.argv[1]):
    r=json.loads(l)
    if r.get('kind')=='run': D[(r['p'],r['curve_seed'],r['seed'])][r['arm']]=r
na=lambda r:r['rel_mul']+r['rel_add']+r['la_mul']+r['la_add']+r['idx_lookups']+r['idx_inserts']
g=lambda r,w: r['group_ops']+w*na(r)
B='Q_diem_2lp_sfb_sge_nofb'
arms=sorted({a for v in D.values() for a in v})
for a in arms:
  for p in sorted({k[0] for k in D}):
    ks=[k for k in D if k[0]==p and a in D[k] and B in D[k]]
    multi=sum(D[k][a]['attempts']>1 for k in ks); cert=sum(D[k][a]['certified'] and D[k][a]['recovered']==D[k][a]['t_secret'] for k in ks)
    rs={w:math.exp(sum(math.log(g(D[k][a],w)/g(D[k][B],w)) for k in ks)/len(ks)) for w in (0,0.1,1)}
    p0r=sum(D[k][a].get('p0_line_relations',0) for k in ks)/len(ks)
    dec=math.exp(sum(math.log((D[k][a]['rel_decomp_mul']+1)/(D[k][B]['rel_decomp_mul']+1)) for k in ks)/len(ks))
    la=math.exp(sum(math.log((D[k][a]['la_mul']+D[k][a]['la_add'])/(D[k][B]['la_mul']+D[k][B]['la_add'])) for k in ks)/len(ks))
    print(f"{a[17:]:14s} p={p} n={len(ks)} cert={cert} multi={multi} gae0={rs[0]:.3f} gae.1={rs[0.1]:.3f} gae1={rs[1]:.3f} decomp={dec:.3f} la={la:.3f} p0rels={p0r:.0f}")
