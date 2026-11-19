#!/usr/bin/env python3
"""Write specs for L4-SCALE. Usage: gen_specs.py <name> <bits> <targets> <rounds> <seedbase> [smoke]"""
import json,sys
name,bits,tg,rounds,seed=sys.argv[1],int(sys.argv[2]),int(sys.argv[3]),int(sys.argv[4]),int(sys.argv[5])
fb="prime-projected-abscissa:size=2,scan_cap=32"
mt="13528399621"
def m(o,extra=None):
    p={"factor_base":fb,"max_trials":mt,"oracle":o}
    if extra:p.update(extra)
    return {"id":"ic.pipeline","params":p}
tw="wwalk:x=2.0,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run"
sb="wwalk:x=2.0,g=1,grow=2,tw=2,cj=1,seed=run,lazy=1,sib=8,pool=256"
arms=[{"name":"rho-neg","role":"reference","method":{"id":"rho.negation"}},
 {"name":"tw2l","role":"baseline","method":m(tw)},
 {"name":"sib8-p256","role":"candidate","method":m(sb,{"targets":"stride-generator"})},
 {"name":"sib8-p256-aa","role":"control","method":m(sb,{"targets":"stride-generator"})}]
spec={"schema":"ecbench.spec/v1","label":f"ic_scale_l4_20261119 {bits}-bit prime-order generic curves: tw2l and sib8-p256 vs rho-neg",
 "description":"Lead L4-SCALE. research/ic_scale_l4_20261119/PROTOCOL.md. Public synthetic curves and targets; no security claim.",
 "arms":arms,
 "workloads":{"curves":[{"kind":"prime_orbit_shape","shape":"generic","bits":bits,"seed":bits*1000+i} for i in (1,2)],
  "targets_per_curve":tg,"target_seed":seed,"target_kind":"public"},
 "measurement":{"rounds":rounds,"warmup":1,"order":"alternate","seed":seed+7,"isolation_required":"L0","timeout_seconds":3600}}
json.dump(spec,open(name,"w"),indent=1)
