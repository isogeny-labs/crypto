#!/usr/bin/env python3
"""Exact first-hit distribution for every nonidentity scalar target."""

import gzip
import hashlib
import json
from decimal import Decimal, getcontext
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def read_locked(protocol,path_key,sha_key):
    raw=(ROOT/protocol[path_key]).read_bytes()
    assert sha(raw)==protocol[sha_key]
    return raw


def main():
    protocol_raw=(HERE/"protocol.json").read_bytes()
    protocol=json.loads(protocol_raw)
    threshold=json.loads(read_locked(protocol,"source_threshold_result","source_threshold_result_raw_sha256"))
    cost=json.loads(read_locked(protocol,"source_rational_cost_result","source_rational_cost_result_raw_sha256"))
    cost_rows_raw=read_locked(protocol,"source_rational_cost_rows","source_rational_cost_rows_raw_sha256")
    cost_rows={r["p"]:r for r in map(json.loads,cost_rows_raw.splitlines())}
    fresh_raw=read_locked(protocol,"source_fresh_target_result","source_fresh_target_result_raw_sha256")
    fresh=json.loads(fresh_raw)
    fresh_rows_raw=(ROOT/protocol["source_fresh_target_rows"]).read_bytes()
    assert sha(fresh_rows_raw)==fresh["arm_rows_sha256"]
    fresh_rows=[json.loads(line) for line in fresh_rows_raw.splitlines()]
    panel=json.loads((ROOT/protocol["source_panel_protocol"]).read_bytes())
    getcontext().prec=50
    hqq,hqr,hrr=[Decimal(z) for z in (panel["height_pairing_matrix"][0][0],
                                        panel["height_pairing_matrix"][0][1],
                                        panel["height_pairing_matrix"][1][1])]
    all_rows=[]
    summaries=[]
    for cell,th in zip(protocol["cells"],threshold["thresholds"]):
        p,q,lam,B=[cell[key] for key in ("p","q","R_over_Q","radius")]
        assert (th["p"],th["q"],th["first_full_radius"])==(p,q,B)
        labels=[(a,b) for a in range(-B,B+1) for b in range(-B,B+1) if (a,b)!=(0,0)]
        labels.sort(key=lambda z:(hqq*z[0]**2+2*hqr*z[0]*z[1]+hrr*z[1]**2,z))
        first={}
        for i,(a,b) in enumerate(labels,1):
            k=(a+lam*b)%q
            if k:
                first.setdefault(k,(i,a,b))
            if len(first)==q-1:
                break
        assert len(first)==q-1
        assert i==cost_rows[p]["candidate_evaluations"]
        assert cost_rows[p]["status"]=="full_coverage"
        for k in range(1,q):
            idx,a,b=first[k]
            all_rows.append({"p":p,"q":q,"target_scalar":k,"first_hit_index":idx,
                             "first_label":[a,b]})
        for r in fresh_rows:
            if r["p"]==p and r["arm"] in ("rational","finite"):
                assert first[r["scalar"]][0]==r["candidate_evaluations"]
                assert list(first[r["scalar"]][1:])==r["matched_label"]
        values=sorted(v[0] for v in first.values())
        n=q-1
        total=sum(values)
        assert 2*total>=q*n
        summaries.append({"p":p,"q":q,"radius":B,"targets":n,"candidate_cap":len(labels),
                          "coverage_index":i,"sum_first_hit_indices":total,
                          "mean_numerator":total,"mean_denominator":n,
                          "combinatorial_mean_lower_bound_numerator":q,
                          "combinatorial_mean_lower_bound_denominator":2,
                          "mean_to_bound_ratio_numerator":2*total,
                          "mean_to_bound_ratio_denominator":q*n,
                          "median_times_two":values[n//2-1]+values[n//2],
                          "p90_nearest_rank":values[(9*n+9)//10-1],
                          "max_first_hit_index":values[-1]})
    rows_raw="".join(json.dumps(row,sort_keys=True)+"\n" for row in all_rows).encode()
    compressed=gzip.compress(rows_raw,mtime=0)
    (HERE/"first-hit-rows.jsonl.gz").write_bytes(compressed)
    return {"schema":"xedni-first-hit-distribution-result-v1","status":"PASS",
            "protocol_sha256":sha(protocol_raw),"script_sha256":sha((HERE/"run.py").read_bytes()),
            "first_hit_rows_raw_sha256":sha(rows_raw),
            "first_hit_rows_gzip_sha256":sha(compressed),
            "target_rows":len(all_rows),"cells":summaries}


if __name__=="__main__":
    print(json.dumps(main(),sort_keys=True,indent=2))
