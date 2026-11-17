"""Repair item 3: non-root false-positive probe + independence/common-factor check at 2^6144. usage: fp_probe.py seed arm"""
import sys,random,json,itertools,hashlib; sys.path.insert(0,'.')
import build_h; from build_h import *; from lattice import *
import sympy as sp
L=6144; build_h.DISTINCT_SQ=True; p=int(pari.nextprime(2**L+777))
while p%4!=3: p=int(pari.nextprime(p+1))
xs=xstar_bits(L); seed=int(sys.argv[1]); arm=sys.argv[2]; rng=random.Random(seed)
while True:
    A,B,c=curve(p,rng); X0=int(round(2**(0.85*xs)))
    if sum(bool(on_curve(c*(1+y)*pow(1-y,-1,p)%p,A,B,p)) for y in range(-X0,X0+1) if y not in (0,1,-1))>=4 and len({abs(y) for y in range(-X0,X0+1) if y not in (0,1,-1) and on_curve(c*(1+y)*pow(1-y,-1,p)%p,A,B,p)})>=4: break
for a_,f in (('below',0.85),('above',1.15)):
    X=int(round(2**(f*xs))); Ys,xR=plant(p,A,B,c,X,rng,'Y'); coef,info=interpolate(xR,A,B,c,p,rng=rng)
    if a_==arm: break
    xRn=rng.randrange(p); interpolate(xRn,A,B,c,p,rng=rng)   # replay null draws
rows,sc=build(coef,p,bounds(X)); vecs,dt=fplll(rows)
nz=[v for v in vecs if any(v)][:3]; hs=[[x//s for x,s in zip(v,sc)] for v in nz]
norms=[round(sum(x*x for x in v).bit_length()/2,1) for v in nz]
S=sp.symbols('s1 s2 s3 t')
def ev(h,iv): return sum(hj*iv[0]**a*iv[1]**b*iv[2]**c*iv[3]**d for hj,(a,b,c,d) in zip(h,MONOS))
piv=tuple(inv_vals(Ys)); vals=[v for v in range(-X,X+1) if v not in (0,1,-1)]
pts={tuple(inv_vals(list(Y))) for Y in itertools.product(vals,repeat=4)}
others=[q for q in pts if q!=piv]
fp=[sum(ev(h,q)==0 for q in others) for h in hs]
M=sp.Matrix(hs); rank=M.rank()
polys=[sp.Poly(sum(hj*S[0]**a*S[1]**b*S[2]**c*S[3]**d for hj,(a,b,c,d) in zip(h,MONOS)),*S) for h in hs]
g3=sp.gcd(sp.gcd(polys[0],polys[1]),polys[2]); gp=[str(sp.gcd(polys[i],polys[j]).as_expr())[:80] for i,j in ((0,1),(0,2),(1,2))]
out={'L':L,'seed':seed,'arm':arm,'X':X,'Ys':Ys,'lll_s':round(dt,1),'log2norms':norms,'planted_vanish':[ev(h,piv)==0 for h in hs],
 'n_other_invariant_points':len(others),'false_vanish_counts':fp,'rank_Q_h123':rank,'gcd_all3_deg':g3.total_degree(),'gcd_pairs':gp,
 'p':str(p),'A':str(A),'B':str(B),'c':str(c),'xR':str(xR),'h_sha256':[hashlib.sha256(json.dumps(h).encode()).hexdigest() for h in hs]}
print(json.dumps(out)); json.dump(out,open(f'raw/fp_probe_L6144_s{seed}_{arm}.json','w'))
json.dump(hs,open(f'raw/fp_probe_L6144_s{seed}_{arm}_vecs.json','w'))
