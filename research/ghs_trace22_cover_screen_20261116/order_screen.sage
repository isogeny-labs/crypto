#!/usr/bin/env sage
"""Exact extension orders for the trace +22 elliptic isogeny class."""
import hashlib
import json
from pathlib import Path

q=ZZ(243)
t=ZZ(22)
traces=[ZZ(2),t]
for s in range(2,14):
    traces.append(t*traces[-1]-q*traces[-2])
rows=[]
for s in (7,11,13):
    order=q^s+1-traces[s]
    factors=[(int(p),int(e)) for p,e in factor(order)]
    assert prod(ZZ(p)^e for p,e in factors)==order
    assert all(is_prime(p) for p,e in factors)
    r=max(p for p,e in factors)
    rows.append({'s':int(s),'small_field':int(3^s),'extension_field':int(q^s),
                 'trace':int(traces[s]),'source_order':int(order),
                 'prime_factors':factors,'largest_prime':r,
                 'largest_prime_bits':ZZ(r).nbits()})
print(json.dumps({'schema':'ghs.odd5.trace22-extension-order-screen/v1',
                  'script_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
                  'base_q':int(q),'base_trace':int(t),'rows':rows},sort_keys=True))
