"""A larger prime-order CM-generated input and quotient construction.

CM supplies candidate orders only. A nonzero point annihilated by the proposed
prime r, together with the Hasse interval containing just one multiple of r,
certifies the order independently. No discrete logarithm is solved here.
"""
import argparse,json
from pathlib import Path
from time import perf_counter
from sage.all import (GF,QuadraticField,PolynomialRing,hilbert_class_polynomial,
                       next_prime,EllipticCurve_from_j,EllipticCurve,Integer,matrix,set_random_seed)
from math import isqrt,gcd
from quotient_theta import normal_three,action_matrices,invariant_tensor,kernels,gradients,riemann_quartic,rational_quartic,bitangent_check
from obstruction_probe import chi_square_class
ROOT=Path(__file__).resolve().parent


def smooth_in_three_charts(f,p):
    """Exact Jacobian-ideal certificates without Sage's large-p factorization."""
    import sympy as sp
    u,v=sp.symbols('u v');certificates=[]
    for fixed in range(3):
        free=[i for i in range(3) if i!=fixed]
        ff=sum(int(c)*u**e[free[0]]*v**e[free[1]] for e,c in f.dict().items())
        gb=sp.groebner([ff,sp.diff(ff,u),sp.diff(ff,v)],u,v,modulus=p,order='grevlex')
        assert len(gb.polys)==1 and gb.polys[0].is_one
        certificates.append({'coordinate_set_to_one':fixed,'jacobian_ideal_groebner_basis':['1']})
    return certificates


def source_instance(bits):
    # Discriminant -59 has class number 3 and 2 is inert. In contrast, -23
    # splits at 2 and its maximal-order family forces even group orders.
    F=QuadraticField(-59,'s');H=hilbert_class_polynomial(-59)
    p=Integer(2)**(bits-1);draws=0
    while True:
        p=next_prime(p);draws+=1
        if p%3!=1:continue
        k=GF(p);h=H.change_ring(k)
        if not h.is_irreducible():continue
        I=F.ideal(p).factor()[0][0]
        if I.norm()!=p:continue
        gen=(I**3).gens_reduced()
        assert len(gen)==1
        alpha=gen[0];assert alpha.norm()==p**3
        t=abs(Integer(alpha.trace()));v=isqrt(int((4*p**3-t*t)//59))
        assert 59*v*v==4*p**3-t*t
        if v%3 or v%9==0:continue
        candidates=[int(p**3+1-s*t) for s in [1,-1]]
        rs=[r for r in candidates if r%9==4 and Integer(r).is_prime()]
        if not rs:continue
        r=rs[0];K=GF(p**3,name='z',modulus=h)
        E=EllipticCurve_from_j(K.gen());P=E.random_point()
        if not (r*P).is_zero():E=E.quadratic_twist()
        P=E.random_point()
        while P.is_zero():P=E.random_point()
        if not (r*P).is_zero():continue
        # Prime r divides #E; only one multiple lies in the Hasse interval.
        lo=int(p**3+1)-isqrt(int(4*p**3));hi=int(p**3+1)+isqrt(int(4*p**3))
        assert (lo+r-1)//r==hi//r==1
        E.set_order(r,num_checks=0)
        assert E.j_invariant()**p!=E.j_invariant() and (p**3+1-r)%p
        assert len(E.division_polynomial(3).roots(multiplicities=False))==4
        descending=next(phi for phi in E.isogenies_prime_degree(3)
                        if len(phi.codomain().division_polynomial(3).roots(multiplicities=False))==1)
        source=descending.codomain();source.set_order(r,num_checks=0)
        prep=source.isogenies_prime_degree(3);assert len(prep)==1
        prepared=prep[0].codomain();prepared.set_order(r,num_checks=0)
        return int(p),r,K,source,prepared,{'cm_discriminant':-59,'hilbert_class_polynomial':str(H),
              'prime_candidates_examined':draws,'certifying_curve':str(E),'nonzero_point':[str(P[0]),str(P[1])],
              'prime_annihilator':r,'hasse_interval':[lo,hi],'only_multiple_of_r_in_hasse_interval':True,
              'source_to_prepared_degree':3}


def main():
    ap=argparse.ArgumentParser(description=__doc__);ap.add_argument('--bits',type=int,default=32)
    args=ap.parse_args();set_random_seed(20261026);start=perf_counter()
    p,r,K,source,E,cert=source_instance(args.bits)
    print('SOURCE',p,'r_bits',int(r).bit_length(),'draws',cert['prime_candidates_examined'],flush=True)
    L=GF(p**18,name='v',modulus='random');emb=K.embeddings(L)[0]
    EL=EllipticCurve(L,[emb(E.a4()),emb(E.a6())]);basis,sec,_=normal_three(EL)
    actions0=action_matrices(EL,basis,sec)
    actions=[{c:M.apply_map(lambda x:x**(p**j)) for c,M in actions0.items()} for j in range(3)]
    Rx=PolynomialRing(L,'x');x=Rx.gen();roots=(x**3+EL.a4()*x+EL.a6()).roots(multiplicities=False)
    rows=[];example=None
    for i,k in enumerate(kernels()):
        T,h=invariant_tensor(actions,k);before=perf_counter()
        value,usable=chi_square_class(T,roots,EL,p,emb)
        row={'kernel_index':i,'chi18_up_to_square_factor':value,'predicts_base_field_jacobian':usable,
             'chi18_seconds':perf_counter()-before}
        if usable and example is None:
            lines=gradients(T,roots,EL.a4(),p);f,_,_=riemann_quartic(lines)
            for line in lines.values():bitangent_check(f,line)
            g,_=rational_quartic(f,emb,p)
            smooth_certificate=smooth_in_three_charts(g,p)
            example={'kernel_index':i,'quartic':str(g),
                     'quartic_coefficients':{str(tuple(e)):int(c) for e,c in g.dict().items()},
                     'smooth':True,'smoothness_certificate':smooth_certificate,
                     'bitangents_verified':28,'base_field_descent':True}
        rows.append(row);print('KERNEL',i,'chi18 square',usable,flush=True)
    assert example is not None
    out={'p':p,'p_bits':p.bit_length(),'r':r,'r_bits':r.bit_length(),'source_curve':str(source),
         'prepared_curve':str(E),'field_modulus':str(K.modulus()),'order_certificate':cert,
         'source_direct_3_kernel_ineligible':True,'one_step_preprocessing_verified':True,
         'kernels':rows,'example_quartic':example,'seconds':perf_counter()-start,
         'scope':'Larger source-order certificate, exact quartic reconstruction, and chi18 screening; no independent large Jacobian point count, divisor-map inversion, or DLP speedup'}
    (ROOT/f'results/large_instance_{args.bits}.json').write_text(json.dumps(out,indent=2)+'\n')
    print('DONE',p,r,'usable',sum(k['predicts_base_field_jacobian'] for k in rows),'seconds',out['seconds'],flush=True)


if __name__=='__main__':main()
