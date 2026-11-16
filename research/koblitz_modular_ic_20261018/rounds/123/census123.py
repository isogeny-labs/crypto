#!/usr/bin/env python3
"""Source-bound x=0 branch and divisor-degree census for marked23."""
import hashlib
import json
import math
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROUNDS = HERE.parent

def main():
    protocol = json.loads((HERE/'protocol.json').read_text())
    source = {}
    for roundno, expected in protocol['input_sha256'].items():
        path = ROUNDS/roundno/'result.json'
        assert hashlib.sha256(path.read_bytes()).hexdigest() == expected, roundno
        source[int(roundno)] = json.loads(path.read_text())
    base = source[116]['branches']
    assert {k: b['x_order'] for k,b in base.items()} == {
        'y0_e1':1,'y0_e3':3,'y1_e1':1,'y1_e2':2}
    out = {'protocol_id':protocol['id'], 'input_sha256':protocol['input_sha256'],
           'base_places':{}, 'normalized_places_over_f2':0, 'x_zero_divisor_degree':0}
    for key, stage in protocol['selected_stage'].items():
        x_e = base[key]['x_order']
        entries = source[stage]['branches'][key]
        assert [e['alpha'] for e in entries] == [2,3]
        local = []
        for e in entries:
            rows = e['translated_rows']
            assert all(r['s_order'] is not None and r['s_order'] < r['known_until_exclusive'] for r in rows)
            h = e['lower_hull_if_exact']
            assert h is not None and len(h)==4
            exps = [next(v for k,v in p.items() if k.endswith('_exponent')) for p in h]
            assert exps == [0,1,8,16]
            v0,v1,v8 = (rows[i]['s_order'] for i in (0,1,3))
            assert h[0]['s_order']==v0 and h[1]['s_order']==v1 and h[2]['s_order']==v8
            simple = v0-v1
            seven_num = v1-v8
            assert simple>0 and seven_num>0 and math.gcd(seven_num,7)==1
            lead = {j:rows[i]['nonzero_support_below_bound'][0][1] for i,j in ((0,0),(1,1),(3,8))}
            assert lead[0] in (2,3) and lead[1]==lead[8]==1
            local.append({'alpha':e['alpha'], 'simple_correction_order':simple,
                          'seven_correction_order_numerator':seven_num,
                          'seven_correction_order_denominator':7,
                          'seven_edge_relation':'W^7=1',
                          'simple_edge_leading_root':lead[0]})
        assert local[1]['simple_correction_order']==local[0]['simple_correction_order']
        assert local[1]['seven_correction_order_numerator']==local[0]['seven_correction_order_numerator']
        assert local[1]['simple_edge_leading_root']==(local[0]['simple_edge_leading_root']^(local[0]['simple_edge_leading_root']>>1))
        contribution = 2*x_e + 2*7*x_e
        out['base_places'][key]={'source_round':stage,'x_order_on_base':x_e,
           'f4_roots':local,'f2_normalized_places':[
             {'kind':'simple','ramification_over_base':1,'residue_degree':2,'x_order':x_e,'x_divisor_degree':2*x_e},
             {'kind':'tame_seven','ramification_over_base':7,'residue_degree':2,'x_order':7*x_e,'x_divisor_degree':14*x_e}],
           'x_divisor_degree':contribution}
        out['normalized_places_over_f2'] += 2
        out['x_zero_divisor_degree'] += contribution
    assert out['normalized_places_over_f2']==8
    assert out['x_zero_divisor_degree']==112
    dest=Path(sys.argv[1]);dest.mkdir(parents=True,exist_ok=True)
    (dest/'result.json').write_text(json.dumps(out,indent=2,sort_keys=True)+'\n')
    print('CENSUS123_ALL_CHECKS_PASS')

if __name__=='__main__': main()
