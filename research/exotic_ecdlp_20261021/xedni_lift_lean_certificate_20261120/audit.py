#!/usr/bin/env python3
"""Independent integer replay of the Lean height-64 certificate instance."""

import hashlib
import json
from math import gcd
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def main():
    protocol = json.loads((HERE / "protocol.json").read_text())
    for key in ("source_fixtures", "source_arms"):
        assert sha((ROOT / protocol[key]).read_bytes()) == protocol[key + "_sha256"]
    fixture = [json.loads(s) for s in (ROOT / protocol["source_fixtures"]).read_text().splitlines()
               if json.loads(s)["p"] == 1009 and json.loads(s)["t"] == 9
               and json.loads(s)["k"] == 3]
    assert len(fixture) == 1 and fixture[0]["target"] == [744,244]
    arms = [json.loads(s) for s in (ROOT / protocol["source_arms"]).read_text().splitlines()
            if json.loads(s)["p"] == 1009 and json.loads(s)["t"] == 9
            and json.loads(s)["k"] == 3]
    assert len(arms) == 2
    assert next(x for x in arms if x["arm"] == "rho")["scalar"] == 3
    assert next(x for x in arms if x["arm"] == "lift")["status"] == "height_cap_exhausted"
    p,t,x,_y = 1009,9,744,244
    cap = 64
    pairs = [(a,b) for b in range(1,cap+1) for a in range(-cap,cap+1)
             if (a-x*b*b)%p == 0]
    assert pairs == [(-51,2),(-55,20),(61,32),(-11,33)]
    checks = []
    for a,b in pairs:
        C = a**3+t*a*b**4+b**6
        entry = {"a":a,"b":b,"coprime":gcd(abs(a),b)==1,"curve_rhs":C}
        if entry["coprime"]:
            assert C < 0 or C%3 == 2
        checks.append(entry)
    return {"schema":"xedni-lift-lean-certificate-audit-v1","status":"PASS",
            "source_target":[744,244],"height_cap":64,
            "x_candidates":checks,"canonical_lift_count":0}


if __name__ == "__main__":
    print(json.dumps(main(),sort_keys=True,indent=2))
