#!/usr/bin/env python3
"""Price both arms of `ic solve-prime-file --matched-rho-runs K` at w = 0, 0.1, 1
(AGENTS.md section 5a). Stdlib only.

For one instance (one JSON from solve-prime-file):
  IC  cost(w) = group_additions + group_doublings            (charged, GAE = adds + doubles)
                + w * sum(every counter named *_uncharged that is an operation)
  rho cost(w) = mean_gae over the matched-rho runs
                + w * sum(counters_per_run[*_uncharged])

Counters the IC arm names but does not emit (counters_unemitted) make the IC arm a LOWER BOUND at every w:
the ranking line then says so. Bytes are not operations (NON_OPERATION_COUNTERS in
src/cryptanalysis/ecbench/band.rs) and are listed, not priced. A ranking is stated only where the order
holds at every w; otherwise 'unresolved at this boundary'.

usage: band.py FILE.json [FILE.json ...]   (writes a JSON summary to stdout, a table to stderr)"""
import json,statistics,sys
W=(0.0,0.1,1.0)
NON_OP={"residual_table_bytes_uncharged","factor_base_vec_bytes_floor_uncharged","native_table_bytes_uncharged"}
def priced(counters):
    return {k:v for k,v in counters.items() if k.endswith("_uncharged") and k not in NON_OP}
def one(d):
    st=d["ic"]["stages"];c=st.get("counters",{})
    ic_ch=c.get("group_additions",0)+c.get("group_doublings",0)
    ic_un=priced(c)
    mr=d.get("matched_rho")
    if not mr: raise SystemExit("no matched_rho block: run with --matched-rho-runs K")
    rho_ch=mr["mean_gae"];rho_un=priced(mr["counters_per_run"])
    row={"p":d["input"]["p"],"n":d["input"]["n"],"cofactor":d["input"]["cofactor"],
         "ic":{"charged_gae":ic_ch,"unpriced":ic_un,"unemitted":st.get("counters_unemitted",[]),
               "not_counted_flags":{k:v for k,v in c.items() if k.endswith("not_counted")},"cost":{}},
         "rho":{"charged_gae":rho_ch,"unpriced":rho_un,"eligible":mr["matched_rho_eligible"],"all_verified":mr["all_verified"],"runs":mr["runs"],"cost":{}}}
    for w in W:
        row["ic"]["cost"][str(w)]=ic_ch+w*sum(ic_un.values())
        row["rho"]["cost"][str(w)]=rho_ch+w*sum(rho_un.values())
    ratios=[row["ic"]["cost"][str(w)]/row["rho"]["cost"][str(w)] for w in W]
    row["ic_over_rho"]={str(w):r for w,r in zip(W,ratios)}
    row["order_holds_across_band"]=all(r>1 for r in ratios) or all(r<1 for r in ratios)
    row["rho_cheaper_at_every_w"]=all(r>1 for r in ratios)
    return row
rows=[one(json.load(open(p))) for p in sys.argv[1:]]
out={"schema":"band/v1","weights":list(W),"instances":len(rows),"rows":rows}
if rows:
    out["median_ic_cost"]={str(w):statistics.median(r["ic"]["cost"][str(w)] for r in rows) for w in W}
    out["median_rho_cost"]={str(w):statistics.median(r["rho"]["cost"][str(w)] for r in rows) for w in W}
    out["median_ic_over_rho"]={str(w):statistics.median(r["ic_over_rho"][str(w)] for r in rows) for w in W}
    out["order_holds_in_every_instance"]=all(r["order_holds_across_band"] for r in rows)
    out["ic_is_lower_bound"]=any(r["ic"]["unemitted"] for r in rows)
print(json.dumps(out,indent=1))
for r in rows[:50]:
    print(f"n={r['n']:>9} h={r['cofactor']} IC {r['ic']['cost']['0.0']:>10.0f}/{r['ic']['cost']['0.1']:>10.0f}/{r['ic']['cost']['1.0']:>10.0f}   rho {r['rho']['cost']['0.0']:>8.1f}/{r['rho']['cost']['0.1']:>8.1f}/{r['rho']['cost']['1.0']:>8.1f}   IC/rho {r['ic_over_rho']['0.0']:.1f}/{r['ic_over_rho']['0.1']:.1f}/{r['ic_over_rho']['1.0']:.1f}",file=sys.stderr)
