"""Public-coordinate-only rational lift search under a frozen height law."""

import hashlib
import json
import math
import time
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROOT=HERE.parent


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def search(label,xy,cap,checkpoint_interval):
    x_public,y_public=xy
    counts={"modular_x_candidates":0,"coprime_candidates":0,
            "exact_polynomial_tests":0,"nonnegative_square_tests":0,
            "perfect_squares":0,"reduced_y_matches":0}
    start=time.monotonic_ns()
    found=None
    completed=0
    with (HERE/f"{label}-checkpoints.jsonl").open("w") as checkpoints:
        for H in range(1,cap+1):
            for b in range(1,H+1):
                if b%47==0:continue
                residue=x_public*b*b%47
                if b==H:
                    first=-H+(residue+H)%47
                    candidates=range(first,H+1,47)
                else:
                    candidates=(a for a in (-H,H) if a%47==residue)
                for a in candidates:
                    counts["modular_x_candidates"]+=1
                    if math.gcd(a,b)!=1:continue
                    counts["coprime_candidates"]+=1
                    b2=b*b
                    c2=a*a*a-47*a*a*b2+751*a*b2*b2-3997*b2*b2*b2
                    counts["exact_polynomial_tests"]+=1
                    if c2<0:continue
                    counts["nonnegative_square_tests"]+=1
                    c=math.isqrt(c2)
                    if c*c!=c2:continue
                    counts["perfect_squares"]+=1
                    inv=pow((b*b*b)%47,-1,47)
                    for signed_c in (c,-c) if c else (0,):
                        if signed_c*inv%47!=y_public:continue
                        counts["reduced_y_matches"]+=1
                        found={"a":a,"b":b,"c":signed_c,"height":H,
                               "rational_x":f"{a}/{b*b}","rational_y":f"{signed_c}/{b*b*b}"}
                        break
                    if found is not None:break
                if found is not None:break
            completed=H
            if H%checkpoint_interval==0 or found is not None or H==cap:
                checkpoints.write(json.dumps({"completed_height":H,"counts":counts},sort_keys=True)+"\n")
                checkpoints.flush()
            if found is not None:break
    stop=time.monotonic_ns()
    return {"arm":label,"public_xy":xy,"status":"found" if found else "height_cap_exhausted",
            "completed_height":completed,"cap":cap,"found":found,"counts":counts,
            "start_ns":start,"stop_ns":stop,"online_ns":stop-start}


def main():
    p=json.loads((HERE/"protocol.json").read_text())
    assert sha(ROOT/"xedni_rank1_generator_certificate_20261121"/"lift-class-result.json")==p["source_lift_class_sha256"]
    assert sha(ROOT/"xedni_process_fresh_target_20261121"/"result.json")==p["source_same_target_rho_sha256"]
    results=[]
    for label,xy,cap in (("control",p["positive_control_public_xy"],p["control_height_cap"]),
                         ("target",p["target_public_xy"],p["target_height_cap"])):
        results.append(search(label,xy,cap,p["checkpoint_interval_heights"]))
    print(json.dumps({"schema":"xedni-rank1-public-lift-cap-result-v1",
                      "status":"PASS" if results[0]["status"]=="found" and results[1]["status"]=="height_cap_exhausted" else "REVIEW",
                      "arms":results,
                      "checkpoints_sha256":{label:sha(HERE/f"{label}-checkpoints.jsonl") for label in ("control","target")}},
                     indent=2,sort_keys=True))


if __name__=="__main__":main()
