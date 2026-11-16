#!/usr/bin/env sage -python
"""Exact three-branch cyclic-11 genus-five factor screen over F5."""
from collections import Counter
from pathlib import Path
import hashlib
import json
from sage.all import GF

F=GF(5**5,name='u')
q=int(F.cardinality())
assert q==3125 and (q-1)%11==0
generator=F.multiplicative_generator()
logmod={}
v=F.one()
for exponent in range(q-1):
    assert v not in logmod
    logmod[v]=exponent%11
    v*=generator
assert v==F.one() and len(logmod)==q-1

hist=Counter()
for x in F:
    if x==0 or x==1:
        continue
    hist[(logmod[x],logmod[x-1])]+=1
assert sum(hist.values())==q-2

descended={s**5-25*s**3+125*s for s in range(-4,5)}
records=[]
for a in range(1,11):
    for b in range(1,11):
        if (a+b)%11==0:
            continue
        residues=sum(count for (i,j),count in hist.items()
                     if (a*i+b*j)%11==0)
        n5=3+11*residues
        assert (3126-n5)%5==0
        trace=(3126-n5)//5
        assert trace*trace<=4*q
        records.append({'a':a,'b':b,'residue_x':residues,
                        'N5':n5,'trace_t':trace,
                        'ordinary_elliptic_eligible':trace%5!=0,
                        'descended_from_F5':trace in descended})
assert len(records)==90
out={'schema':'ghs.cyclic11-f5-three-branch/v1',
     'source_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
     'field_modulus':[int(c) for c in F.modulus().list()],
     'generator':[int(c) for c in generator.polynomial().list()],
     'q':q,'equations':len(records),
     'histogram':[[i,j,hist.get((i,j),0)] for i in range(11) for j in range(11)],
     'trace_histogram':dict(sorted(Counter(r['trace_t'] for r in records).items())),
     'descended_fifth_traces':sorted(descended),'records':records}
target=Path(__file__).with_name('SCREEN.json')
assert not target.exists(), 'refusing to overwrite finite screen'
target.write_text(json.dumps(out,sort_keys=True,indent=2)+'\n')
print(json.dumps({'schema':out['schema'],'equations':out['equations'],
                  'trace_histogram':out['trace_histogram'],
                  'ordinary_nondescended':sum(r['ordinary_elliptic_eligible']
                                              and not r['descended_from_F5']
                                              for r in records)},sort_keys=True))
