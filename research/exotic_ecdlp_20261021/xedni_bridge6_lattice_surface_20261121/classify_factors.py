"""Classify full-lattice numerator factors by elliptic-point sign."""

import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, PolynomialRing, QQ, ZZ, gcd

HERE = Path(__file__).resolve().parent


def sha(data):
    return hashlib.sha256(data).hexdigest()


def sparse(poly):
    return "".join(json.dumps({"exponents": list(e), "coefficient": str(c)},
                              sort_keys=True, separators=(",", ":")) + "\n"
                   for e, c in sorted(poly.dict().items())).encode()


def load_terms(folder, ring):
    files = sorted((HERE / folder).glob("[0-9][0-9][0-9].jsonl"))
    raw = b"".join(f.read_bytes() for f in files)
    out = ring.zero()
    gens = ring.gens()
    for line in raw.splitlines():
        row = json.loads(line)
        term = ring(ZZ(row["coefficient"]))
        for g, exponent in zip(gens, row["exponents"]):
            term *= g**exponent
        out += term
    return out, raw


def write_parts(folder, raw):
    path = HERE / folder
    path.mkdir(exist_ok=True)
    chunks, part = b"", 0
    for line in raw.splitlines(keepends=True):
        assert len(line) <= 48000
        if len(chunks)+len(line) > 48000:
            (path / f"{part:03d}.jsonl").write_bytes(chunks)
            part += 1
            chunks = b""
        chunks += line
    if chunks:
        (path / f"{part:03d}.jsonl").write_bytes(chunks)
        part += 1
    return part


def coefficients(u, v, t):
    x1, y1, x2, y2 = QQ(2), QQ(2), QQ(-16), QQ(17)
    a1, a2, a3 = 47*(-3+77*u), 47*(-11+77*v), 47*(1+77*t)
    z1 = y1*y1+a1*x1*y1+a3*y1-x1**3-a2*x1*x1
    z2 = y2*y2+a1*x2*y2+a3*y2-x2**3-a2*x2*x2
    a4 = (z1-z2)/(x1-x2)
    return [a1, a2, a3, a4, z1-a4*x1]


def main():
    protocol = json.loads((HERE / "factor-protocol.json").read_text())
    assert sha((HERE / "result.json").read_bytes()) == protocol["surface_result_sha256"]
    surface = json.loads((HERE / "result.json").read_text())
    Rz = PolynomialRing(ZZ, names=("u", "v", "t"))
    N, raw = load_terms("numerator-terms", Rz)
    assert sha(raw) == surface["numerator_raw_sha256"]
    factors = N.factor()
    R = PolynomialRing(QQ, names=("u", "v", "t"))
    u, v, t = R.gens()
    K = R.fraction_field()
    a1, a2, a3, a4, a6 = coefficients(K(u), K(v), K(t))
    E = EllipticCurve(K, [K(c) for c in (a1, a2, a3, a4, a6)])
    Q = E(K(-16), K(17))
    Q6 = 6*Q
    yrel = Q6[1]+K(2)+K(2)*K(a1)+K(a3)
    ysame = Q6[1]-K(2)
    Nrel = yrel.numerator()
    Nsame = ysame.numerator()
    out = []
    for i, (factor, multiplicity) in enumerate(factors):
        fp = R(factor)
        rel_gcd = gcd(fp, Nrel)
        same_gcd = gcd(fp, Nsame)
        rel = int(rel_gcd.total_degree()) == int(fp.total_degree())
        same = int(same_gcd.total_degree()) == int(fp.total_degree())
        data = sparse(factor)
        parts = write_parts(f"factor-{i:02d}-terms", data)
        out.append({"index": i, "multiplicity": int(multiplicity),
                    "total_degree": int(factor.total_degree()),
                    "terms": len(factor.dict()), "sha256": sha(data), "parts": parts,
                    "divides_relation_y_numerator": rel,
                    "divides_same_y_numerator": same,
                    "relation_gcd_degree": int(rel_gcd.total_degree()),
                    "same_gcd_degree": int(same_gcd.total_degree())})
    print(json.dumps({"schema": "xedni-bridge6-lattice-factor-result-v1",
                      "status": "PASS" if len(out) == 2 and sum(x["divides_relation_y_numerator"] for x in out) == 1 and sum(x["divides_same_y_numerator"] for x in out) == 1 else "UNRESOLVED",
                      "surface_result_sha256": sha((HERE / "result.json").read_bytes()),
                      "factor_unit": str(factors.unit()), "factors": out},
                     sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
