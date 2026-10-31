"""Test a theta-gradient evaluation of chi_18, up to a fixed square factor.

Jacobi's derivative formula on the fundamental system with odd indices
63,38,52 and even indices 0,4,32,55,62 gives
 chi18 ~ prod_even(theta) * (det(grad_odd)/prod_five(theta))^18.
Here ~ denotes the nonzero square constant 2^26 in the usual conventions.
The code eliminates square-root choices algebraically and compares the resulting
base-field square class with independently certified Jacobian orders.
"""
import itertools,json
from pathlib import Path
from time import perf_counter
from sage.all import GF,EllipticCurve,PolynomialRing,matrix,vector,prod,set_random_seed
from quotient_sweep import parse_curve
from quotient_theta import (normal_three,action_matrices,invariant_tensor,kernels,
                            theta_null,gradients,contract,bits_code,riemann_quartic,rational_quartic)
ROOT=Path(__file__).resolve().parent


def chi_square_class(T,roots,E,p,emb):
    L=E.base_field();e1,e2,e3=roots
    a=((e1-e3)/(e1-e2)).sqrt();b=((e2-e3)/(e2-e1)).sqrt()
    R=E.random_point();Q1=E(e1,0)
    c=-a*(R[0]-e2)/(R[0]-e3)*b*((R+Q1)[0]-e1)/((R+Q1)[0]-e3)*(R[0]-e3)
    gs=[L.one(),a,b,c]
    vs=[[L.one(),e,L.zero()] for e in [e3,e2,e1]]+[[L.zero(),L.zero(),-L.one()]]
    even=[i for i in range(64) if not sum(((i>>j)&1)*((i>>(j+3))&1) for j in range(3))%2]
    contractions={}
    for i in even:
        codes=bits_code(i)
        contractions[i]=contract(T,[[v**(p**j) for v in vs[codes[j]]] for j in range(3)])
    # Each of a,b occurs ten times and c six times per elliptic coordinate.
    theta_product=prod(contractions.values())*prod((a**15*b**15*c**9)**(p**j) for j in range(3))
    lines=gradients(T,roots,E.a4(),p)
    odd=[63,38,52];five=[0,4,32,55,62]
    det=matrix(L,[lines[i] for i in odd]).det()
    z=emb.domain().gen();B=matrix(L,3,3,lambda i,j:emb(z**j)**(p**i))
    det18=(det*B.det())**18
    for i in odd:
        codes=bits_code(i)
        det18*=prod(gs[codes[j]]**(27*p**j) for j in range(3))
    denominator=L.one()
    for i in five:
        codes=bits_code(i)
        theta_squared=contractions[i]**2*prod(gs[codes[j]]**(3*p**j) for j in range(3))
        denominator*=theta_squared**9
    chi=theta_product*det18/denominator
    assert chi and chi**p==chi, 'chi18 square representative failed base-field rationality'
    value=GF(p)(chi)
    return int(value),bool(value.is_square())


def one_curve(E,p):
    L=GF(p**18,name='v');emb=E.base_field().embeddings(L)[0]
    EL=EllipticCurve(L,[emb(E.a4()),emb(E.a6())]);basis,sec,_=normal_three(EL)
    action=action_matrices(EL,basis,sec)
    actions=[{c:M.apply_map(lambda x:x**(p**j)) for c,M in action.items()} for j in range(3)]
    Rx=PolynomialRing(L,'x');x=Rx.gen();roots=(x**3+EL.a4()*x+EL.a6()).roots(multiplicities=False)
    rows=[]
    for i,k in enumerate(kernels()):
        T,h=invariant_tensor(actions,k)
        start=perf_counter();v,square=chi_square_class(T,roots,EL,p,emb)
        chi_seconds=perf_counter()-start
        f,_,_=riemann_quartic(gradients(T,roots,EL.a4(),p));g,_=rational_quartic(f,emb,p)
        rows.append({'kernel_index':i,'chi18_up_to_square_factor':v,'predicts_base_field_jacobian':square,
                     'quartic_coefficients':{str(tuple(e)):int(c) for e,c in g.dict().items()},
                     'obstruction_evaluation_seconds':chi_seconds,
                     'including_quartic_validation_seconds':perf_counter()-start})
    return rows


if __name__=='__main__':
    set_random_seed(20261026);start=perf_counter()
    source=json.loads((ROOT/'results/curve_probe.json').read_text())
    inputs=[r for r in source['curves'] if r['isogeny_steps']==1]
    results=[]
    for i,row in enumerate(inputs):
        p=row['p'];K=GF(p**3,name='z');E=parse_curve(row['curve'],K).isogenies_prime_degree(3)[0].codomain()
        rows=one_curve(E,p); record={'input_index':i,'p':p,'kernels':rows}
        certified=json.loads((ROOT/'results/quotient_sweep.json').read_text())['curves']
        if i<len(certified) and certified[i]['status']=='passed':
            lookup={tuple(sorted(c['quartic_coefficients'].items())):c for c in certified[i]['kernels']}
            for k in rows:
                c=lookup[tuple(sorted(k['quartic_coefficients'].items()))]
                assert k['predicts_base_field_jacobian']==c['quotient_is_base_field_jacobian']
            record['independent_order_comparison_passed']=True
        results.append(record)
        (ROOT/'results/obstruction_probe.json').write_text(json.dumps({'curves':results,'seconds':perf_counter()-start},indent=2)+'\n')
        print('chi18',i,p,sum(r['predicts_base_field_jacobian'] for r in rows),'matched',record.get('independent_order_comparison_passed'),flush=True)
