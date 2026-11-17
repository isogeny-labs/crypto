#!/usr/bin/env python3
"""Proves-too-much controls for the validator's own checks (VAL-20261115-276156).

Each control feeds recompute.py's comparison code an input whose answer is known to be "different" and
requires that the code says so; a check that passes these is not vacuous.
  C1 deep diff, ksum5 against ksum4 of the same session/round/target: must report disallowed paths
     (other factor base, other relations, other counters).
  C2 deep diff, one ksum5-p record against a copy with relations.adds + 1: must report the relations path.
  C3 deep diff, this study's ksum5-p against the pre-prune sibling's ksum5 of the same target: must report
     oracle_setup adds (the pruned arm is not the old arm).
  C4 record seal on a line with one digit of total_gae changed: must fail.
  C5 pricing: rho-neg against ksum5 on B15-r1 must give a ratio far from 1 (the pairing is not degenerate).
usage: python3 -I negative_controls.py <repo root> <out.json>
"""
import copy, gzip, hashlib, importlib.util, json, os, sys

root, out = sys.argv[1:3]
spec = importlib.util.spec_from_file_location("rc", os.path.join(os.path.dirname(os.path.abspath(__file__)), "recompute.py"))
sys.argv = [sys.argv[0], root, "/dev/null"]
rc = importlib.util.module_from_spec(spec); spec.loader.exec_module(rc)

study = os.path.join(root, "research/ic_ksum_prune_20261115/sessions/B15-r1/records.jsonl.gz")
lines = gzip.open(study, "rt").read().splitlines()
recs = [json.loads(l) for l in lines]
key = lambda r: (r["round"], r["workload"]["target_index"])
by = {(r["arm"],) + key(r): r for r in recs}
k0 = (1, 5)
res = {}

d = []; rc.deep_diff(by[("ksum5",) + k0], by[("ksum4",) + k0], "", d)
dis = sorted(p for p in d if not rc.volatile(p) and p not in rc.ALLOWED_DIFF)
res["C1_ksum5_vs_ksum4"] = {"disallowed_paths": len(dis), "examples": dis[:6], "detected": len(dis) > 0}

t = copy.deepcopy(by[("ksum5-p",) + k0])
for p in t["phases"]:
    if p["name"] == "relations": p["adds"] += 1
d = []; rc.deep_diff(by[("ksum5-p",) + k0], t, "", d)
res["C2_tampered_relations_adds"] = {"paths": d, "detected": ".phases[relations].adds" in d}

sib = os.path.join(root, "research/ic_tableb_binary_ksum_20261115/sessions/B15-r1/records.jsonl.gz")
srec = {(r["arm"],) + key(r): r for r in map(json.loads, gzip.open(sib, "rt"))}
d = []; rc.deep_diff(srec[("ksum5",) + k0], by[("ksum5-p",) + k0], "", d)
d = [p for p in d if not rc.volatile(p)]
res["C3_pruned_vs_prebuild_ksum5"] = {"detected": ".phases[oracle_setup].adds" in d, "paths": sorted(d)[:12]}

l = lines[100]; r = recs[100]
g = str(r["cost"]["total_gae"]); bad = l.replace('"total_gae":' + g, '"total_gae":' + g[:-1] + str((int(g[-1]) + 1) % 10), 1)
fld = '"record_id":"%s"' % r["record_id"]
ok = lambda s: "ECR1h" + hashlib.sha256(s.replace(fld, '"record_id":""', 1).encode()).hexdigest()[:12] == r["record_id"]
res["C4_seal"] = {"original_ok": ok(l), "tampered_ok": ok(bad), "changed": bad != l, "detected": ok(l) and not ok(bad) and bad != l}

pa = [(rc.priced(rc.price_parts(by[("rho-neg",) + k]), 0.0), rc.priced(rc.price_parts(by[("ksum5",) + k]), 0.0))
      for k in {key(r) for r in recs if not r["warmup"]}]
ratio = sum(b for _, b in pa) / sum(a for a, _ in pa)
res["C5_rho_vs_ksum5_ratio_w0"] = {"ratio": ratio, "detected": ratio > 2}
res["all_detected"] = all(v["detected"] for v in res.values() if isinstance(v, dict))
json.dump(res, open(out, "w"), indent=1)
print("all_detected", res["all_detected"])
