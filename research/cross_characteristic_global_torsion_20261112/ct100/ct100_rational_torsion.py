#!/usr/bin/env python3
"""CT100 independent rational five-torsion common-lift positive control.
Full toy group map; no cross-characteristic curve morphism or speedup.
"""
import json
from fractions import Fraction
from math import isqrt

P=(Fraction(5),Fraction(-6))

def check(ok, msg):
    if not ok: raise ArithmeticError(msg)

def add(P,Q):
    if P is None: return Q
    if Q is None: return P
    x,y=P;u,v=Q
    if x==u and y+v+1==0:return None
    if P==Q: lam=(3*x*x-2*x-10)/(2*y+1)
    else: lam=(v-y)/(u-x)
    xx=lam*lam+1-x-u
    return (xx,-lam*(xx-x)-y-1)

def modpt(Q,p):
    if Q is None:return None
    return tuple(int(t.numerator%p)*pow(int(t.denominator%p),-1,p)%p for t in Q)

def ffadd(P,Q,p):
    if P is None:return Q
    if Q is None:return P
    x,y=P;u,v=Q
    if x==u and (y+v+1)%p==0:return None
    if P==Q:
        num=3*x*x-2*x-10
        den=2*y+1
    else:
        num=v-y
        den=u-x
    lam=num*pow(den%p,-1,p)%p
    xx=(lam*lam+1-x-u)%p
    return xx,(-lam*(xx-x)-y-1)%p

def bsgs(G,Q,p):
    m=isqrt(5)+1 # ceil(sqrt(5))
    table={None:0};b=None
    for j in range(1,m):
        b=ffadd(b,G,p);table[b]=j
    v=None
    for _ in range(m):v=ffadd(v,G,p)
    stride=None if v is None else (v[0],(-v[1]-1)%p)
    R=Q
    for i in range(m+1):
        if R in table:return (i*m+table[R])%5
        R=ffadd(R,stride,p)
    raise ArithmeticError('BSGS failure')

def on_curve(Q,p):
    if Q is None:return True
    x,y=Q
    return (y*y+y-(x*x*x-x*x-10*x-20))%p==0

def count_curve(p):
    return 1+sum((y*y+y-(x*x*x-x*x-10*x-20))%p==0
                 for x in range(p) for y in range(p))

check(count_curve(7)==10 and count_curve(2)==5,'wrong source/target full curve orders')
pts=[None]; v=None
for i in range(1,5):
    v=add(v,P)
    check(v is not None,'premature rational zero')
    pts.append(v)
check(add(v,P) is None,'rational torsion must have order 5')
check(pts==[None,(Fraction(5),Fraction(-6)),(Fraction(16),Fraction(60)),
           (Fraction(16),Fraction(-61)),(Fraction(5),Fraction(5))],
      'wrong global rational torsion coordinates')
source=[modpt(t,7) for t in pts];target=[modpt(t,2) for t in pts]
check(source==[None,(5,1),(2,4),(2,2),(5,5)],'bad source reduction')
check(target==[None,(1,0),(0,0),(0,1),(1,1)],'bad target reduction')
check(len(set(source))==len(set(target))==5,'noninjective reduction')
check(all(on_curve(Q,7) for Q in source) and all(on_curve(Q,2) for Q in target),
      'point not on source or target curve')
for i in range(5):
    check((i==0 or bsgs(source[1],source[i],7)==i),'source BSGS mismatch')
    check((i==0 or bsgs(target[1],target[i],2)==i),'target BSGS mismatch')
    for j in range(5):
        k=(i+j)%5
        check(add(pts[i],pts[j])==pts[k],'rational group law mismatch')
        check(ffadd(source[i],source[j],7)==source[k],'source group law mismatch')
        check(ffadd(target[i],target[j],2)==target[k],'target group law mismatch')
# Input selection uses only the source encoded coordinates and a frozen rational point table.
# The representation contains the full order-five log table; it is not an attack.
for i,Q in enumerate(source):
    matched=[j for j,S in enumerate(source) if S==Q]
    check(len(matched)==1 and target[matched[0]]==target[i],'online lift mismatch')
print(json.dumps(dict(experiment='CT100-rational-torsion-positive-control',
  group_order=5,source_curve_order=10,target_curve_order=5,global_points=[None]+[[int(v) for v in q] for q in pts[1:]],
  source_points=[None]+[list(q) for q in source[1:]],
  target_points=[None]+[list(q) for q in target[1:]],
  all_25_pair_operations=True,source_bsgs_cases=4,target_bsgs_cases=4,
  scalar_free_input_lookup=True,precomputed_full_torsion_table=True,
  pcc='unverified'),sort_keys=True,separators=(',',':')))
