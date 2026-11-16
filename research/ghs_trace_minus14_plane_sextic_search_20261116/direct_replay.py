#!/usr/bin/env sage -python
"""Direct Sage polynomial evaluation of the four first-case F9 survivors."""
from pathlib import Path
from sage.all import GF, PolynomialRing, matrix
import hashlib
import json

here = Path(__file__).parent
input_path = here / "N2_SURVIVORS.json"
data = json.loads(input_path.read_text())
assert data["counts"]["N2"] == len(data["near"]) == 4
F3 = GF(3)
R = PolynomialRing(F3, names=("x", "y", "z"))
x, y, z = R.gens()
vs = (x, y, z)
nodes = tuple(tuple(F3(v) for v in p) for p in data["nodes"])
mons = [x**i * y**j * z**(6-i-j) for i in range(7) for j in range(7-i)]
conditions = []
for p in nodes:
    k = next(i for i, a in enumerate(p) if a)
    conditions.append([m(*p) for m in mons])
    for j in range(3):
        if j != k:
            conditions.append([m.derivative(vs[j])(*p) for m in mons])
ker = matrix(F3, conditions).right_kernel().basis()
basis = [sum((v[i] * mons[i] for i in range(28)), R(0)) for v in ker]
assert len(basis) == 13


def plane_points(k):
    K = GF(3**k, name="a")
    es = list(K)
    one, zero = K(1), K(0)
    return K, ([(one, u, v) for u in es for v in es]
               + [(zero, one, u) for u in es]
               + [(zero, zero, one)])


points = {r: plane_points(r) for r in (1, 2, 3)}
results = []
for c in data["near"]:
    f = sum((F3(c[i]) * basis[i] for i in range(13)), R(0))
    split = 0
    for p in nodes:
        k = next(i for i, a in enumerate(p) if a)
        u, v = [vs[i] for i in range(3) if i != k]
        a = F3(2) * f.derivative(u).derivative(u)(*p)
        b = f.derivative(u).derivative(v)(*p)
        cc = F3(2) * f.derivative(v).derivative(v)(*p)
        disc = b*b - a*cc
        assert disc != 0
        split += disc == 1
    counts = {}
    for r, (K, pts) in points.items():
        q = f.change_ring(K)
        plane = sum(q(*p) == 0 for p in pts)
        normalized = plane + (5 if r % 2 == 0 else 2*split-5)
        counts[str(r)] = {"plane": int(plane), "normalized": int(normalized)}
    assert [counts[str(r)]["normalized"] for r in (1, 2)] == [4, 10]
    assert counts["3"]["normalized"] != 28
    results.append({"coefficients": c, "sextic": str(f), "split_nodes": int(split),
                    "counts": counts})
print(json.dumps({"schema": "ghs.trace-minus14.five-node-direct-replay/v1",
                  "source_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                  "input_sha256": hashlib.sha256(input_path.read_bytes()).hexdigest(),
                  "results": results}, sort_keys=True))
