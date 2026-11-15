#!/usr/bin/env sage -python
"""Independent finite checks of the coprime degree-five base-change theorem."""
import hashlib,json,math
from pathlib import Path
from sage.all import PolynomialRing,QQ
from sage.version import version as sage_version
P=Path(__file__).resolve().parent
R=PolynomialRing(QQ,'X');X=R.gen();rows=[]
for t in (-26,-22,-14,-10,10,14,22,26):
    a,b=2,t
    for s in range(1,11):
        if s>1:a,b=b,t*b-243*a
        f=X**10-b*X**5+3**(5*s)
        irr=bool(f.is_irreducible())
        assert irr==(math.gcd(s,5)==1),(t,s,b,irr)
        rows.append({'source_trace':t,'s':s,'base_change_trace':b,'irreducible':irr})
out={'schema':'ghs-odd5-coprime-base-change-check/v1','sage_version':sage_version,'script_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'rows':rows,'all_coprime_irreducible_and_multiples_five_reducible':True}
target=P/'base_change_check.json'
if target.exists():raise SystemExit('refusing to overwrite immutable receipt')
target.write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps({'checked':len(rows),'all_match':True},indent=2))
