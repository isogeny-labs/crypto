#!/usr/bin/env python3
"""Recompute the exact GF(8) direct-cover order-closure partition."""

import hashlib
import json
from collections import Counter, defaultdict
from pathlib import Path

ROOT=Path(__file__).resolve().parent
PRIOR=ROOT.parent/'ghs_binary_cubic_isogeny_20261026'
SOURCE=PRIOR/'results_q8.json'
SOURCE_REPLAY=PRIOR/'verification_q8.json'
NEW_REPLAY=ROOT/'verification.json'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    source=json.loads(SOURCE.read_text())
    previous=json.loads(SOURCE_REPLAY.read_text())
    new=json.loads(NEW_REPLAY.read_text())
    assert previous['screen_sha256']==sha(SOURCE)
    assert previous['source_models_recounted_total']==1022
    assert new['source_sha256']==sha(SOURCE)
    assert len(source['source_population'])==1022
    by_order=defaultdict(list)
    for row in source['source_population']:
        by_order[row['order']].append(row)
    direct_orders=sorted({row['order'] for row in source['source_population']
                          if row['direct_class']=='B'})
    inside=[row for row in source['source_population']
            if row['order'] in direct_orders]
    outside=[row for row in source['source_population']
             if row['order'] not in direct_orders]
    assert len(direct_orders)==12
    assert len(inside)==324 and len(outside)==698
    assert Counter(row['direct_class'] for row in inside)==Counter(B=126,C=198)
    assert Counter(row['direct_class'] for row in outside)==Counter(C=684,A=14)
    selected=[]
    for record in new['records']:
        order=record['group_order']
        rows=by_order[order]
        assert order not in direct_orders
        assert all(row['direct_class']=='C' for row in rows)
        assert len(rows)==record['source_model_count']
        selected.append({'order':order,'source_models':len(rows),
                         'source_a_b':[[row['a'],row['b']] for row in rows]})
    assert sum(x['source_models'] for x in selected)==54
    certificate={'schema':'ghs-q8-direct-order-closure/v1',
                 'source_sha256':sha(SOURCE),
                 'source_replay_sha256':sha(SOURCE_REPLAY),
                 'new_replay_sha256':sha(NEW_REPLAY),
                 'script_sha256':sha(Path(__file__)),
                 'normalized_ordinary_models':1022,
                 'direct_B_models':126,
                 'direct_B_orders':direct_orders,
                 'inside_direct_B_order_closure':len(inside),
                 'outside_direct_B_order_closure':len(outside),
                 'inside_class_counts':dict(Counter(row['direct_class'] for row in inside)),
                 'outside_class_counts':dict(Counter(row['direct_class'] for row in outside)),
                 'new_structural_source_classes':selected}
    (ROOT/'bounds_certificate.json').write_text(json.dumps(certificate,
                                                        indent=2,sort_keys=True)+'\n')
    print('DIRECT CLOSURE',len(inside),'OUTSIDE',len(outside),
          'NEW SOURCE CLASSES',len(selected))


if __name__=='__main__':
    main()
