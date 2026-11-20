#!/usr/bin/env python3
"""Independent n35 direct-integer-index full-pipeline audit."""
from sage.all import *
from sage.groups.generic import discrete_log_rho
from collections import Counter
import gzip
import hashlib, inspect, json, random
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
SPEC = json.loads((HERE / "protocol.json").read_text())
RESULT_PATH = HERE / "result.json"
RESULT = json.loads(RESULT_PATH.read_text())
C = json.loads((HERE / "candidate.json").read_text())
W = json.loads((HERE / "workload.json").read_text())
SOURCE = json.loads((ROOT / SPEC["source_instance_result"]).read_text())
assert RESULT["status"] == "complete"
def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()
def digest(obj): return hashlib.sha256(json.dumps(obj,sort_keys=True,separators=(",",":"),ensure_ascii=False).encode()).hexdigest()
def enc(e): return sum(int(c)<<i for i,c in enumerate(e.polynomial().list()))
def rec(P): return [enc(P[0]),enc(P[1])]
assert sha(HERE / "run.py") == SPEC["runner_sha256"]
assert sha(HERE / "protocol.json") == RESULT["protocol_sha256"]
assert sha(ROOT / SPEC["source_instance_result"]) == SPEC["source_instance_sha256"]
assert hashlib.sha256(inspect.getsource(discrete_log_rho).encode()).hexdigest() == SPEC["sage_rho_source_sha256"]
record=C["record"]
candidate_id=f'IC1N35Ch4randfb4096PDP3mitmRCsampleLAgaussTDdescentISO0h{digest(record)[:12]}'
assert candidate_id==C["candidate_id"]==RESULT["candidate_id"]
curve=dict(record["curve"]);curve_id=curve.pop("curve_id")
assert curve_id==RESULT["curve_id"]=="EC1N35Ch4randh"+digest({"field":record["field"],"curve":curve})[:12]
assert record["implementation"]["source_sha256"]["this_runner"]==SPEC["runner_sha256"]
assert record["isogeny"]=="none"
assert W["workload_id"]==RESULT["workload_id"]=="W"+digest(W["record"])[:12]
assert RESULT["run_id"]==candidate_id+W["workload_id"]+"R1"
n=record["field"]["n"];poly=record["field"]["defining_polynomial_int"]
assert n==35 and poly==SOURCE["field_modulus"]
F2=GF(2);PR=PolynomialRing(F2,"z");z=PR.gen()
mod=sum(F2((poly>>i)&1)*z**i for i in range(n+1))
K=GF(1<<n,"z",modulus=mod)
def elt(v): return K([(v>>i)&1 for i in range(n)])
coeff=record["curve"]["coefficients"]
E=EllipticCurve(K,[1,elt(coeff["a2"]),0,0,elt(coeff["a6"])])
r=record["curve"]["r"]
assert r==SPEC["subgroup_order"]==520604191
assert int(E.cardinality())==record["curve"]["curve_order"]==SOURCE["group_order"]
def point(pair): return E(elt(pair[0]),elt(pair[1]))
G=point(record["curve"]["G"])
assert r*G==E(0) and rec(G)==SOURCE["generator"]
base=[point(pair) for pair in record["factor_base"]["encoded_points"]]
assert len(base)==RESULT["base_points"]==SOURCE["base_count"]==4096
assert digest([rec(P) for P in base])==record["factor_base"]["enumerated_set_digest"]==SOURCE["base_sha256"]
assert all(r*P==E(0) for P in base)
reps=sorted({min((P,-P),key=rec) for P in base},key=rec)
assert len(reps)==RESULT["folded_columns"]==2048
col_of={P:i for i,P in enumerate(reps)}
def row_for(indices):
    row=[0]*len(reps)
    for i in indices:
        P=base[i]; rep=min((P,-P),key=rec)
        j=col_of[rep];row[j]=(row[j]+(1 if P==rep else -1))%r
    return row
# Rebuild the first-pair table in column-major order with an explicit minimum rule.
def point_key(P):
    if P==E(0): return 1<<(2*n)
    return int(P[0].to_integer()) | (int(P[1].to_integer())<<n)
pair_index={}
for j,B in enumerate(base):
    for i,A in enumerate(base):
        key=point_key(A+B)
        payload=i*len(base)+j
        old=pair_index.get(key)
        if old is None or payload<old: pair_index[key]=payload
assert len(pair_index)==RESULT["distinct_pair_sums"]==SOURCE["distinct_pair_sums"]
assert RESULT["pair_entries"]==len(base)**2

anchor=[0]*len(reps);rep=min((G,-G),key=rec)
anchor[col_of[rep]]=1 if G==rep else -1
rows=[anchor];rhs=[1]
# Independently update modular pivots to check the full reported rank trajectory.
pivots={}
def insert(row):
    v=[x%r for x in row]
    for j,basis_row in sorted(pivots.items()):
        if v[j]:
            scale=v[j]
            for k in range(j,len(v)):
                v[k]=(v[k]-scale*basis_row[k])%r
    j=next((k for k,x in enumerate(v) if x),None)
    if j is None:return False
    inv=pow(v[j],-1,r)
    pivots[j]=[x*inv%r for x in v]
    return True
assert insert(anchor)
trace_meta=RESULT["relation_trace"]
trace_bytes=gzip.decompress((HERE/trace_meta["file"]).read_bytes())
assert hashlib.sha256(trace_bytes).hexdigest()==trace_meta["sha256"]
trace=json.loads(trace_bytes)
assert digest(trace)==trace_meta["sha256"] and len(trace)==trace_meta["rows"]
assert len(trace)==RESULT["relation_attempts"]
assert [x["query_number"] for x in trace]==list(range(1,len(trace)+1))
relation_rng=random.Random(61035)
for entry in trace:
    scalar=entry["known_scalar"]
    expected_scalar=relation_rng.randrange(1,r)
    while expected_scalar==RESULT["target_fixture_scalar"]:
        expected_scalar=relation_rng.randrange(1,r)
    assert scalar==expected_scalar
    start=relation_rng.randrange(len(base))
    assert entry["rank_before"]==len(pivots)
    Q=scalar*G
    if entry["status"]=="zero_yield":
        assert entry["witness_indices"] is None and entry["row_sha256"] is None
        assert entry["third_point_attempts"]==len(base)
        assert all(point_key(Q-C) not in pair_index for C in base)
        gained=False
    else:
        assert entry["status"]=="verified"
        indices=entry["witness_indices"]
        assert len(indices)==3 and all(0<=i<len(base) for i in indices)
        assert sum((base[i] for i in indices),E(0))==Q
        first=next((offset for offset in range(len(base))
                    if point_key(Q-base[(start+offset)%len(base)]) in pair_index),None)
        assert first is not None and entry["third_point_attempts"]==first+1
        assert indices[2]==(start+first)%len(base)
        assert indices[0]*len(base)+indices[1]==pair_index[point_key(Q-base[indices[2]])]
        row=row_for(indices)
        assert digest(row)==entry["row_sha256"]
        rows.append(row);rhs.append(scalar)
        gained=insert(row)
    assert entry["rank_gain"]==int(gained)
    assert entry["rank_after"]==len(pivots)
assert len(pivots)==RESULT["final_rank"]==len(reps)
assert Counter(x["status"] for x in trace)==RESULT["relation_status_counts"]
assert RESULT["novel_rows"]==len(reps)-1
assert len(rows)==RESULT["matrix_rows"]
assert digest(list(zip(rows,rhs)))==RESULT["relation_matrix_sha256"]
F=GF(r);M=matrix(F,rows);b=vector(F,rhs)
assert M.rank()==len(reps)
logs=[int(x) for x in M.solve_right(b)]
assert logs==RESULT["factor_logs"] and digest(logs)==RESULT["factor_logs_sha256"]
assert all(logs[i]*G==reps[i] for i in range(len(reps)))

target=point(RESULT["target"])
assert rec(target)==W["record"]["target"]
assert digest(rec(target))==RESULT["target_point_sha256"]
fixture=int.from_bytes(hashlib.sha256(b"h4-n35-full-target-0").digest(),"big")%(r-1)+1
assert fixture==RESULT["target_fixture_scalar"]
assert fixture*G==target
ic=RESULT["ic"];rho=RESULT["rho"]
assert ic["status"]==rho["status"]=="verified"
rng=random.Random(73035)
for j,shift_row in enumerate(ic["target_shifts"]):
    shift=0 if j==0 else rng.randrange(1,r)
    assert shift_row["shift_number"]==j and shift_row["shift_scalar"]==shift
    shifted=target+shift*G if shift else target
    start=rng.randrange(len(base))
    first=next((offset for offset in range(len(base))
                if point_key(shifted-base[(start+offset)%len(base)]) in pair_index),None)
    if first is None:
        assert shift_row["status"]=="exhausted"
        assert shift_row["third_point_attempts"]==len(base)
    else:
        assert shift_row["status"]=="verified"
        assert shift_row["third_point_attempts"]==first+1
        indices=ic["witness_indices"]
        assert indices[2]==(start+first)%len(base)
        assert [rec(base[i]) for i in indices]==ic["witness"]
        assert sum((base[i] for i in indices),E(0))==shifted
        row=row_for(indices)
        assert digest(row)==ic["row_sha256"]
        recovered=(sum(v*logs[i] for i,v in enumerate(row))-shift)%r
        assert recovered==ic["scalar"]==rho["scalar"]==fixture
        assert recovered*G==target
        assert j==len(ic["target_shifts"])-1
assert len(ic["target_shifts"])<=SPEC["target_shift_cap"]
assert sum(ic["phase_wall_ns"].values())==ic["wall_ns"]==RESULT["online_wall_ns"]
assert sum(RESULT["cold_phase_wall_ns"].values())==RESULT["cold_wall_ns"]
assert rho["wall_ns"]==RESULT["rho_online_wall_ns"]
assert rho["sage_rho_source_sha256"]==SPEC["sage_rho_source_sha256"]
assert rho["scalar"]*G==target

phase_names=("setup","isogeny","factor_base","precompute","queries","pdp",
             "relation_check","matrix_build","relation_la","target_descent","recovery_check")
online_names=("target_query","target_pdp","target_relation_check",
              "target_descent","target_recovery_check")
assert set(RESULT["cold_phase_wall_ns"])==set(phase_names)
assert set(ic["phase_wall_ns"])==set(online_names)
counts=Counter(x["status"] for x in trace)
run={"schema_version":2,"kind":"full_dlp","status":"complete","proposal_id":None,
     "candidate_id":candidate_id,"run_id":RESULT["run_id"],"workload_id":W["workload_id"],
     "pair_block_id":W["workload_id"],
     "source_curve_ref":"research/exotic_ecdlp_20261021/H4_prime_descent/n35_packed_pair_20261120",
     "profile_id":"h4_n35_instance1_cofactor66_m3_direct_integer_pair_index_shift16","isogeny_route_ref":"none",
     "subgroup_order":str(r),
     "provenance":{"workload_fixture_sha256":digest(W["record"]),"source_sha256":SPEC["runner_sha256"],
                   "host_id":RESULT["host"]["node"],
                   "resource_envelope_id":"one_local_sage_process_one_cpu_8GiB_observed_900s_cap",
                   "calibration_id":"ordinary_local_unisolated_no_AA_calibration",
                   "relation_precompute_seed":61035,"target_descent_seed":73035,"rho_walk_seed":92035,
                   "rho_source_sha256":SPEC["sage_rho_source_sha256"]},
     "counts":{"ordinary_queries":len(trace),"solved_queries":counts["verified"],
               "verified_decompositions":counts["verified"],"verified_relations":counts["verified"],
               "novel_rows":RESULT["novel_rows"],"effective_columns":len(reps),
               "final_rank":len(pivots),"pdp_attempts":len(trace),
               "pdp_verified":counts["verified"],"pdp_proved_unsat":counts["zero_yield"],
               "pdp_timeout":0,"pdp_budget":0,"pdp_error":0,"pdp_lift_rejected":0},
     "phase_operations":{name:None for name in phase_names},
     "phase_wall_ns":RESULT["cold_phase_wall_ns"],"online_phase_wall_ns":ic["phase_wall_ns"],
     "online_wall_ns":ic["wall_ns"],"rho_online_wall_ns":rho["wall_ns"],
     "rho_verified":True,"target_count":1,"target_point_sha256":RESULT["target_point_sha256"],
     "precomputation_ready":True,"total_operations":None,"rho_operations":None,
     "verified_scalar":True,"scalar_certificate_ref":"result.json#ic",
     "peak_rss_bytes":RESULT["peak_rss_bytes"],
     "wall_ns":RESULT["cold_wall_ns"]+ic["wall_ns"]+rho["wall_ns"]}
(HERE/"run.jsonl").write_text(json.dumps(run,sort_keys=True)+"\n")
validation={"schema":"h4-n35-packed-full-pipeline-validation-v1","status":"passed",
            "result_sha256":sha(RESULT_PATH),"candidate_id":candidate_id,
            "curve_id":curve_id,"workload_id":W["workload_id"],"run_id":RESULT["run_id"],
            "relation_queries":len(trace),"verified_relations":counts["verified"],
            "exhausted_queries":counts["zero_yield"],"matrix_rank":int(M.rank()),
            "factor_logs_verified":len(logs),"target_scalar":fixture,
            "target_shift_count":len(ic["target_shifts"]),"rho_scalar":rho["scalar"],
            "host_tier":"ordinary_exploratory"}
(HERE/"validation.json").write_text(json.dumps(validation,indent=2,sort_keys=True)+"\n")
print(json.dumps(validation,indent=2,sort_keys=True))
