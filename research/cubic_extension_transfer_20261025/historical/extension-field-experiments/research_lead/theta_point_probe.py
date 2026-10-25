"""Evaluate a newly constructed quotient map in level-two Weil coordinates.

These projective coordinates represent A/{+1,-1}; no conversion to divisors
on the reconstructed plane quartic is implemented here. This is a map audit,
not a target index-calculus benchmark or a new DLP solver.
"""
import hashlib
import itertools
import json
from pathlib import Path
import sys
from time import perf_counter
from sage.all import GF,EllipticCurve,PolynomialRing,vector,set_random_seed
from quotient_theta import (normal_three,action_matrices,invariant_tensor,kernels,theta_null,
                            contract,bits_code,dot,gradients,riemann_quartic,rational_quartic,
                            jacobian_order_certificate)

ROOT=Path(__file__).resolve().parent
sys.path.insert(0,str(ROOT.parent))
from common import point_data,field_data


def field_tuple(a): return tuple(int(c) for c in a.polynomial().list())


def projective_key(values):
    pivot=next(v for v in values if v)
    return tuple(field_tuple(v/pivot) for v in values)


def quotient_evaluator(E,p,kernel_index=0):
    L=GF(p**18,name='v'); emb=E.base_field().embeddings(L)[0]
    EL=EllipticCurve(L,[emb(E.a4()),emb(E.a6())]); basis,sec,_=normal_three(EL)
    actions0=action_matrices(EL,basis,sec)
    actions=[{c:M.apply_map(lambda x:x**(p**j)) for c,M in actions0.items()} for j in range(3)]
    rows=kernels()[kernel_index]; T,h=invariant_tensor(actions,rows)
    Rx=PolynomialRing(L,'x'); x=Rx.gen()
    roots=(x**3+EL.a4()*x+EL.a6()).roots(multiplicities=False)
    e1,e2,e3=roots
    a=((e1-e3)/(e1-e2)).sqrt(); b=((e2-e3)/(e2-e1)).sqrt()
    # Match the normality convention used by theta_null.
    V=EL.random_point(); Q1=EL(e1,0)
    c=-a*(V[0]-e2)/(V[0]-e3)*b*((V+Q1)[0]-e1)/((V+Q1)[0]-e3)*(V[0]-e3)
    curves=[EllipticCurve(L,[EL.a4()**(p**j),EL.a6()**(p**j)]) for j in range(3)]
    q=[EL(0),EL(e1,0),EL(e2,0),EL(e3,0)]
    qs=[[curves[j](Q[0]**(p**j),Q[1]**(p**j)) if not Q.is_zero() else curves[j](0) for Q in q] for j in range(3)]
    def evaluate(points):
        tables=[]; gs=[]
        for j,R in enumerate(points):
            assert not R.is_zero()
            table=[]
            for code in range(4):
                U=R+qs[j][code]-qs[j][3]
                assert not U.is_zero()
                table.append([L.one(),U[0],U[1]])
            tables.append(table)
            X=R[0]; e1j,e2j,e3j=[e**(p**j) for e in roots]
            gs.append([L.one(),a**(p**j)*(X-e2j)/(X-e3j),
                       b**(p**j)*(X-e1j)/(X-e3j),c**(p**j)/(X-e3j)])
        vals=[]
        for idx in range(64):
            codes=bits_code(idx)
            gg=gs[0][codes[0]]*gs[1][codes[1]]*gs[2][codes[2]]
            vals.append(gg**3*contract(T,[tables[j][codes[j]] for j in range(3)])**2)
        return projective_key(vals)
    def lift(P):
        assert not P.is_zero()
        return tuple(curves[j](emb(P[0])**(p**j),emb(P[1])**(p**j)) for j in range(3))
    identity=projective_key(theta_null(T,roots,p))
    kernel=[]
    BB=[tuple(curves[j](B[0]**(p**j),B[1]**(p**j)) for B in basis) for j in range(3)]
    for cs in itertools.product(range(3),repeat=3):
        vv=[sum(cs[i]*rows[i][j] for i in range(3))%3 for j in range(6)]
        kernel.append(tuple(vv[2*j]*BB[j][0]+vv[2*j+1]*BB[j][1] for j in range(3)))
    f,_,_=riemann_quartic(gradients(T,roots,EL.a4(),p));g,_=rational_quartic(f,emb,p)
    certificate=jacobian_order_certificate(g,p,int(E.cardinality()))
    assert certificate['quotient_is_base_field_jacobian']
    return evaluate,lift,identity,kernel,emb,L,g,certificate


def main():
    start=perf_counter(); set_random_seed(20261026)
    p=7; K=GF(p**3,name='z'); z=K.gen()
    E=EllipticCurve(K,[4*z*z+3*z+3,4*z*z+5*z+2]); phi=E.isogenies_prime_degree(3)[0]
    E1=phi.codomain(); r=int(E.cardinality()); assert r==373
    P=E(2*z*z+5*z+5,6*z*z+6*z+1); Q=E(4*z*z+2*z,z+2)
    assert 218*P==Q
    ev,lift,zero,kernel,emb,L,g,certificate=quotient_evaluator(E1,p,0)
    S=phi(P); T=phi(Q); S3=lift(S); key=ev(S3)
    assert key!=zero and key==ev(tuple(-V for V in S3))
    for V in kernel:
        assert ev(tuple(A+B for A,B in zip(S3,V)))==key
    # Exhaustive, finite faithfulness test for this toy r-order subgroup.
    keys={zero}; R=E1(0)
    for i in range(1,r):
        R+=S; keys.add(ev(lift(R)))
    assert len(keys)==1+(r-1)//2
    target_key=ev(lift(T)); assert target_key!=key and target_key!=zero
    out={'p':p,'field':field_data(K),'source_curve':str(E),'prepared_curve':str(E1),
         'source_order':r,'kernel_index':0,'isogeny_degree':3,'quotient_degree':27,
         'source_point':point_data(P),'source_target':point_data(Q),'known_test_scalar':218,
         'all_27_kernel_translations_verified':True,'opposite_points_identified':True,
         'subgroup_points_enumerated':r,'distinct_projective_images':len(keys),
         'expected_distinct_images':1+(r-1)//2,
         'associated_quartic':str(g),'associated_jacobian_order_certificate':certificate,
         'image_generator':key,'image_target':target_key,'image_identity':zero,
         'ambient_field':field_data(L),'seconds':perf_counter()-start,
         'scope':'Exact new quotient point evaluation modulo sign; no Jacobian-divisor inversion or target DLP solve'}
    (ROOT/'results/theta_point_probe.json').write_text(json.dumps(out,indent=2)+'\n')
    print(json.dumps({k:v for k,v in out.items() if not k.startswith('image_')}),flush=True)


if __name__=='__main__': main()
