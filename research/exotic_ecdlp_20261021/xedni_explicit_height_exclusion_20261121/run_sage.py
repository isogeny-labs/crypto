"""Extract exact curve invariants and the large-multiple x-height witness."""

import hashlib
import json
from pathlib import Path

from sage.all import EllipticCurve, QQ

HERE=Path(__file__).resolve().parent
ROOT=HERE.parent


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    p=json.loads((HERE/"protocol.json").read_text())
    assert sha(ROOT/"xedni_rank1_generator_certificate_20261121"/"result.json")==p["source_generator_sha256"]
    assert sha(ROOT/"xedni_rank1_generator_certificate_20261121"/"lift-class-result.json")==p["source_lift_class_sha256"]
    E=EllipticCurve(QQ,[QQ(v) for v in p["curve_a_invariants"]])
    Q=E([QQ(v) for v in p["generator_Q"]])
    P=p["witness_multiple"]*Q
    x=P[0]
    numerator=int(x.numerator())
    denominator=int(x.denominator())
    xbits=max(abs(numerator).bit_length(),denominator.bit_length())
    assert xbits==2894
    bound_default=E.silverman_height_bound()
    bound_mwrank=E.silverman_height_bound(algorithm="mwrank")
    assert bound_default<14 and bound_mwrank<14
    print(json.dumps({"schema":"xedni-explicit-height-input-v1","status":"PASS",
        "discriminant":str(E.discriminant()),"j_invariant":str(E.j_invariant()),
        "b2":str(E.b2()),"curve_is_minimal":bool(E.is_minimal()),
        "witness_multiple":p["witness_multiple"],"witness_x_numerator_bits":abs(numerator).bit_length(),
        "witness_x_denominator_bits":denominator.bit_length(),"witness_x_max_bits":xbits,
        "witness_x_sha256":hashlib.sha256(str(x).encode()).hexdigest(),
        "silverman_default_bound_diagnostic":str(bound_default),
        "silverman_mwrank_bound_diagnostic":str(bound_mwrank)},indent=2,sort_keys=True))


if __name__=="__main__":main()
