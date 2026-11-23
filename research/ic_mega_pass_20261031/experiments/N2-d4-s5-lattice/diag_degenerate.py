"""Seed-4 L=4096 below arm rebuilt: do short vectors vanish on non-planted points with a +-Y pair (degenerate locus)?"""
import sys,random,json; sys.path.insert(0,'.')
from build_h import *; from lattice import *
L=4096; p=int(pari.nextprime(2**L+777))
while p%4!=3: p=int(pari.nextprime(p+1))
xs=xstar_bits(L); seed=4; rng=random.Random(seed)
while True:
    A,B,c=curve(p,rng); X0=int(round(2**(0.85*xs)))
    if sum(bool(on_curve(c*(1+y)*pow(1-y,-1,p)%p,A,B,p)) for y in range(-X0,X0+1) if y not in (0,1,-1))>=4: break
X=int(round(2**(0.85*xs))); Ys,xR=plant(p,A,B,c,X,rng,'Y'); coef,info=interpolate(xR,A,B,c,p,rng=rng)
rows,sc=build(coef,p,bounds(X)); vecs,dt=fplll(rows)
nz=[v for v in vecs if any(v)]; hs=[[x//s for x,s in zip(v,sc)] for v in nz[:6]]
def ev(h,Y): iv=inv_vals(Y); return sum(hj*iv[0]**a*iv[1]**b*iv[2]**c*iv[3]**d for hj,(a,b,c,d) in zip(h,MONOS))
r=random.Random(5); out={'Ys':Ys,'planted':[ev(h,Ys)==0 for h in hs]}
pair=[]; gen=[]
for _ in range(20):
    a,b,d=[r.randint(2,40) for _ in range(3)]
    pair.append([ev(h,[a,-a,b,d])==0 for h in hs]); pair.append([ev(h,[a,a,b,d])==0 for h in hs])
    e,f,g,k=[r.randint(2,40) for _ in range(4)]; gen.append([ev(h,[e,f,g,k])==0 for h in hs])
out['frac_vanish_pairY']=[sum(x[i] for x in pair)/len(pair) for i in range(len(hs))]
out['frac_vanish_generic']=[sum(x[i] for x in gen)/len(gen) for i in range(len(hs))]
# Y-sets with two equal squares but not of the planted values
print(json.dumps(out)); json.dump(out,open('raw/diag_degenerate_s4.json','w'))
