#!/usr/bin/env sage -python
"""Freeze first simple F243 source curves for the eight selected trace classes."""
import hashlib,json
from pathlib import Path
from sage.all import GF,PolynomialRing,EllipticCurve,ZZ
from sage.version import version as sage_version
P=Path(__file__).resolve().parent
R=PolynomialRing(GF(3),'x');x=R.gen();K=GF(3**5,'a',modulus=x**5+2*x+1)
traces=[-26,-22,-14,-10,10,14,22,26]
def elem(mask):return K(R([(mask//3**i)%3 for i in range(5)]))
found={}
for mask in range(1,3**5):
    E=EllipticCurve(K,[0,1,0,0,elem(mask)])
    order=ZZ(E.cardinality());t=int(3**5+1-order)
    if t in traces and str(t) not in found:
        found[str(t)]={'a2':1,'a4':0,'a6_mask':mask,'order':int(order),'trace':t,'equation':str(E)}
    if len(found)==len(traces):break
result={'schema':'ghs-odd5-source-search/v1','sage_version':sage_version,'script_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'field_modulus':'x^5 + 2*x + 1','searched_masks_through':mask,'traces':traces,'found':found,'all_found':len(found)==len(traces)}
target=P/'sources.json'
if target.exists():raise SystemExit('refusing to overwrite immutable source receipt')
target.write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result,indent=2))
