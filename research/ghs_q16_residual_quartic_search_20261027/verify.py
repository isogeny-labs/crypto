#!/usr/bin/env sage -python
"""Independent alternate-chart and subgroup replay of retained quartics."""

from hashlib import sha256
import importlib.util
import json
from pathlib import Path

from sage.all import EllipticCurve, GF


HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
PARENT = ROOT/'ghs_binary_cubic_isogeny_q16_20261026'
CATALOG = ROOT/'ghs_automorphism_screen_q16_20261026/catalog_models_raw.json'
CLOSURE = ROOT/'ghs_cubic_hess_closure_q16_20261027/certificate.json'
RAW = HERE/'results.json'
parent_spec = importlib.util.spec_from_file_location(
    'ghs_q16_parent_quartic_verify', PARENT/'verify.py')
parent_verify = importlib.util.module_from_spec(parent_spec)
parent_spec.loader.exec_module(parent_verify)
quartic = parent_verify.quartic
count_alternate_chart = parent_verify.count_alternate_chart


def digest(path):
    return sha256(path.read_bytes()).hexdigest()


def coords(P):
    return [int(P[0].to_integer()), int(P[1].to_integer())]


def main():
    raw = json.loads(RAW.read_text())
    assert raw['schema']=='ghs-q16-residual-quartic-search/v1'
    assert raw['draw_count']==raw['tally']['drawn']==30000
    closure = json.loads(CLOSURE.read_text())
    catalog = json.loads(CATALOG.read_text())
    source_by_order = {z['order']:z for z in closure['orders']}
    catalog_by_label = {z['label']:z for z in catalog['rows']}
    k = GF(16,'a',modulus='conway')
    L2 = GF(256,'u',modulus='conway')
    K = GF(4096,'w',modulus='conway')
    fields = [(k,lambda x:x),(L2,k.embeddings(L2)[0]),
              (K,k.embeddings(K)[0])]
    checked = []
    for witness in raw['witnesses']:
        order = witness['source_order']
        source = source_by_order[order]
        assert not source['in_hess_isogeny_closure']
        assert source['catalog_has_jacobian']==1
        assert source['largest_prime']==witness['subgroup_prime']
        assert source['catalog_label']==witness['catalog_label']
        cat = catalog_by_label[witness['catalog_label']]
        assert int(cat['hyp_count'])>0 and int(cat['has_jacobian'])==1
        assert all(x.startswith('y^2+') for x in
                   cat['curves'].strip('{}').split(','))
        counts = []
        for field, embed in fields:
            C = quartic(field,k,embed,witness['coefficients'])
            assert C.is_smooth()
            counts.append(count_alternate_chart(
                field,k,embed,witness['coefficients']))
        assert counts==witness['point_counts']
        assert (counts[2]-4097)%3==0
        c3=(counts[2]-4097)//3
        assert order==4097+c3
        assert witness['weil_polynomial_coefficients']==[1,0,0,c3,0,0,4096]
        b_code=source['by_a']['32']['outside'][0]
        a2,a6=K.from_integer(32),K.from_integer(b_code)
        E=EllipticCurve(K,[K.one(),a2,K.zero(),K.zero(),a6])
        assert E.cardinality()==order==2*source['largest_prime']
        r=source['largest_prime']
        G=next((2*P for P in E.points() if not (2*P).is_zero()),None)
        assert G is not None and r*G==E(0)
        Q=7*G
        assert Q!=E(0) and r*Q==E(0)
        C=quartic(k,k,lambda x:x,witness['coefficients'])
        points=C.rational_points()
        assert len(points)==17
        base=points[0].place().divisor()
        J=C.jacobian(model='hess',base_div=3*base).group()
        assert J.order()==order
        classes=[J.point(P.place().divisor()-base) for P in points[1:]]
        reach=any(not (2*D).is_zero() for D in classes)
        rec={
            'source_order':order,
            'draw_index':witness['draw_index'],
            'point_counts':counts,
            'smooth':True,
            'catalog_label':witness['catalog_label'],
            'catalog_stored_models':int(cat['hyp_count']),
            'catalog_models_hyperelliptic_only':True,
            'source_a2':32,'source_a6':b_code,
            'source_models':source['source_models'],
            'subgroup_order':r,'cofactor':2,
            'G':coords(G),'Q':coords(Q),'known_scalar':7,
            'jacobian_order':int(J.order()),
            'rational_point_classes_reach_prime_subgroup':reach,
            'evaluated_source_to_jacobian_map':False,
        }
        checked.append(rec)
        print('VERIFIED',order,'G',rec['G'],'Q',rec['Q'],flush=True)
    output={'schema':'ghs-q16-residual-quartic-replay/v1',
            'screen_sha256':digest(RAW),
            'verifier_sha256':digest(Path(__file__)),
            'closure_sha256':digest(CLOSURE),
            'catalog_sha256':digest(CATALOG),
            'checked':checked}
    (HERE/'verification.json').write_text(json.dumps(output,indent=2)+'\n')
    print('FINAL',[x['source_order'] for x in checked],flush=True)


if __name__=='__main__':
    main()
