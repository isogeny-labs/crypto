#!/usr/bin/env python3
"""Regression and group-law checks for the companion-quotient toy pilot."""
import random
import run as s

EXPECTED = ((101,0,8,53,6,19,0.991035,1.007457),
            (211,2,9,109,6,19,0.95,0.952605),
            (307,10,1,313,10,51,0.982836,0.985805))

def check():
    result=s.run()
    assert result['seed']==20261118 and result['nulls']==128
    for c, exp in zip(result['cases'],EXPECTED):
        assert (c['p'],c['a'],c['b'],c['prime_subgroup_order'],c['B'],c['coverage'],
                c['coverage_ratio_uniform'],c['coverage_ratio_cover_image'])==exp
        rng=random.Random(c['p']+20261118)
        for curve in (s.EC(c['p'],c['a'],c['b']),s.EC(c['p'],c['b'],c['a'])):
            pts=curve.all_points()
            for _ in range(256):
                A,B,C=(rng.choice(pts) for _ in range(3))
                assert curve.add(curve.add(A,B),C)==curve.add(A,curve.add(B,C))
                if A is not None:
                    assert curve.add(A,(A[0],(-A[1])%curve.p)) is None
    print('PASS: 3 frozen cells + 1536 elliptic group-law triples + inverses')

if __name__=='__main__': check()
