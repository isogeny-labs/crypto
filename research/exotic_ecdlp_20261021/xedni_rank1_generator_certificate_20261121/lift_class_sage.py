"""Exact target-lift multiplier class and numerical canonical heights."""

import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ, RealField

HERE=Path(__file__).resolve().parent


def point_bits(P):
    return max(max(abs(v.numerator()).nbits(),v.denominator().nbits()) for v in P.xy())


def main():
    p=json.loads((HERE/"lift-class-protocol.json").read_text())
    source=(HERE/"result.json").read_bytes()
    assert hashlib.sha256(source).hexdigest()==p["source_generator_result_sha256"]
    certificate=json.loads(source)
    assert certificate["status"]=="PASS" and certificate["rank"]==1
    assert certificate["generators_certain"] and certificate["torsion_order"]==1
    assert certificate["Q_equals_signed_generator"]
    E=EllipticCurve(QQ,[QQ(v) for v in p["curve_a_invariants"]])
    Q=E([QQ(v) for v in p["Q"]])
    E0=EllipticCurve(GF(47),[0,0,0,-1,-2])
    Q0=E0([GF(47)(v) for v in p["Q"]])
    T=E0(p["reduced_target"])
    assert Q0.order()==43 and 22*Q0==T
    assert all(n%43!=22 for n in range(-20,21))
    assert (-21)%43==22
    R=RealField(128)
    hQ=R(Q.height(precision=128))
    assert hQ>0
    rows=[]
    for n in p["representatives_to_compute"]:
        P=n*Q
        P0=E0([GF(47)(v) for v in P.xy()])
        assert P0==T
        hP=R(P.height(precision=128))
        ratio=hP/hQ
        error=abs(ratio-R(n*n))
        assert error<R(2)**(-60)
        rows.append({"n":n,"n_mod_43":n%43,"affine_coordinate_max_bits":point_bits(P),
                     "canonical_height_128bit":str(hP),"height_ratio_to_Q_128bit":str(ratio),
                     "ratio_error_128bit":str(error)})
    print(json.dumps({"schema":"xedni-rank1-target-lift-class-result-v1","status":"PASS",
                      "target":p["reduced_target"],"Q_canonical_height_128bit":str(hQ),
                      "minimal_absolute_multiplier":-21,"rows":rows},indent=2,sort_keys=True))


if __name__=="__main__":main()
