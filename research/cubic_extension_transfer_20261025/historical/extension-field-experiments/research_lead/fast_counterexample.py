"""Early-rejection search in a full-three-torsion Hesse family.

A quadratic twist of a Hesse elliptic curve has scalar -I three-torsion.
One unobstructed kernel suffices to reject a counterexample, so usually only
one or two of the 12 chi18 evaluations are needed. All-negative hits trigger
the independent quartic/order construction.
"""
import argparse,json
from pathlib import Path
from time import perf_counter
from sage.all import GF,PolynomialRing,EllipticCurve,Integer,set_random_seed
from quotient_theta import normal_three,action_matrices,invariant_tensor,kernels,construct
from obstruction_probe import chi_square_class
ROOT=Path(__file__).resolve().parent


def witness(E,p):
    L=GF(p**18,name='v');emb=E.base_field().embeddings(L)[0]
    EL=EllipticCurve(L,[emb(E.a4()),emb(E.a6())]);basis,sec,_=normal_three(EL)
    action=action_matrices(EL,basis,sec)
    actions=[{c:M.apply_map(lambda x:x**(p**j)) for c,M in action.items()} for j in range(3)]
    Rx=PolynomialRing(L,'t');t=Rx.gen();roots=(t**3+EL.a4()*t+EL.a6()).roots(multiplicities=False)
    values=[]
    for i,rows in enumerate(kernels()):
        T,h=invariant_tensor(actions,rows);v,good=chi_square_class(T,roots,EL,p,emb);values.append(v)
        if good:return i,values
    return None,values


def main():
    ap=argparse.ArgumentParser(description=__doc__);ap.add_argument('--count',type=int,default=5000)
    ap.add_argument('--seconds',type=int,default=1800);args=ap.parse_args();seed=20261031
    set_random_seed(seed);start=perf_counter();primes=[67,73,79,97,103,109,127,139]
    fields={p:GF(p**3,name='z') for p in primes};rows=[];seen=set();draws=0;hit=False
    nonsquares={p:next(a for a in range(2,p) if not GF(p)(a).is_square()) for p in primes}
    for _ in range(300000):
        if len(rows)>=args.count or perf_counter()-start>args.seconds:break
        p=primes[len(rows)%len(primes)];K=fields[p];lam=K.random_element();draws+=1
        if lam**3==1:continue
        A=-27*lam*(lam**3+8);B=54*(lam**6-20*lam**3-8)
        T=EllipticCurve(K,[A,B]);E=T.quadratic_twist(K(nonsquares[p]));j=E.j_invariant()
        if j**p==j:continue
        Rx=PolynomialRing(K,'t');t=Rx.gen()
        if not (t**3+E.a4()*t+E.a6()).is_irreducible():continue
        r=int(E.cardinality())
        if (p**3+1-r)%p==0 or not Integer(r).is_prime():continue
        assert r%9==4
        assert len(E.division_polynomial(3).roots(multiplicities=False))==4
        orbit=min(tuple(int(c) for c in v.polynomial().list()) for v in [j,j**p,j**(p*p)])
        key=(p,r,orbit)
        if key in seen:continue
        seen.add(key);idx,values=witness(E,p)
        row={'p':p,'r':r,'curve':str(E),'hesse_parameter':str(lam),'frobenius_orbit_key':orbit,
             'unobstructed_kernel_witness':idx,'chi18_values_tested':values}
        if idx is None:
            print('CANDIDATE',p,r,str(E),flush=True)
            cert=construct(E,p,certify=True)
            assert all(not k['quotient_is_base_field_jacobian'] for k in cert['kernels'])
            row['independent_certificate']=cert;hit=True
            floors=[phi.codomain() for phi in E.isogenies_prime_degree(3)
                    if len(phi.codomain().division_polynomial(3).roots(multiplicities=False))==1]
            if floors:row['forced_one_step_source']=str(floors[0])
        rows.append(row)
        (ROOT/'results/fast_counterexample.json').write_text(json.dumps({'seed':seed,'curves':rows,
             'draws':draws,'seconds':perf_counter()-start,'all_obstructed_found':hit,
             'scope':'Early-rejection Hesse-family search; one positive witness suffices; no density claim'},indent=2)+'\n')
        if len(rows)%100==0 or hit:print('FAST',len(rows),'seconds',int(perf_counter()-start),'tested',len(values),flush=True)
        if hit:break
    print('FAST COMPLETE',len(rows),'hit',hit,'seconds',int(perf_counter()-start),flush=True)


if __name__=='__main__':main()
