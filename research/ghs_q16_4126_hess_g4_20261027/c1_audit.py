#!/usr/bin/env sage -python
"""Alternate trace-on-b replay of the exact Hess c=1 genus census."""
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
    source=json.loads(PRIOR.read_text())
    cert=json.loads((HERE/'c1_certificate.json').read_text())
    assert cert['schema']=='ghs-q16-c1-hess-genus-census/v1'
    assert cert['source_closure_sha256']==digest(PRIOR)
    assert cert['script_sha256']==digest(HERE/'c1_census.py')
    K=GF(4096,'w',modulus='conway')
    totals=Counter()
    residual=Counter()
    per_order=[]
    subfield=[]
    for row in source['orders']:
        counts=Counter()
        for atext,partitions in row['by_a'].items():
            for status in ('hess','outside'):
                for b in partitions[status]:
                    z=K.from_integer(int(b))
                    # This independently uses Tr(b), rather than sqrt(b).
                    if z**16==z:
                        genus=1
                    elif z+z**16+z**256==0:
                        genus=3
                    else:
                        genus=4
                    totals[genus]+=1
                    counts[genus]+=1
                    if not row['in_hess_isogeny_closure']:
                        residual[genus]+=1
                        if genus==1:
                            subfield.append({'source_order':row['order'],
                                             'a2':int(atext),'a6':int(b)})
        per_order.append({'source_order':row['order'],
                          'source_models':row['source_models'],
                          'outside_genus_three_hess_isogeny_closure':
                              not row['in_hess_isogeny_closure'],
                          'c1_genus_counts':{str(g):counts[g] for g in (1,3,4)}})
    assert cert['model_count']==8190
    assert cert['c1_genus_counts']=={str(g):totals[g] for g in (1,3,4)}
    assert cert['outside_closure_c1_genus_counts']==\
        {str(g):residual[g] for g in (1,3,4)}
    assert cert['per_order']==per_order
    assert cert['outside_closure_subfield_sources']==subfield
    assert totals==Counter({1:30,3:510,4:7650})
    assert residual==Counter({1:10,4:2865})
    path=HERE/'c1_controls.magma'
    receipt=json.loads((HERE/'c1_controls_receipt.json').read_text())
    assert receipt['input_sha256']==digest(path)
    assert receipt['http_status']==200
    answer=receipt['response']
    assert answer['success'] and answer['exit_code']==0
    assert answer['truncated'] is False
    text=answer['stdout']
    assert 'control_b 14 source_order 4002 expected_genus 3 magma_genus 3' in text
    assert 'control_b 63 source_order 4126 expected_genus 4 magma_genus 4' in text
    assert 'control_b 372 source_order 3982 expected_genus 1 magma_genus 1' in text
    print('PASS: 8190-model c=1 genus partition and three Magma controls')

if __name__=='__main__':
    main()
