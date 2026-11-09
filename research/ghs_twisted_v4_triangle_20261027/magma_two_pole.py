#!/usr/bin/env python3
"""Retain Magma canonical model and map for the two-pole witness."""
import hashlib
import json
import ssl
import urllib.request
from pathlib import Path

import certifi

from magma_canonical import k_expr, polynomial

HERE = Path(__file__).resolve().parent
URL = "https://calculator.magma-maths.org/execute"
source = HERE/"two_pole_4132.json"
c = json.loads(source.read_text())
L,N,D = (c[k] for k in ("additive_quartic_coefficients_low_to_high",
                          "rhs_numerator_coefficients_low_to_high",
                          "rhs_denominator_coefficients_low_to_high"))
code = f"""F2 := GF(2);
P<X> := PolynomialRing(F2);
K<w> := ext<F2 | X^12+X^7+X^6+X^5+X^3+X+1>;
k := FiniteField(2,4); Embed(k,K);
A2<x,y> := AffineSpace(k,2);
D := {polynomial(D,'x')};
N := {polynomial(N,'x')};
f := D*(y^4 + {k_expr(L[2])}*y^2 + {k_expr(L[1])}*y)+N;
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
print "Q_COEFF_BITS"; print [Eltseq(K!v): v in Coefficients(FQ)];
polys := DefiningPolynomials(phi);
print "MAP_EXPONENTS"; print [[Exponents(m): m in Monomials(g)]: g in polys];
print "MAP_COEFF_BITS"; print [[Eltseq(K!v): v in Coefficients(g)]: g in polys];
"""
input_path=HERE/"canonical_two_pole_4132.magma"
input_path.write_text(code)
req=urllib.request.Request(URL,json.dumps({"code":code}).encode(),
                           {"Content-Type":"application/json"},method="POST")
with urllib.request.urlopen(req,timeout=300,context=ssl.create_default_context(cafile=certifi.where())) as response:
    status=response.status
    payload=json.loads(response.read())
assert status == 200 and payload.get("success") and payload.get("exit_code") == 0
out=payload["stdout"]
assert "GEOMETRIC_GENUS\n3\n" in out
assert "HYPERELLIPTIC\nfalse\n" in out
assert "NONSINGULAR\ntrue\n" in out
assert "4096*$.1^6 + 35*$.1^3 + 1" in out
markers=("Q_EXPONENTS","Q_COEFF_BITS","MAP_EXPONENTS","MAP_COEFF_BITS")
def section(key):
    tail=out.split(key+"\n",1)[1]
    for next_key in markers:
        if next_key != key and "\n"+next_key+"\n" in tail:
            tail=tail.split("\n"+next_key+"\n",1)[0]
    return json.loads(tail.strip())
def bitcode(bits):
    return sum(int(v)<<i for i,v in enumerate(bits))
data={"schema":"ghs-two-pole-canonical-data-v1",
      "source_b":c["source_b"],"source_order":4132,
      "quartic_exponents":section("Q_EXPONENTS"),
      "quartic_coefficients":list(map(bitcode,section("Q_COEFF_BITS"))),
      "map_exponents":section("MAP_EXPONENTS"),
      "map_coefficients":[list(map(bitcode,row)) for row in section("MAP_COEFF_BITS")]}
(HERE/"canonical_two_pole_4132.json").write_text(json.dumps(data,indent=2)+"\n")
receipt={"schema":"ghs-two-pole-magma-canonical-v1","endpoint":URL,
         "input_sha256":hashlib.sha256(code.encode()).hexdigest(),
         "source_sha256":hashlib.sha256(source.read_bytes()).hexdigest(),
         "http_status":status,"response":payload}
(HERE/"canonical_two_pole_4132_receipt.json").write_text(json.dumps(receipt,indent=2)+"\n")
print("PASS genus 3 smooth nonhyperelliptic quartic; L-polynomial 1+35T^3+4096T^6")
