"""Exhaustive F_3 linear-algebra audit of cubic Weil-restriction kernels.

No elliptic-curve or Jacobian construction is used in this independent audit.
Enumerate every 3-dimensional subspace of F_3^6, retain Lagrangians for the
product Weil pairing, then test Frobenius stability and coordinate transversality.
"""
from itertools import combinations,product
from pathlib import Path
from time import perf_counter
import json


def rref_subspaces(p,n,k):
    for pivots in combinations(range(n),k):
        free=[j for j in range(n) if j not in pivots]
        slots=[(i,j) for i in range(k) for j in free if j>pivots[i]]
        for values in product(range(p),repeat=len(slots)):
            rows=[[0]*n for _ in range(k)]
            for i,j in enumerate(pivots): rows[i][j]=1
            for (i,j),v in zip(slots,values): rows[i][j]=v
            yield tuple(tuple(r) for r in rows),pivots


def rank(rows,p):
    if not rows: return 0
    a=[list(r) for r in rows]; i=0
    for j in range(len(a[0])):
        pivot=next((t for t in range(i,len(a)) if a[t][j]%p),None)
        if pivot is None: continue
        a[i],a[pivot]=a[pivot],a[i]
        inv=pow(a[i][j],-1,p); a[i]=[v*inv%p for v in a[i]]
        for t in range(len(a)):
            if t!=i:
                c=a[t][j]%p
                a[t]=[(x-c*y)%p for x,y in zip(a[t],a[i])]
        i+=1
        if i==len(a): break
    return i


def pairing(v,w,q,p=3):
    return sum(pow(q,i,p)*(v[2*i]*w[2*i+1]-v[2*i+1]*w[2*i])
               for i in range(3))%p


def contains(rows,pivots,v,p=3):
    return all(sum(v[pivots[i]]*rows[i][j] for i in range(3))%p==v[j]%p
               for j in range(6))


def frobenius(v,A,p=3):
    return ((A[0][0]*v[4]+A[0][1]*v[5])%p,
            (A[1][0]*v[4]+A[1][1]*v[5])%p,
            v[0],v[1],v[2],v[3])


def screen(q,t,scalar=False):
    p=3; d=pow(q,3,p)
    if scalar:
        lam=next(a for a in range(1,p) if 2*a%p==t and a*a%p==d)
        A=((lam,0),(0,lam))
    else:
        A=((0,-d%p),(1,t))
    # Verify the modeled q-Frobenius is a q-symplectic similitude.
    standard=[tuple(int(i==j) for j in range(6)) for i in range(6)]
    assert all(pairing(frobenius(v,A),frobenius(w,A),q)==q*pairing(v,w,q)%3
               for v in standard for w in standard)
    total=0; lagrangians=0; stable=0; mixed=0; witness=None; kernels=[]
    for rows,pivots in rref_subspaces(3,6,3):
        total+=1
        if any(pairing(v,w,q) for v,w in combinations(rows,2)): continue
        lagrangians+=1
        if not all(contains(rows,pivots,frobenius(v,A)) for v in rows): continue
        stable+=1
        transverse=all(rank([[v[j] for j in range(6) if j//2!=axis] for v in rows],3)==3
                       for axis in range(3))
        if transverse:
            mixed+=1
            kernels.append([list(r) for r in rows])
            if witness is None: witness=[list(r) for r in rows]
    assert total==33880 and lagrangians==1120
    prime_order_compatible=(pow(q,3,3)+1-t)%3!=0
    return {'q_mod_3':q,'trace_mod_3':t,'source_frobenius_type':'scalar' if scalar else 'nonscalar',
            'source_matrix':[list(r) for r in A],
            'subspaces_enumerated':total,'lagrangians':lagrangians,
            'frobenius_stable_lagrangians':stable,'transverse_stable_lagrangians':mixed,
            'prime_order_compatible':prime_order_compatible,'witness':witness,
            'transverse_kernel_bases':kernels}


def main():
    started=perf_counter(); results=[]
    for q in [1,2]:
        for t in range(3):
            results.append(screen(q,t))
            if any(2*a%3==t and a*a%3==pow(q,3,3) for a in [1,2]):
                results.append(screen(q,t,True))
    compatible=[r for r in results if r['prime_order_compatible'] and r['transverse_stable_lagrangians']]
    assert len(compatible)==1
    row=compatible[0]
    assert (row['q_mod_3'],row['trace_mod_3'],row['source_frobenius_type'])==(1,1,'scalar')
    path=Path(__file__).resolve().parent/'results/kernel_classification.json'
    path.write_text(json.dumps({'models':results,'seconds':perf_counter()-started,
                               'scope':'Exact torsion-module test; does not certify quotient Jacobians'},indent=2)+'\n')
    for r in results:
        print(r['q_mod_3'],r['trace_mod_3'],r['source_frobenius_type'],
              'stable',r['frobenius_stable_lagrangians'],
              'transverse',r['transverse_stable_lagrangians'],
              'prime-compatible',r['prime_order_compatible'])


if __name__=='__main__': main()
