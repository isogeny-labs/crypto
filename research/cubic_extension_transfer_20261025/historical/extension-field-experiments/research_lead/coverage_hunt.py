"""Bounded search for a torsion-eligible curve with all 12 quotients obstructed.

This is exploratory sampling, not a proof or a population-density estimate.
Any all-obstructed hit is reconstructed and order-certified before reporting.
"""
import argparse,json
from pathlib import Path
from time import perf_counter
from sage.all import GF,EllipticCurve,Integer,set_random_seed
from obstruction_probe import one_curve
from quotient_theta import construct
from quotient_sweep import parse_curve
ROOT=Path(__file__).resolve().parent


def main():
    ap=argparse.ArgumentParser(description=__doc__);ap.add_argument('--count',type=int,default=200)
    ap.add_argument('--resume',action='store_true')
    ap.add_argument('--max-draws-per-prime',type=int,default=6000)
    args=ap.parse_args();set_random_seed(271828);start=perf_counter()
    primes=[13,19,31,37,43,61];fields={p:GF(p**3,name='z') for p in primes}
    seen=set();rows=[];draws=0;turn=0;budgets={p:0 for p in primes}
    if args.resume and (ROOT/'results/coverage_hunt.json').exists():
        old=json.loads((ROOT/'results/coverage_hunt.json').read_text())
        rows=old['curves'];draws=old['draws']
        for row in rows:
            E=parse_curve(row['prepared_curve'],fields[row['p']])
            seen.add((row['p'],str(E.j_invariant()),row['r']))
    while len(rows)<args.count and any(n<args.max_draws_per_prime for n in budgets.values()):
        p=primes[turn%len(primes)];turn+=1;K=fields[p]
        if budgets[p]>=args.max_draws_per_prime:continue
        accepted=False
        while True:
            if budgets[p]>=args.max_draws_per_prime:break
            draws+=1;budgets[p]+=1;a,b=K.random_element(),K.random_element()
            if 4*a**3+27*b*b==0:continue
            E=EllipticCurve(K,[a,b]);j=E.j_invariant()
            if j**p==j or E.is_supersingular():continue
            r=int(E.cardinality())
            if r%9!=4 or not Integer(r).is_prime():continue
            source=E;roots=len(E.division_polynomial(3).roots(multiplicities=False))
            if roots==1:E=E.isogenies_prime_degree(3)[0].codomain()
            else:assert roots==4
            key=(p,str(E.j_invariant()),r)
            if key in seen:continue
            seen.add(key);accepted=True;break
        if not accepted:continue
        ks=one_curve(E,p);usable=sum(k['predicts_base_field_jacobian'] for k in ks)
        row={'p':p,'r':r,'source_curve':str(source),'prepared_curve':str(E),
             'preprocessing_steps':int(roots==1),'usable_quotient_count':usable,'kernels':ks}
        if usable==0:
            cert=construct(E,p,certify=True)
            assert all(not k['quotient_is_base_field_jacobian'] for k in cert['kernels'])
            row['independent_all_obstructed_certificate']=cert
        rows.append(row)
        out={'seed':271828,'resumed':args.resume,'curves':rows,'draws':draws,
             'draws_per_prime_this_run':budgets,'max_draws_per_prime':args.max_draws_per_prime,
             'seconds':perf_counter()-start,
             'scope':'Exploratory chi18 coverage search; independent quotient-order certificates only for all-obstructed hits'}
        (ROOT/'results/coverage_hunt.json').write_text(json.dumps(out,indent=2)+'\n')
        if len(rows)%20==0 or usable==0:print('HUNT',len(rows),'all-obstructed',sum(r['usable_quotient_count']==0 for r in rows),'latest',p,usable,flush=True)
        if usable==0:break
    print('HUNT COMPLETE',len(rows),'all-obstructed',sum(r['usable_quotient_count']==0 for r in rows),flush=True)


if __name__=='__main__':main()
