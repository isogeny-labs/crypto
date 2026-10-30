"""Price a follow-up-A session in instructions retired (corrected rule, PILOT_20261030.md section 9).

usage: python3 price.py runs/<SESSION> [runs/<SESSION>_tm]
Per record: child Ir from the shim's <seq>.time (the ecbench child only: msolve grandchildren are NOT included,
shown by records where the child retires fewer instructions than its msolve calls), msolve Ir from the record.
whole = child + msolve. S_instr = (whole - ECB_START) / IR_PER_GAE / sqrt(r).
Lower bound for the published arm, most favourable to it: msolve Ir minus msolve start-up only (system
construction, root extraction, group work and the pipeline charged at 0), against the lab arm's whole run.
"""
import json, sys, glob, math, statistics as st, os

ECB_START = 3.51e7     # ecbench child start-up (pilot A9r, rho-only children)
MSOLVE_START = 3.8e7   # msolve start-up on a one-variable system (this follow-up, 3 runs 3.74e7..4.02e7)
IR_PER_GAE = 3265.0    # pilot section 3 (K_0 n=41 rho-negation)

def ir(path):
    for line in open(path):
        if "instructions retired" in line:
            return int(line.split()[0])

def main():
    sess = sys.argv[1]
    tmdir = sys.argv[2] if len(sys.argv) > 2 else sess + "_tm"
    recs = [json.loads(l) for l in open(os.path.join(sess, "records.jsonl"))]
    tms = sorted(glob.glob(os.path.join(tmdir, "*.time")))
    assert len(tms) == len(recs), (len(tms), len(recs))
    rows = []
    for i, r in enumerate(recs):
        assert r["seq"] == i
        child = ir(tms[i])
        ex = (r.get("solver") or {}).get("extra") or {}
        ms_ir = ex.get("msolve_ir", 0)
        ms_calls = ex.get("msolve_calls", 0)
        rr = r["workload"]["curve"]["r"]
        included = False
        whole = child + ms_ir
        rows.append(dict(arm=r["arm"], t=r["workload"]["target_index"], status=r["outcome"]["status"],
                         s_counted=r["cost"]["s"], child_ir=child, msolve_ir=ms_ir, msolve_calls=ms_calls,
                         solver_calls=(r.get("solver") or {}).get("calls"), grandchild_included=included,
                         s_instr=(whole - ECB_START) / IR_PER_GAE / math.sqrt(rr),
                         s_instr_ex_msolve_start=(whole - ECB_START - ms_calls * MSOLVE_START) / IR_PER_GAE / math.sqrt(rr),
                         ir_per_target=whole - ECB_START, solver_only_ir=ms_ir - ms_calls * MSOLVE_START, r=rr))
    out = {"session": sess, "constants": dict(ECB_START=ECB_START, MSOLVE_START=MSOLVE_START, IR_PER_GAE=IR_PER_GAE), "rows": rows, "arms": {}}
    for arm in dict.fromkeys(x["arm"] for x in rows):
        a = [x for x in rows if x["arm"] == arm]
        ver = [x for x in a if x["status"] == "verified"]
        def med(k):
            v = [x[k] for x in ver if x[k] is not None]
            return st.median(v) if v else None
        out["arms"][arm] = dict(n=len(a), verified=len(ver), s_counted_median=med("s_counted"), s_instr_median=med("s_instr"),
                                s_instr_min=min((x["s_instr"] for x in ver), default=None), s_instr_max=max((x["s_instr"] for x in ver), default=None),
                                s_instr_ex_msolve_start_median=med("s_instr_ex_msolve_start"), ir_per_target_median=med("ir_per_target"),
                                msolve_calls_median=med("msolve_calls"), solver_calls_median=med("solver_calls"),
                                grandchild_included=sorted(set(x["grandchild_included"] for x in a)))
    # paired per-target ratios against the lab arm
    lab = {x["t"]: x for x in rows if x["arm"] == "inherited-f4" and x["status"] == "verified"}
    for arm in out["arms"]:
        if arm.startswith("msolve"):
            rat = [x["ir_per_target"] / lab[x["t"]]["ir_per_target"] for x in rows if x["arm"] == arm and x["status"] == "verified" and x["t"] in lab]
            rat2 = [(x["ir_per_target"] - x["msolve_calls"] * MSOLVE_START) / lab[x["t"]]["ir_per_target"] for x in rows if x["arm"] == arm and x["status"] == "verified" and x["t"] in lab]
            rat3 = [x["solver_only_ir"] / lab[x["t"]]["ir_per_target"] for x in rows if x["arm"] == arm and x["status"] == "verified" and x["t"] in lab]
            if rat3:
                out["arms"][arm].update(lower_bound_ratio_median=st.median(rat3), lower_bound_min=min(rat3), lower_bound_max=max(rat3))
            if rat:
                out["arms"][arm].update(ratio_to_lab_median=st.median(rat), ratio_min=min(rat), ratio_max=max(rat),
                                        ratio_ex_msolve_start_median=st.median(rat2), ratio_ex_min=min(rat2), ratio_ex_max=max(rat2))
    json.dump(out, open(sess + "_priced.json", "w"), indent=1)
    for arm, v in out["arms"].items():
        print(arm, json.dumps({k: (f"{x:.3g}" if isinstance(x, float) else x) for k, x in v.items()}))

main()
