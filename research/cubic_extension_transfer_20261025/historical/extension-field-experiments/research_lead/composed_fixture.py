"""A new input curve reduced to the already reproduced Tian Jacobian.

Walk two 3-isogenies away from the published curve, choosing an endpoint with
non-scalar Frobenius on E[3]. Reverse with dual isogenies and evaluate Tian's
published map. The endpoint has no transverse F_3 kernel in the audited model.
"""
from pathlib import Path
import json
import sys
from time import perf_counter
from sage.all import PolynomialRing

ROOT=Path(__file__).resolve().parent
sys.path.insert(0,str(ROOT.parent))
from tian import curve_data,quartic_model
from common import bsgs_ec,bsgs_generic,point_data
from isogeny_probe import three_type


def published_transfer(K,w):
    data=json.loads((ROOT.parent/'tian_data.json').read_text())
    R=PolynomialRing(K,'u')
    def poly(rows): return R([sum(K(v)*w**i for i,v in enumerate(row)) for row in rows])
    numerators=[(poly(c['A']),poly(c['B'])) for c in data['functions']]
    denominator=poly(data['denominator'])
    F,G,S,T=quartic_model(k=K)
    x=F(F.base_field().gen()); y=F.gen(); O=F.maximal_order()
    B=next(pl for pl in F.places_infinite() if (x/y).evaluate(pl)==359).divisor()
    def phi(P):
        if P.is_zero(): return G.zero()
        X,Y=P[0],P[1]; den=denominator(X)
        if not den: raise ValueError('Exceptional published map denominator')
        coeff=[(a(X)+Y*b(X))/den for a,b in numerators]
        result=G.zero()
        for j in range(3):
            a,b,c,d,e,f=[v**(523**j) for v in coeff]
            D=O.ideal(x**3-a*x*x+b*x-c,y**3-d*y*y+e*y-f).divisor()
            assert D.degree()==3
            result+=G.point(D-3*B)
        return result
    return phi,G,S,T


def main():
    started=perf_counter(); k,K,w,E0,P0,Q0=curve_data(); r=143069629
    chosen=None
    for f1 in E0.isogenies_prime_degree(3):
        E1=f1.codomain()
        for f2 in E1.isogenies_prime_degree(3):
            E2=f2.codomain()
            if three_type(E2)[0]==1:
                chosen=f1,f2; break
        if chosen: break
    assert chosen is not None
    f1,f2=chosen; E1=f1.codomain(); E2=f2.codomain()
    assert int(E2.cardinality())==r
    assert three_type(E0)[0]==4 and three_type(E1)[0]==4 and three_type(E2)[0]==1
    P2=f2(f1(P0)); Q2=f2(f1(Q0))
    back1=f1.dual(); back2=f2.dual()
    preP=back1(back2(P2)); preQ=back1(back2(Q2))
    assert preP==9*P0 and preQ==9*Q0
    assert not P2.is_zero() and r*P2==E2(0)
    source=bsgs_ec(P2,Q2,r); assert source['scalar']==1987
    phi,G,S,T=published_transfer(K,w)
    before=perf_counter(); Sp=phi(preP); Tp=phi(preQ)
    transfer_seconds=perf_counter()-before
    assert Sp==9*S and Tp==9*T and Tp==1987*Sp
    assert not Sp.is_zero() and r*Sp==G.zero()
    # Use the independently built F_523 Jacobian to recover a bounded log.
    _,Gbase,Sbase,Tbase=quartic_model()
    bounded=bsgs_generic(9*Sbase,9*Tbase,4096)
    assert bounded['scalar']==source['scalar']==1987
    result={'status':'passed','p':523,'r':r,'field_modulus':'w^3+2',
            'published_curve':str(E0),'intermediate_curve':str(E1),'new_input_curve':str(E2),
            'kernel_polynomials':[str(f.kernel_polynomial()) for f in [f1,f2]],
            'source_rational_3_kernel_lines':1,'source_transverse_3_kernels':0,
            'after_one_step_rational_3_kernel_lines':4,
            'reverse_isogeny_chain_degree':9,'subgroup_preserved':True,
            'source_point':point_data(P2),'source_target':point_data(Q2),
            'scalar':1987,'source_bsgs_full_order':source,
            'target_bsgs_interval_4096':bounded,
            'composed_map_images_match_nine_times_published_images':True,
            'two_point_jacobian_transfer_seconds':transfer_seconds,'seconds':perf_counter()-started,
            'scope':'Correct composed reduction on a new isogenous fixture; not a new quotient constructor or a full-order target speed benchmark'}
    (ROOT/'results/composed_fixture.json').write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps(result),flush=True)


if __name__=='__main__': main()
