#!/usr/bin/env python3
"""256 fresh public targets through the frozen n29 target-shift descent policy."""
from sage.all import *
import hashlib,importlib.util,json,platform,resource,signal,time
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[3]
SPEC=json.loads((HERE/"protocol.json").read_text())
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
for key,hkey in (("candidate","candidate_sha256"),
                 ("precomputation_receipt","precomputation_receipt_sha256"),
                 ("candidate_runner","candidate_runner_sha256")):
    assert sha(ROOT/SPEC[key])==SPEC[hkey]
assert sha(HERE/"fixtures.json")==SPEC["fixtures_sha256"]
assert sha(HERE/"make_fixtures.py")==SPEC["fixture_generator_sha256"]
C=json.loads((ROOT/SPEC["candidate"]).read_text())
PREP=json.loads((ROOT/SPEC["precomputation_receipt"]).read_text())
FIX=json.loads((HERE/"fixtures.json").read_text())
assert FIX["candidate_id"]==C["candidate_id"]==PREP["candidate_id"]
assert len(FIX["targets"])==SPEC["target_count"]==256
module_spec=importlib.util.spec_from_file_location("h4_n29_candidate",ROOT/SPEC["candidate_runner"])
candidate=importlib.util.module_from_spec(module_spec)
module_spec.loader.exec_module(candidate)
def checkpoint():
    (HERE/"result.json").write_text(json.dumps(out,indent=2,sort_keys=True)+"\n")
def timeout(*_):raise TimeoutError("frozen process or target cap")
def calibration(G):
    t=time.perf_counter_ns();P=G
    for _ in range(512):P+=G
    assert P==513*G
    return time.perf_counter_ns()-t
signal.signal(signal.SIGALRM,timeout)
out={"schema":"h4-n29-shift-reliability-result-v1","status":"running",
     "protocol_sha256":sha(HERE/"protocol.json"),
     "fixture_sha256":SPEC["fixtures_sha256"],"candidate_id":C["candidate_id"],
     "host":{"node":platform.node(),"system":platform.system(),
             "machine":platform.machine(),"tier":"ordinary_exploratory"},
     "targets":[]}
checkpoint()
deadline=time.monotonic()+SPEC["process_cap_seconds"]
signal.alarm(SPEC["process_cap_seconds"])
try:
    setup_start=time.perf_counter_ns()
    E,elt=candidate.build_context()
    base,reps,encoding,G=candidate.build_base(E,elt)
    index=candidate.build_index(base)
    logs=PREP["factor_logs"]
    assert len(logs)==len(reps)==536
    assert all(logs[i]*G==reps[i] for i in range(len(reps)))
    assert [candidate.point_record(P) for P in base]==C["record"]["factor_base"]["encoded_points"]
    assert candidate.point_record(G)==FIX["generator"]
    assert len(index)==PREP["distinct_pair_sums"]
    out["shared_setup_ns"]=time.perf_counter_ns()-setup_start
    out["calibration_pre_ns"]=calibration(G)
    checkpoint()
    for fixture in FIX["targets"]:
        if time.monotonic()>=deadline:raise TimeoutError("frozen process cap")
        Q=E(elt(fixture["target"][0]),elt(fixture["target"][1]))
        assert candidate.point_record(Q)==fixture["target"]
        signal.alarm(min(SPEC["per_target_cap_seconds"],max(1,int(deadline-time.monotonic()))))
        start=time.perf_counter_ns()
        try:
            ic=candidate.ic_online(base,reps,encoding,G,index,logs,Q)
            ic["outer_wall_ns"]=time.perf_counter_ns()-start
        except TimeoutError as exc:
            ic={"status":"censored_timeout","scalar":None,
                "outer_wall_ns":time.perf_counter_ns()-start,"error":str(exc)}
        except Exception as exc:
            ic={"status":"error","scalar":None,
                "outer_wall_ns":time.perf_counter_ns()-start,"error":repr(exc)}
        signal.alarm(0)
        certificate=(ic["status"]=="verified" and
                     ic["scalar"]==fixture["fixture_scalar"] and ic["scalar"]*G==Q)
        out["targets"].append({"index":fixture["index"],"workload_id":fixture["workload_id"],
                               "target":fixture["target"],"target_point_sha256":fixture["target_point_sha256"],
                               "ic":ic,"scalar_certificate_pass":certificate,
                               "peak_rss_so_far_bytes":resource.getrusage(resource.RUSAGE_SELF).ru_maxrss})
        if len(out["targets"])%16==0:checkpoint()
        if out["targets"][-1]["peak_rss_so_far_bytes"]>SPEC["observed_peak_rss_bytes"]:
            out["status"]="censored_memory_limit";break
    out["calibration_post_ns"]=calibration(G)
    a,b=out["calibration_pre_ns"],out["calibration_post_ns"]
    out["calibration_ratio"]=max(a,b)/min(a,b)
    if out["status"]=="running":
        out["status"]="complete" if len(out["targets"])==256 else "incomplete"
except TimeoutError as exc:
    out.update(status="censored_process_timeout",error=str(exc))
finally:
    signal.alarm(0)
    out["peak_rss_bytes"]=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    checkpoint()
print(json.dumps({"status":out["status"],"targets":len(out["targets"]),
                  "verified":sum(t["scalar_certificate_pass"] for t in out["targets"]),
                  "shift_cap":sum(t["ic"]["status"]=="censored_shift_cap" for t in out["targets"]),
                  "peak_rss_bytes":out["peak_rss_bytes"]},sort_keys=True))
