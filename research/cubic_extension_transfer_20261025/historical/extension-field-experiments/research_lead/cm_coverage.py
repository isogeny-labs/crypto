"""Complete coverage screening for a fixed CM order and small-isogeny actions.

The CM polynomial has degree 36; Frobenius identifies the roots in triples.
This is a complete finite experiment for this order, not a universal criterion.
"""
import ast,json
from pathlib import Path
from time import perf_counter
from sage.all import GF,EllipticCurve,hilbert_class_polynomial,PolynomialRing,set_random_seed
from obstruction_neighborhoods import screen,orbit_key
from obstruction_probe import one_curve
from quotient_theta import jacobian_order_certificate
ROOT=Path(__file__).resolve().parent


def main():
    set_random_seed(20261115);start=perf_counter();p=109;r=1293169;K=GF(p**3,name='z')
    H=hilbert_class_polynomial(-21195);roots=H.change_ring(K).roots(multiplicities=False)
    assert len(roots)==H.degree()==36
    models={};rows=[];nonsquare=K(next(a for a in range(2,p) if not GF(p)(a).is_square()))
    for j in roots:
        E=EllipticCurve(j=j)
        if int(E.cardinality())!=r:E=E.quadratic_twist(nonsquare)
        assert int(E.cardinality())==r and len(E.division_polynomial(9).roots(multiplicities=False))==40
        key=orbit_key(E,p)
        if key not in models:models[key]=E
    assert len(models)==12
    for key,E in models.items():
        ss=screen(E,p,complete=True);edges=[]
        for degree in [5,7]:
            for phi in E.isogenies_prime_degree(degree):
                E1=phi.codomain();kk=orbit_key(E1,p);assert kk in models
                edges.append({'degree':degree,'codomain_orbit_key':kk,'kernel':str(phi.kernel_polynomial())})
        rows.append({'orbit_key':key,'j_minimal_polynomial':str(E.j_invariant().minimal_polynomial()),
                     'curve':str(E),'usable_count':sum(s['unobstructed'] for s in ss),'signs':ss,'edges':edges})
    lookup={tuple(row['orbit_key']):row for row in rows};repairs=[]
    for row in rows:
        if row['usable_count']:continue
        good=next((e for e in row['edges'] if lookup[tuple(e['codomain_orbit_key'])]['usable_count']),None)
        if good is None:continue
        E1=models[tuple(good['codomain_orbit_key'])];ks=one_curve(E1,p)
        candidate=next(k for k in ks if k['predicts_base_field_jacobian'])
        R=PolynomialRing(GF(p),['X','Y','Z'])
        g=R({ast.literal_eval(e):c for e,c in candidate['quartic_coefficients'].items()})
        certificate=jacobian_order_certificate(g,p,r);assert certificate['quotient_is_base_field_jacobian']
        repairs.append({'bad_orbit_key':row['orbit_key'],'isogeny':good,'target_quartic':str(g),
                        'independent_order_certificate':certificate,
                        'preserves_three_torsion_type':True,'preserves_endomorphism_order':True})
    out={'status':'passed','p':p,'r':r,'endomorphism_discriminant':-21195,
         'class_polynomial_degree':36,'all_roots_in_cubic_field':True,
         'frobenius_orbits':rows,'one_step_coprime_three_repairs':repairs,'seconds':perf_counter()-start,
         'scope':'Complete screen of all 36 CM roots / 12 Frobenius orbits for this order. Positive repair targets independently order-certified.'}
    (ROOT/'results/cm_coverage.json').write_text(json.dumps(out,indent=2)+'\n')
    print('CM COVERAGE COMPLETE','bad',sum(row['usable_count']==0 for row in rows),'repairs',len(repairs),'seconds',out['seconds'],flush=True)


if __name__=='__main__':main()
