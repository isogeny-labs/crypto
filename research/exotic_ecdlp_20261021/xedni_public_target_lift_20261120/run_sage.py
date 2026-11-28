#!/usr/bin/env python3
"""Public-coordinate canonical rational lift search with same-point rho."""

import hashlib
import importlib.util
import json
import os
import platform
import resource
import signal
import time
from math import gcd,isqrt
from pathlib import Path

from sage.all import EllipticCurve,GF,QQ

HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[2]


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def cap(_signal,_frame):
    raise TimeoutError("per-arm wall cap")


def search(E,Ep,T,p,Hcap):
    xt,yt=int(T[0]),int(T[1])
    b_steps=residue_pairs=tests=roots=0
    H=0
    start=time.monotonic_ns()
    try:
        for H in range(1,Hcap+1):
            for b in range(1,H+1):
                b_steps+=1
                if b%p==0:
                    continue
                residue=xt*b*b%p
                if b==H:
                    first=-H+(residue+H)%p
                    aa=range(first,H+1,p)
                else:
                    aa=(a for a in (-H,H) if a%p==residue)
                for a in aa:
                    residue_pairs+=1
                    if gcd(abs(a),b)!=1:
                        continue
                    tests+=1
                    B=94*a*b+188*b**3
                    C=a**3+1738*a*b**4+3805*b**6
                    D=B*B+4*C
                    if D<0:
                        continue
                    s=isqrt(D)
                    if s*s!=D:
                        continue
                    for signed in ((s,) if s==0 else (s,-s)):
                        if (signed-B)%2:
                            continue
                        c=(signed-B)//2
                        roots+=1
                        if c*pow(b,-3,p)%p==yt:
                            P=E([QQ(a)/b**2,QQ(c)/b**3])
                            reduced=Ep([GF(p)(P[0].numerator())/GF(p)(P[0].denominator()),
                                        GF(p)(P[1].numerator())/GF(p)(P[1].denominator())])
                            assert reduced==T
                            stop=time.monotonic_ns()
                            return {"status":"found","height":H,"lift":[a,b,c],
                                    "b_steps":b_steps,"residue_pairs":residue_pairs,
                                    "discriminant_tests":tests,"rational_roots":roots,
                                    "start_ns":start,"stop_ns":stop,"online_ns":stop-start}
    except TimeoutError:
        stop=time.monotonic_ns()
        return {"status":"wall_cap_exhausted","height_reached":H,"lift":None,
                "b_steps":b_steps,"residue_pairs":residue_pairs,
                "discriminant_tests":tests,"rational_roots":roots,
                "start_ns":start,"stop_ns":stop,"online_ns":stop-start}
    stop=time.monotonic_ns()
    return {"status":"height_cap_exhausted","height":Hcap,"lift":None,
            "b_steps":b_steps,"residue_pairs":residue_pairs,
            "discriminant_tests":tests,"rational_roots":roots,
            "start_ns":start,"stop_ns":stop,"online_ns":stop-start}


def main():
    protocol_raw=(HERE/"protocol.json").read_bytes()
    protocol=json.loads(protocol_raw)
    search_source=ROOT/protocol["source_search"]
    assert sha(search_source.read_bytes())==protocol["source_search_raw_sha256"]
    rho_path=ROOT/protocol["source_rho_solver"]
    assert sha(rho_path.read_bytes())==protocol["source_rho_solver_raw_sha256"]
    spec=importlib.util.spec_from_file_location("xedni_public_rho_source",rho_path)
    rho_source=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(rho_source)
    panel=json.loads((ROOT/protocol["source_panel_protocol"]).read_bytes())
    assert panel["rational_curve_coefficients"]==[94,0,188,1738,3805]
    setup_start=time.monotonic_ns()
    E=EllipticCurve(QQ,panel["rational_curve_coefficients"])
    Q=E(panel["Q"])
    setup_ns=time.monotonic_ns()-setup_start
    signal.signal(signal.SIGALRM,cap)
    fixtures=[]
    rows=[]
    for cell in protocol["cells"]:
        p,q=cell["p"],cell["q"]
        Ep=EllipticCurve(GF(p),[GF(p)(z) for z in panel["rational_curve_coefficients"]])
        assert Ep.cardinality()==q
        G=Ep([GF(p)(Q[0]),GF(p)(Q[1])])
        assert G.order()==q
        scalar_specs=[("control",k,None) for k in protocol["control_scalars"]]
        random_seen=set()
        for j in range(protocol["random_targets_per_cell"]):
            digest=sha(f"{protocol['random_target_seed']}:{p}:{j}".encode())
            k=1+int(digest,16)%(q-1)
            assert k not in random_seen and k not in protocol["control_scalars"]
            random_seen.add(k)
            scalar_specs.append(("seeded",k,digest))
        for target_index,(kind,fixture_k,digest) in enumerate(scalar_specs):
            T=fixture_k*G
            target=[int(T[0]),int(T[1])]
            fixtures.append({"p":p,"q":q,"target_index":target_index,"input_kind":kind,
                             "target":target,"fixture_scalar":fixture_k,"target_digest":digest})
            order=("lift","rho") if target_index%2==0 else ("rho","lift")
            for position,arm in enumerate(order,1):
                seconds=(protocol["lift_wall_cap_seconds"] if arm=="lift"
                         else protocol["rho_wall_cap_seconds"])
                signal.setitimer(signal.ITIMER_REAL,seconds)
                try:
                    if arm=="lift":
                        r=search(E,Ep,T,p,protocol["height_cap"])
                    else:
                        r=rho_source.rho_solve(Ep,G,T,q,p,target_index,
                                               protocol["random_target_seed"])
                except Exception as e:
                    r={"status":"error","error":type(e).__name__,"detail":str(e)}
                finally:
                    signal.setitimer(signal.ITIMER_REAL,0)
                if arm=="rho" and r.get("status")=="verified":
                    assert r["scalar"]==fixture_k and r["scalar"]*G==T
                if arm=="lift" and kind=="control":
                    assert r["status"]=="found"
                rows.append({"p":p,"q":q,"target_index":target_index,"input_kind":kind,
                             "target":target,"position":position,"arm":arm,**r})
                (HERE/"progress.jsonl").write_text("".join(json.dumps(x,sort_keys=True)+"\n" for x in rows))
    raw="".join(json.dumps(x,sort_keys=True)+"\n" for x in rows).encode()
    fixtures_raw="".join(json.dumps(x,sort_keys=True)+"\n" for x in fixtures).encode()
    (HERE/"arm-rows.jsonl").write_bytes(raw)
    (HERE/"target-fixtures.jsonl").write_bytes(fixtures_raw)
    good=all(x["status"] in ("found","height_cap_exhausted","wall_cap_exhausted")
             if x["arm"]=="lift" else x["status"]=="verified" for x in rows)
    return {"schema":"xedni-public-target-lift-result-v1","status":"PASS" if good else "PARTIAL",
            "protocol_sha256":sha(protocol_raw),"script_sha256":sha((HERE/"run_sage.py").read_bytes()),
            "source_search_sha256":sha(search_source.read_bytes()),
            "source_rho_solver_sha256":sha(rho_path.read_bytes()),
            "arm_rows_sha256":sha(raw),"target_fixtures_sha256":sha(fixtures_raw),
            "targets":len(fixtures),"arms":len(rows),
            "lift_found":sum(x["arm"]=="lift" and x["status"]=="found" for x in rows),
            "lift_height_cap_exhausted":sum(x["arm"]=="lift" and x["status"]=="height_cap_exhausted" for x in rows),
            "lift_wall_cap_exhausted":sum(x["arm"]=="lift" and x["status"]=="wall_cap_exhausted" for x in rows),
            "rho_verified":sum(x["arm"]=="rho" and x["status"]=="verified" for x in rows),
            "setup_ns":setup_ns,
            "host":{"system":platform.system(),"machine":platform.machine(),
                    "cpu_count":os.cpu_count(),
                    "process_max_rss_bytes":resource.getrusage(resource.RUSAGE_SELF).ru_maxrss}}


if __name__=="__main__":
    print(json.dumps(main(),sort_keys=True,indent=2))
