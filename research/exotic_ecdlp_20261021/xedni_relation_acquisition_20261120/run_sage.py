#!/usr/bin/env python3
"""Same-public-point timing of rational collision acquisition and rho."""

import hashlib
import importlib.util
import json
import os
import platform
import resource
import signal
import time
from decimal import Decimal, getcontext
from pathlib import Path

from sage.all import EllipticCurve, GF, QQ

HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[2]


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def cap(_signal,_frame):
    raise TimeoutError("per-arm wall cap")


def collision_solve(E,Q,R,Ep,G,T,p,q,labels):
    seen={}
    attempts=0
    completed=0
    start=time.monotonic_ns()
    try:
        for a,b in labels:
            attempts+=1
            P=a*Q+b*R
            assert P!=E(0)
            if int(P[0].denominator())%p==0:
                reduced=Ep(0)
            else:
                reduced=Ep([GF(p)(P[0].numerator())/GF(p)(P[0].denominator()),
                            GF(p)(P[1].numerator())/GF(p)(P[1].denominator())])
            completed+=1
            relation=None
            if reduced==Ep(0):
                relation={"kind":"zero_reduction","labels":[[a,b]],"delta":[a,b]}
            elif reduced in seen:
                old=seen[reduced]
                relation={"kind":"finite_collision","labels":[list(old),[a,b]],
                          "delta":[a-old[0],b-old[1]]}
            else:
                seen[reduced]=(a,b)
            if relation is not None:
                da,db=relation["delta"]
                assert db%q
                lam=-da*pow(db,-1,q)%q
                assert lam*G==T
                stop=time.monotonic_ns()
                return {"status":"verified","scalar":lam,"relation":relation,
                        "candidate_evaluations":completed,"attempts_started":attempts,
                        "start_ns":start,"stop_ns":stop,"online_ns":stop-start}
    except TimeoutError:
        stop=time.monotonic_ns()
        return {"status":"wall_cap_exhausted","scalar":None,
                "candidate_evaluations":completed,"attempts_started":attempts,
                "start_ns":start,"stop_ns":stop,"online_ns":stop-start}
    stop=time.monotonic_ns()
    return {"status":"coefficient_cap_exhausted","scalar":None,
            "candidate_evaluations":completed,"attempts_started":attempts,
            "start_ns":start,"stop_ns":stop,"online_ns":stop-start}


def main():
    protocol_raw=(HERE/"protocol.json").read_bytes()
    protocol=json.loads(protocol_raw)
    source_raw=(ROOT/protocol["source_prime_rows"]).read_bytes()
    assert sha(source_raw)==protocol["source_prime_rows_raw_sha256"]
    source={x["p"]:x for x in map(json.loads,source_raw.splitlines())}
    solver_path=ROOT/protocol["source_rho_solver"]
    assert sha(solver_path.read_bytes())==protocol["source_rho_solver_raw_sha256"]
    spec=importlib.util.spec_from_file_location("xedni_rho_source",solver_path)
    rho_source=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(rho_source)
    panel=json.loads((ROOT/protocol["source_panel_protocol"]).read_bytes())
    setup_start=time.monotonic_ns()
    E=EllipticCurve(QQ,panel["rational_curve_coefficients"])
    Q=E(panel["Q"])
    R=E([QQ(z) for z in panel["R"]])
    pairing=E.height_pairing_matrix([Q,R],precision=panel["height_pairing_precision_bits"])
    assert [[str(pairing[i,j]) for j in range(2)] for i in range(2)]==panel["height_pairing_matrix"]
    getcontext().prec=50
    hqq,hqr,hrr=[Decimal(z) for z in (panel["height_pairing_matrix"][0][0],
                                        panel["height_pairing_matrix"][0][1],
                                        panel["height_pairing_matrix"][1][1])]
    setup_ns=time.monotonic_ns()-setup_start
    signal.signal(signal.SIGALRM,cap)
    rows=[]
    setup_rows=[]
    for cell in protocol["cells"]:
        p,q,B=[cell[k] for k in ("p","q","coefficient_radius")]
        old=source[p]
        assert (old["q"],old["coefficient_cap"],old["first_relation"]["derived_R_over_G"])==(
            q,B,cell["expected_lambda_for_postsolve_audit"])
        cell_start=time.monotonic_ns()
        Ep=EllipticCurve(GF(p),[GF(p)(z) for z in panel["rational_curve_coefficients"]])
        assert Ep.cardinality()==q
        G=Ep([GF(p)(Q[0]),GF(p)(Q[1])])
        T=Ep([GF(p)(R[0].numerator())/GF(p)(R[0].denominator()),
              GF(p)(R[1].numerator())/GF(p)(R[1].denominator())])
        assert G.order()==q and T!=Ep(0)
        labels=[(a,b) for a in range(-B,B+1) for b in range(-B,B+1) if (a,b)!=(0,0)]
        labels.sort(key=lambda z:(hqq*z[0]**2+2*hqr*z[0]*z[1]+hrr*z[1]**2,z))
        setup_rows.append({"p":p,"q":q,"coefficient_radius":B,
                           "candidate_cap":len(labels),"cell_setup_ns":time.monotonic_ns()-cell_start,
                           "public_target":[int(T[0]),int(T[1])]})
        for repetition in range(1,protocol["repetitions_per_cell"]+1):
            order=("collision","rho") if repetition%2 else ("rho","collision")
            for position,arm in enumerate(order,1):
                signal.setitimer(signal.ITIMER_REAL,protocol["time_cap_per_arm_seconds"])
                try:
                    if arm=="collision":
                        result=collision_solve(E,Q,R,Ep,G,T,p,q,labels)
                    else:
                        result=rho_source.rho_solve(Ep,G,T,q,p,repetition,protocol["rho_seed_text"])
                except Exception as e:
                    result={"status":"error","error":type(e).__name__,"detail":str(e)}
                finally:
                    signal.setitimer(signal.ITIMER_REAL,0)
                if result.get("status")=="verified":
                    assert result["scalar"]==cell["expected_lambda_for_postsolve_audit"]
                    assert result["scalar"]*G==T
                rows.append({"p":p,"q":q,"coefficient_radius":B,"repetition":repetition,
                             "position":position,"arm":arm,"public_target":[int(T[0]),int(T[1])],
                             **result})
                (HERE/"progress.jsonl").write_text("".join(json.dumps(x,sort_keys=True)+"\n" for x in rows))
    rows_raw="".join(json.dumps(x,sort_keys=True)+"\n" for x in rows).encode()
    setup_raw="".join(json.dumps(x,sort_keys=True)+"\n" for x in setup_rows).encode()
    (HERE/"pair-rows.jsonl").write_bytes(rows_raw)
    (HERE/"setup-rows.jsonl").write_bytes(setup_raw)
    return {"schema":"xedni-relation-acquisition-result-v1",
            "status":"PASS" if all(x["status"]=="verified" for x in rows) else "PARTIAL",
            "protocol_sha256":sha(protocol_raw),"script_sha256":sha((HERE/"run_sage.py").read_bytes()),
            "source_prime_rows_raw_sha256":sha(source_raw),"source_rho_solver_sha256":sha(solver_path.read_bytes()),
            "pair_rows_sha256":sha(rows_raw),"setup_rows_sha256":sha(setup_raw),
            "global_setup_ns":setup_ns,"pairs":len(rows)//2,"verified_arms":sum(x["status"]=="verified" for x in rows),
            "host":{"system":platform.system(),"machine":platform.machine(),
                    "cpu_count":os.cpu_count(),
                    "process_max_rss_bytes":resource.getrusage(resource.RUSAGE_SELF).ru_maxrss}}


if __name__=="__main__":
    print(json.dumps(main(),sort_keys=True,indent=2))
