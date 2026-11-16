import glob, json, math
R='/home/user/crypto/research/'
fs=sorted(glob.glob(R+'curve25519_exponent_study_20261108/cells/*_sym_*.json'))+sorted(glob.glob(R+'curve25519_exponent_study_ext_20261110/cells/*.json'))
rows=[]
for f in fs:
    j=json.load(open(f))
    if j['capped'] or not j.get('per_target'): continue
    pt=j['per_target']; sd=max(r['solving_degree_max'] for r in pt); mc=max(r['max_cols'] for r in pt)
    nv=j['n_vars']; T=j['mean_field_ops']
    rows.append((j['k'],j['m'],j['d_or_t'],nv,sd,mc,T, math.comb(nv+sd,nv)))
def r2(xs,ys):
    n=len(xs);mx=sum(xs)/n;my=sum(ys)/n
    sxy=sum((x-mx)*(y-my) for x,y in zip(xs,ys));sxx=sum((x-mx)**2 for x in xs);syy=sum((y-my)**2 for y in ys)
    b=sxy/sxx; return (sxy*sxy/(sxx*syy), b)
lT=[math.log(r[6]) for r in rows]
print('n cells',len(rows))
print('R2 lnT ~ ln SD      ', r2([math.log(r[4]) for r in rows],lT))
print('R2 lnT ~ ln maxcols ', r2([math.log(r[5]) for r in rows],lT))
print('R2 lnT ~ ln C(nv+SD,nv)', r2([math.log(r[7]) for r in rows],lT))
from collections import defaultdict
g=defaultdict(list)
for r in rows: g[(r[1],r[4])].append(r)
for k,v in sorted(g.items()):
    if len(v)>1:
        Ts=[x[6] for x in v]; print('m=%d SD=%d'%k, [(x[0],x[2],x[3],'%.3g'%x[6]) for x in v], 'spread %.3g'%(max(Ts)/min(Ts)))
