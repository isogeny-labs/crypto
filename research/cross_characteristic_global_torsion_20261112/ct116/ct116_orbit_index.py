#!/usr/bin/env python3
"""CT116: precomputed Frobenius-orbit index as a toy cross-prime Kummer map.
The precomputed source lookup is charged as source projective DLP, not a new recognizer.
"""
import json, math

P_MOD=59
GEN=(19,22)
ORDER=11
POLY=0x130027
MASK=(1<<20)-1

def curve(P):
    if P is None: return True
    x,y=P
    return (y*y+y-x*x*x+x*x+10*x+20)%P_MOD==0

def neg(P):
    return None if P is None else (P[0],(-P[1]-1)%P_MOD)

def add(P,Q):
    if P is None: return Q
    if Q is None: return P
    x,y=P; a,b=Q
    if x==a:
        if (y+b+1)%P_MOD==0: return None
        if y!=b: raise ValueError('non-etale doubling')
        slope=((3*x*x-2*x-10)*pow(2*y+1,-1,P_MOD))%P_MOD
    else:
        slope=((b-y)*pow(a-x,-1,P_MOD))%P_MOD
    xx=(slope*slope+1-x-a)%P_MOD
    yy=(-1-y+slope*(x-xx))%P_MOD
    return xx,yy

def mul(n,P):
    R=None
    while n:
        if n&1:R=add(R,P)
        P=add(P,P)
        n>>=1
    return R

def fmul(a,b):
    z=0
    for _ in range(20):
        if b&1:z^=a
        b>>=1;a<<=1
        if a&(1<<20):a^=POLY
    if b:raise ArithmeticError('overflow')
    return z&MASK

def frob4(a):
    for _ in range(4):a=fmul(a,a)
    return a

if not curve(GEN) or mul(ORDER,GEN) is not None:
    raise ArithmeticError('bad source generator')
for k in range(1,ORDER):
    if not curve(mul(k,GEN)) or mul(k,GEN) is None:
        raise ArithmeticError('bad subgroup')

# Full five-entry source projective-log dictionary. This is explicitly
# counted as COLD PRECOMPUTATION, not presented as scalar-independent lifting.
source_x={}; R=None; source_additions=0
for i in range(1,6):
    R=add(R,GEN)
    if i>1:source_additions+=1
    if R is None or R[0] in source_x:raise ArithmeticError('duplicate source')
    source_x[R[0]]=i
if len(source_x)!=5:raise ArithmeticError('bad source table')

# Frobenius^4 on the target x-coordinate is multiplication by 7 mod 11.
orbit=[2]
for j in range(1,5):orbit.append(frob4(orbit[-1]))
if len(set(orbit))!=5 or frob4(orbit[-1])!=orbit[0]:
    raise ArithmeticError('bad binary orbit')
index_to_scalar=[pow(7,j,11) for j in range(5)]
if index_to_scalar!=[1,7,5,2,3]:
    raise ArithmeticError('bad orbit scalars')
for m in index_to_scalar:
    if source_x[mul(m,GEN)[0]]!=min(m,11-m):
        raise ArithmeticError('bad orbit-source correspondence')

# All ten public points: Q is supplied by its coordinates. The loop's
# hidden_k is only an independent known-answer verification control.
public_cases=[]
for hidden_k in range(1,11):
    Q=mul(hidden_k,GEN)
    if Q is None or not curve(Q):raise ArithmeticError('bad input')
    small=source_x.get(Q[0])
    if small is None:raise ArithmeticError('source lookup miss')
    candidates=[j for j,m in enumerate(index_to_scalar) if min(m,11-m)==small]
    if len(candidates)!=1:raise ArithmeticError('ambiguous orbit')
    j=candidates[0]
    target_x=orbit[j]
    target_j=next((i for i,x in enumerate(orbit) if x==target_x),None)
    if target_j is None:raise ArithmeticError('target orbit log miss')
    a=index_to_scalar[target_j]
    recovered=a if mul(a,GEN)==Q else (-a)%11
    if mul(recovered,GEN)!=Q or recovered!=hidden_k:
        raise ArithmeticError('source sign recovery')
    public_cases.append({'source_x':Q[0],'source_y':Q[1],
                         'binary_x':target_x,'orbit_index':j,'recovered':recovered})

if len({x['source_x'] for x in public_cases})!=5:
    raise ArithmeticError('source projective classes')
if len({x['binary_x'] for x in public_cases})!=5:
    raise ArithmeticError('target projective classes')

def bsgs(Q):
    m=math.isqrt(ORDER)+1
    babies={};R=None; operations=0
    for j in range(m):
        babies[R]=j
        R=add(R,GEN);operations+=1
    step=neg(R)
    cur=Q
    for i in range(m+1):
        if cur in babies:
            k=(i*m+babies[cur])%ORDER
            if mul(k,GEN)!=Q:raise ArithmeticError('BSGS wrong')
            return k,operations
        cur=add(cur,step);operations+=1
    raise ArithmeticError('BSGS miss')

bsgs_cases=[bsgs(mul(k,GEN)) for k in range(1,11)]
if [k for k,_ in bsgs_cases]!=list(range(1,11)):
    raise ArithmeticError('BSGS coverage')
print(json.dumps({
    'case':'ct116-frobenius-orbit-index-barrier',
    'pcc':'unverified','subgroup_order':11,
    'source_x_precomputed_entries':len(source_x),
    'source_x_precompute_additions':source_additions,
    'binary_frobenius_orbit_entries':len(orbit),
    'binary_frobenius_orbit':orbit,
    'orbit_scalar_representatives':index_to_scalar,
    'all_public_targets_recovered':len(public_cases),
    'public_cases':public_cases,
    'source_bsgs_additions_min':min(v for _,v in bsgs_cases),
    'source_bsgs_additions_max':max(v for _,v in bsgs_cases),
    'source_bsgs_additions_sum':sum(v for _,v in bsgs_cases),
    'online_global_root_recognizer':False,
    'shortcut_inherent':False,
    'full_cold_cost_ratio_available':False
},sort_keys=True,separators=(',',':')))
