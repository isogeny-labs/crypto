#!/usr/bin/env python3
"""Independent prime selection and Fraction/finite-field prefix replay."""

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


def finite_multiples(P,limit,curve,p):
    out={0:None}
    current=None
    for k in range(1,limit+1):
        current=add(current,P,curve,p)
        out[k]=current
        out[-k]=tuple(v%p for v in neg(current,curve))
    return out


def prime(n):
    return n>=2 and all(n%d for d in range(2,isqrt(n)+1))


def count_curve(curve,p):
    a1,a2,a3,a4,a6=[int(v)%p for v in curve]
    total=p+1
    for x in range(p):
        d=((a1*x+a3)**2+4*(x**3+a2*x*x+a4*x+a6))%p
        chi=0 if d==0 else (1 if pow(d,(p-1)//2,p)==1 else -1)
        total+=chi
    return total


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
    curve = [F(v) for v in protocol["rational_curve_coefficients"]]
    assert discriminant(curve) != 0
    Q = tuple(F(v) for v in protocol["Q"])
    R = tuple(F(v) for v in protocol["R"])
    selection_raw=(HERE / "selection-rows.jsonl").read_bytes()
    windows_raw=(HERE / "window-rows.jsonl").read_bytes()
    assert sha(selection_raw)==result["selection_rows_raw_sha256"]
    assert sha(windows_raw)==result["window_rows_raw_sha256"]
    selection_saved=[json.loads(line) for line in selection_raw.splitlines()]
    windows_saved=[json.loads(line) for line in windows_raw.splitlines()]
    selection=[]
    windows=[]
    selected=[]
    for lower,upper in protocol["prime_windows_lower_inclusive_upper_exclusive"]:
        choice=None
        for p in range(lower,upper):
            if not prime(p):
                continue
            row={"window":[lower,upper],"p":p,"status":None,"q":None}
            if discriminant(curve)%p==0:
                row["status"]="bad_reduction"
            elif any(v.denominator%p==0 for v in R):
                row["status"]="nonintegral_R"
            else:
                q=count_curve(curve,p)
                row["q"]=q
                if not prime(q):
                    row["status"]="composite_group_order"
                elif reduce_point(Q,p) is None:
                    row["status"]="identity_Q_reduction"
                else:
                    row["status"]="selected"
                    choice=[p,q]
            selection.append(row)
            if choice:
                break
        windows.append({"window":[lower,upper],"status":"selected" if choice else "no_eligible_prime",
                        "chosen_pq":choice})
        if choice:
            selected.append(choice)
    assert selection==selection_saved and windows==windows_saved
    assert len(selection)==result["selection_tests"]
    assert len(selected)==len(rows)==result["selected_cells"]
    limit = max((isqrt(q)+2 for _,q in selected),default=0)
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
    for saved,(p,q) in zip(rows,selected):
        B=isqrt(q)+2
        labels=[(a,b) for a in range(-B,B+1) for b in range(-B,B+1) if (a,b)!=(0,0)]
        labels.sort(key=lambda z:(hqq*z[0]**2+2*hqr*z[0]*z[1]+hrr*z[1]**2,z[0],z[1]))
        assert saved["candidate_cap"]==len(labels) and saved["coefficient_cap"]==B
        field_curve=[int(c)%p for c in curve]
        G=tuple(v.numerator*pow(v.denominator,-1,p)%p for v in Q)
        Rbar=tuple(v.numerator*pow(v.denominator,-1,p)%p for v in R)
        gfm=finite_multiples(G,B,field_curve,p)
        rfm=finite_multiples(Rbar,B,field_curve,p)
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
            eq=add(gfm[a],rfm[b],field_curve,p)
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
    assert max((bits for _,bits in cache.values()),default=0)==result["max_rational_coordinate_bits_constructed"]
    return {"status":"PASS","panel_primes":len(rows),
            "full_coverage_cells":sum(row["status"]=="full_coverage" for row in rows),
            "independent_rational_points_rebuilt":len(cache),
            "independently_verified_selection_tests":len(selection),
            "selection_rows_raw_sha256":sha(selection_raw),
            "prime_rows_raw_sha256":sha(raw)}


if __name__=="__main__":
    print(json.dumps(main(),sort_keys=True,indent=2))
