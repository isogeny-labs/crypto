"""Certify a source whose unique rational degree-three step is all-obstructed.

This distinguishes total preprocessing length from the extra length allowed
after the torsion screen. The former one-step claim can fail even if the latter
one-extra-step claim survives.
"""
import ast,json
from math import isqrt
from pathlib import Path
from time import perf_counter
from sage.all import GF,Integer,PolynomialRing,Curve,set_random_seed
from quotient_sweep import parse_curve
from quotient_theta import construct,jacobian_order_certificate
from obstruction_probe import one_curve
from common import point_data,field_data
ROOT=Path(__file__).resolve().parent


def order_certificate(E,r):
    q=int(E.base_field().cardinality());lo=q+1-isqrt(4*q);hi=q+1+isqrt(4*q)
    assert Integer(r).is_prime() and (lo+r-1)//r==hi//r==1
    P=E.random_point()
    while P.is_zero():P=E.random_point()
    assert (r*P).is_zero();E.set_order(r,num_checks=0)
    return P,{'point':point_data(P),'prime_annihilator':r,'hasse_interval':[lo,hi],'unique_multiple':True}


def main():
    seed=20261113;set_random_seed(seed);start=perf_counter()
    data=json.loads((ROOT/'results/neighborhood_random.json').read_text())
    row=next(row for row in data['curves'] if row['usable_witness'] is None
             and any(n['three_lines']==1 for n in row['neighbors']))
    p=row['p'];r=row['r'];K=GF(p**3,name='z');E=parse_curve(row['curve'],K)
    assert p%3==1 and r%9==4 and (p**3+1-r)%p and E.j_invariant()**p!=E.j_invariant()
    P,prepared_order=order_certificate(E,r)
    cert=construct(E,p,certify=True)
    twist=2*(p**3+1)-r
    assert len(cert['kernels'])==12 and all(k['jacobian_order']==twist and not k['quotient_is_base_field_jacobian'] for k in cert['kernels'])
    sources=[];repair=None
    for phi in E.isogenies_prime_degree(3):
        E1=phi.codomain();E1.set_order(r,num_checks=0)
        nr=len(E1.division_polynomial(3).roots(multiplicities=False))
        if nr==1:
            S,source_order=order_certificate(E1,r);unique=E1.isogenies_prime_degree(3)
            assert len(unique)==1 and unique[0].codomain().is_isomorphic(E)
            assert not unique[0](S).is_zero() and (r*unique[0](S)).is_zero()
            sources.append({'curve':str(E1),'prime_order_certificate':source_order,
                            'unique_degree3_kernel':str(unique[0].kernel_polynomial()),
                            'unique_codomain':str(unique[0].codomain()),
                            'codomain_isomorphic_to_bad_prepared_curve':True,
                            'subgroup_retained':True})
        elif repair is None:
            ks=one_curve(E1,p);good=next(k for k in ks if k['predicts_base_field_jacobian'])
            R=PolynomialRing(GF(p),['X','Y','Z'])
            g=R({ast.literal_eval(e):c for e,c in good['quartic_coefficients'].items()})
            assert not Curve(g).is_singular();target=jacobian_order_certificate(g,p,r)
            assert target['quotient_is_base_field_jacobian']
            repair={'degree3_kernel_from_bad_curve':str(phi.kernel_polynomial()),'curve':str(E1),
                    'target_quartic':str(g),'target_independent_order_certificate':target}
    assert sources and repair
    out={'status':'passed','seed':seed,'p':p,'r':r,'field':field_data(K),
         'bad_prepared_curve':str(E),'prepared_order_certificate':prepared_order,
         'nine_torsion_x_roots':row['nine_torsion_x_roots'],
         'all_12_independent_quotient_certificates':cert,'obstructed_jacobian_order':twist,
         'forced_sources':sources,'repair_from_prepared':repair,'minimum_total_degree3_steps_from_sources':2,
         'seconds':perf_counter()-start,
         'refuted':'Every eligible ordinary non-subfield input can reach a usable prepared quotient in at most one rational degree-three step.',
         'not_refuted':'One extra rational degree-three step always suffices after torsion preparation.',
         'limits':'Minimum is for this degree-three transverse-kernel construction; it is not a lower bound for every possible attack.'}
    (ROOT/'results/forced_two_step_audit.json').write_text(json.dumps(out,indent=2)+'\n')
    print('FORCED TWO STEP AUDIT COMPLETE',p,r,'sources',len(sources),'seconds',out['seconds'],flush=True)


if __name__=='__main__':main()
