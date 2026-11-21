#!/usr/bin/env python3
"""Exploratory same-target rational-lift and rho wall-time pairs."""

import hashlib
import json
import os
import platform
import resource
import statistics
import time
from decimal import Decimal, getcontext
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ

HERE = Path(__file__).resolve().parent


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def red(q):
    return int(q.numerator())*pow(int(q.denominator()),-1,47)%47


def lift_solve(E,Q,R,Ep,G,T,order):
    start = time.monotonic_ns()
    seen = {}
    target_label = None
    lam = None
    for count,(a,b) in enumerate(order,1):
        P = a*Q+b*R
        point = (red(P[0]),red(P[1]))
        if point == (int(T[0]),int(T[1])) and target_label is None:
            target_label = (a,b)
        if point in seen and lam is None:
            old_a,old_b = seen[point]
            db = b-old_b
            assert db%43
            lam = -(a-old_a)*pow(db,-1,43)%43
        seen.setdefault(point,(a,b))
        if target_label is not None and (target_label[1] == 0 or lam is not None):
            k = (target_label[0]+(lam or 0)*target_label[1])%43
            assert k*G == T
            stop = time.monotonic_ns()
            return {"scalar":k,"candidate_evaluations":count,"collision_scalar":lam,
                    "start_ns":start,"stop_ns":stop,"online_ns":stop-start,
                    "status":"verified"}
    stop = time.monotonic_ns()
    return {"scalar":None,"candidate_evaluations":len(order),"collision_scalar":lam,
            "start_ns":start,"stop_ns":stop,"online_ns":stop-start,"status":"height_box_exhausted"}


def rho_solve(Ep,G,T,pair_index):
    start = time.monotonic_ns()
    updates = 0
    for restart in range(1,44):
        seed = pair_index*17+restart
        a = seed%43
        b = (7*seed+3)%43
        X = a*G+b*T
        def step(state):
            nonlocal updates
            P,u,v = state
            choice = 0 if P == Ep(0) else int(P[0])%3
            updates += 1
            if choice == 0:
                return (P+G,(u+1)%43,v)
            if choice == 1:
                return (P+T,u,(v+1)%43)
            return (2*P,(2*u)%43,(2*v)%43)
        slow = (X,a,b)
        fast = (X,a,b)
        for iteration in range(1,10001):
            slow = step(slow)
            fast = step(step(fast))
            if slow[0] != fast[0]:
                continue
            den = (slow[2]-fast[2])%43
            if den == 0:
                break
            k = (fast[1]-slow[1])*pow(den,-1,43)%43
            assert k*G == T
            stop = time.monotonic_ns()
            return {"scalar":k,"seed_mod_43":seed%43,"walk_updates":updates,"iterations":iteration,
                    "restarts":restart-1,"start_ns":start,"stop_ns":stop,
                    "online_ns":stop-start,"status":"verified"}
    stop = time.monotonic_ns()
    return {"scalar":None,"walk_updates":updates,"restarts":43,
            "start_ns":start,"stop_ns":stop,"online_ns":stop-start,
            "status":"rho_cap_exhausted"}


def main():
    protocol_raw = (HERE / "protocol.json").read_bytes()
    protocol = json.loads(protocol_raw)
    setup_start = time.monotonic_ns()
    E = EllipticCurve(QQ,protocol["rational_curve_coefficients"])
    Q = E(protocol["Q"])
    R = E([QQ(s) for s in protocol["R"]])
    pairing = E.height_pairing_matrix([Q,R],precision=128)
    assert [[str(pairing[i,j]) for j in range(2)] for i in range(2)] == protocol["height_pairing_matrix"]
    getcontext().prec = 50
    hqq,hqr,hrr = [Decimal(s) for s in (protocol["height_pairing_matrix"][0][0],
                                          protocol["height_pairing_matrix"][0][1],
                                          protocol["height_pairing_matrix"][1][1])]
    order = [(a,b) for a in range(-4,5) for b in range(-4,5) if (a,b)!=(0,0)]
    order.sort(key=lambda x:(hqq*x[0]**2+2*hqr*x[0]*x[1]+hrr*x[1]**2,x[0],x[1]))
    Ep = EllipticCurve(GF(47),protocol["reduced_curve_coefficients"])
    G = Ep(protocol["reduced_generator"])
    T = Ep(protocol["target"])
    assert Ep.cardinality() == protocol["subgroup_order"] and G.order() == 43
    setup_stop = time.monotonic_ns()
    rows = []
    for pair in range(1,protocol["pairs"]+1):
        arms = ("lift","rho") if pair%2 else ("rho","lift")
        for position,arm in enumerate(arms,1):
            r = lift_solve(E,Q,R,Ep,G,T,order) if arm=="lift" else rho_solve(Ep,G,T,pair)
            rows.append({"pair":pair,"position":position,"arm":arm,**r})
    raw = "".join(json.dumps(row,sort_keys=True)+"\n" for row in rows).encode()
    (HERE / "pair-rows.jsonl").write_bytes(raw)
    assert len(rows)==2*protocol["pairs"] and all(row["status"]=="verified" for row in rows)
    assert len({row["scalar"] for row in rows})==1
    return {"schema":"xedni-same-target-timing-result-v1","status":"PASS",
            "protocol_sha256":sha(protocol_raw),"script_sha256":sha((HERE / "run_sage.py").read_bytes()),
            "pair_rows_raw_sha256":sha(raw),"target":protocol["target"],
            "setup_start_ns":setup_start,"setup_stop_ns":setup_stop,
            "setup_ns":setup_stop-setup_start,
            "paired_runs":protocol["pairs"],"verified_arms":len(rows),
            "lift_online_median_ns":statistics.median(row["online_ns"] for row in rows if row["arm"]=="lift"),
            "rho_online_median_ns":statistics.median(row["online_ns"] for row in rows if row["arm"]=="rho"),
            "host":{"system":platform.system(),"machine":platform.machine(),
                    "processor":platform.processor(),"cpu_count":os.cpu_count(),
                    "maxrss":resource.getrusage(resource.RUSAGE_SELF).ru_maxrss}}


if __name__ == "__main__":
    print(json.dumps(main(),sort_keys=True,indent=2))
