#!/usr/bin/env python3
"""Frozen fresh-target rational search, finite control, and same-point rho."""

import hashlib
import json
import os
import platform
import resource
import signal
import time
from decimal import Decimal, getcontext
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ, ZZ

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def timed_out(_signal,_frame):
    raise TimeoutError("120-second arm cap")


def rational_solve(E,Q,R,Ep,G,T,p,q,lam,labels):
    attempts = 0
    completed = 0
    start = time.monotonic_ns()
    try:
        for a,b in labels:
            attempts += 1
            P = a*Q+b*R
            if int(P[0].denominator())%p == 0:
                reduced = Ep(0)
            else:
                reduced = Ep([GF(p)(P[0].numerator())/GF(p)(P[0].denominator()),
                              GF(p)(P[1].numerator())/GF(p)(P[1].denominator())])
            completed += 1
            if reduced == T:
                k = (a+lam*b)%q
                assert k*G == T
                stop = time.monotonic_ns()
                return {"status":"verified","scalar":k,"matched_label":[a,b],
                        "attempts_started":attempts,"candidate_evaluations":completed,
                        "start_ns":start,"stop_ns":stop,"online_ns":stop-start}
    except TimeoutError:
        stop = time.monotonic_ns()
        return {"status":"wall_cap_exhausted","scalar":None,
                "attempts_started":attempts,"candidate_evaluations":completed,
                "start_ns":start,"stop_ns":stop,"online_ns":stop-start}
    stop = time.monotonic_ns()
    return {"status":"candidate_cap_exhausted","scalar":None,
            "attempts_started":attempts,"candidate_evaluations":completed,
            "start_ns":start,"stop_ns":stop,"online_ns":stop-start}


def finite_solve(G,T,q,lam,labels):
    attempts = 0
    completed = 0
    start = time.monotonic_ns()
    try:
        for a,b in labels:
            attempts += 1
            k = (a+lam*b)%q
            candidate = k*G
            completed += 1
            if candidate == T:
                assert k*G == T
                stop = time.monotonic_ns()
                return {"status":"verified","scalar":k,"matched_label":[a,b],
                        "attempts_started":attempts,"candidate_evaluations":completed,
                        "start_ns":start,"stop_ns":stop,"online_ns":stop-start}
    except TimeoutError:
        stop = time.monotonic_ns()
        return {"status":"wall_cap_exhausted","scalar":None,
                "attempts_started":attempts,"candidate_evaluations":completed,
                "start_ns":start,"stop_ns":stop,"online_ns":stop-start}
    stop = time.monotonic_ns()
    return {"status":"candidate_cap_exhausted","scalar":None,
            "attempts_started":attempts,"candidate_evaluations":completed,
            "start_ns":start,"stop_ns":stop,"online_ns":stop-start}


def rho_solve(Ep,G,T,q,p,j,seed_text):
    updates = 0
    degenerate = 0
    start = time.monotonic_ns()
    try:
        for restart in range(64):
            seed = int(sha(f"{seed_text}:rho:{p}:{j}:{restart}".encode()),16)
            a,b = seed%q,(seed//q)%q
            X = a*G+b*T
            def step(state):
                nonlocal updates
                P,u,v = state
                bucket = 0 if P == Ep(0) else int(P[0])%3
                updates += 1
                if bucket == 0:
                    return P+G,(u+1)%q,v
                if bucket == 1:
                    return P+T,u,(v+1)%q
                return 2*P,(2*u)%q,(2*v)%q
            slow = (X,a,b)
            fast = (X,a,b)
            for iteration in range(1,100001):
                slow = step(slow)
                fast = step(step(fast))
                if slow[0] != fast[0]:
                    continue
                den = (slow[2]-fast[2])%q
                if den == 0:
                    degenerate += 1
                    break
                k = (fast[1]-slow[1])*pow(den,-1,q)%q
                assert k*G == T
                stop = time.monotonic_ns()
                return {"status":"verified","scalar":k,"walk_updates":updates,
                        "degenerate_collisions":degenerate,"restarts":restart,
                        "collision_iteration":iteration,"start_ns":start,"stop_ns":stop,
                        "online_ns":stop-start}
    except TimeoutError:
        stop = time.monotonic_ns()
        return {"status":"wall_cap_exhausted","scalar":None,"walk_updates":updates,
                "degenerate_collisions":degenerate,"start_ns":start,"stop_ns":stop,
                "online_ns":stop-start}
    stop = time.monotonic_ns()
    return {"status":"rho_cap_exhausted","scalar":None,"walk_updates":updates,
            "degenerate_collisions":degenerate,"restarts":64,"start_ns":start,
            "stop_ns":stop,"online_ns":stop-start}


def main():
    protocol_raw = (HERE/"protocol.json").read_bytes()
    protocol = json.loads(protocol_raw)
    source_raw = (ROOT/protocol["source_threshold_result"]).read_bytes()
    assert sha(source_raw) == protocol["source_threshold_result_raw_sha256"]
    source = json.loads(source_raw)
    panel = json.loads((ROOT/protocol["source_panel_protocol"]).read_bytes())
    setup_start = time.monotonic_ns()
    E = EllipticCurve(QQ,panel["rational_curve_coefficients"])
    Q = E(panel["Q"])
    R = E([QQ(z) for z in panel["R"]])
    pairing = E.height_pairing_matrix([Q,R],precision=panel["height_pairing_precision_bits"])
    assert [[str(pairing[i,k]) for k in range(2)] for i in range(2)] == panel["height_pairing_matrix"]
    getcontext().prec = 50
    hqq,hqr,hrr = [Decimal(z) for z in (panel["height_pairing_matrix"][0][0],
                                        panel["height_pairing_matrix"][0][1],
                                        panel["height_pairing_matrix"][1][1])]
    def score(label):
        a,b=label
        return hqq*a*a+2*hqr*a*b+hrr*b*b
    setup_stop = time.monotonic_ns()
    fixtures=[]
    rows=[]
    signal.signal(signal.SIGALRM,timed_out)
    for cell,th in zip(protocol["cells"],source["thresholds"]):
        p,q,lam,B=[cell[key] for key in ("p","q","R_over_Q","radius")]
        assert (th["p"],th["q"],th["first_full_radius"])==(p,q,B)
        assert ZZ(E.discriminant())%p and ZZ(q).is_prime()
        cell_setup_start = time.monotonic_ns()
        Ep = EllipticCurve(GF(p),[GF(p)(z) for z in panel["rational_curve_coefficients"]])
        assert Ep.cardinality()==q
        G=Ep([GF(p)(Q[0]),GF(p)(Q[1])])
        Rbar=Ep([GF(p)(R[0].numerator())/GF(p)(R[0].denominator()),
                 GF(p)(R[1].numerator())/GF(p)(R[1].denominator())])
        assert G.order()==q and Rbar==lam*G
        labels=[(a,b) for a in range(-B,B+1) for b in range(-B,B+1) if (a,b)!=(0,0)]
        labels.sort(key=lambda z:(score(z),z))
        cell_setup_ns = time.monotonic_ns()-cell_setup_start
        seen_k=set()
        for j in range(protocol["targets_per_cell"]):
            digest=sha(f"{protocol['target_seed']}:{p}:{j}".encode())
            fixture_k=1+int(digest,16)%(q-1)
            assert fixture_k not in seen_k
            seen_k.add(fixture_k)
            T=fixture_k*G
            target=[int(T[0]),int(T[1])]
            fixtures.append({"p":p,"q":q,"target_index":j,"target":target,
                             "fixture_scalar":fixture_k,"target_digest":digest})
            order=protocol["arm_order_by_target_index"][j]
            for position,arm in enumerate(order,1):
                signal.setitimer(signal.ITIMER_REAL,protocol["time_cap_per_arm_seconds"])
                try:
                    if arm=="rational":
                        r=rational_solve(E,Q,R,Ep,G,T,p,q,lam,labels)
                    elif arm=="finite":
                        r=finite_solve(G,T,q,lam,labels)
                    else:
                        assert arm=="rho"
                        r=rho_solve(Ep,G,T,q,p,j,protocol["target_seed"])
                except Exception as e:
                    r={"status":"error","error":type(e).__name__,"detail":str(e)}
                finally:
                    signal.setitimer(signal.ITIMER_REAL,0)
                if r.get("status")=="verified":
                    assert r["scalar"]==fixture_k and r["scalar"]*G==T
                rows.append({"p":p,"q":q,"radius":B,"target_index":j,"target":target,
                             "position":position,"arm":arm,"cell_setup_ns":cell_setup_ns,**r})
                (HERE/"progress.jsonl").write_text("".join(json.dumps(x,sort_keys=True)+"\n" for x in rows))
    rows_raw="".join(json.dumps(x,sort_keys=True)+"\n" for x in rows).encode()
    fixture_raw="".join(json.dumps(x,sort_keys=True)+"\n" for x in fixtures).encode()
    (HERE/"arm-rows.jsonl").write_bytes(rows_raw)
    (HERE/"target-fixtures.jsonl").write_bytes(fixture_raw)
    return {"schema":"xedni-fresh-target-pair-result-v1",
            "status":"PASS" if all(r["status"]=="verified" for r in rows) else "PARTIAL",
            "protocol_sha256":sha(protocol_raw),"script_sha256":sha((HERE/"run_sage.py").read_bytes()),
            "source_threshold_result_raw_sha256":sha(source_raw),
            "arm_rows_sha256":sha(rows_raw),"target_fixtures_sha256":sha(fixture_raw),
            "targets":len(fixtures),"arms":len(rows),
            "verified_arms":sum(r["status"]=="verified" for r in rows),
            "setup_start_ns":setup_start,"setup_stop_ns":setup_stop,"setup_ns":setup_stop-setup_start,
            "host":{"system":platform.system(),"machine":platform.machine(),
                    "cpu_count":os.cpu_count(),
                    "process_max_rss_bytes":resource.getrusage(resource.RUSAGE_SELF).ru_maxrss}}


if __name__=="__main__":
    print(json.dumps(main(),sort_keys=True,indent=2))
