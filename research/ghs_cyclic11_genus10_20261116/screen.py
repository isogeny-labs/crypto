#!/usr/bin/env sage -python
"""Exact Kummer point-count screen for six branch-orbit representatives over F3."""
from sage.all import GF
from collections import Counter
from pathlib import Path
import hashlib
import json
from math import isqrt

here = Path(__file__).parent

def cases():
    # Four rational branch points, including infinity.
    yield ('rrrr', ['x', 'x-1', 'x+1'], [(a,b,c)
           for a in range(1,11) for b in range(1,11) for c in range(1,11)
           if (a+b+c)%11])
    # Two rational branch points and one quadratic Frobenius orbit.
    yield ('rrq_fixed', ['x', 'x^2+1'], [(a,b)
           for a in range(1,11) for b in range(1,11) if (a+2*b)%11])
    yield ('rrq_other', ['x', 'x-1', 'x^2+1'], [(a,b,c)
           for a in range(1,11) for b in range(1,11) for c in range(1,11)
           if (a+b+2*c)%11==0])
    # One rational branch point and a cubic Frobenius orbit.
    yield ('rc_fixed', ['x^3-x-1'], [(1,)])
    yield ('rc_other', ['x', 'x^3-x-1'], [(8,1)])
    # Two quadratic Frobenius orbits. Negative exponent denotes a pole.
    yield ('qq', ['x^2+1', 'x^2+x+2'], [(1,-1)])

def field_histograms(r, specs):
    K = GF(3**r, name='v')
    gen = K.multiplicative_generator()
    logs = {}
    value = K(1)
    for i in range(3**r-1):
        logs[value] = i % 11
        value *= gen
    assert len(logs) == 3**r-1 and value == 1
    out = {}
    for name, factors, exponent_vectors in specs:
        histogram = Counter()
        branch_count = 0
        for x in K:
            vals = []
            for factor in factors:
                if factor == 'x': val = x
                elif factor == 'x-1': val = x-1
                elif factor == 'x+1': val = x+1
                elif factor == 'x^2+1': val = x*x+1
                elif factor == 'x^2+x+2': val = x*x+x+2
                elif factor == 'x^3-x-1': val = x**3-x-1
                else: raise AssertionError(factor)
                vals.append(val)
            if any(v == 0 for v in vals):
                branch_count += 1
            else:
                histogram[tuple(logs[v] for v in vals)] += 1
        assert sum(histogram.values())+branch_count == 3**r
        out[name] = (histogram,branch_count)
    return out

specs = list(cases())
assert [len(s[2]) for s in specs] == [910,90,90,1,1,1]
by_degree = {r:field_histograms(r,specs) for r in (5,10)}
records = []
for name,factors,vectors in specs:
    degrees = {'x':1,'x-1':1,'x+1':1,'x^2+1':2,
               'x^2+x+2':2,'x^3-x-1':3}
    for exponents in vectors:
        order_at_infinity = sum(e*degrees[f] for e,f in zip(exponents,factors))%11
        counts = []
        for r in (5,10):
            histogram,finite_branches = by_degree[r][name]
            m = sum(n for logs,n in histogram.items()
                    if sum(e*v for e,v in zip(exponents,logs))%11 == 0)
            infinity = 1 if order_at_infinity else 11
            counts.append(finite_branches+infinity+11*m)
        n5,n10 = counts
        assert (244-n5)%5 == 0 and (59050-n10)%5 == 0
        s1,s2 = (244-n5)//5,(59050-n10)//5
        assert (s1*s1-s2)%2 == 0
        e2 = (s1*s1-s2)//2
        d = s1*s1-4*(e2-486)
        pair = None
        if d >= 0 and isqrt(d)**2 == d and (s1+isqrt(d))%2 == 0:
            pair = sorted([(s1+isqrt(d))//2,(s1-isqrt(d))//2])
            assert all(abs(t)<=31 for t in pair)
        records.append({'case':name,'factors':factors,'exponents':exponents,
                        'N5':n5,'N10':n10,'s1':s1,'e2':e2,
                        'trace_pair':pair})

assert len(records) == 1093
trace_pairs = Counter(tuple(r['trace_pair']) for r in records if r['trace_pair'])
assert all(all(t in (-31,2,-24,-9,9,24,31) for t in pair) for pair in trace_pairs)
print(json.dumps({'schema':'ghs.cyclic11-genus10-screen/v1',
                  'source_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                  'equations':len(records),'trace_pair_histogram':
                  [{'pair':list(k),'count':v} for k,v in sorted(trace_pairs.items())],
                  'records':records},sort_keys=True))
