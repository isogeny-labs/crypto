#!/usr/bin/env python3
"""Paired baseline/deduplicated rational search and same-point rho."""

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


def locked(protocol,path_key,sha_key):
    raw=(ROOT/protocol[path_key]).read_bytes()
    assert sha(raw)==protocol[sha_key]
    return raw


def main():
    protocol_raw=(HERE/"protocol.json").read_bytes()
    protocol=json.loads(protocol_raw)
    threshold=json.loads(locked(protocol,"source_threshold_result","source_threshold_result_raw_sha256"))
    first_hit=json.loads(locked(protocol,"source_first_hit_result","source_first_hit_result_raw_sha256"))
    solver_path=ROOT/protocol["source_solver"]
    assert sha(solver_path.read_bytes())==protocol["source_solver_raw_sha256"]
    spec=importlib.util.spec_from_file_location("xedni_fresh_target_solver",solver_path)
    solver=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(solver)
    previous_raw=locked(protocol,"previous_target_fixtures","previous_target_fixtures_raw_sha256")
    previous={}
    for row in map(json.loads,previous_raw.splitlines()):
        previous.setdefault(row["p"],set()).add(row["fixture_scalar"])
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
    signal.signal(signal.SIGALRM,solver.timed_out)
    fixtures=[]
    rows=[]
    setup_rows=[]
    for cell,th,old_distribution in zip(protocol["cells"],threshold["thresholds"],first_hit["cells"]):
        p,q,lam,B=[cell[k] for k in ("p","q","R_over_Q","radius")]
        assert (th["p"],th["q"],th["first_full_radius"])==(p,q,B)
        assert (old_distribution["p"],old_distribution["q"],old_distribution["radius"])==(p,q,B)
        cell_start=time.monotonic_ns()
        Ep=EllipticCurve(GF(p),[GF(p)(z) for z in panel["rational_curve_coefficients"]])
        assert Ep.cardinality()==q
        G=Ep([GF(p)(Q[0]),GF(p)(Q[1])])
        Rbar=Ep([GF(p)(R[0].numerator())/GF(p)(R[0].denominator()),
                 GF(p)(R[1].numerator())/GF(p)(R[1].denominator())])
        assert G.order()==q and Rbar==lam*G
        labels=[(a,b) for a in range(-B,B+1) for b in range(-B,B+1) if (a,b)!=(0,0)]
        def score(label):
            a,b=label
            return hqq*a*a+2*hqr*a*b+hrr*b*b
        order_start=time.monotonic_ns()
        labels.sort(key=lambda z:(score(z),z))
        order_ns=time.monotonic_ns()-order_start
        filter_start=time.monotonic_ns()
        selected=[]
        seen=set()
        for a,b in labels:
            k=(a+lam*b)%q
            if k and k not in seen:
                seen.add(k)
                selected.append((a,b))
        filter_ns=time.monotonic_ns()-filter_start
        assert len(selected)==len(seen)==q-1
        assert len(labels)==old_distribution["candidate_cap"]
        setup_rows.append({"p":p,"q":q,"radius":B,"source_lambda":lam,
                           "full_candidate_count":len(labels),"unique_nonidentity_count":len(selected),
                           "curve_and_list_setup_ns":time.monotonic_ns()-cell_start,
                           "height_order_ns":order_ns,"dedup_filter_ns":filter_ns})
        used=set()
        for j in range(protocol["targets_per_cell"]):
            digest=sha(f"{protocol['target_seed']}:{p}:{j}".encode())
            fixture_k=1+int(digest,16)%(q-1)
            assert fixture_k not in used and fixture_k not in previous[p]
            used.add(fixture_k)
            T=fixture_k*G
            target=[int(T[0]),int(T[1])]
            fixtures.append({"p":p,"q":q,"target_index":j,"target":target,
                             "fixture_scalar":fixture_k,"target_digest":digest})
            for position,arm in enumerate(protocol["arm_order_by_target_index"][j],1):
                signal.setitimer(signal.ITIMER_REAL,protocol["time_cap_per_arm_seconds"])
                try:
                    if arm=="baseline":
                        r=solver.rational_solve(E,Q,R,Ep,G,T,p,q,lam,labels)
                    elif arm=="dedup":
                        r=solver.rational_solve(E,Q,R,Ep,G,T,p,q,lam,selected)
                    else:
                        assert arm=="rho"
                        r=solver.rho_solve(Ep,G,T,q,p,j,protocol["target_seed"])
                except Exception as e:
                    r={"status":"error","error":type(e).__name__,"detail":str(e)}
                finally:
                    signal.setitimer(signal.ITIMER_REAL,0)
                if r.get("status")=="verified":
                    assert r["scalar"]==fixture_k and r["scalar"]*G==T
                rows.append({"p":p,"q":q,"radius":B,"target_index":j,"target":target,
                             "position":position,"arm":arm,**r})
                (HERE/"progress.jsonl").write_text("".join(json.dumps(x,sort_keys=True)+"\n" for x in rows))
    rows_raw="".join(json.dumps(x,sort_keys=True)+"\n" for x in rows).encode()
    fixtures_raw="".join(json.dumps(x,sort_keys=True)+"\n" for x in fixtures).encode()
    setup_raw="".join(json.dumps(x,sort_keys=True)+"\n" for x in setup_rows).encode()
    (HERE/"arm-rows.jsonl").write_bytes(rows_raw)
    (HERE/"target-fixtures.jsonl").write_bytes(fixtures_raw)
    (HERE/"setup-rows.jsonl").write_bytes(setup_raw)
    return {"schema":"xedni-dedup-target-result-v1",
            "status":"PASS" if all(x["status"]=="verified" for x in rows) else "PARTIAL",
            "protocol_sha256":sha(protocol_raw),"script_sha256":sha((HERE/"run_sage.py").read_bytes()),
            "source_solver_sha256":sha(solver_path.read_bytes()),
            "arm_rows_sha256":sha(rows_raw),"target_fixtures_sha256":sha(fixtures_raw),
            "setup_rows_sha256":sha(setup_raw),"global_setup_ns":setup_ns,
            "targets":len(fixtures),"arms":len(rows),
            "verified_arms":sum(x["status"]=="verified" for x in rows),
            "host":{"system":platform.system(),"machine":platform.machine(),
                    "cpu_count":os.cpu_count(),
                    "process_max_rss_bytes":resource.getrusage(resource.RUSAGE_SELF).ru_maxrss}}


if __name__=="__main__":
    print(json.dumps(main(),sort_keys=True,indent=2))
