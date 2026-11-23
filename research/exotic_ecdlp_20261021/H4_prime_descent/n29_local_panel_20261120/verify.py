#!/usr/bin/env python3
"""Independent five-target n29 witness, shift, identity, and timing audit."""
from sage.all import *
from sage.groups.generic import discrete_log_rho
import hashlib, inspect, itertools, json, random, statistics
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[3]
SPEC=json.loads((HERE/"protocol.json").read_text())
FIX=json.loads((HERE/"fixtures.json").read_text())
RESULT_PATH=HERE/"result.json"
RESULT=json.loads(RESULT_PATH.read_text())
C=json.loads((ROOT/SPEC["candidate"]).read_text())
PREP=json.loads((ROOT/SPEC["precomputation_receipt"]).read_text())
assert RESULT["status"]=="complete" and len(RESULT["pairs"])==5
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def digest(obj):return hashlib.sha256(json.dumps(obj,sort_keys=True,separators=(",",":"),ensure_ascii=False).encode()).hexdigest()
for path_key,hash_key in (("candidate","candidate_sha256"),
                          ("precomputation_receipt","precomputation_receipt_sha256"),
                          ("candidate_runner","candidate_runner_sha256")):
    assert sha(ROOT/SPEC[path_key])==SPEC[hash_key]
assert sha(HERE/"fixtures.json")==SPEC["fixtures_sha256"]==RESULT["fixture_sha256"]
assert sha(HERE/"make_fixtures.py")==SPEC["fixture_generator_sha256"]
assert sha(HERE/"run.py")==SPEC["panel_runner_sha256"]
assert sha(HERE/"protocol.json")==RESULT["protocol_sha256"]
assert hashlib.sha256(inspect.getsource(discrete_log_rho).encode()).hexdigest()==SPEC["sage_rho_source_sha256"]
assert FIX["candidate_id"]==C["candidate_id"]==PREP["candidate_id"]==RESULT["candidate_id"]
assert FIX["curve_id"]==C["record"]["curve"]["curve_id"]
assert len(FIX["targets"])==5
field=C["record"]["field"]
n,poly=field["n"],field["defining_polynomial_int"]
assert n==29
F2=GF(2);PR=PolynomialRing(F2,"z");z=PR.gen()
mod=sum(F2((poly>>i)&1)*z**i for i in range(n+1))
K=GF(1<<n,"z",modulus=mod)
def elt(v):return K([(v>>i)&1 for i in range(n)])
def enc(e):return sum(int(c)<<i for i,c in enumerate(e.polynomial().list()))
def rec(P):return [enc(P[0]),enc(P[1])]
coeff=C["record"]["curve"]["coefficients"]
E=EllipticCurve(K,[1,elt(coeff["a2"]),0,0,elt(coeff["a6"])])
def point(pair):return E(elt(pair[0]),elt(pair[1]))
G=point(C["record"]["curve"]["G"])
r=C["record"]["curve"]["r"]
assert r==SPEC["subgroup_order"]==268447807 and r*G==E(0)
assert FIX["generator"]==rec(G)
base=[point(pair) for pair in C["record"]["factor_base"]["encoded_points"]]
reps=sorted({min((P,-P),key=rec) for P in base},key=rec)
assert len(base)==1072 and len(reps)==len(PREP["factor_logs"])==536
assert all(r*P==E(0) for P in base)
logs=PREP["factor_logs"]
assert all(logs[i]*G==reps[i] for i in range(len(reps)))
col_of={P:i for i,P in enumerate(reps)}
def row_for(indices):
    row=[0]*len(reps)
    for i in indices:
        P=base[i]; rep=min((P,-P),key=rec)
        j=col_of[rep];row[j]=(row[j]+(1 if P==rep else -1))%r
    return row
# Complete point-only pair-sum set for independent first-hit and exhaustion checks.
pair_sums={A+B for A in base for B in base}
assert len(pair_sums)==PREP["distinct_pair_sums"]==573927
known={entry["known_scalar"] for entry in PREP["relation_trace"]}
known.add(PREP["target_fixture_scalar"])
assert FIX["excluded_prior_relation_scalars"]==len({entry["known_scalar"] for entry in PREP["relation_trace"]})
phase_names=("setup","isogeny","factor_base","precompute","queries","pdp",
             "relation_check","matrix_build","relation_la","target_descent","recovery_check")
online_names=("target_query","target_pdp","target_relation_check",
              "target_descent","target_recovery_check")
runs=[];ratios=[]
for i,(fixture,pair) in enumerate(zip(FIX["targets"],RESULT["pairs"])):
    assert pair["index"]==fixture["index"]==i
    assert pair["order"]==SPEC["arm_order"][i].split(",")
    assert pair["workload_id"]==fixture["workload_id"]
    assert pair["target"]==fixture["target"]
    assert pair["target_point_sha256"]==fixture["target_point_sha256"]
    # Recompute the first nonexcluded label rather than trusting its recorded scalar.
    counter=0
    while True:
        label=f"h4-n29-panel-target-{i}-{counter}"
        scalar=int.from_bytes(hashlib.sha256(label.encode()).digest(),"big")%(r-1)+1
        if scalar not in known:break
        counter+=1
    assert label==fixture["target_label"] and scalar==fixture["fixture_scalar"]
    known.add(scalar)
    assert fixture["workload_id"]=="W"+digest(fixture["workload_record"])[:12]
    Q=point(fixture["target"])
    assert scalar*G==Q and digest(rec(Q))==fixture["target_point_sha256"]
    ic,rho=pair["arms"]["IC"],pair["arms"]["rho"]
    assert ic["status"]==rho["status"]=="verified"
    assert ic["scalar"]==rho["scalar"]==scalar
    assert ic["scalar"]*G==rho["scalar"]*G==Q
    assert set(ic["phase_wall_ns"])==set(online_names)
    assert sum(ic["phase_wall_ns"].values())==ic["wall_ns"]
    assert 0<ic["wall_ns"]<=ic["outer_wall_ns"] and rho["wall_ns"]>0
    rng=random.Random(73029)
    assert 1<=len(ic["target_shifts"])<=16
    for j,shift_row in enumerate(ic["target_shifts"]):
        shift=0 if j==0 else rng.randrange(1,r)
        assert shift_row["shift_number"]==j and shift_row["shift_scalar"]==shift
        shifted=Q+shift*G if shift else Q
        start=rng.randrange(len(base))
        first=next((offset for offset in range(len(base))
                    if shifted-base[(start+offset)%len(base)] in pair_sums),None)
        if first is None:
            assert shift_row["status"]=="exhausted"
            assert shift_row["third_point_attempts"]==len(base)
        else:
            assert shift_row["status"]=="verified"
            assert shift_row["third_point_attempts"]==first+1
            indices=ic["witness_indices"]
            assert indices[2]==(start+first)%len(base)
            assert [rec(base[k]) for k in indices]==ic["witness"]
            assert sum((base[k] for k in indices),E(0))==shifted
            row=row_for(indices)
            assert digest(row)==ic["row_sha256"]
            recovered=(sum(v*logs[k] for k,v in enumerate(row))-shift)%r
            assert recovered==scalar and recovered*G==Q
            assert j==len(ic["target_shifts"])-1
    a,b=pair["calibration_pre_ns"],pair["calibration_post_ns"]
    assert a>0 and b>0
    assert abs(pair["calibration_ratio"]-max(a,b)/min(a,b))<1e-12
    assert pair["calibration_pass"]==(max(a,b)/min(a,b)<=1.25)
    assert pair["scalar_certificate_pass"]
    ratios.append(rho["wall_ns"]/ic["wall_ns"])
    run_id=C["candidate_id"]+fixture["workload_id"]+"R1"
    run={"schema_version":2,"kind":"full_dlp","status":"complete","proposal_id":None,
         "candidate_id":C["candidate_id"],"run_id":run_id,"workload_id":fixture["workload_id"],
         "pair_block_id":fixture["workload_id"],
         "source_curve_ref":"research/exotic_ecdlp_20261021/H4_prime_descent/n29_instance1_pair_20261120",
         "profile_id":"h4_n29_instance1_cofactor2_m3_pair_index_shift16","isogeny_route_ref":"none",
         "subgroup_order":str(r),
         "provenance":{"workload_fixture_sha256":digest(fixture["workload_record"]),
                       "source_sha256":SPEC["candidate_runner_sha256"],
                       "host_id":RESULT["host"]["node"],
                       "resource_envelope_id":"one_local_sage_process_one_cpu_8GiB_observed_240s_panel_cap",
                       "calibration_id":"pre_post_512_additions_ratio_le_1.25",
                       "shared_precomputation_sha256":SPEC["precomputation_receipt_sha256"],
                       "panel_runner_sha256":SPEC["panel_runner_sha256"],
                       "rho_source_sha256":SPEC["sage_rho_source_sha256"]},
         "counts":{"ordinary_queries":0,"solved_queries":0,"verified_decompositions":0,
                   "verified_relations":0,"novel_rows":0,"effective_columns":len(reps),
                   "final_rank":PREP["final_rank"],"pdp_attempts":0,"pdp_verified":0,
                   "pdp_proved_unsat":0,"pdp_timeout":0,"pdp_budget":0,
                   "pdp_error":0,"pdp_lift_rejected":0},
         "phase_operations":{name:None for name in phase_names},
         "phase_wall_ns":{name:None for name in phase_names},
         "online_phase_wall_ns":ic["phase_wall_ns"],"online_wall_ns":ic["wall_ns"],
         "rho_online_wall_ns":rho["wall_ns"],"rho_verified":True,
         "target_count":1,"target_point_sha256":fixture["target_point_sha256"],
         "precomputation_ready":True,"total_operations":None,"rho_operations":None,
         "verified_scalar":True,"scalar_certificate_ref":f"result.json#pairs/{i}",
         "peak_rss_bytes":pair["peak_rss_so_far_bytes"],
         "wall_ns":ic["wall_ns"]+rho["wall_ns"]}
    runs.append(run)
(HERE/"runs.jsonl").write_text("".join(json.dumps(run,sort_keys=True)+"\n" for run in runs))
bootstrap=sorted(statistics.median(sample) for sample in itertools.product(ratios,repeat=5))
validation={"schema":"h4-n29-five-target-local-panel-validation-v1","status":"passed",
            "result_sha256":sha(RESULT_PATH),"fixture_sha256":SPEC["fixtures_sha256"],
            "candidate_id":C["candidate_id"],"verified_pairs":len(runs),
            "calibration_passes":sum(pair["calibration_pass"] for pair in RESULT["pairs"]),
            "ratios":ratios,"median_ratio":statistics.median(ratios),
            "bootstrap_median_ratio_95pct":[bootstrap[int(.025*(len(bootstrap)-1))],
                                             bootstrap[int(.975*(len(bootstrap)-1))]],
            "host_tier":"ordinary_exploratory"}
(HERE/"validation.json").write_text(json.dumps(validation,indent=2,sort_keys=True)+"\n")
print(json.dumps(validation,indent=2,sort_keys=True))
