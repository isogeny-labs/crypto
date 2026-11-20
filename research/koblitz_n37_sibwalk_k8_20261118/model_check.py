#!/usr/bin/env python3
"""Secondary, descriptive (PROTOCOL.md section 3): the count model (model.py, copied unchanged from
koblitz_k1_sibwalk_20261118/prof/model.py) against this study's records.
(a) every ls record: model(k=8, d=5, m=0) == the record's (group ops, native ops);
(b) every sw record: model(k=8, d=5, m=8, ladder, key reuse) == the record's counts;
(c) the model's sw/ls ratio of pooled means at w = 0, 0.1, 1 from the ls records alone, beside the measured one.
Usage: python3 model_check.py [SESS_DIR]  -> tables/model_check.json"""
import json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
from model import recs, cost, model
SESS = sys.argv[1] if len(sys.argv) > 1 else "/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/sw8-sessions"
def path(rep):
    p = os.path.join(SESS, f"n37-r{rep}", "records.jsonl")
    return p if os.path.exists(p) else p + ".gz"
R = [x for rep in (1, 2, 3) for x in recs(path(rep))]
ls = [x for x in R if x["arm"] == "ls"]; sw = [x for x in R if x["arm"] == "sw"]
out = {"ls_records": len(ls), "sw_records": len(sw),
       "ls_mismatch": sum(cost(x) != model(x, 37, k=8, d=5) for x in ls),
       "sw_mismatch": sum(cost(x) != model(x, 37, k=8, d=5, m=8, ladder=True, minkey_reuse=True) for x in sw),
       "model_ratio": {}, "measured_ratio_of_means": {}}
for w in (0, 0.1, 1):
    base = sum(g + w * n for g, n in map(cost, ls))
    out["model_ratio"][str(w)] = sum(g + w * n for g, n in (model(x, 37, k=8, d=5, m=8, ladder=True, minkey_reuse=True) for x in ls)) / base
    out["measured_ratio_of_means"][str(w)] = sum(g + w * n for g, n in map(cost, sw)) / base
os.makedirs(os.path.join(HERE, "tables"), exist_ok=True)
json.dump(out, open(os.path.join(HERE, "tables", "model_check.json"), "w"), indent=1)
print(json.dumps(out, indent=1))
