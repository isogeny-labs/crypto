#!/usr/bin/env python3
"""Independent rational-height, finite-scalar, and sorted-lift replay."""

import hashlib
import importlib.util
import json
from fractions import Fraction
from math import isqrt
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
FAMILY_AUDIT = ROOT / "research/exotic_ecdlp_20261021/xedni_public_lift_curve_family_20261120/audit.py"
FAMILY_AUDIT_SHA = "2e258129dfc7d341a36a606d44b5e6344657f3d6eabde0a989faaf67246d5b21"


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def rational_add(P, Q, t):
    if P is None:
        return Q
    if Q is None:
        return P
    x,y = P
    u,v = Q
    if x == u and y == -v:
        return None
    if P == Q:
        m = (3*x*x+t)/(2*y)
    else:
        m = (v-y)/(u-x)
    nu = m*m-x-u
    nv = m*(x-nu)-y
    assert nv*nv == nu**3+t*nu+1
    return nu,nv


def rational_multiple(k, t):
    R = None
    Q = (Fraction(0),Fraction(1))
    while k:
        if k & 1:
            R = rational_add(R,Q,t)
        Q = rational_add(Q,Q,t)
        k >>= 1
    return R


def main():
    assert sha(FAMILY_AUDIT.read_bytes()) == FAMILY_AUDIT_SHA
    spec = importlib.util.spec_from_file_location("xedni_family_independent_audit",FAMILY_AUDIT)
    family = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(family)
    protocol = json.loads((HERE/"protocol.json").read_text())
    result_raw = (HERE/"result.json").read_bytes()
    result = json.loads(result_raw)
    data = {}
    for name,digest in result["output_hashes"].items():
        raw = (HERE/name).read_bytes()
        assert sha(raw) == digest
        data[name] = [json.loads(x) for x in raw.splitlines()]
    fixtures = data["target-fixtures.jsonl"]
    rows = data["arm-rows.jsonl"]
    checks = []
    for f in fixtures:
        p,t,q,k = f["p"],f["t"],f["q"],f["k"]
        target = tuple(f["target"])
        assert family.multiply(k,(0,1),p,t) == target
        rat = rational_multiple(k,t)
        assert rat is not None
        x,y = rat
        a,d = x.numerator,x.denominator
        b = isqrt(d)
        assert b*b == d
        assert f["known_rational_x_height"] == max(abs(a),b)
        assert f["known_rational_x_numerator_bits"] == abs(a).bit_length()
        assert f["known_rational_x_denominator_bits"] == d.bit_length()
        assert a*pow(b,-2,p)%p == target[0]
        assert y.numerator*pow(y.denominator,-1,p)%p == target[1]
        matching = [r for r in rows if (r["p"],r["t"],r["k"]) == (p,t,k)]
        assert len(matching) == 2 and {r["arm"] for r in matching} == {"lift","rho"}
        lift = next(r for r in matching if r["arm"] == "lift")
        rho = next(r for r in matching if r["arm"] == "rho")
        assert rho["status"] == "verified" and rho["scalar"] == k
        assert family.multiply(rho["scalar"],(0,1),p,t) == target
        expected = family.check_lift(target,p,t,protocol["height_cap"])
        for key,value in expected.items():
            assert lift[key] == value,(p,t,k,key,lift[key],value)
        checks.append({"p":p,"t":t,"k":k,"known_height":max(abs(a),b),
                       "lift_status":lift["status"],"lift_height":lift.get("height"),
                       "rational_multiple_checked":True,"rho_scalar_checked":True,
                       "sorted_lift_checked":True})
    assert len(checks) == len(protocol["curves"])*len(protocol["scalars"])
    return {"schema":"xedni-small-scalar-ladder-audit-v1",
            "status":"PASS","result_sha256":sha(result_raw),"checks":checks}


if __name__ == "__main__":
    print(json.dumps(main(),sort_keys=True,indent=2))
