#!/usr/bin/env python3
"""Independently normalize the additive model to a plane quartic in Magma."""
import hashlib
import json
import ssl
import sys
import urllib.request
from pathlib import Path

import certifi

HERE = Path(__file__).resolve().parent
URL = "https://calculator.magma-maths.org/execute"
ORDERS = (3972,4006,4198,4222)


def w_expr(n):
    terms = ["1" if i == 0 else "w" if i == 1 else f"w^{i}"
             for i in range(12) if (n >> i) & 1]
    return "+".join(terms) or "0"


def k_expr(n):
    return f"k!(K!({w_expr(n)}))"


def polynomial(coeffs, var):
    return "+".join(f"{k_expr(v)}*{var}^{i}"
                    for i,v in enumerate(coeffs) if v) or "0"


def one(order):
    source = HERE/f"descent_{order}.json"
    cert = json.loads(source.read_text())
    assert cert["source_order"] == order
    L = cert["additive_quartic_coefficients_low_to_high"]
    N = cert["rhs_numerator_coefficients_low_to_high"]
    D = cert["rhs_denominator_coefficients_low_to_high"]
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
    input_path = HERE/f"canonical_{order}.magma"
    input_path.write_text(code)
    request = urllib.request.Request(URL,json.dumps({"code":code}).encode(),
                                     {"Content-Type":"application/json"},method="POST")
    context = ssl.create_default_context(cafile=certifi.where())
    with urllib.request.urlopen(request,timeout=300,context=context) as response:
        status = response.status
        payload = json.loads(response.read())
    assert status == 200 and payload.get("success") and payload.get("exit_code") == 0
    out = payload["stdout"]
    assert "GEOMETRIC_GENUS\n3\n" in out
    assert "HYPERELLIPTIC\nfalse\n" in out
    assert "NONSINGULAR\ntrue\n" in out
    assert "CANONICAL_QUARTIC\nCurve over GF(2^4) defined by" in out
    t = 4096+1-order
    term = f"+ {abs(t)}*$.1^3" if t < 0 else f"- {t}*$.1^3"
    assert f"4096*$.1^6 {term} + 1" in out
    markers = ("Q_EXPONENTS","Q_COEFF_BITS","MAP_EXPONENTS","MAP_COEFF_BITS")
    def section(key):
        tail = out.split(key+"\n",1)[1]
        for next_key in markers:
            if next_key != key and "\n"+next_key+"\n" in tail:
                tail = tail.split("\n"+next_key+"\n",1)[0]
        return json.loads(tail.strip())
    def bitcode(bits):
        return sum(int(v) << i for i,v in enumerate(bits))
    qexp = section("Q_EXPONENTS")
    qcoef = list(map(bitcode,section("Q_COEFF_BITS")))
    mexp = section("MAP_EXPONENTS")
    mcoef = [[bitcode(bits) for bits in row] for row in section("MAP_COEFF_BITS")]
    assert len(qexp)==len(qcoef) and len(mexp)==len(mcoef)==3
    assert all(len(a)==len(b) for a,b in zip(mexp,mcoef))
    data = {"schema":"ghs-twisted-v4-canonical-data-v1","source_order":order,
            "quartic_exponents":qexp,"quartic_coefficients":qcoef,
            "map_exponents":mexp,"map_coefficients":mcoef,
            "field_modulus":"w^12+w^7+w^6+w^5+w^3+w+1"}
    (HERE/f"canonical_data_{order}.json").write_text(json.dumps(data,indent=2)+"\n")
    receipt = {
        "schema":"ghs-twisted-v4-magma-canonical-v1",
        "endpoint":URL,"script_sha256":hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        "input_file":input_path.name,"input_sha256":hashlib.sha256(code.encode()).hexdigest(),
        "descent_certificate_sha256":hashlib.sha256(source.read_bytes()).hexdigest(),
        "source_order":order,"http_status":status,"response":payload,
    }
    (HERE/f"canonical_{order}_receipt.json").write_text(json.dumps(receipt,indent=2)+"\n")
    print("PASS",order,"genus 3 nonhyperelliptic smooth canonical quartic")


def main():
    orders = ORDERS if len(sys.argv)==1 else tuple(map(int,sys.argv[1:]))
    assert set(orders).issubset(ORDERS)
    for order in orders:
        one(order)


if __name__ == "__main__":
    main()
