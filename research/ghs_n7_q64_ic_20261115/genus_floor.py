#!/usr/bin/env sage -python
"""Rabin certificate for the q64 restriction polynomial modulo 83."""
import hashlib, json
from pathlib import Path
from sage.all import GF, PolynomialRing, gcd
from sage.version import version as sage_version
R=PolynomialRing(GF(83),"X"); X=R.gen()
f=X**14+1226009*X**7+64**7
assert f==X**14+16*X**7+81
rem={d:pow(X,83**d,f) for d in (2,7,14)}
assert rem[14]==X
assert gcd(f,rem[2]-X)==1 and gcd(f,rem[7]-X)==1
assert f.is_irreducible()
result={"schema":"ghs-n7-q64-genus-floor/v1","sage_version":sage_version,"script_sha256":hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),"restriction_polynomial_Z":"X^14 + 1226009*X^7 + 4398046511104","modulus_prime":83,"polynomial_mod_83":str(f),"remainders":{str(d):str(rem[d]) for d in (2,7,14)},"gcd_for_14_over_7":"1","gcd_for_14_over_2":"1","irreducible_mod_83":True,"conclusion":"the seven-dimensional Weil restriction is simple, forcing genus at least seven for every nonzero algebraic Jacobian image"}
path=Path(__file__).with_name("genus_floor.json")
if path.exists():raise SystemExit("refusing to overwrite immutable certificate")
path.write_text(json.dumps(result,indent=2)+"\n")
print(json.dumps(result,indent=2))
