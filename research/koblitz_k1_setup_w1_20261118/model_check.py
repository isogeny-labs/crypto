#!/usr/bin/env python3
"""Secondary, descriptive (PROTOCOL.md section 3): the count model (model.py = koblitz_n37_sibwalk_k8_20261118/model.py
plus model_w1) against this study's records, per curve:
(a) every inc record: model(incumbent config) == the record's (group ops, priced native ops);
(b) every w1 / w1-neg / w1-ids record: model_w1(...) of the PAIRED inc record, with hit_negations read from the
    candidate record's relation phase (0..2), == the candidate record's counts;
(c) the model's ratio of pooled means at w = 0, 0.1, 1 (hit_negations = 1 for every record) beside the measured one.
Usage: python3 model_check.py [SESS_DIR] -> tables/model_check.json"""
import json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
from model import recs, cost, model, model_w1, INC
SESS = sys.argv[1] if len(sys.argv) > 1 else "/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/w1-sessions"
LEV = {"w1": dict(lazy_neg=True, orbit_ids=True), "w1-neg": dict(lazy_neg=True), "w1-ids": dict(orbit_ids=True)}
def path(n, rep):
    p = os.path.join(SESS, f"n{n}-r{rep}", "records.jsonl")
    return p if os.path.exists(p) else p + ".gz"
def hitneg(x):
    return sum((p.get("native") or {}).get("point_negations_uncharged", 0) for p in x["phases"] if p["name"] != "factor_base")
out = {}
for n in (23, 29, 37):
    R = [(rep, x) for rep in (1, 2, 3) for x in recs(path(n, rep))]
    key = lambda rep, x: (rep, x["round"], x["workload"]["target_index"])
    inc = {key(r, x): x for r, x in R if x["arm"] == "inc"}
    o = {"inc_records": len(inc), "inc_mismatch": sum(cost(x) != model(x, n, **{k: v for k, v in INC[n].items()}) for x in inc.values()),
         "model_ratio": {}, "measured_ratio_of_means": {}}
    for arm, lev in LEV.items():
        c = {key(r, x): x for r, x in R if x["arm"] == arm}
        o[f"{arm}_records"] = len(c)
        o[f"{arm}_mismatch"] = sum(cost(c[k]) != model_w1(inc[k], n, **INC[n], **lev, hit_negations=hitneg(c[k])) for k in c)
        o[f"{arm}_hit_negations_mean"] = sum(map(hitneg, c.values())) / max(1, len(c))
        for w in (0, 0.1, 1):
            base = sum(g + w * a for g, a in map(cost, inc.values()))
            o["model_ratio"][f"{arm}|{w}"] = sum(g + w * a for g, a in (model_w1(x, n, **INC[n], **lev, hit_negations=1) for x in inc.values())) / base
            o["measured_ratio_of_means"][f"{arm}|{w}"] = sum(g + w * a for g, a in map(cost, c.values())) / base
    out[str(n)] = o
os.makedirs(os.path.join(HERE, "tables"), exist_ok=True)
json.dump(out, open(os.path.join(HERE, "tables", "model_check.json"), "w"), indent=1)
print(json.dumps(out, indent=1))
