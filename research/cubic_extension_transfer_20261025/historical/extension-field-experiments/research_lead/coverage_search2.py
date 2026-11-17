"""Broader bounded counterexample search with independent certification of hits."""
import argparse,json,itertools
from pathlib import Path
from time import perf_counter
from sage.all import GF,EllipticCurve,Integer,PolynomialRing,set_random_seed
from quotient_theta import normal_three,action_matrices,invariant_tensor,kernels,construct
from obstruction_probe import chi_square_class
ROOT=Path(__file__).resolve().parent


def signs(E,p):
    L=GF(p**18,name='v');emb=E.base_field().embeddings(L)[0]
    EL=EllipticCurve(L,[emb(E.a4()),emb(E.a6())]);basis,sec,_=normal_three(EL)
    action=action_matrices(EL,basis,sec)
    actions=[{c:M.apply_map(lambda x:x**(p**j)) for c,M in action.items()} for j in range(3)]
    Rx=PolynomialRing(L,'t');t=Rx.gen();roots=(t**3+EL.a4()*t+EL.a6()).roots(multiplicities=False)
    out=[]
    for i,rows in enumerate(kernels()):
        T,h=invariant_tensor(actions,rows);value,good=chi_square_class(T,roots,EL,p,emb)
        out.append({'kernel_index':i,'chi18':value,'unobstructed':good})
    return out


def main():
    ap=argparse.ArgumentParser(description=__doc__);ap.add_argument('--count',type=int,default=1000)
    ap.add_argument('--seconds',type=int,default=900);args=ap.parse_args()
    seed=20261029;set_random_seed(seed);start=perf_counter()
    primes=[67,73,79,97,103,109,127,139];fields={p:GF(p**3,name='z') for p in primes}
    rows=[];seen=set();draws=0;hit=False
    for turn in range(200000):
        if len(rows)>=args.count or perf_counter()-start>args.seconds:break
        p=primes[len(rows)%len(primes)];K=fields[p];a,b=K.random_element(),K.random_element();draws+=1
        if not (4*a**3+27*b*b):continue
        E=EllipticCurve(K,[a,b]);j=E.j_invariant()
        if j**p==j:continue
        Rx=PolynomialRing(K,'t');t=Rx.gen()
        # A prime-order group has no rational two- or three-torsion.
        if not (t**3+a*t+b).is_irreducible():continue
        xs=E.division_polynomial(3).roots(multiplicities=False)
        if len(xs) not in [1,4] or (xs[0]**3+a*xs[0]+b).is_square():continue
        r=int(E.cardinality());trace=p**3+1-r
        if r%9!=4 or trace%p==0 or not Integer(r).is_prime():continue
        source=str(E)
        if len(xs)==1:E=E.isogenies_prime_degree(3)[0].codomain();E.set_order(r,num_checks=0)
        jj=E.j_invariant();orbit=min(tuple(int(c) for c in v.polynomial().list()) for v in [jj,jj**p,jj**(p*p)])
        key=(p,r,orbit)
        if key in seen:continue
        seen.add(key);ks=signs(E,p);good=sum(k['unobstructed'] for k in ks)
        row={'p':p,'r':r,'source_curve':source,'prepared_curve':str(E),'frobenius_orbit_key':orbit,
             'preprocessing_steps':int(len(xs)==1),'usable_count':good,'kernels':ks}
        if good==0:
            print('CANDIDATE COUNTEREXAMPLE',p,r,str(E),flush=True)
            cert=construct(E,p,certify=True)
            assert all(not k['quotient_is_base_field_jacobian'] for k in cert['kernels'])
            row['independent_certificate']=cert;hit=True
            floors=[phi.codomain() for phi in E.isogenies_prime_degree(3)
                    if len(phi.codomain().division_polynomial(3).roots(multiplicities=False))==1]
            if floors:row['forced_one_step_source']=str(floors[0])
        rows.append(row)
        out={'seed':seed,'curves':rows,'draws':draws,'seconds':perf_counter()-start,'all_obstructed_found':hit,
             'scope':'Distinct prepared Frobenius orbits; exploratory chi18 search with independent certificates on zero-coverage hits'}
        (ROOT/'results/coverage_search2.json').write_text(json.dumps(out,indent=2)+'\n')
        if len(rows)%25==0 or hit:print('SEARCH',len(rows),'latest',p,good,'seconds',int(perf_counter()-start),flush=True)
        if hit:break
    print('SEARCH COMPLETE',len(rows),'hit',hit,'seconds',int(perf_counter()-start),flush=True)


if __name__=='__main__':main()
