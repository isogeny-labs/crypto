#!/usr/bin/env python3
"""Cross-prime exact rational coordinate and reduction-prefix census."""

import hashlib
import json
from decimal import Decimal, getcontext
from math import isqrt
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ, ZZ

HERE = Path(__file__).resolve().parent


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def bit_height(P):
    return max(max(abs(int(v.numerator())).bit_length(),int(v.denominator()).bit_length()) for v in (P[0],P[1]))


def main():
    protocol_raw = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_raw)
    E = EllipticCurve(QQ,protocol["rational_curve_coefficients"])
    Q = E(protocol["Q"])
    R = E([QQ(s) for s in protocol["R"]])
    pairing = E.height_pairing_matrix([Q,R],precision=protocol["height_pairing_precision_bits"])
    assert [[str(pairing[i,j]) for j in range(2)] for i in range(2)] == protocol["height_pairing_matrix"]
    getcontext().prec = 50
    hqq,hqr,hrr = [Decimal(s) for s in (protocol["height_pairing_matrix"][0][0],
                                          protocol["height_pairing_matrix"][0][1],
                                          protocol["height_pairing_matrix"][1][1])]
    def score(label):
        a,b = label
        return hqq*a*a+2*hqr*a*b+hrr*b*b
    rational_cache = {}
    def rational(label):
        if label not in rational_cache:
            a,b = label
            P = a*Q+b*R
            assert P != E(0)
            rational_cache[label] = (P,bit_height(P))
        return rational_cache[label]
    rows = []
    for p,q in protocol["prime_order_reductions"]:
        assert ZZ(p).is_prime() and ZZ(q).is_prime() and ZZ(E.discriminant())%p
        assert all(int(v.denominator())%p for v in (R[0],R[1]))
        Ep = EllipticCurve(GF(p),[GF(p)(c) for c in protocol["rational_curve_coefficients"]])
        assert int(Ep.cardinality()) == q
        G = Ep([GF(p)(Q[0]),GF(p)(Q[1])])
        Rbar = Ep([GF(p)(R[0].numerator())/GF(p)(R[0].denominator()),
                   GF(p)(R[1].numerator())/GF(p)(R[1].denominator())])
        assert G.order() == q and Rbar != Ep(0)
        B = isqrt(q)+2
        labels = [(a,b) for a in range(-B,B+1) for b in range(-B,B+1) if (a,b)!=(0,0)]
        labels.sort(key=lambda z:(score(z),z[0],z[1]))
        seen = {}
        first_relation = None
        peak = 0
        coverage_index = None
        for i,label in enumerate(labels,1):
            P,bits = rational(label)
            peak = max(peak,bits)
            if int(P[0].denominator())%p == 0:
                reduced = Ep(0)
            else:
                reduced = Ep([GF(p)(P[0].numerator())/GF(p)(P[0].denominator()),
                              GF(p)(P[1].numerator())/GF(p)(P[1].denominator())])
            a,b = label
            assert reduced == a*G+b*Rbar
            if reduced == Ep(0):
                if first_relation is None:
                    da,db = a,b
                    first_relation = {"index":i,"kind":"zero_reduction","labels":[list(label)],
                                      "delta":[da,db],"prefix_max_coordinate_bits":peak}
            elif reduced in seen:
                if first_relation is None:
                    old = seen[reduced]
                    da,db = a-old[0],b-old[1]
                    first_relation = {"index":i,"kind":"finite_collision","labels":[list(old),list(label)],
                                      "delta":[da,db],"prefix_max_coordinate_bits":peak}
            else:
                seen[reduced] = label
            if first_relation is not None and "derived_R_over_G" not in first_relation:
                assert db%q
                lam = -da*pow(db,-1,q)%q
                assert lam*G == Rbar
                first_relation["derived_R_over_G"] = lam
            if len(seen) == q-1:
                coverage_index = i
                break
        row = {"p":p,"q":q,"coefficient_cap":B,"candidate_cap":len(labels),
               "status":"full_coverage" if coverage_index is not None else "coefficient_cap_exhausted",
               "first_relation":first_relation,"coverage_index":coverage_index,
               "distinct_nonidentity_reductions":len(seen),
               "prefix_max_coordinate_bits":peak,"candidate_evaluations":i}
        rows.append(row)
    raw = "".join(json.dumps(row,sort_keys=True)+"\n" for row in rows).encode()
    (HERE / "prime-rows.jsonl").write_bytes(raw)
    return {"schema":"xedni-rank-two-prime-scaling-result-v1","status":"PASS",
            "protocol_sha256":sha(protocol_raw),"script_sha256":sha((HERE / "run_sage.py").read_bytes()),
            "prime_rows_raw_sha256":sha(raw),"panel_primes":len(rows),
            "full_coverage_cells":sum(row["status"]=="full_coverage" for row in rows),
            "unique_rational_points_constructed":len(rational_cache),
            "max_rational_coordinate_bits_constructed":max(bits for _,bits in rational_cache.values())}


if __name__ == "__main__":
    print(json.dumps(main(),sort_keys=True,indent=2))
