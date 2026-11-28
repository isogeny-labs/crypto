"""Independent denominator-first census of the frozen rational lift law."""

import hashlib
import json
import math
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROOT=HERE.parent


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def candidate_records(x,cap):
    records=[]
    modular=0
    for b in range(1,cap+1):
        if b%47==0:continue
        residue=x*b*b%47
        first=-cap+(residue+cap)%47
        for a in range(first,cap+1,47):
            if max(abs(a),b)>cap:continue
            modular+=1
            if math.gcd(a,b)!=1:continue
            records.append((max(abs(a),b),b,a))
    return modular,records


def test_candidate(b,a,y_public):
    b2=b*b
    value=a*a*a-47*a*a*b2+751*a*b2*b2-3997*b2*b2*b2
    if value<0:return (False,False,None)
    root=math.isqrt(value)
    if root*root!=value:return (True,False,None)
    inv=pow((b*b*b)%47,-1,47)
    choices=(root,-root) if root else (0,)
    matched=next((c for c in choices if c*inv%47==y_public),None)
    return (True,True,matched)


def replay(xy,cap,stop_at_first):
    x,y=xy
    modular,records=candidate_records(x,cap)
    if stop_at_first:
        records.sort()
        # The runner stops at the first hit, so its modular count also stops.
        visited_modular=0
        visited_coprime=0
        for H in range(1,cap+1):
            for b in range(1,H+1):
                if b%47==0:continue
                residue=x*b*b%47
                possible=(range(-H+(residue+H)%47,H+1,47) if b==H else
                          (a for a in (-H,H) if a%47==residue))
                for a in possible:
                    visited_modular+=1
                    if math.gcd(a,b)!=1:continue
                    visited_coprime+=1
                    nonnegative,square,c=test_candidate(b,a,y)
                    if c is not None:
                        return {"modular_x_candidates":visited_modular,
                                "coprime_candidates":visited_coprime,
                                "exact_polynomial_tests":visited_coprime,
                                "nonnegative_square_tests":1 if nonnegative else 0,
                                "perfect_squares":1 if square else 0,
                                "reduced_y_matches":1},(H,b,a,c)
        raise AssertionError("control was not found")
    nonnegative=squares=matches=0
    for H,b,a in records:
        nonneg,square,c=test_candidate(b,a,y)
        nonnegative+=nonneg
        squares+=square
        matches+=c is not None
    return {"modular_x_candidates":modular,"coprime_candidates":len(records),
            "exact_polynomial_tests":len(records),
            "nonnegative_square_tests":nonnegative,"perfect_squares":squares,
            "reduced_y_matches":matches},None


def main():
    p=json.loads((HERE/"protocol.json").read_text())
    assert sha(ROOT/"xedni_rank1_generator_certificate_20261121"/"lift-class-result.json")==p["source_lift_class_sha256"]
    assert sha(ROOT/"xedni_process_fresh_target_20261121"/"result.json")==p["source_same_target_rho_sha256"]
    result=json.loads((HERE/"result.json").read_text())
    assert result["status"]=="PASS" and [a["arm"] for a in result["arms"]]==["control","target"]
    for arm in result["arms"]:
        label=arm["arm"]
        assert sha(HERE/f"{label}-checkpoints.jsonl")==result["checkpoints_sha256"][label]
        checkpoints=[json.loads(s) for s in (HERE/f"{label}-checkpoints.jsonl").read_text().splitlines()]
        assert checkpoints[-1]["completed_height"]==arm["completed_height"]
        assert checkpoints[-1]["counts"]==arm["counts"]
        assert arm["online_ns"]==arm["stop_ns"]-arm["start_ns"]>0
        counts,found=replay(arm["public_xy"],arm["cap"],label=="control")
        assert counts==arm["counts"]
        if label=="control":
            assert arm["status"]=="found" and found==(17,1,17,10)
            assert arm["found"]=={"a":17,"b":1,"c":10,"height":17,
                                  "rational_x":"17/1","rational_y":"10/1"}
        else:
            assert arm["status"]=="height_cap_exhausted" and arm["completed_height"]==4096
            assert arm["found"] is None and counts["perfect_squares"]==0
    print(json.dumps({"schema":"xedni-rank1-public-lift-cap-audit-v1","status":"PASS",
                      "control_found_height":17,"target_cap":4096,
                      "target_exact_polynomial_tests":result["arms"][1]["counts"]["exact_polynomial_tests"]},
                     indent=2,sort_keys=True))


if __name__=="__main__":main()
