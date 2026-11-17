#!/usr/bin/env sage -python
"""Exact c=1 Hess-genus partition of the frozen GF(4096) source census."""
from collections import Counter
from hashlib import sha256
import json
from pathlib import Path

from sage.all import GF

HERE=Path(__file__).resolve().parent
PRIOR=HERE.parent/'ghs_cubic_hess_closure_q16_20261027/certificate.json'


def digest(path):
    return sha256(path.read_bytes()).hexdigest()


def main():
    prior=json.loads(PRIOR.read_text())
    assert prior['schema'].startswith('ghs-')
    K=GF(4096,'w',modulus='conway')
    assert str(K.modulus())=='x^12 + x^7 + x^6 + x^5 + x^3 + x + 1'
    by_a={0:[],32:[]}
    total=Counter()
    residual=Counter()
    per_order=[]
    subfield_residual=[]
    for source in prior['orders']:
        counts=Counter()
        for atext,parts in source['by_a'].items():
            a=int(atext)
            for part in ('hess','outside'):
                for b in parts[part]:
                    b=int(b)
                    assert 1<=b<=4095
                    z=K.from_integer(b)
                    delta=z.sqrt()
                    relative_trace=delta+delta**16+delta**256
                    assert relative_trace**16==relative_trace
                    # Relative trace commutes with Frobenius square.
                    assert relative_trace**2==z+z**16+z**256
                    if z**16==z:
                        genus=1
                        assert relative_trace==delta
                    elif relative_trace==0:
                        genus=3
                    else:
                        genus=4
                    by_a[a].append(b)
                    counts[genus]+=1
                    total[genus]+=1
                    if not source['in_hess_isogeny_closure']:
                        residual[genus]+=1
                        if genus==1:
                            subfield_residual.append({'source_order':source['order'],
                                                      'a2':a,'a6':b})
        assert sum(counts.values())==source['source_models']
        per_order.append({'source_order':source['order'],
                          'source_models':source['source_models'],
                          'outside_genus_three_hess_isogeny_closure':
                              not source['in_hess_isogeny_closure'],
                          'c1_genus_counts':{str(g):counts[g] for g in (1,3,4)}})
    assert all(sorted(by_a[a])==list(range(1,4096)) for a in (0,32))
    assert total==Counter({1:30,3:510,4:7650})
    assert residual==Counter({1:10,4:2865})
    assert len(subfield_residual)==10
    assert all(K.from_integer(z['a6'])**16==K.from_integer(z['a6'])
               for z in subfield_residual)
    out={'schema':'ghs-q16-c1-hess-genus-census/v1',
         'source_closure_sha256':digest(PRIOR),
         'script_sha256':digest(Path(__file__)),
         'field_modulus':'x^12+x^7+x^6+x^5+x^3+x+1',
         'subfield_size':16,'extension_degree':3,
         'normalized_a2':[0,32],
         'model_count':8190,
         'c1_genus_counts':{str(g):total[g] for g in (1,3,4)},
         'outside_genus_three_hess_isogeny_closure_count':2875,
         'outside_closure_c1_genus_counts':
             {str(g):residual[g] for g in (1,3,4)},
         'outside_closure_subfield_sources':subfield_residual,
         'per_order':per_order}
    (HERE/'c1_certificate.json').write_text(json.dumps(out,indent=2)+'\n')
    print('ALL',dict(total),'RESIDUAL',dict(residual),
          'SUBFIELD_RESIDUAL',subfield_residual)

if __name__=='__main__':
    main()
