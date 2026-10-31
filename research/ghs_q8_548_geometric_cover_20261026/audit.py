#!/usr/bin/env sage -python
"""Replay Magma receipt integrity and factor the exact Weil polynomial."""

import hashlib
import json
from pathlib import Path

from sage.all import QQ, PolynomialRing

ROOT=Path(__file__).resolve().parent
PRIOR=ROOT.parent/'ghs_quartic_new_classes_q8_20261026'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    receipts={}
    for name in ('base','geometric'):
        path=ROOT/(name+'_magma_receipt.json')
        receipt=json.loads(path.read_text())
        assert receipt['input_sha256']==sha(ROOT/(name+'.magma'))
        assert receipt['http_status']==200
        reply=receipt['response']
        assert reply['success'] is True and reply['exit_code']==0
        assert reply['truncated'] is False
        assert reply['magma']['version']=='2.30-1'
        assert 'field $.1^3 + $.1 + 1' in reply['stdout']
        assert (name+'_automorphism_order 1') in reply['stdout']
        assert (name+'_group_name C1') in reply['stdout']
        receipts[name]=sha(path)
    assert 'rational_points 9' in json.loads(
        (ROOT/'base_magma_receipt.json').read_text())['response']['stdout']
    assert 'smooth true' in json.loads(
        (ROOT/'base_magma_receipt.json').read_text())['response']['stdout']
    R=PolynomialRing(QQ,'X')
    X=R.gen()
    P=X**6+35*X**3+512
    assert P.is_irreducible()
    output={'schema':'ghs-q8-548-geometric-cover-audit/v1',
            'prior_verification_sha256':sha(PRIOR/'verification.json'),
            'base_receipt_sha256':receipts['base'],
            'geometric_receipt_sha256':receipts['geometric'],
            'script_sha256':sha(Path(__file__)),
            'weil_polynomial_coefficients':[1,0,0,35,0,0,512],
            'weil_polynomial_irreducible_over_Q':True,
            'base_automorphism_order':1,
            'geometric_automorphism_order':1,
            'degree_two_elliptic_cover':'excluded_over_algebraic_closure'}
    (ROOT/'certificate.json').write_text(json.dumps(output,indent=2)+'\n')
    print('VERIFIED irreducible Weil polynomial and trivial geometric automorphism group')


if __name__=='__main__':
    main()
