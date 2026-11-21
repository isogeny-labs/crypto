#!/usr/bin/env python3
"""Independent 256-target n29 shift-descent and prior-target exclusion audit."""
from sage.all import *
import collections,hashlib,json,random,statistics
from pathlib import Path

HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[3]
SPEC=json.loads((HERE/"protocol.json").read_text())
FIX=json.loads((HERE/"fixtures.json").read_text())
RESULT_PATH=HERE/"result.json"
RESULT=json.loads(RESULT_PATH.read_text())
C=json.loads((ROOT/SPEC["candidate"]).read_text())
PREP=json.loads((ROOT/SPEC["precomputation_receipt"]).read_text())
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def digest(obj):return hashlib.sha256(json.dumps(obj,sort_keys=True,separators=(",",":"),ensure_ascii=False).encode()).hexdigest()
for key,hkey in (("candidate","candidate_sha256"),
                 ("precomputation_receipt","precomputation_receipt_sha256"),
                 ("candidate_runner","candidate_runner_sha256")):
    assert sha(ROOT/SPEC[key])==SPEC[hkey]
for path,h in SPEC["exclusion_sources"].items():assert sha(ROOT/path)==h
assert sha(HERE/"fixtures.json")==SPEC["fixtures_sha256"]==RESULT["fixture_sha256"]
assert sha(HERE/"make_fixtures.py")==SPEC["fixture_generator_sha256"]
assert sha(HERE/"run.py")==SPEC["panel_runner_sha256"]
assert sha(HERE/"protocol.json")==RESULT["protocol_sha256"]
assert RESULT["status"]=="complete" and len(RESULT["targets"])==256
assert FIX["candidate_id"]==C["candidate_id"]==PREP["candidate_id"]==RESULT["candidate_id"]
assert FIX["curve_id"]==C["record"]["curve"]["curve_id"]
assert len(FIX["targets"])==256
field=C["record"]["field"];n,poly=field["n"],field["defining_polynomial_int"]
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
assert rec(G)==FIX["generator"]
base=[point(p) for p in C["record"]["factor_base"]["encoded_points"]]
reps=sorted({min((P,-P),key=rec) for P in base},key=rec)
assert len(base)==1072 and len(reps)==536
logs=PREP["factor_logs"]
assert len(logs)==536 and all(logs[i]*G==reps[i] for i in range(536))
col_of={P:i for i,P in enumerate(reps)}
def row_for(indices):
    row=[0]*536
    for i in indices:
        P=base[i]; rep=min((P,-P),key=rec)
        j=col_of[rep];row[j]=(row[j]+(1 if P==rep else -1))%r
    return row
pair_sums={A+B for A in base for B in base}
assert len(pair_sums)==PREP["distinct_pair_sums"]==573927
P=ROOT/"research/exotic_ecdlp_20261021/H4_prime_descent"
panel=json.loads((P/"n29_local_panel_20261120/fixtures.json").read_text())
prior32=json.loads((P/"n29_instance1_pair_20261120/result.json").read_text())
prior256=json.loads((P/"n29_instance1_yield_20261120/result.json").read_text())
used={x["known_scalar"] for x in PREP["relation_trace"]}
used.add(PREP["target_fixture_scalar"])
used.update(x["fixture_scalar"] for x in panel["targets"])
used.update(x["fixture_scalar"] for x in prior32["targets"])
used.update(x["fixture_scalar"] for x in prior256["targets"])
assert len(used)==FIX["excluded_prior_scalars"]
hist=collections.Counter();statuses=collections.Counter();exhausted=0
phase_names={"target_query","target_pdp","target_relation_check",
             "target_descent","target_recovery_check"}
for i,(fixture,entry) in enumerate(zip(FIX["targets"],RESULT["targets"])):
    assert fixture["index"]==entry["index"]==i
    assert fixture["workload_id"]==entry["workload_id"]
    assert fixture["target"]==entry["target"]
    assert fixture["target_point_sha256"]==entry["target_point_sha256"]
    counter=0
    while True:
        label=f"h4-n29-shift-reliability-target-{i}-{counter}"
        scalar=int.from_bytes(hashlib.sha256(label.encode()).digest(),"big")%(r-1)+1
        if scalar not in used:break
        counter+=1
    assert fixture["target_label"]==label and fixture["fixture_scalar"]==scalar
    used.add(scalar)
    assert fixture["workload_id"]=="W"+digest(fixture["workload_record"])[:12]
    Q=point(fixture["target"])
    assert scalar*G==Q and digest(rec(Q))==fixture["target_point_sha256"]
    ic=entry["ic"]
    assert ic["status"]=="verified" and entry["scalar_certificate_pass"]
    assert ic["scalar"]==scalar and ic["scalar"]*G==Q
    assert set(ic["phase_wall_ns"])==phase_names
    assert sum(ic["phase_wall_ns"].values())==ic["wall_ns"]
    assert 0<ic["wall_ns"]<=ic["outer_wall_ns"]
    rng=random.Random(73029)
    shifts=ic["target_shifts"]
    assert 1<=len(shifts)<=16
    for j,shift_row in enumerate(shifts):
        shift=0 if j==0 else rng.randrange(1,r)
        assert shift_row["shift_number"]==j and shift_row["shift_scalar"]==shift
        shifted=Q+shift*G if shift else Q
        start=rng.randrange(len(base))
        first=next((offset for offset in range(len(base))
                    if shifted-base[(start+offset)%len(base)] in pair_sums),None)
        if first is None:
            assert shift_row["status"]=="exhausted"
            assert shift_row["third_point_attempts"]==len(base)
            exhausted+=1
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
            assert j==len(shifts)-1
    hist[len(shifts)]+=1;statuses[ic["status"]]+=1
assert sum(hist.values())==256
a,b=RESULT["calibration_pre_ns"],RESULT["calibration_post_ns"]
assert a>0 and b>0
assert abs(RESULT["calibration_ratio"]-max(a,b)/min(a,b))<1e-12
validation={"schema":"h4-n29-shift-reliability-validation-v1","status":"passed",
            "result_sha256":sha(RESULT_PATH),"fixture_sha256":SPEC["fixtures_sha256"],
            "target_count":256,"verified_scalars":statuses["verified"],
            "shift_count_histogram":{str(k):v for k,v in sorted(hist.items())},
            "exhausted_shift_scans":exhausted,"direct_hits":hist[1],
            "max_shift_scans":max(hist),"calibration_ratio":RESULT["calibration_ratio"],
            "median_online_ns":statistics.median(x["ic"]["wall_ns"] for x in RESULT["targets"]),
            "host_tier":"ordinary_exploratory"}
(HERE/"validation.json").write_text(json.dumps(validation,indent=2,sort_keys=True)+"\n")
print(json.dumps(validation,indent=2,sort_keys=True))
