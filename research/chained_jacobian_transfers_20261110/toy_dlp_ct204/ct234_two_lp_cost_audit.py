#!/usr/bin/env python3
"""CT234 fail-closed record-only auditor for CT233's exact hosted native two-LP panel.

No claim of independent solver replay, PCC signature or optimized rho comparison.
Record provenance: run 44528817307, native job 135576407970, source
84a62f1190dcf05e11af58b22c4243dff74079d3.
Run: python3 ct234_two_lp_cost_audit.py --self-test
"""
import json
import sys
from pathlib import Path

SOURCE = "84a62f1190dcf05e11af58b22c4243dff74079d3"
FILE = Path(__file__).with_name("ct234_ct233_hosted_panel.jsonl")

def require(ok, why):
    if not ok:
        raise ValueError(why)

def inv(a):
    return pow(a % 101, -1, 101)

def plus(p, q):
    if p is None: return q
    if q is None: return p
    x, y = p
    u, v = q
    if x == u and (y + v) % 101 == 0: return None
    if x == u:
        require(y != 0, "invalid doubling")
        slope = (3*x*x+1)*inv(2*y) % 101
    else:
        slope = (v-y)*inv(u-x) % 101
    a = (slope*slope-x-u) % 101
    return (a,(slope*(x-a)-y) % 101)

def mul(n, p):
    out = None
    while n:
        if n&1: out = plus(out,p)
        p = plus(p,p)
        n >>= 1
    return out

def check(meta, rows):
    require(meta == {"run_id":44528817307,
                     "native_job_id":135576407970,
                     "oracle_job_id":135576408543,
                     "source_sha":SOURCE}, "provenance mismatch")
    require([x.get("basis") for x in rows] == [24,36,48,54], "panel missing, reordered or duplicated")
    g=(2,37)
    require(mul(109,g) is None, "generator order")
    require(mul(37,g) == (64,19) and mul(73,g) == (19,37), "public scalar checks")
    last_rss=0
    for r in rows:
        b=r["basis"]
        outside=54-b
        require(r["model"]=="F101_signed_two_lp_graph" and r["pcc"]=="unverified", "model/standing")
        require(r["rank"]==b and r["target37"]==37 and r["target73"]==73, "incomplete/incorrect solve")
        require(0 < r["candidates"] <= 6540 and 0 < r["offsets"] <= 60, "candidate budget")
        require(r["full"]+r["single_lp"]+r["double_lp"] == r["verified_relations"], "relation counts")
        require(r["graph_edges"] == r["single_lp"] + r["double_lp"], "graph edge count")
        require(r["graph_edges"]-outside-1+r["graph_components"] == r["graph_cycles"], "graph cycle identity")
        require(1 <= r["graph_components"] <= outside+1, "graph components")
        require(0 <= r["known_lp"] <= r["lp_pivot_rank"] <= outside, "LP independence")
        require(0 <= r["b_only_rows"] <= r["verified_relations"], "B-row count")
        require(r["target_candidates37"]==3 and r["target_candidates73"]==21, "target protocol")
        require(r["target_adds"] > 0 and r["cantor_adds"] >= r["candidates"], "Cantor work")
        require(all(r[k] > 0 for k in ("bsgs_ns","field_add","field_mul","field_inv",
                                          "logical_bytes_peak","vmhwm_kib")), "missing physical/logical costs")
        parts=sum(r[k] for k in ("setup_ns","relations_ns","la_ns","verification_ns","target_ns"))
        require(0 <= r["total_ns"]-parts < 10000, "phase sum does not explain cold time")
        require(r["relations_ns"] <= r["total_ns"], "relation time invalid")
        require(r["vmhwm_kib"] >= last_rss, "nonmonotone process-wide VmHWM")
        last_rss=r["vmhwm_kib"]
    require((rows[3]["full"],rows[3]["candidates"],rows[3]["double_lp"],
             rows[3]["single_lp"]) == (62,1421,0,0), "full-base positive control")
    require(rows[0]["known_lp"]==28 and rows[1]["known_lp"]==17, "LP recovery drift")
    return [(r["basis"],r["total_ns"]/1e6,r["total_ns"]/r["bsgs_ns"],
             rows[3]["total_ns"]/r["total_ns"]) for r in rows]

def read():
    lines=FILE.read_text().splitlines()
    require(len(lines)==5, "expected provenance and exactly 4 native records")
    return json.loads(lines[0]), [json.loads(s) for s in lines[1:]]

def mutations(meta, rows):
    import copy
    patches = [
        (0,"rank",23),(0,"target37",38),(1,"graph_cycles",34),
        (1,"lp_pivot_rank",16),(1,"verified_relations",59),
        (2,"bsgs_ns",0),(2,"target_candidates73",22),
        (2,"total_ns",1),(3,"full",61),(3,"vmhwm_kib",1000)
    ]
    for idx,key,value in patches:
        v=copy.deepcopy(rows)
        v[idx][key]=value
        try: check(meta,v)
        except (ValueError,KeyError,TypeError): continue
        raise RuntimeError("mutation escaped rejection: "+str((idx,key,value)))
    bad=dict(meta,source_sha="0"*40)
    try: check(bad,rows)
    except ValueError: pass
    else: raise RuntimeError("source mutation escaped rejection")
    return len(patches)+1

def main():
    meta,rows=read()
    summary=check(meta,rows)
    if "--self-test" in sys.argv:
        rejected=mutations(meta,rows)
        require(rejected==11,"mutation count")
    else: rejected=None
    for basis,ms,slow,ratio in summary:
        print("CT234 basis=%d full_cost_ms=%.6f slowdown_vs_same_job_BSGS=%.3f full54_over_candidate=%.9f"%
              (basis,ms,slow,ratio))
    if rejected is not None: print("CT234_MUTATION_PASS",rejected)
    print("CT234_RECORD_AUDIT_PASS source="+SOURCE+" records="+str(len(rows)))
    print("WARNING: one GitHub-runner invocation, two-target shared setup; not a cold single-target comparison, independent solver replay, signed PCC, or calibrated rho/IC.")

if __name__=="__main__":
    main()
