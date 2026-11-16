"""Certify that local 3-adic data alone do not determine quotient coverage."""
import ast,json
from collections import deque
from pathlib import Path
from time import perf_counter
from sage.all import GF,PolynomialRing,Curve,set_random_seed
from obstruction_neighborhoods import invariants,orbit_key
from obstruction_probe import one_curve
from quotient_sweep import parse_curve
from quotient_theta import jacobian_order_certificate
ROOT=Path(__file__).resolve().parent


def main():
    set_random_seed(20261109);start=perf_counter()
    data=json.loads((ROOT/'results/neighborhood_component.json').read_text())
    assert data['component_closed'];rows=data['curves'];lookup={tuple(r['orbit_key']):r for r in rows}
    boundary=[tuple(r['orbit_key']) for r in rows if any(e['three_lines']==1 for e in r['edges'])]
    distance={k:0 for k in boundary};queue=deque(boundary)
    while queue:
        k=queue.popleft()
        for e in lookup[k]['edges']:
            kk=tuple(e['orbit_key'])
            if e['three_lines']==4 and kk not in distance:
                assert kk in lookup;distance[kk]=distance[k]+1;queue.append(kk)
    assert len(distance)==len(rows)
    bad=next(r for r in rows if r['usable_witness'] is None);p=bad['p'];r=bad['r']
    candidates=[a for a in rows if a['usable_witness'] is not None and a['nine_torsion_x_roots']==40
                and distance[tuple(a['orbit_key'])]==distance[tuple(bad['orbit_key'])]]
    good=candidates[0];K=GF(p**3,name='z');E=parse_curve(good['curve'],K);E.set_order(r,num_checks=0)
    ks=one_curve(E,p);kk=next(k for k in ks if k['predicts_base_field_jacobian'])
    R=PolynomialRing(GF(p),['X','Y','Z'])
    g=R({ast.literal_eval(e):c for e,c in kk['quartic_coefficients'].items()})
    assert not Curve(g).is_singular();cert=jacobian_order_certificate(g,p,r)
    assert cert['quotient_is_base_field_jacobian'] and cert['jacobian_order']==r
    a=invariants(parse_curve(bad['curve'],K),p,r,matrix9=True);b=invariants(E,p,r,matrix9=True)
    assert a['frobenius_mod9']==b['frobenius_mod9']==[[8,0],[0,8]]
    out={'status':'passed','p':p,'r':r,'seconds':perf_counter()-start,
         'component_size_up_to_frobenius':len(rows),'component_closed':True,
         'bad_curve':bad['curve'],'good_curve':str(E),'bad_invariants':a,'good_invariants':b,
         'common_distance_from_prepared_boundary':distance[tuple(bad['orbit_key'])],
         'good_curve_usable_count':sum(s['unobstructed'] for s in good['kernels_tested']),
         'good_target_quartic':str(g),'good_target_independent_order_certificate':cert,
         'bad_target_certificates_file':'counterexample_audit.json',
         'node_labels':[{'orbit_key':key,'distance_to_prepared_boundary':d,
                         'nine_torsion_x_roots':lookup[key]['nine_torsion_x_roots'],
                         'usable_count':sum(s['unobstructed'] for s in lookup[key]['kernels_tested'])}
                        for key,d in distance.items()],
         'conclusion':'At fixed field and isogeny class, scalar Frobenius mod 9 and the same boundary distance do not determine coverage.',
         'limits':'No assertion that all higher torsion invariants are insufficient; no universal escape theorem.'}
    (ROOT/'results/structure_audit.json').write_text(json.dumps(out,indent=2)+'\n')
    print('STRUCTURE AUDIT COMPLETE',len(rows),'same-distance candidates',len(candidates),flush=True)


if __name__=='__main__':main()
