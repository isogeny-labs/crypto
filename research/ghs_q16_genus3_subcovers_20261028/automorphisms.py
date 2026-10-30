#!/usr/bin/env sage -python
"""Check every branch-set Möbius symmetry for a lift to three exact C_3 models."""

import hashlib
import json
from pathlib import Path

from sage.all import GF, PolynomialRing


HERE = Path(__file__).resolve().parent


def main():
    k = GF(16, "a", modulus="conway")
    K = GF(4096, "w", modulus="conway")
    a = k.embeddings(K)[0](k.gen())
    R = PolynomialRing(K, "x")
    x = R.gen()
    F = R.fraction_field()
    h = a * a * x**4 + a * x
    roots = [r for r, _ in h.roots()]
    assert len(roots) == 4
    curves = {
        3974: x**8 + x**3 + a * x**2 + (a*a+a+1)*x + a**3+a+1,
        3978: x**8 + a*x**3 + (a**3+1)*x**2 + (a**3+a)*x + a**3,
        4178: x**8 + a*x**3 + a*x**2 + (a*a+a+1)*x + 1,
    }
    results = []
    # Every Möbius map permuting these four finite roots fixes infinity:
    # the branch-set cross ratio has the equianharmonic A4 stabilizer.
    # Enumerating all ordered image pairs below independently verifies that
    # there are exactly 12 affine maps; non-affine cases are checked below.
    from itertools import permutations
    from sage.all import Matrix

    mobius = []
    for target in permutations(roots):
        M = Matrix(K, [[u, K(1), -v*u, -v]
                       for u, v in zip(roots[:3], target[:3])])
        kernel = M.right_kernel()
        if kernel.dimension() != 1:
            continue
        A, B, C, D = kernel.basis()[0]
        if A*D == B*C or any(A*u+B != v*(C*u+D)
                             for u, v in zip(roots, target)):
            continue
        mobius.append((target, A, B, C, D))
    assert len(mobius) == 12 and all(C == 0 for _, _, _, C, _ in mobius)

    for order, f in curves.items():
        assert h.gcd(f) == 1
        lifted = []
        residue_passes = 0
        for target, A, B, C, D in mobius:
            u, v = A/D, B/D
            assert h(u*x+v) == u**4*h
            g = f(u*x+v)/u**8 + f
            c2 = [g(r)/a**2 for r in roots]
            residues = [g.derivative()(r)/a**2 + z.sqrt()
                        for r, z in zip(roots, c2)]
            if any(z != 0 for z in residues):
                continue
            residue_passes += 1
            H = sum((z.sqrt()/(x-r) for z, r in zip(c2, roots)), F(0))
            remainder = F(g)/h**2 + H**2 + H
            if remainder.denominator().degree() != 0:
                continue
            constant = remainder.numerator()[0]/remainder.denominator()[0]
            if remainder.numerator().degree() > 0 or constant.trace() != 0:
                continue
            lifted.append([roots.index(vv) for vv in target])
        results.append({"source_order": order, "curve_h": str(h),
                        "curve_f": str(f), "mobius_count": len(mobius),
                        "local_residue_passes": residue_passes,
                        "lifted_branch_permutations": lifted})
        print(order, "residue passes", residue_passes, "lifts", lifted, flush=True)

    receipt = {"schema": "ghs-q16-genus3-branch-automorphisms-v1",
               "field_modulus": str(K.modulus()),
               "branch_roots_masks": [int(r.to_integer()) for r in roots],
               "results": results,
               "search_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    (HERE/"automorphism_receipt.json").write_text(json.dumps(receipt, indent=2)+"\n")


if __name__ == "__main__":
    main()
