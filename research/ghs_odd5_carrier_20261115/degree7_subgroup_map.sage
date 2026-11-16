#!/usr/bin/env sage
"""Verify degree-seven cover pullback and nonzero F3 Galois trace."""
import hashlib
import json
import sys
from pathlib import Path

PTH=Path(__file__).resolve().parent
assert len(sys.argv)==2
bmask=int(sys.argv[1]);assert bmask in (3,27,30,103,139,167,182,194,230,233)
receipt=json.loads((PTH/('degree7_quadratic_square_%d.json'%bmask)).read_text())
assert receipt['bmask']==bmask and len(receipt['hits'])>0
F3=GF(3);Z=PolynomialRing(F3,'z');z=Z.gen()
K=GF(3^5,name='a',modulus=z^5+2*z+1);a=K.gen()
R=PolynomialRing(K,'x');x=R.gen()
f=x^11+x^9+2*x^8+x^7+2*x^5+x^3+x^2+x+1
b=sum(K((bmask//3^i)%3)*a^i for i in range(5))
E=EllipticCurve(K,[0,2,0,0,b]);assert E.cardinality()==218
Y=HyperellipticCurve(f);J=Y.jacobian();O=J(0)
Q=[p for p,n in f.factor() if p.degree()==2 for _ in range(n)]
assert len(Q)==5

def vec(t):return [int(c) for c in t.polynomial().list()]
def polyvec(g):return [vec(c) for c in g.list()]
def pointvec(P):return None if P.is_zero() else [vec(P[0]),vec(P[1])]
def divvec(D):return [polyvec(D[0]),polyvec(D[1])]
def frob(D,i):
 return J(R([c^(3^i) for c in D[0].list()]),R([c^(3^i) for c in D[1].list()]))

def run_hit(hit,hit_index):
 assert hit['shape']=='QiHquadratic' and len(hit['poles'])==1
 pole=hit['poles'][0]
 A=R(sage_eval(hit['A'],locals={'a':a,'x':x},preparse=True))
 B=R(sage_eval(hit['B'],locals={'a':a,'x':x},preparse=True))
 C=R(sage_eval(hit['C'],locals={'a':a,'x':x},preparse=True))
 H=R(sage_eval(hit['H'],locals={'a':a,'x':x},preparse=True))
 assert B==Q[pole]*H^2 and A.degree()==7 and B.degree()==6
 assert gcd(A,B)==1 and f*C^2==B*(A^3+2*A^2*B+b*B^3)
 D0=J(Q[pole],R(0));assert D0!=O and 2*D0==O
 def pullback(P):
  if P.is_zero():return O
  U=(A-P[0]*B).monic()
  assert U.degree()==7 and U.is_squarefree() and gcd(U,C)==1
  V=(P[1]*B^2*C.inverse_mod(U))%U
  assert (V^2-f)%U==0
  return J(U,V)-D0
 def trace(D):return sum((frob(D,i) for i in range(5)),O)
 set_random_seed(20261116+hit_index+bmask)
 P=E(0)
 for _ in range(30):
  candidate=2*E.random_point()
  if not candidate.is_zero():P=candidate;break
 assert P!=E(0) and 109*P==E(0)
 k=37;P2=k*P
 D=pullback(P);D2=pullback(P2)
 T=trace(D);T2=trace(D2)
 assert D!=O and 109*D==O and D2==k*D
 assert T!=O and 109*T==O and frob(T,1)==T and T2==k*T
 return {'hit_index':hit_index,'bmask':bmask,'degree':7,'pole_quadratic':pole,
         'A':polyvec(A),'B':polyvec(B),'C':polyvec(C),'H':polyvec(H),
         'source_P':pointvec(P),'source_37P':pointvec(P2),
         'pullback_P':divvec(D),'pullback_37P':divvec(D2),
         'trace_P':divvec(T),'trace_37P':divvec(T2),
         'checks':{'identity':True,'degree_seven':True,'pole_fiber_twotorsion':True,'source_order_109':True,'pullback_order_109':True,'trace_nonzero':True,'trace_order_109':True,'trace_F3_fixed':True,'pullback_37P_eq_37_pullback_P':True,'trace_37P_eq_37_trace_P':True}}

out={'schema':'ghs.odd5.degree7-subgroup-map/v1','source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),'bmask':bmask,'maps':[run_hit(h,i) for i,h in enumerate(receipt['hits'])]}
(PTH/('degree7_subgroup_map_%d.json'%bmask)).write_text(json.dumps(out,indent=2,sort_keys=True,default=int)+'\n')
print(json.dumps({'schema':out['schema'],'bmask':bmask,'map_count':len(out['maps']),'all_checks':all(all(m['checks'].values()) for m in out['maps'])},sort_keys=True))
