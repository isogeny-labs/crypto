"""Test one-3-isogeny preprocessing for Tian-type mixed kernels.

The hypothesis is only about torsion-kernel eligibility. No new Jacobian
quotient is reconstructed, so these experiments do not demonstrate new attacks.
"""
import argparse
from collections import Counter
import json
from pathlib import Path
import random
import sys
from time import perf_counter
from sage.all import GF,EllipticCurve,Integer,factor,set_random_seed,version

ROOT=Path(__file__).resolve().parent
sys.path.insert(0,str(ROOT.parent))
from common import bsgs_ec,point_data,field_data


def three_type(E):
    f=E.division_polynomial(3)
    roots=f.roots(multiplicities=False)
    assert len(roots) in [0,1,2,4]
    # For a prime-order group with r>3, four rational x-coordinates imply
    # Frobenius = -I on E[3], and all corresponding y-coordinates are nonrational.
    if len(roots)==4:
        assert all(not (x**3+E.a2()*x*x+E.a4()*x+E.a6()).is_square() for x in roots)
    return len(roots),[int(g.degree()) for g,e in f.factor()]


def one_curve(E,p,verify_dlp=False):
    before=perf_counter(); K=E.base_field(); Q=int(K.cardinality())
    r=int(E.cardinality()); t=Q+1-r; D=t*t-4*Q
    assert Integer(r).is_prime() and r>3
    ordinary=not E.is_supersingular(); assert ordinary
    roots,degrees=three_type(E)
    scalar_minus_one=(roots==4)
    gate=(p%3==1 and (2*(Q+1)-r)%9==0)
    direct_kernel=(p%3==1 and scalar_minus_one)
    row={'p':p,'field_degree':3,'field':field_data(K),'curve':str(E),
         'j_invariant':str(E.j_invariant()),'prime_group_order':r,'trace':t,
         'frobenius_discriminant':D,'twist_order':2*(Q+1)-r,
         'division_polynomial_factor_degrees':degrees,'rational_3_kernel_lines':roots,
         'scalar_minus_one_on_3_torsion':scalar_minus_one,
         'cheap_preprocessing_gate':gate,'direct_transverse_kernel_eligible':direct_kernel,
         'preprocessed_transverse_kernel_eligible':direct_kernel,
         'isogeny_steps':0,'status':'passed'}
    # Cheap screen for prime-order cubic curves: r == 4 mod 9 when p == 1 mod 3.
    assert gate==(p%3==1 and r%9==4)
    assert direct_kernel<=gate
    if gate and not scalar_minus_one:
        assert roots==1
        phis=E.isogenies_prime_degree(3)
        assert len(phis)==1
        phi=phis[0]; E1=phi.codomain()
        after_roots,after_degrees=three_type(E1)
        row.update(neighbor_curve=str(E1),neighbor_rational_3_kernel_lines=after_roots,
                   neighbor_factor_degrees=after_degrees,
                   isogeny_steps=1,preprocessed_transverse_kernel_eligible=after_roots==4)
        assert int(E1.cardinality())==r and after_roots==4
        P=E.random_point()
        while P.is_zero(): P=E.random_point()
        S=phi(P)
        assert not S.is_zero() and r*S==E1(0)
        m=random.randrange(1,r); target=m*P
        assert phi(target)==m*S
        assert phi(2*P)+phi(3*P)==phi(5*P)
        row.update(subgroup_order_preserved=True,scalar_relation_verified=True,
                   isogeny_degree=int(phi.degree()))
        if verify_dlp:
            src=bsgs_ec(P,target,r); dst=bsgs_ec(S,phi(target),r)
            assert src['scalar']==dst['scalar']==m
            row.update(source_point=point_data(P),source_target=point_data(target),
                       scalar=m,source_bsgs=src,neighbor_bsgs=dst)
    assert row['preprocessed_transverse_kernel_eligible']==gate
    row['seconds']=perf_counter()-before
    return row


def sample(p,count,dlp_limit):
    K=GF(p**3,name='z'); seen=set(); rows=[]; draws=0
    while len(rows)<count and draws<20000:
        draws+=1; a,b=K.random_element(),K.random_element()
        if 4*a**3+27*b*b==0: continue
        E=EllipticCurve(K,[a,b]); j=E.j_invariant()
        if j**p==j or E.is_supersingular(): continue
        N=int(E.cardinality())
        if N<=3 or not Integer(N).is_prime(): continue
        key=(j,N)
        if key in seen: continue
        seen.add(key)
        row=one_curve(E,p,verify_dlp=sum('source_bsgs' in r for r in rows)<dlp_limit)
        rows.append(row)
    return rows,draws


def main():
    ap=argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--samples',type=int,default=24)
    ap.add_argument('--seed',type=int,default=20261025)
    ap.add_argument('--primes',type=int,nargs='+',default=[5,7,11,13,17,19,23,29,31,37,41,43])
    args=ap.parse_args(); set_random_seed(args.seed); random.seed(args.seed)
    started=perf_counter(); rows=[]; strata=[]
    out=ROOT/'results'; out.mkdir(exist_ok=True)
    for p in args.primes:
        rs,draws=sample(p,args.samples,2)
        rows.extend(rs)
        stratum={'p':p,'p_mod_3':p%3,'draws':draws,'sampled_prime_order_curves':len(rs),
                 'direct_eligible':sum(r['direct_transverse_kernel_eligible'] for r in rs),
                 'after_one_step_eligible':sum(r['preprocessed_transverse_kernel_eligible'] for r in rs),
                 'gated':sum(r['cheap_preprocessing_gate'] for r in rs)}
        strata.append(stratum); print(json.dumps(stratum),flush=True)
        (out/'curve_probe.json').write_text(json.dumps({'seed':args.seed,'sage':version(),
                 'strata':strata,'curves':rows,'seconds':perf_counter()-started},indent=2)+'\n')
    # A fixture tied to the already reproduced published map.
    from tian import curve_data
    _,_,_,E,_,_=curve_data()
    fixture=one_curve(E,523)
    assert fixture['direct_transverse_kernel_eligible']
    neighbors=E.isogenies_prime_degree(3)
    fixture['three_isogeny_neighbors']=[]
    for phi in neighbors:
        E1=phi.codomain()
        row=one_curve(E1,523,verify_dlp=True)
        fixture['three_isogeny_neighbors'].append(row)
    payload={'seed':args.seed,'sage':version(),'strata':strata,'curves':rows,
             'published_fixture':fixture,'seconds':perf_counter()-started,
             'scope':'Torsion eligibility and subgroup preservation, not quotient reconstruction or an ECDLP speedup'}
    (out/'curve_probe.json').write_text(json.dumps(payload,indent=2)+'\n')
    print('RESULT',len(rows),'direct',sum(r['direct_transverse_kernel_eligible'] for r in rows),
          'after',sum(r['preprocessed_transverse_kernel_eligible'] for r in rows),flush=True)


if __name__=='__main__': main()
