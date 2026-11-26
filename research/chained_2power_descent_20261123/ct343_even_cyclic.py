#!/usr/bin/env python3
"""CT343 exact F103 even-order cyclic transfer known-answer qualification.

Protocol: research/chained_2power_descent_20261123/CT343_PROTOCOL.md
No timing claims. Source-bound to unmodified CT339. Producer-only [pcc:unverified].
"""
import hashlib
import importlib.util
import json
import pathlib
import sys

P = 103
PIN = 'c798c3201da743091be50e1a3c3cd63200f2e995e72d17991518db3ca5345695'
SOURCE = pathlib.Path(__file__).resolve().with_name('ct339.py')

def need(cond, message):
    if not cond:
        raise RuntimeError(message)

def initialize():
    actual = hashlib.sha256(SOURCE.read_bytes()).hexdigest()
    need(actual == PIN, f'CT339 source digest changed: {actual}')
    spec = importlib.util.spec_from_file_location('ct339_pinned', SOURCE)
    c = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(c)
    c.P = P
    c.DELTA = next(i for i in range(2,P) if pow(i,(P-1)//2,P)==P-1)
    c.COUNTERS = {}
    c.F = c.odd_f()
    c.CS = (c.ecoeff('+'), c.ecoeff('-'))
    c.TS = (c.ecpoint(c.CS[0],c.K(25)/4,c.K(0)),
            c.ecpoint(c.CS[1],c.K(4)/25,c.K(0)))
    need(c.mulprod(c.TS,2)==(None,None), 'CT339 torsion basepoint wrong')
    return c

def jac_record(D):
    return {'u':list(D[0]),'v':list(D[1])}

def target_record(A):
    return [None if x is None else [x[0].a,x[1].a] for x in A]

def subgroup_exact(c, D, n, factors):
    need(c.jmul(D,n)==c.O,f'candidate not annihilated by {n}')
    for d in factors:need(c.jmul(D,n//d)!=c.O,f'candidate order below {n}: factor {d}')

def make_generators(c, mutant):
    H23=c.jmul(c.source_point(10,15),100)
    subgroup_exact(c,H23,23,[23])
    K=c.source_point(6,0)
    need(K!=c.O and c.jmul(K,2)==c.O,'kernel K is not a nonzero 2-torsion point')
    need(c.image(K)[0]==(None,None),'kernel K not killed by Phi')
    G46=c.jadd(H23, K) if mutant != 'drop-kernel' else H23
    subgroup_exact(c,G46,46,[2,23])
    VG46=c.image(G46)[0]
    need(c.mulprod(VG46,23)==(None,None) and VG46!=(None,None), 'Phi G46 order not 23')
    eligible=[]
    H25=None
    seed=None
    for x in range(2,103):
        f=1
        for a in range(1,7):f=f*(x-a)%P
        for y in range(103):
            if y*y%P!=f:continue
            D=c.source_point(x,y)
            H=c.jmul(D,368)
            if H!=c.O and c.jmul(H,25)==c.O and c.jmul(H,5)!=c.O:
                H25=H; seed=[x,y];break
        if H25 is not None:break
    need(H25 is not None,'no rational affine H25 witness in preregistered search')
    subgroup_exact(c,H25,25,[5])
    H575=c.jadd(H23,H25)
    subgroup_exact(c,H575,575,[5,23])
    G1150=c.jadd(H575,K)
    subgroup_exact(c,G1150,1150,[2,5,23])
    VG1150=c.image(G1150)[0]
    need(c.mulprod(VG1150,575)==(None,None),'Phi G1150 not killed by 575')
    for divisor in (5,23):
        need(c.mulprod(VG1150,575//divisor)!=(None,None), 'Phi G1150 lost odd-primary factor')
    return [(G46,VG46,46,23),(G1150,VG1150,1150,575)], H23,H25,K,seed

def run_cohort(c,G,V,n,m,mutant):
    good=0
    false=0
    samples={}
    seen=set()
    for k in range(n):
        Q=c.jmul(G,k)
        need(Q not in seen, f'repeated source target k={k}')
        seen.add(Q)
        W=c.image(Q)[0]
        need(W==c.mulprod(V,k%m),f'homomorphism target at {k}')
        r=c.ebs_pair(V,W,m) # Fresh baby-step table per public target; no k in API.
        need(r is not None and r==k%m, f'BSGS failed at {k} got {r}')
        candidates=(r,r+m)
        source_replays=[c.jmul(G,x)==Q for x in candidates]
        need(sum(source_replays)==1,f'source candidate ambiguity not resolved at {k}')
        need(candidates[source_replays.index(True)]==k,f'recovered wrong original k={k}')
        good+=1
        false+=1
        if k in (0,1,m-1,m,m+1,n-1):samples[str(k)]={'mapped_log':r,'candidates':list(candidates),'accepted':k}
    need(len(seen)==n, 'incomplete unique subgroup enumeration')
    if mutant=='poison-target' and n==1150:
        Q=c.jmul(G,7)
        W=c.image(Q)[0]
        TW=c.sumprod(W,(c.TS[0],None))
        need(c.ebs_pair(V,TW,m) is not None, 'mutant poisoned target correctly rejected: no mapped DLP')
    if mutant=='skip-source' and n==1150:
        Q=c.jmul(G,7)
        r=c.ebs_pair(V,c.image(Q)[0],m)
        need(c.jmul(G,r)==c.jmul(G,r+m),'mutant false second scalar lift correctly rejected')
    return {'order':n,'mapped_order':m,'known_answer_dlp_count':good,'rejected_false_lifts':false,'unique_source_targets':len(seen),'selected_examples':samples,
            'source_generator':jac_record(G),'target_generator':c.record_prod(V)}

def main():
    mutant=sys.argv[1].removeprefix('--') if len(sys.argv)>1 else ''
    need(mutant in ('','drop-kernel','poison-target','skip-source'),'unknown mode')
    c=initialize()
    gens,H23,H25,K,seed=make_generators(c,mutant)
    rows=[run_cohort(c,*item,mutant) for item in gens]
    need(sum(row['known_answer_dlp_count'] for row in rows)==1196,'missing scalar cases')
    print(json.dumps({'schema':'ct343-even-cyclic-lift/v1','field':103,
                      'source_sha256':PIN,'status':'producer_checks_passed',
                      'seed_for_order25':seed,'source_H23':jac_record(H23),
                      'source_H25':jac_record(H25),'kernel':jac_record(K),
                      'cohorts':rows,'primitive_counts':dict(sorted(c.COUNTERS.items()))},sort_keys=True,separators=(',',':')))

if __name__=='__main__':
    try: main()
    except Exception as e:
        print('CT343 FAIL:',str(e),file=sys.stderr)
        sys.exit(1)
