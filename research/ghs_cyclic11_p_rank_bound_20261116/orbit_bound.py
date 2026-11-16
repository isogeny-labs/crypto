#!/usr/bin/env python3
"""Exact inertia-signature arithmetic for all three-branch 11-covers."""
from collections import Counter
from pathlib import Path
import hashlib
import json

ell=11
orbits=[(1,3,4,5,9),(2,6,7,8,10)]
assert all({(p*i)%ell for i in range(1,ell)}==set(range(1,ell))
           for p in (3,4,5,9))
assert all({(p*i)%ell for i in orbit}==set(orbit)
           for orbit in orbits for p in (3,4,5,9))
records=[]
for a in range(1,ell):
    for b in range(1,ell):
        c=(-a-b)%ell
        if c==0:
            continue
        sig=[]
        for i in range(1,ell):
            numerator=sum((-i*e)%ell for e in (a,b,c))
            assert numerator in (ell,2*ell)
            sig.append(numerator//ell-1)
        assert sum(sig)==5 and set(sig)<={0,1}
        minima=[min(sig[i-1] for i in orbit) for orbit in orbits]
        bound=sum(len(orbit)*m for orbit,m in zip(orbits,minima))
        assert minima==[0,0] and bound==0
        records.append({'a':a,'b':b,'c':c,'signature':sig,
                        'orbit_minima':minima,'Bouw_bound':bound})
assert len(records)==90
out={'schema':'ghs.cyclic11-three-branch-p-rank-bound/v1',
     'source_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
     'ell':ell,'order_five_residues':[3,4,5,9],
     'frobenius_orbits':[list(o) for o in orbits],
     'parameter_rows':len(records),
     'signature_patterns':len({tuple(r['signature']) for r in records}),
     'bound_histogram':dict(sorted(Counter(r['Bouw_bound'] for r in records).items())),
     'records':records}
target=Path(__file__).with_name('ORBIT_BOUND.json')
assert not target.exists(), 'refusing to overwrite orbit receipt'
target.write_text(json.dumps(out,sort_keys=True,indent=2)+'\n')
print(json.dumps({key:out[key] for key in ('schema','parameter_rows',
      'signature_patterns','bound_histogram')},sort_keys=True))
