#!/usr/bin/env sage -python
"""Replay the degree-two quotient obstruction for all cataloged models."""

import hashlib
import itertools
import json
import re
from pathlib import Path

from sage.all import GF, HyperellipticCurve, PolynomialRing, matrix, sage_eval


ROOT = Path(__file__).resolve().parent
MODEL = ("y^2+(a^2*x^4+a*x)*y="
         "x^8+x^3+a*x^2+(a^2+a+1)*x+a^3+a+1")
SOURCE_URL = "https://www.lmfdb.org/Variety/Abelian/Fq/download_curves/3.16.a_a_aet"


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def encoded(element):
    return int(element.to_integer())


def audit_model(line, k, K, embed):
    a = k.gen()
    Rk = PolynomialRing(k, "x")
    xk = Rk.gen()
    match = re.fullmatch(r"y\^2\+\((.+)\)\*y=(.+)", line)
    assert match is not None
    h0 = Rk(sage_eval(match.group(1), locals={"a": a, "x": xk}))
    f0 = Rk(sage_eval(match.group(2), locals={"a": a, "x": xk}))
    assert h0.degree() == 4 and h0[0] == 0 and h0[2] == h0[3] == 0
    assert f0.degree() == 8 and f0[8] == 1
    assert all(f0[i] == 0 for i in range(4, 8))
    assert h0[4] * h0[1] * f0[3] != 0
    curve = HyperellipticCurve(f0, h0)
    assert curve.genus() == 3
    point_counts = [int(n) for n in curve.count_points(3)]
    assert point_counts == [17, 257, 3728]

    R = PolynomialRing(K, "x")
    x = R.gen()
    h = R([embed(v) for v in h0.list()])
    f = R([embed(v) for v in f0.list()])
    beta, gamma = h[1], f[3]
    roots = sorted((r for r, multiplicity in h.roots()
                    if multiplicity == 1), key=encoded)
    assert len(roots) == 4 and roots[0] == 0
    branch_coeff = [(f.derivative()(r) / beta**2) + f(r).sqrt() / beta
                    for r in roots]
    assert all(v != 0 for v in branch_coeff)

    maps = []
    for permutation in itertools.permutations(range(4)):
        M = matrix(K, [[roots[i], K.one(),
                        roots[permutation[i]] * roots[i],
                        roots[permutation[i]]] for i in range(3)])
        assert M.rank() == 3
        A, B, C, D = M.right_kernel().basis()[0]
        assert A * D + B * C != 0
        denominator = C * roots[3] + D
        if denominator == 0:
            continue
        if (A * roots[3] + B) / denominator != roots[permutation[3]]:
            continue
        assert C == 0 and D != 0
        c, d = A / D, B / D
        assert c**3 == 1 and h(d) == 0
        assert h(c*x+d) == c*h
        rhs = f(c*x+d) + c**2*f
        identity = c == 1 and d == 0
        if identity:
            assert rhs == 0
        elif c != 1:
            assert rhs.degree() == 3 and rhs[3] == gamma * (1+c**2)
        else:
            assert d != 0 and rhs.degree() == 2 and rhs[2] == gamma*d
        maps.append({"permutation": list(permutation), "c": encoded(c),
                     "d": encoded(d), "identity": identity,
                     "rhs_degree": -1 if rhs == 0 else int(rhs.degree()),
                     "rhs_leading": 0 if rhs == 0 else encoded(rhs.leading_coefficient())})
    assert len(maps) == 12 and sum(m["identity"] for m in maps) == 1
    assert sum(m["rhs_degree"] == 2 for m in maps) == 3
    assert sum(m["rhs_degree"] == 3 for m in maps) == 8
    return {"model": line, "point_counts_k_k2_k3": point_counts,
            "branch_roots": [encoded(r) for r in roots],
            "branch_simple_pole_coefficients": [encoded(v) for v in branch_coeff],
            "branch_set_stabilizer_count": len(maps), "affine_maps": maps,
            "liftable_nonidentity_maps": 0}


def main():
    parent = ROOT.parent / "ghs_cubic_catalog_q16_20261026/certificate.json"
    prior = json.loads(parent.read_text())
    assert prior["example"]["model"] == MODEL
    assert prior["example"]["order"] == 3974
    snapshot = ROOT / "lmfdb_curves_3974.txt"
    lines = [line.strip() for line in snapshot.read_text().splitlines()
             if line.strip() and not line.startswith("#")]
    assert len(lines) == 4 and len(set(lines)) == 4 and lines[0] == MODEL

    k = GF(16, "a", modulus="conway")
    K = GF(4096, "w", modulus="conway")
    embed = k.embeddings(K)[0]
    rows = [audit_model(line, k, K, embed) for line in lines]
    certificate = {
        "schema": "ghs-cubic-q16-degree2-obstruction/v2",
        "parent_certificate_sha256": digest(parent),
        "protocol_sha256": digest(ROOT / "PROTOCOL.md"),
        "script_sha256": digest(Path(__file__)),
        "catalog_curve_snapshot_sha256": digest(snapshot),
        "catalog_curve_snapshot_url": SOURCE_URL,
        "field_moduli": {"k": str(k.modulus()), "K": str(K.modulus())},
        "source_order": 3974,
        "cataloged_hyperelliptic_model_count": len(rows),
        "models": rows,
        "conclusion": "none of the four listed hyperelliptic models has a separable degree-two elliptic quotient",
        "limit": "does not exclude unlisted curves, higher-degree maps or Jacobian correspondences",
    }
    (ROOT / "certificate.json").write_text(
        json.dumps(certificate, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"models": len(rows), "point_counts_verified": True,
                      "all_branch_points_verified": True,
                      "stabilizers": [row["branch_set_stabilizer_count"] for row in rows],
                      "liftable_nonidentity_maps": sum(row["liftable_nonidentity_maps"] for row in rows)},
                     sort_keys=True))


if __name__ == "__main__":
    main()
