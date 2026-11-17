#!/usr/bin/env python3
"""Rebuild the combined N83 first-fall cohort index from immutable attempt summaries."""
import json, pathlib, collections
BASE=pathlib.Path(__file__).resolve().parents[1]
a1=json.loads((BASE/"attempt-01/summary.json").read_text())
a2=json.loads((BASE/"attempt-02/summary.json").read_text())
rows=[]
for r in a1["first_fall_rows"]:
    p=BASE/"attempt-01/cases/spike35/koblitz-m2-planted/attempt.json"
    raw=json.loads(p.read_text())
    rows.append({**r,"attempt":"attempt-01","attempt_path":str(p.relative_to(BASE)),"input_row_sha256":raw.get("input_row_sha256"),"producer_exit_code":raw.get("returncode"),"producer_elapsed_seconds":raw.get("elapsed_seconds"),"producer_s3_key":raw.get("s3_certificate_key"),"replay_returncode":raw.get("replay",{}).get("returncode"),"replay_elapsed_seconds":raw.get("replay",{}).get("elapsed_seconds")})
for r in a2["rows"]:
    p=BASE/"attempt-02/cases"/r["candidate_id"]/r["id"]/"attempt.json"
    raw=json.loads(p.read_text())
    rows.append({"candidate_id":r["candidate_id"],"certificate_sha256":r["certificate_sha256"],"family":r["family"],"first_fall_degree":r["first_fall_degree"],"first_fall_lower_bound":r["first_fall_lower_bound"],"id":r["id"],"m":r["m"],"replay_verified":bool(r["replay"]["verified"]),"system_sha256":r["system_sha256"],"target_kind":r["target_kind"],"status":r["status"],"attempt":"attempt-02","attempt_path":str(p.relative_to(BASE)),"input_row_sha256":raw.get("input_row_sha256"),"producer_exit_code":raw.get("returncode"),"producer_elapsed_seconds":raw.get("elapsed_seconds"),"producer_s3_key":raw.get("s3_certificate_key") or raw.get("s3_certificate_prefix"),"replay_returncode":raw.get("replay",{}).get("returncode"),"replay_elapsed_seconds":raw.get("replay",{}).get("elapsed_seconds")})
rows.sort(key=lambda r:(r["candidate_id"],r["family"],r["m"],r["target_kind"]))
assert len(rows)==144 and len({(r["candidate_id"],r["id"]) for r in rows})==144
assert all(r["replay_verified"] for r in rows)
out={"schema":"n83.candidate-fall-panel-combined/v1","protocol_id":a2["protocol_id"],"protocol_freeze_sha256":a2["protocol_freeze_sha256"],"scope":"Combined attempt-01 recovered certificate and 143 attempt-02 cases; first-fall only, with no exact LFD, affine degree-of-regularity, solving-degree, or factor-base-ranking claim.","systems_planned":144,"systems_attempted":144,"certificates_produced":144,"fresh_replays_verified":144,"attempt_provenance":{"attempt-01":{"attempted":1,"runner_error_preserved":True,"producer_exit_code_captured":False},"attempt-02":{"attempted":143,"independent_validation":json.loads((BASE/"attempt-02/independent-validation.json").read_text())}},"status_counts":dict(collections.Counter(r["status"] for r in rows)),"rows":rows}
(BASE/"combined-144/combined-summary.json").write_text(json.dumps(out,indent=2,sort_keys=True)+"\n")
print(json.dumps({"rows":len(rows),"status_counts":out["status_counts"]}))
