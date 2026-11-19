#!/usr/bin/env python3
"""CT241 strict independent *record-only* F103 native cold cost auditor.

No native solver replay or PCC. No benchmark records are fabricated if panel
is absent. Built-in synthetic mutation tests are not native observations.
"""
import copy
import json
from pathlib import Path

BASES = (12, 16, 18, 20, 22)
ROUNDS = (0, 1, 2)
EXPECTED = {
    12: (589, 5, 11, 15, 9, 21),
    16: (381, 8, 11, 7, 1, 20),
    18: (381, 14, 10, 2, 1, 22),
    20: (381, 19, 7, 1, 0, 25),
    22: (381, 27, 0, 0, 0, 27),
}
PHASES = ("setup_ns", "relation_online_la_ns", "backsolve_ns",
          "recovery_ns", "verification_ns")
COUNTERS = ("cantor_adds", "target_adds", "direct_bsgs_adds",
            "field_add", "field_mul", "field_sqr", "field_inv")
PANEL = Path(__file__).with_name("ct241_f103_native_15_pairs.jsonl")

def check(ok, msg):
    if not ok:
        raise ValueError(msg)

def audit(rows):
    check(len(rows) == 15, "not exactly 15 original cold pairs")
    check(set((r.get("base"),r.get("round")) for r in rows) ==
          {(b,n) for b in BASES for n in ROUNDS}, "missing/duplicate arm or round")
    for r in rows:
        b, n = r["base"], r["round"]
        check(r.get("order") == ("IC_BSGS" if n % 2 == 0 else "BSGS_IC"),
              "not preregistered pairing order")
        check(r.get("pcc") == "unverified", "PCC standing was changed")
        check(r.get("memory") == "vmhwm_processwide", "unregistered memory claim")
        for key in PHASES + COUNTERS + (
            "base", "round", "candidate_ns", "direct_bsgs_ns", "candidates",
            "rank", "full", "single_lp", "double_lp", "discarded",
            "b_only", "lp_pivots", "determined_lp", "target_candidates",
            "logical_matrix_bytes", "vmhwm_kb"):
            check(type(r.get(key)) is int and r[key] >= 0,
                  "missing/invalid numeric field "+key)
        check(r["candidate_ns"] > 0 and r["direct_bsgs_ns"] > 0,
              "zero wall time")
        check(sum(r[k] for k in PHASES) == r["candidate_ns"],
              "phase total mismatch")
        check(r["rank"] == b, "incomplete rank")
        check(r["candidates"] <= 6540 and r["target_candidates"] == 27,
              "relation or target budget drift")
        check((r["candidates"], r["full"], r["single_lp"],
               r["double_lp"], r["discarded"], r["b_only"]) == EXPECTED[b],
              "CT240 finite source-model relation bookkeeping drift")
        check(r["lp_pivots"] == r["determined_lp"] == 22-b,
              "LP rank/recovery mismatch")
        check(r["logical_matrix_bytes"] ==
              8*(b*(b+1)+(22-b)*23), "matrix accounting drift")
        check(r["direct_bsgs_adds"] == 17, "direct BSGS operation drift")
        check(abs(r["ratio_direct_over_candidate"] -
                  r["direct_bsgs_ns"]/r["candidate_ns"]) < 2e-10,
              "inconsistent paired wall ratio")
        check(r["full"]+r["single_lp"]+r["double_lp"]+r["discarded"] <=
              r["candidates"], "impossible relation counts")
    for b in BASES:
        ar = sorted((r for r in rows if r["base"] == b),
                    key=lambda r:r["round"])
        for key in ("candidates", "rank", "full", "single_lp", "double_lp",
                    "discarded", "b_only", "lp_pivots", "determined_lp",
                    "target_candidates") + COUNTERS:
            check(len({r[key] for r in ar}) == 1,
                  "deterministic round drift in "+key)
    return True

def synthetic_panel():
    rows=[]
    for b in BASES:
        for n in ROUNDS:
            c,f,one,two,rej,brows=EXPECTED[b]
            r=dict(base=b,round=n,
                order=("IC_BSGS" if n % 2 == 0 else "BSGS_IC"),
                candidate_ns=1400000, direct_bsgs_ns=1400,
                ratio_direct_over_candidate=0.001,
                setup_ns=50000, relation_online_la_ns=1300000,
                backsolve_ns=20000, recovery_ns=20000, verification_ns=10000,
                candidates=c, rank=b, full=f,single_lp=one,
                double_lp=two, discarded=rej,b_only=brows,
                lp_pivots=22-b,determined_lp=22-b,target_candidates=27,
                logical_matrix_bytes=8*(b*(b+1)+(22-b)*23),
                vmhwm_kb=10240, pcc="unverified",
                memory="vmhwm_processwide",cantor_adds=c+4,
                target_adds=28,direct_bsgs_adds=17,
                field_add=100,field_mul=200,field_sqr=300,field_inv=400)
            rows.append(r)
    return rows

def selftest():
    p=synthetic_panel()
    audit(p)
    mutations = []
    q=copy.deepcopy(p);q.pop();mutations.append(q)
    q=copy.deepcopy(p);q[0]=copy.deepcopy(q[1]);mutations.append(q)
    for key,value in (
        ("candidate_ns",0), ("ratio_direct_over_candidate",0.5),
        ("rank",11),("lp_pivots",9),("determined_lp",9),
        ("candidates",590),("target_candidates",28),
        ("relation_online_la_ns",1),("order","BSGS_IC"),
        ("pcc","verified"),("memory","attributed"),
        ("logical_matrix_bytes",1),("full",40),
        ("direct_bsgs_adds",0)):
        q=copy.deepcopy(p);q[0][key]=value;mutations.append(q)
    for i,q in enumerate(mutations):
        try:
            audit(q)
        except (ValueError,TypeError,KeyError):
            continue
        raise AssertionError(f"synthetic mutated record {i} accepted")
    return len(mutations)

def main():
    n=selftest()
    print("CT241_SYNTHETIC_AUDIT_SELFTEST_PASS",n,"mutations")
    if not PANEL.exists():
        print("CT241_PANEL_PENDING: synthetic controls only; no native outcome checked")
        return
    lines=PANEL.read_text().splitlines()
    rows=[json.loads(t) for t in lines if t.strip()]
    audit(rows)
    print("CT241_RECORD_ONLY_AUDIT_PASS",len(rows),"original native rows")
    for b in BASES:
        ar=sorted((r for r in rows if r["base"]==b), key=lambda x:x["round"])
        med=sorted(r["ratio_direct_over_candidate"] for r in ar)[1]
        med_ns=sorted(r["candidate_ns"] for r in ar)[1]
        print(f"CT241_ARM basis={b} candidate_median_ns={med_ns} paired_baseline_over_candidate_median={med:.12f}")
    print("CT241_LIMIT: no independent native replay, no standalone PCC/strongest-rho")

if __name__=="__main__":
    main()
