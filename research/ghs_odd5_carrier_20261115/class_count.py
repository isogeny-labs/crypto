#!/usr/bin/env sage -python
"""Exhaust ordinary F243 source isomorphism classes and audit eight orders."""
import hashlib,json,collections
from pathlib import Path
from sage.all import GF,PolynomialRing,EllipticCurve,ZZ
from sage.version import version as sage_version
P=Path(__file__).resolve().parent
R=PolynomialRing(GF(3),'x');x=R.gen();K=GF(3**5,'a',modulus=x**5+2*x+1)
source_bytes=(P/'sources2.json').read_bytes();sources=json.loads(source_bytes)['found']
selected_traces={-26,-22,-14,-10,10,14,22,26}
def elem(mask):return K(R([(mask//3**i)%3 for i in range(5)]))
counts=collections.Counter(); rows=[];jgroups=collections.defaultdict(list)
for a2 in (1,2):
    for mask in range(1,3**5):
        E=EllipticCurve(K,[0,a2,0,0,elem(mask)])
        order=int(ZZ(E.cardinality()));t=244-order
        counts[t]+=1;jgroups[str(E.j_invariant())].append((a2,mask,t))
        if t in selected_traces:rows.append([a2,mask,t,order])
assert sum(counts.values())==484
assert len(jgroups)==242 and all(len(v)==2 for v in jgroups.values())
assert all({u[0] for u in v}=={1,2} for v in jgroups.values())
for v in jgroups.values():
    a2,mask,_=v[0]; b2,mask2,_=v[1]
    A=EllipticCurve(K,[0,a2,0,0,elem(mask)])
    B=EllipticCurve(K,[0,b2,0,0,elem(mask2)])
    assert not A.is_isomorphic(B)
    assert v[0][2]==-v[1][2]
assert set(int(t) for t in sources)==selected_traces
for key,row in sources.items():
    a2=row['a2'];b=elem(row['a6_mask']);summed=0
    for u in K:
        v=u**3+a2*u**2+b
        if v==0:continue
        chi=v**((3**5-1)//2)
        assert chi==1 or chi==-1
        summed+=1 if chi==1 else -1
    assert 244+summed==row['order'] and 244-row['order']==int(key)
result={'schema':'ghs-odd5-source-class-count/v1','sage_version':sage_version,'script_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'sources2_sha256':hashlib.sha256(source_bytes).hexdigest(),'normal_form_pairs':484,'nonzero_j_count':len(jgroups),'each_j_has_two_nonisomorphic_twists':True,'all_source_orders_directly_recounted':True,'trace_histogram':{str(t):counts[t] for t in sorted(counts)},'selected_traces':sorted(selected_traces),'selected_class_count':len(rows),'selected_rows':rows}
target=P/'class_count.json'
if target.exists():raise SystemExit('refusing to overwrite immutable class census')
target.write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({'selected_class_count':len(rows),'selected_histogram':{str(t):counts[t] for t in sorted(selected_traces)},'all_source_orders_directly_recounted':True},indent=2))
