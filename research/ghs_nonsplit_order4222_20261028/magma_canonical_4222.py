#!/usr/bin/env python3
"""Normalize the nonsplit V4 cover to a smooth F16 plane quartic in Magma."""
import hashlib
import json
import ssl
import urllib.request
from pathlib import Path

import certifi

HERE=Path(__file__).resolve().parent
URL="https://calculator.magma-maths.org/execute"
source=HERE/"source_4222.json"
c=json.loads(source.read_text())

def z_expr(n):
    return "+".join("1" if i==0 else "z" if i==1 else f"z^{i}"
                    for i in range(24) if (n>>i)&1) or "0"

def k_expr(n):
    return f"k!(L!({z_expr(n)}))"

def polynomial(coeffs,var):
    return "+".join(f"{k_expr(v)}*{var}^{i}"
                    for i,v in enumerate(coeffs) if v) or "0"

lc=c["additive_quartic_coefficients_low_to_high"]
nc=c["rhs_numerator_coefficients_low_to_high"]
dc=c["rhs_denominator_coefficients_low_to_high"]
code=f"""F2 := GF(2);
P<X> := PolynomialRing(F2);
L<z> := ext<F2 | X^24+X^16+X^15+X^14+X^13+X^10+X^9+X^7+X^5+X^3+1>;
k := FiniteField(2,4); Embed(k,L);
A2<x,y> := AffineSpace(k,2);
D := {polynomial(dc,'x')};
N := {polynomial(nc,'x')};
f := D*(y^4 + {k_expr(lc[2])}*y^2 + {k_expr(lc[1])}*y)+N;
C := Curve(A2,f);
print "GEOMETRIC_GENUS"; print Genus(C);
phi := CanonicalMap(C);
Q, hyp := CanonicalImage(C,phi);
print "HYPERELLIPTIC"; print hyp;
print "CANONICAL_QUARTIC"; print Q;
print "NONSINGULAR"; print IsNonsingular(Q);
print "L_POLYNOMIAL"; print LPolynomial(Q);
print "CANONICAL_MAP"; print phi;
FQ := DefiningPolynomial(Q);
print "Q_EXPONENTS"; print [Exponents(m): m in Monomials(FQ)];
print "Q_COEFF_BITS"; print [Eltseq(L!v): v in Coefficients(FQ)];
polys := DefiningPolynomials(phi);
print "MAP_EXPONENTS"; print [[Exponents(m): m in Monomials(g)]: g in polys];
print "MAP_COEFF_BITS"; print [[Eltseq(L!v): v in Coefficients(g)]: g in polys];
"""
input_path=HERE/"canonical_4222.magma"
input_path.write_text(code)
req=urllib.request.Request(URL,json.dumps({"code":code}).encode(),
                           {"Content-Type":"application/json"},method="POST")
with urllib.request.urlopen(req,timeout=300,context=ssl.create_default_context(cafile=certifi.where())) as resp:
    status=resp.status
    payload=json.loads(resp.read())
receipt={"schema":"ghs-nonsplit-magma-canonical-v1","endpoint":URL,
         "source_sha256":hashlib.sha256(source.read_bytes()).hexdigest(),
         "input_sha256":hashlib.sha256(code.encode()).hexdigest(),
         "http_status":status,"response":payload}
(HERE/"canonical_4222_receipt.json").write_text(json.dumps(receipt,indent=2)+"\n")
assert status==200 and payload.get("success") and payload.get("exit_code")==0,payload
out=payload["stdout"]
assert "GEOMETRIC_GENUS\n3\n" in out
assert "HYPERELLIPTIC\nfalse\n" in out
assert "NONSINGULAR\ntrue\n" in out
assert "4096*$.1^6 + 125*$.1^3 + 1" in out
markers=("Q_EXPONENTS","Q_COEFF_BITS","MAP_EXPONENTS","MAP_COEFF_BITS")
def section(key):
    tail=out.split(key+"\n",1)[1]
    for next_key in markers:
        if next_key!=key and "\n"+next_key+"\n" in tail:
            tail=tail.split("\n"+next_key+"\n",1)[0]
    return json.loads(tail.strip())
def bitcode(bits):
    return sum(int(v)<<i for i,v in enumerate(bits))
data={"schema":"ghs-nonsplit-canonical-data-v1",
      "source_order":4222,"quartic_exponents":section("Q_EXPONENTS"),
      "quartic_coefficients":list(map(bitcode,section("Q_COEFF_BITS"))),
      "map_exponents":section("MAP_EXPONENTS"),
      "map_coefficients":[list(map(bitcode,row)) for row in section("MAP_COEFF_BITS")]}
(HERE/"canonical_4222.json").write_text(json.dumps(data,indent=2)+"\n")
print("PASS genus 3, smooth nonhyperelliptic quartic, L-polynomial 1+125T^3+4096T^6")
