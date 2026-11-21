"""Derive the full three-variable coefficient-six necessary surface."""

import hashlib
import json
import signal
from pathlib import Path

from sage.all import EllipticCurve, PolynomialRing, QQ, ZZ, gcd, lcm

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
BRIDGE = ROOT / "xedni_bridge6_aux8_20261120/result.json"
GRID = ROOT / "xedni_bridge6_k1k2_grid_20261120/result.json"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def event(stage, **data):
    with (HERE / "events.jsonl").open("a") as f:
        f.write(json.dumps({"stage": stage, **data}, sort_keys=True) + "\n")


def coefficients(k1, k2, k3):
    (x1, y1), (x2, y2) = (QQ(2), QQ(2)), (QQ(-16), QQ(17))
    a1, a2, a3 = 47*k1, 47*k2, 47*k3
    z1 = y1*y1+a1*x1*y1+a3*y1-x1**3-a2*x1*x1
    z2 = y2*y2+a1*x2*y2+a3*y2-x2**3-a2*x2*x2
    a4 = (z1-z2)/(x1-x2)
    return [a1, a2, a3, a4, z1-a4*x1]


def primitive(poly, ring):
    coeffs = list(poly.dict().values())
    scale = lcm([c.denominator() for c in coeffs])
    integers = [ZZ(scale*c) for c in coeffs]
    content = gcd(integers)
    out = ring.zero()
    gens = ring.gens()
    for exps, coeff in poly.dict().items():
        term = ring(ZZ(scale*coeff) // content)
        for g, e in zip(gens, exps):
            term *= g**e
        out += term
    return -out if out.leading_coefficient() < 0 else out


def sparse(poly):
    lines = [json.dumps({"exponents": list(exps), "coefficient": str(coeff)},
                        sort_keys=True, separators=(",", ":")) + "\n"
             for exps, coeff in sorted(poly.dict().items())]
    return "".join(lines).encode()


def write_parts(label, data):
    folder = HERE / label
    folder.mkdir(exist_ok=True)
    lines = data.splitlines(keepends=True)
    part, chunk = 0, b""
    for line in lines:
        assert len(line) <= 48000
        if len(chunk)+len(line) > 48000:
            (folder / f"{part:03d}.jsonl").write_bytes(chunk)
            part += 1
            chunk = b""
        chunk += line
    if chunk:
        (folder / f"{part:03d}.jsonl").write_bytes(chunk)
        part += 1
    return part


class FactorTimeout(Exception):
    pass


def alarm(_signum, _frame):
    raise FactorTimeout()


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    assert sha(BRIDGE.read_bytes()) == protocol["source_bridge_result_sha256"]
    assert sha(GRID.read_bytes()) == protocol["source_grid_result_sha256"]
    (HERE / "events.jsonl").write_text("")
    event("started")
    R = PolynomialRing(QQ, names=("u", "v", "t"))
    u, v, t = R.gens()
    K = R.fraction_field()
    coeffs = coefficients(K(-3+77*u), K(-11+77*v), K(1+77*t))
    event("curve_constructed")
    E = EllipticCurve(K, [K(c) for c in coeffs])
    assert E.discriminant() != 0
    P, Q = E(K(2), K(2)), E(K(-16), K(17))
    X = (6*Q)[0]-K(2)
    event("rational_function_computed")
    Rz = PolynomialRing(ZZ, names=("u", "v", "t"))
    numerator = primitive(X.numerator(), Rz)
    denominator = primitive(X.denominator(), Rz)
    event("primitive_polynomials_computed", numerator_terms=len(numerator.dict()),
          denominator_terms=len(denominator.dict()))
    num_raw, den_raw = sparse(numerator), sparse(denominator)
    num_parts = write_parts("numerator-terms", num_raw)
    den_parts = write_parts("denominator-terms", den_raw)
    event("term_files_written", numerator_parts=num_parts, denominator_parts=den_parts)
    signal.signal(signal.SIGALRM, alarm)
    signal.alarm(120)
    try:
        factors = []
        for factor, mult in numerator.factor():
            factors.append({"multiplicity": int(mult), "total_degree": int(factor.total_degree()),
                            "terms": len(factor.dict()), "sha256": sha(sparse(factor))})
        factor_status = "complete"
    except FactorTimeout:
        factors = []
        factor_status = "timeout_120s"
    finally:
        signal.alarm(0)
    event("factorization_finished", status=factor_status)
    print(json.dumps({"schema": "xedni-bridge6-lattice-surface-result-v1",
                      "status": "PASS" if factor_status == "complete" else "FACTOR_CENSORED",
                      "source_bridge_result_sha256": sha(BRIDGE.read_bytes()),
                      "source_grid_result_sha256": sha(GRID.read_bytes()),
                      "numerator_raw_sha256": sha(num_raw), "denominator_raw_sha256": sha(den_raw),
                      "numerator_parts": num_parts, "denominator_parts": den_parts,
                      "numerator_terms": len(numerator.dict()),
                      "denominator_terms": len(denominator.dict()),
                      "numerator_total_degree": int(numerator.total_degree()),
                      "denominator_total_degree": int(denominator.total_degree()),
                      "numerator_degrees": {str(x): int(numerator.degree(x)) for x in Rz.gens()},
                      "denominator_degrees": {str(x): int(denominator.degree(x)) for x in Rz.gens()},
                      "factor_status": factor_status, "factors": factors},
                     sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
