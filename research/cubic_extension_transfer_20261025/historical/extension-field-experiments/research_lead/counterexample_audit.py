"""Audit the all-obstructed prepared curve and search a one-step repair.

The counterexample refutes immediate coverage on every prepared curve. If an
additional isogeny repairs it, that does not refute the weaker existential
claim allowing a freely chosen extra step even for already prepared inputs.
"""
import ast,json
from pathlib import Path
from time import perf_counter
from math import isqrt
from sage.all import GF,Integer,PolynomialRing,set_random_seed,Curve
from quotient_sweep import parse_curve
from obstruction_probe import one_curve
from quotient_theta import jacobian_order_certificate
from common import point_data,field_data
ROOT=Path(__file__).resolve().parent


def main():
    set_random_seed(20261103);start=perf_counter()
    search=json.loads((ROOT/'results/fast_counterexample.json').read_text());row=search['curves'][-1]
    assert row['unobstructed_kernel_witness'] is None and len(row['chi18_values_tested'])==12
    p=row['p'];r=row['r'];K=GF(p**3,name='z');E=parse_curve(row['curve'],K)
    assert Integer(p).is_prime() and Integer(r).is_prime()
    assert p%3==1 and r%9==4 and E.j_invariant()**p!=E.j_invariant()
    trace=p**3+1-r;assert trace%p
    P=E.random_point()
    while P.is_zero():P=E.random_point()
    assert (r*P).is_zero()
    lo=p**3+1-isqrt(4*p**3);hi=p**3+1+isqrt(4*p**3)
    assert (lo+r-1)//r==hi//r==1
    E.set_order(r,num_checks=0)
    assert len(E.division_polynomial(3).roots(multiplicities=False))==4
    order9_roots=len(E.division_polynomial(9).roots(multiplicities=False))
    cert=row['independent_certificate'];twist=2*(p**3+1)-r
    assert len(cert['kernels'])==12
    assert all(k['jacobian_order']==twist and not k['quotient_is_base_field_jacobian'] for k in cert['kernels'])
    neighbors=[];repair=None
    for phi in E.isogenies_prime_degree(3):
        E1=phi.codomain();E1.set_order(r,num_checks=0)
        nr=len(E1.division_polynomial(3).roots(multiplicities=False))
        record={'curve':str(E1),'three_kernel_lines':nr,'isogeny_kernel':str(phi.kernel_polynomial())}
        if nr==4:
            ks=one_curve(E1,p);record['unobstructed_quotients']=sum(k['predicts_base_field_jacobian'] for k in ks)
            if repair is None and record['unobstructed_quotients']:
                good=next(k for k in ks if k['predicts_base_field_jacobian'])
                R=PolynomialRing(GF(p),['X','Y','Z'])
                g=R({ast.literal_eval(e):c for e,c in good['quartic_coefficients'].items()})
                assert not Curve(g).is_singular()
                target=jacobian_order_certificate(g,p,r);assert target['quotient_is_base_field_jacobian']
                image=phi(P);assert not image.is_zero() and (r*image).is_zero()
                assert phi(29*P)==29*image
                repair={'neighbor_curve':str(E1),'isogeny_kernel':str(phi.kernel_polynomial()),
                        'degree':3,'image_point':point_data(image),'subgroup_order_preserved':True,
                        'target_quartic':str(g),'target_order_certificate':target,
                        'neighbor_unobstructed_count':record['unobstructed_quotients']}
        neighbors.append(record)
        print('NEIGHBOR',len(neighbors),'3-lines',nr,'good',record.get('unobstructed_quotients'),flush=True)
    out={'status':'passed','p':p,'r':r,'trace':trace,'field':field_data(K),'curve':str(E),
         'ordinary':True,'j_not_in_base_field':True,'three_kernel_lines':4,'nine_torsion_x_roots':order9_roots,
         'prime_order_certificate':{'point':point_data(P),'prime_annihilator':r,'hasse_interval':[lo,hi],
                                    'unique_multiple_of_r':True},
         'all_12_quotient_certificates':cert,'obstructed_jacobian_order':twist,
         'neighbors':neighbors,'one_additional_isogeny_repair':repair,
         'has_nonscalar_three_neighbor':any(k['three_kernel_lines']==1 for k in neighbors),
         'refuted_claim':'Every prepared scalar-minus-I curve has an unobstructed kernel among the twelve.',
         'not_refuted_claim':'At most one freely chosen isogeny can reach some usable prepared model.',
         'seconds':perf_counter()-start}
    (ROOT/'results/counterexample_audit.json').write_text(json.dumps(out,indent=2)+'\n')
    print('AUDIT COMPLETE','repair',repair is not None,'order9roots',order9_roots,flush=True)


if __name__=='__main__':main()
