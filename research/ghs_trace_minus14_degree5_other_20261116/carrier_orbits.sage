#!/usr/bin/env sage
"""Exact PGL2(F3) coordinate orbits of the twelve trace -14 carriers."""
import hashlib
import itertools
import json
from pathlib import Path

F3=GF(3);R=PolynomialRing(F3,'x');x=R.gen()
expected={
  9735:{9735,44227,64725,90937,124586,144548},
  17371:{17371,21624,92153,100118,156795,159256},
}
orbits=[]
for mask,wanted in expected.items():
    f=x^11+sum(F3((mask//(3^i))%3)*x^i for i in range(11))
    seen=set();maps={}
    for aa,bb,cc,dd in itertools.product(F3,repeat=4):
        if aa*dd-bb*cc==0:continue
        t=tuple(int(v) for v in (aa,bb,cc,dd))
        nt=tuple(int(-v) for v in (aa,bb,cc,dd))
        key=min(t,nt)
        if key in seen:continue
        seen.add(key)
        # y'=(c*x+d)^6*y, with original x=(a*x+b)/(c*x+d).
        transformed=sum(f[i]*(aa*x+bb)^i*(cc*x+dd)^(12-i)
                        for i in range(12))
        if transformed.degree()!=11 or transformed[11]!=1:continue
        target=int(sum(int(transformed[i])*3^i for i in range(11)))
        assert transformed==x^11+sum(F3((target//(3^i))%3)*x^i
                                      for i in range(11))
        maps.setdefault(target,list(key))
    assert len(seen)==24 and set(maps)==wanted
    orbits.append({'representative':mask,'models':sorted(wanted),
                   'pgl2_classes':len(seen),'maps':maps})
assert not (expected[9735]&expected[17371])
out={'schema':'ghs.trace-minus14.carrier-pgl2-orbits/v1',
     'source_sha256':hashlib.sha256(Path(__file__).with_suffix('').read_bytes()).hexdigest(),
     'field':'F3','orbits':orbits,'covered_models':12}
path=Path(__file__).resolve().parent/'CARRIER_ORBITS.json'
assert not path.exists(),'refusing to overwrite receipt'
path.write_text(json.dumps(out,indent=2,sort_keys=True,default=int)+'\n')
print(json.dumps({'covered_models':12,'orbit_sizes':[len(r['models']) for r in orbits]},sort_keys=True))
