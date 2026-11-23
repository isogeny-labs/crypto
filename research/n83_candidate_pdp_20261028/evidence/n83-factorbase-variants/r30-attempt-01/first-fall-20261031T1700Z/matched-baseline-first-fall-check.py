#!/usr/bin/env python3
import json,hashlib
from pathlib import Path
study=Path("research/n83_candidate_pdp_20261028")
base=json.loads((study/"evidence/pdp/baseline/summary.json").read_text())
candidate=json.loads(Path("/tmp/n83-candidate-fall/spike30/summary.json").read_text())
inputs=json.loads(Path("/tmp/n83-candidate-inputs/spike30/inputs.json").read_text())
b={r["id"]:r for r in base["rows"]};c={r["id"]:r for r in candidate["rows"]};i={r["id"]:r for r in inputs}
assert b.keys()==c.keys()==i.keys() and len(b)==48
rows=[]
for id in sorted(b):
 x=b[id]["first_fall"];old=x["input"];new=i[id]
 for k in ("family","m","target_kind","target_x","target_y","points"): assert old[k]==new[k],(id,k)
 oldffd=(x.get("first_fall_degree"),x.get("first_fall_lower_bound"));newffd=(c[id].get("first_fall_degree"),c[id].get("first_fall_lower_bound"))
 assert oldffd==newffd,(id,oldffd,newffd)
 rows.append({"id":id,"family":new["family"],"m":new["m"],"target_kind":new["target_kind"],"same_target_and_points":True,"baseline_first_fall":oldffd,"candidate_first_fall":newffd,"same_first_fall_result":True,"candidate_system_sha256":new["source_sha256"],"candidate_manifest_sha256":new["factor_base_manifest_sha256"]})
out={"schema":"n83.r30-matched-first-fall-check/v1","candidate":"spike30","baseline_summary_sha256":hashlib.sha256((study/"evidence/pdp/baseline/summary.json").read_bytes()).hexdigest(),"candidate_inputs_sha256":hashlib.sha256(Path("/tmp/n83-candidate-inputs/spike30/inputs.json").read_bytes()).hexdigest(),"matched_cases":48,"target_point_match_verified":True,"same_first_fall_result_rows":sum(x["same_first_fall_result"] for x in rows),"interpretation":"Descriptive matched panel validation: all 48 first-fall outcomes equal baseline for these paired target tuples. It does not compare LFD, top-form dreg, solving degree, or other coordinate bits; no general factor-base ranking follows.","rows":rows}
p=Path("research/n83_candidate_pdp_20261028/evidence/n83-factorbase-variants/r30-attempt-01/first-fall-20261031T1700Z/matched-baseline-first-fall-check.json")
p.write_text(json.dumps(out,indent=2,sort_keys=True)+"\n")
print(json.dumps({"matched_cases":48,"same_ffd":out["same_first_fall_result_rows"]}))
