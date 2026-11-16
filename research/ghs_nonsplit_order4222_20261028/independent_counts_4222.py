#!/usr/bin/env sage -python
"""Recount the quartic over F16/F256 and the V4 cover over F4096."""
import hashlib
import json
from pathlib import Path

from sage.all import PolynomialRing
from source_4222 import L, dec, q

HERE = Path(__file__).resolve().parent
canonical_path = HERE / "canonical_4222.json"
source_path = HERE / "source_4222.json"
canonical = json.loads(canonical_path.read_text())
source = json.loads(source_path.read_text())
terms = list(zip(canonical["quartic_exponents"],
                 canonical["quartic_coefficients"]))


def count_quartic(degree):
    field = L.subfield(degree)
    R = PolynomialRing(field, "Y")
    Y = R.gen()
    coeffs = [(ex, field(dec(c))) for ex,c in terms]
    total = 0
    for x in field:
        f = sum((c*x**ex[0]*Y**ex[1] for ex,c in coeffs), R(0))
        total += len(f.roots())
    infinity = sum((c*Y**ex[1] for ex,c in coeffs if ex[2] == 0), R(0))
    total += len(infinity.roots())
    total += int(sum((c for ex,c in coeffs if ex == [0,4,0]), field(0)) == 0)
    return total


n1 = count_quartic(4)
n2 = count_quartic(8)
K = L.subfield(12)
A = dec(source["residue_A"])
t = dec(source["pole_t"])
abar = dec(source["residue_Aq3"])
tf = t**q
good = 0
for x in K:
    xx = L(x)
    f0 = K(A/(xx+t) + abar/(xx+tf))
    f1 = K(A**q/(xx+tf) + abar**q/(xx+t))
    if f0.trace() == 0 and f1.trace() == 0:
        good += 1
n3 = 4*(good+1)  # the four unramified rational points above infinity
assert (n1,n2,n3) == (17,257,4472)
a1 = n1-(q+1)
s2 = q**2+1-n2
a2 = (a1*a1-s2)//2
assert a1 == 0 and a2 == 0
s3 = q**3+1-n3
a3 = -s3//3
assert (a1,a2,a3) == (0,0,125)
out = {
    "schema":"ghs-order4222-independent-point-counts-v1",
    "script_sha256":hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
    "canonical_sha256":hashlib.sha256(canonical_path.read_bytes()).hexdigest(),
    "source_sha256":hashlib.sha256(source_path.read_bytes()).hexdigest(),
    "quartic_F16_points":n1,
    "quartic_F256_points":n2,
    "v4_F4096_affine_split_x_count":good,
    "v4_F4096_points":n3,
    "genus3_l_polynomial_low_to_high":[1,a1,a2,a3,q*a2,q*q*a1,q**3],
}
(HERE / "independent_counts_4222.json").write_text(json.dumps(out,indent=2)+"\n")
print(json.dumps(out,indent=2))
