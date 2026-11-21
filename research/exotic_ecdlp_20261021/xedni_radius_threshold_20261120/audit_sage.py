#!/usr/bin/env python3
"""Independent elliptic-curve audit at the two measured radius boundaries."""

import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def main():
    protocol = json.loads((HERE / "protocol.json").read_bytes())
    result = json.loads((HERE / "result.json").read_bytes())
    raw = (HERE / "radius-rows.jsonl").read_bytes()
    assert sha(raw) == result["rows_sha256"]
    rows = {(r["p"],r["radius"]):r for r in map(json.loads,raw.splitlines())}
    assert len(rows) == result["radius_rows"] == 258
    old = json.loads((ROOT / "research/exotic_ecdlp_20261021/xedni_preregistered_prime_panel_20261120/protocol.json").read_bytes())
    E = EllipticCurve(QQ,old["rational_curve_coefficients"])
    Q = E(old["Q"])
    R = E([QQ(v) for v in old["R"]])
    checked = []
    for cell, threshold in zip(protocol["cells"],result["thresholds"]):
        p,q,lam = [cell[k] for k in ("p","q","R_over_Q")]
        B = threshold["first_full_radius"]
        assert threshold["p"] == p and threshold["q"] == q
        Ep = EllipticCurve(GF(p),[GF(p)(c) for c in old["rational_curve_coefficients"]])
        assert Ep.cardinality() == q
        G = Ep([GF(p)(Q[0]),GF(p)(Q[1])])
        Rbar = Ep([GF(p)(R[0].numerator())/GF(p)(R[0].denominator()),
                   GF(p)(R[1].numerator())/GF(p)(R[1].denominator())])
        assert G.order() == q and Rbar == lam*G
        for radius in (B-1,B):
            points = {a*G+b*Rbar for b in range(-radius,radius+1)
                                    for a in range(-radius,radius+1)}
            row = rows[(p,radius)]
            assert len(points) == row["attained_residues_including_zero"]
            assert (Ep(0) in points) and len(points)-1 == row["attained_nonidentity_residues"]
            full = len(points) == q
            assert full == (row["status"] == "full_coverage")
            if full:
                assert points == {k*G for k in range(q)}
            checked.append({"p":p,"q":q,"radius":radius,"point_image_count":len(points),
                            "status":"full_coverage" if full else "incomplete"})
    return {"schema":"xedni-radius-threshold-sage-audit-v1","status":"PASS",
            "rows_sha256":sha(raw),"boundary_rows":checked}


if __name__ == "__main__":
    print(json.dumps(main(),sort_keys=True,indent=2))
