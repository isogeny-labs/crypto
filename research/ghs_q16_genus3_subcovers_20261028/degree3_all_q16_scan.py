#!/usr/bin/env sage -python
"""Screen all cataloged ordinary q16 genus-three Weil classes for degree-three covers.

The residue systems are necessary conditions; exact identities are checked for hits.
"""

import hashlib
import json
from collections import Counter
from itertools import permutations
from pathlib import Path

from sage.all import GF, LaurentSeriesRing, PolynomialRing, sage_eval


HERE = Path(__file__).resolve().parent
CATALOG = HERE / "catalog_all_q16_weil_classes.jsonl"


def mask(z):
    return int(z.to_integer())


def linear_basis(columns):
    pivots = {}
    for j, column in enumerate(columns):
        v, code = column, 1 << j
        while v:
            bit = v.bit_length() - 1
            if bit not in pivots:
                pivots[bit] = (v, code)
                break
            w, c = pivots[bit]
            v ^= w
            code ^= c
        else:
            raise AssertionError("residue matrix lacks full column rank")
    assert len(pivots) == 24
    return pivots


def solve(pivots, rhs):
    code = 0
    while rhs:
        bit = rhs.bit_length() - 1
        if bit not in pivots:
            return None
        v, c = pivots[bit]
        rhs ^= v
        code ^= c
    return code


def main():
    raw = CATALOG.read_bytes()
    catalog = [json.loads(s) for s in raw.splitlines()]
    k = GF(16, "a", modulus="conway")
    K = GF(4096, "w", modulus="conway")
    embed = k.embeddings(K)[0]
    R2 = PolynomialRing(k, names=("x", "y"))
    x, y = R2.gens()
    RK = PolynomialRing(K, "x")
    xx = RK.gen()
    F = RK.fraction_field()
    L = LaurentSeriesRing(K, "s", default_prec=7)
    s = L.gen()
    patterns = sorted(set(permutations((2, 1, -2, -1))))
    cached = {}
    hits = []
    counts = Counter()
    for row in catalog:
        for index, equation in enumerate(row["curves"]):
            counts["models"] += 1
            left, right = equation.split("=")
            p = sage_eval(left, locals={"a": k.gen(), "x": x, "y": y}) - sage_eval(
                right, locals={"a": k.gen(), "x": x, "y": y})
            h0, f0 = R2(p.coefficient({y: 1})), R2(-p.coefficient({y: 0}))
            h = RK([embed(h0.coefficient({x: i})) for i in range(h0.degree(x) + 1)])
            f = RK([embed(f0.coefficient({x: i})) for i in range(f0.degree(x) + 1)])
            key = tuple(mask(z) for z in h.list())
            if key not in cached:
                roots = [z for z, multiplicity in h.roots() if multiplicity == 1]
                assert h.degree() == 4 and len(roots) == 4
                systems = []
                for exponents in patterns:
                    U = F(1)
                    for z, e in zip(roots, exponents):
                        U *= F(xx-z)**e
                    V = 1/U
                    local = []
                    for z in roots:
                        us, vs = L(U(x=s+z)), L(V(x=s+z))
                        local.append((us[-2], us[-1], vs[-2], vs[-1]))

                    def residues(lam, mu):
                        return [lam*u1+mu*v1+(lam*u2+mu*v2).sqrt()
                                for u2, u1, v2, v1 in local]

                    columns = []
                    for j in range(24):
                        basis = K.from_integer(1 << (j % 12))
                        values = residues(basis, K(0)) if j < 12 else residues(K(0), basis)
                        columns.append(sum(((mask(value) >> i) & 1) << (12*n+i)
                                           for n, value in enumerate(values) for i in range(12)))
                    systems.append((exponents, U, linear_basis(columns)))
                cached[key] = (roots, systems)
            roots, systems = cached[key]
            residues = [f.derivative()(z)/h.derivative()(z)**2
                        +(f(z)/h.derivative()(z)**2).sqrt() for z in roots]
            assert all(z != 0 for z in residues)
            rhs = sum(((mask(value) >> i) & 1) << (12*n+i)
                      for n, value in enumerate(residues) for i in range(12))
            for exponents, U, pivots in systems:
                answer = solve(pivots, rhs)
                if answer is None:
                    continue
                lam = K.from_integer(answer & 4095)
                mu = K.from_integer(answer >> 12)
                if not lam or not mu:
                    counts["degenerate_solutions"] += 1
                    continue
                beta, b = lam*mu, (lam*mu)**2
                X = lam*U
                D = F(f/h**2+X+beta/X)
                H = sum((F(L(D(x=s+z))[-2].sqrt())/F(xx-z) for z in roots), F(0))
                T = D+H**2+H
                exact = T.denominator().degree() == T.numerator().degree() == 0
                if not exact:
                    counts["residue_only"] += 1
                    continue
                u = T.numerator()[0]/T.denominator()[0]
                tr = sum((b**(16**i) for i in range(3)), K(0))
                hits.append({"class": row["label"], "model_index": index,
                             "equation": equation, "exponents": list(exponents),
                             "lambda_mask": mask(lam), "mu_mask": mask(mu),
                             "u_mask": mask(u), "b_mask": mask(b),
                             "trace_b_mask": mask(tr), "expected_order": sum(row["poly"])})
                counts["exact_hits"] += 1
        print("class", row["label"], "models", counts["models"], "hits", counts["exact_hits"], flush=True)
    receipt = {"schema": "ghs-q16-degree3-all-ordinary-weil-classes-v1",
               "catalog_sha256": hashlib.sha256(raw).hexdigest(),
               "catalog_classes": len(catalog), "models_scanned": counts["models"],
               "branch_polynomials": len(cached), "patterns_per_model": len(patterns),
               "counts": dict(counts), "hits": hits,
               "script_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    (HERE / "degree3_all_q16_scan_receipt.json").write_text(json.dumps(receipt, indent=2)+"\n")
    print("COMPLETE", dict(counts), "classes", len(catalog), "branch forms", len(cached))


if __name__ == "__main__":
    main()
