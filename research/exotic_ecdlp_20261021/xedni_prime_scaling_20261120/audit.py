#!/usr/bin/env python3
"""Independent Fraction and finite-field replay of all seven prime prefixes."""

import hashlib
import json
import sys
from decimal import Decimal, getcontext
from fractions import Fraction as F
from math import isqrt
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0,str(HERE.parent / "xedni_coefficient_choice_20261120"))
from audit import add, neg, discriminant


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def multiples(P,limit,curve):
    out = {0:None}
    current = None
    for k in range(1,limit+1):
        current = add(current,P,curve)
        out[k] = current
        out[-k] = neg(current,curve)
    return out


def reduce_point(P,p):
    if P is None or P[0].denominator%p==0:
        return None
    return tuple(v.numerator*pow(v.denominator,-1,p)%p for v in P)


def bit_height(P):
    return max(max(abs(v.numerator).bit_length(),v.denominator.bit_length()) for v in P)


def main():
    protocol_raw = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_raw)
    result = json.loads((HERE / "result.json").read_text())
    assert sha(protocol_raw) == result["protocol_sha256"]
    raw = (HERE / "prime-rows.jsonl").read_bytes()
    assert sha(raw) == result["prime_rows_raw_sha256"]
    rows = [json.loads(line) for line in raw.splitlines()]
    assert len(rows) == 7
    curve = [F(v) for v in protocol["rational_curve_coefficients"]]
    assert discriminant(curve) != 0
    Q = tuple(F(v) for v in protocol["Q"])
    R = tuple(F(v) for v in protocol["R"])
    limit = max(isqrt(q)+2 for _,q in protocol["prime_order_reductions"])
    qmult = multiples(Q,limit,curve)
    rmult = multiples(R,limit,curve)
    cache = {}
    def rational(label):
        if label not in cache:
            a,b = label
            P = add(qmult[a],rmult[b],curve)
            assert P is not None
            cache[label] = (P,bit_height(P))
        return cache[label]
    getcontext().prec = 50
    hqq,hqr,hrr = [Decimal(s) for s in (protocol["height_pairing_matrix"][0][0],
                                          protocol["height_pairing_matrix"][0][1],
                                          protocol["height_pairing_matrix"][1][1])]
    for saved,(p,q) in zip(rows,protocol["prime_order_reductions"]):
        B=isqrt(q)+2
        labels=[(a,b) for a in range(-B,B+1) for b in range(-B,B+1) if (a,b)!=(0,0)]
        labels.sort(key=lambda z:(hqq*z[0]**2+2*hqr*z[0]*z[1]+hrr*z[1]**2,z[0],z[1]))
        assert saved["candidate_cap"]==len(labels) and saved["coefficient_cap"]==B
        field_curve=[int(c)%p for c in curve]
        G=tuple(v.numerator*pow(v.denominator,-1,p)%p for v in Q)
        Rbar=tuple(v.numerator*pow(v.denominator,-1,p)%p for v in R)
        negG=tuple(v%p for v in neg(G,field_curve))
        negR=tuple(v%p for v in neg(Rbar,field_curve))
        seen={}
        first=None
        peak=0
        coverage=None
        for i,label in enumerate(labels,1):
            P,bits=rational(label)
            peak=max(peak,bits)
            reduced=reduce_point(P,p)
            a,b=label
            # Independent group-law evaluation for every tested candidate.
            eq=None
            for _ in range(abs(a)):
                eq=add(eq,G if a>=0 else negG,field_curve,p)
            for _ in range(abs(b)):
                eq=add(eq,Rbar if b>=0 else negR,field_curve,p)
            assert reduced==eq
            if reduced is None:
                if first is None:
                    first={"index":i,"kind":"zero_reduction","labels":[list(label)],
                           "delta":[a,b],"prefix_max_coordinate_bits":peak}
            elif reduced in seen:
                if first is None:
                    old=seen[reduced]
                    first={"index":i,"kind":"finite_collision","labels":[list(old),list(label)],
                           "delta":[a-old[0],b-old[1]],"prefix_max_coordinate_bits":peak}
            else:
                seen[reduced]=label
            if first is not None and "derived_R_over_G" not in first:
                da,db=first["delta"]
                assert db%q
                first["derived_R_over_G"]=-da*pow(db,-1,q)%q
                calc=None
                for _ in range(first["derived_R_over_G"]):
                    calc=add(calc,G,field_curve,p)
                assert calc==Rbar
            if len(seen)==q-1:
                coverage=i
                break
        assert saved == {"p":p,"q":q,"coefficient_cap":B,"candidate_cap":len(labels),
                         "status":"full_coverage" if coverage else "coefficient_cap_exhausted",
                         "first_relation":first,"coverage_index":coverage,
                         "distinct_nonidentity_reductions":len(seen),
                         "prefix_max_coordinate_bits":peak,"candidate_evaluations":i}
    assert len(cache)==result["unique_rational_points_constructed"]
    assert max(bits for _,bits in cache.values())==result["max_rational_coordinate_bits_constructed"]
    return {"status":"PASS","panel_primes":len(rows),
            "full_coverage_cells":sum(row["status"]=="full_coverage" for row in rows),
            "independent_rational_points_rebuilt":len(cache),
            "prime_rows_raw_sha256":sha(raw)}


if __name__=="__main__":
    print(json.dumps(main(),sort_keys=True,indent=2))
