#!/usr/bin/env sage -python
"""Check the closed-form V4 identity behind all 15 q16 transfers."""

import hashlib
import json
from pathlib import Path

from sage.all import GF, PolynomialRing


HERE = Path(__file__).resolve().parent
MAP = HERE / "map15_receipt.json"


def main():
    rows = json.loads(MAP.read_text())["rows"]
    k = GF(16, "a", modulus="conway")
    K = GF(4096, "w", modulus="conway")
    embedding = k.embeddings(K)[0]
    a = embedding(k.gen())
    lookup = {embedding(z): z for z in k}
    R = PolynomialRing(K, "x")
    x = R.gen()
    h = a*a*x**4+a*x
    roots = [z for z, _ in h.roots()]
    r = roots[1]
    assert r**3 == 1/a and roots == [K(0),r,r**16,r**256]
    V = R.fraction_field()((x-roots[2])*(x-roots[3]))/((x-roots[0])*(x-roots[1]))
    assert V+1/V == a*r/h
    out = []
    for row in rows:
        lam = K.from_integer(row["lambda_mask"])
        alpha = lam.sqrt()
        delta = alpha*a*r
        assert delta in lookup and delta != 0
        assert lam == delta**2/(a*a*r*r)
        assert K.from_integer(row["b_mask"]) == lam**4
        S = alpha*(V+1/V)
        assert S == delta/h
        even_f = delta*h+a**3*h*h
        assert even_f == h*h*(S+a**3)
        odd_coeffs = [lookup[even_f[8-i]] for i in range(9)]
        while odd_coeffs and odd_coeffs[-1] == 0:
            odd_coeffs.pop()
        assert [int(z.to_integer()) for z in odd_coeffs] == row["base_curve_odd_model_f_coeff_masks"]
        out.append({"b_mask":row["b_mask"],
                    "delta_base_mask":int(lookup[delta].to_integer()),
                    "lambda_mask":row["lambda_mask"]})
    assert len({row["delta_base_mask"] for row in out}) == 15
    receipt = {"schema":"ghs-q16-v4-closed-form-v1",
               "identity":"V+1/V=a*r/h; lambda=delta^2/(a^2*r^2); b=lambda^4; C:y^2+h*y=delta*h+a^3*h^2",
               "field_modulus":str(K.modulus()),
               "base_generator_embedding_mask":int(a.to_integer()),
               "root_mask":int(r.to_integer()),
               "rows":out,
               "map_receipt_sha256":hashlib.sha256(MAP.read_bytes()).hexdigest(),
               "search_sha256":hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    (HERE/"family_formula_receipt.json").write_text(json.dumps(receipt,indent=2)+"\n")
    print("closed-form V4 identity verified for all 15 delta in F16*")


if __name__ == "__main__":
    main()
