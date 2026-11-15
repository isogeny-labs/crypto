#!/usr/bin/env sage -python
"""Rabin certificates for the eight odd-characteristic degree-five carrier classes."""
import hashlib,json
from pathlib import Path
from sage.all import GF,PolynomialRing,gcd
from sage.version import version as sage_version
P=Path(__file__).resolve().parent
traces=[-26,-22,-14,-10,10,14,22,26]
prime_by_trace={-26:59,-22:59,-14:19,-10:19,10:19,14:19,22:59,26:59}
rows=[]
for t in traces:
    p=prime_by_trace[t];R=PolynomialRing(GF(p),'X');X=R.gen();f=X**10-t*X**5+3**5
    remainders={d:pow(X,p**d,f) for d in (2,5,10)}
    assert remainders[10]==X
    assert gcd(f,remainders[2]-X)==1 and gcd(f,remainders[5]-X)==1
    assert f.is_irreducible()
    rows.append({'trace':t,'reduction_prime':p,'polynomial_mod_p':str(f),'remainders':{str(d):str(remainders[d]) for d in (2,5,10)},'gcd_10_over_5':'1','gcd_10_over_2':'1','irreducible':True})
result={'schema':'ghs-odd5-carrier-floor/v1','sage_version':sage_version,'script_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'rows':rows,'conclusion':'each five-dimensional Weil restriction is simple; every nonzero algebraic Jacobian image has genus at least five'}
target=P/'floor_cert.json'
if target.exists():raise SystemExit('refusing to overwrite immutable floor certificate')
target.write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({'traces':traces,'primes':prime_by_trace,'all_irreducible':True},indent=2))
