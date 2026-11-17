#!/usr/bin/env python3
"""Check frozen inputs, exact retained identities, and replay linkage."""
from hashlib import sha256
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent

def digest(path):
    return sha256(path.read_bytes()).hexdigest()

def main():
    raw = json.loads((HERE/'results.json').read_text())
    replay = json.loads((HERE/'verification.json').read_text())
    prior = json.loads((HERE/'prior_work.json').read_text())
    assert raw['protocol_sha256'] == digest(HERE/'PROTOCOL.md')
    assert raw['script_sha256'] == digest(HERE/'search.py')
    assert replay['screen_sha256'] == digest(HERE/'results.json')
    assert replay['verifier_sha256'] == digest(HERE/'verify.py')
    assert replay['closure_sha256'] == raw['closure_sha256'] == digest(
        ROOT/'ghs_cubic_hess_closure_q16_20261027/certificate.json')
    assert replay['catalog_sha256'] == digest(
        ROOT/'ghs_automorphism_screen_q16_20261026/catalog_models_raw.json')
    assert raw['draw_count'] == raw['tally']['drawn'] == 30000
    assert raw['tally'] == dict(drawn=30000,N1_17=2854,N2_257=156,
                                smooth=151,integral_c3=151,target_match=8)
    assert raw['target_orders'] == [3974,3986,3998,4006,4022,4034,
                                    4054,4078,4126,4166,4174,4178,4198]
    expected = [(4006,2003,-91),(4054,2027,-43),
                (4078,2039,-19),(4126,2063,29)]
    assert [(w['source_order'],w['subgroup_prime'],
             w['weil_polynomial_coefficients'][3]) for w in raw['witnesses']] == expected
    assert len(replay['checked']) == len(expected)
    for w,v in zip(raw['witnesses'],replay['checked']):
        assert w['source_order'] == v['source_order'] == v['jacobian_order']
        assert w['source_order'] == 2*v['subgroup_order']
        r=v['subgroup_order']
        assert r>2 and all(r%d for d in range(2,int(r**0.5)+1))
        assert w['point_counts'] == v['point_counts']
        assert w['catalog_label'] == v['catalog_label']
        assert v['smooth'] and v['rational_point_classes_reach_prime_subgroup']
        assert v['known_scalar'] == 7
        assert v['evaluated_source_to_jacobian_map'] is False
    assert [w['seen_order_in_prior_screen'] for w in raw['witnesses']] == \
        [False,False,False,True]
    assert [p['source_order'] for p in prior['candidates']] == \
        [w['source_order'] for w in raw['witnesses']]
    for p,w,v in zip(prior['candidates'],raw['witnesses'],replay['checked']):
        assert p['label'] == w['catalog_label']
        assert p['catalog_model_count'] == v['catalog_stored_models']
        assert p['catalog_models_hyperelliptic_only']
        assert p['prior_local_quartic_order'] == w['seen_order_in_prior_screen']
        assert p['exact_equation_publication_status'].startswith('unresolved')
    print('PASS: frozen search, independent replay, four exact witness identities')

if __name__ == '__main__':
    main()
