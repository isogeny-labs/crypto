"""Independent exact-Fraction replay of the target lift class."""

import hashlib
import json
from decimal import Decimal, getcontext
from fractions import Fraction as F
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROOT=HERE.parent
O=None


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def add(P,Q):
    if P is O:return Q
    if Q is O:return P
    x1,y1=P;x2,y2=Q
    if x1==x2 and y1==-y2:return O
    slope=((3*x1*x1-94*x1+751)/(2*y1) if P==Q else (y2-y1)/(x2-x1))
    x3=slope*slope+47-x1-x2
    return (x3,slope*(x1-x3)-y1)


def mul(n,Q):
    if n<0:
        x,y=mul(-n,Q)
        return (x,-y)
    R=O
    while n:
        if n&1:R=add(R,Q)
        Q=add(Q,Q);n//=2
    return R


def bits(P):
    return max(max(abs(v.numerator).bit_length(),v.denominator.bit_length()) for v in P)


def mod(v):
    return v.numerator*pow(v.denominator,-1,47)%47


def main():
    p=json.loads((HERE/"protocol.json").read_text())
    q=json.loads((HERE/"lift-class-protocol.json").read_text())
    generator=json.loads((HERE/"result.json").read_text())
    lifts=json.loads((HERE/"lift-class-result.json").read_text())
    assert sha(ROOT/"xedni_anchor_rank1_search_20261121"/"final-audit-result.json")==p["source_rank_audit_sha256"]
    assert sha(ROOT/"xedni_process_fresh_target_20261121"/"result.json")==p["source_target_result_sha256"]
    assert sha(HERE/"result.json")==q["source_generator_result_sha256"]
    assert generator["status"]==lifts["status"]=="PASS"
    assert generator["rank"]==1 and generator["generators_certain"] and generator["torsion_order"]==1
    assert generator["generator_xy"]==["17","-10"] and generator["Q_xy"]==["17","10"]
    assert generator["Q_equals_signed_generator"] and generator["bounded_multiplier_matches"]==[-1]
    assert lifts["target"]==q["reduced_target"]==[10,1]
    assert all(n%43!=22 for n in range(-20,21)) and (-21)%43==22
    assert [r["n"] for r in lifts["rows"]]==q["representatives_to_compute"]
    Q=(F(17),F(10))
    getcontext().prec=50
    hQ=Decimal(lifts["Q_canonical_height_128bit"])
    assert hQ>0
    for r in lifts["rows"]:
        n=r["n"]
        P=mul(n,Q)
        x,y=P
        assert y*y==x**3-47*x*x+751*x-3997
        assert (mod(x),mod(y))==(10,1)
        assert bits(P)==r["affine_coordinate_max_bits"]
        assert r["n_mod_43"]==n%43==22
        ratio=Decimal(r["height_ratio_to_Q_128bit"])
        assert abs(ratio-Decimal(n*n))<Decimal(2)**(-60)
    assert next(r for r in lifts["rows"] if r["n"]==-21)["affine_coordinate_max_bits"]==455
    print(json.dumps({"schema":"xedni-rank1-generator-audit-v1","status":"PASS",
                      "generator_sign":"Q=-generator","torsion_order":1,"rank":1,
                      "target_class_mod_43":22,"minimum_abs_multiplier":21,
                      "exact_replayed_lifts":len(lifts["rows"])},indent=2,sort_keys=True))


if __name__=="__main__":main()
