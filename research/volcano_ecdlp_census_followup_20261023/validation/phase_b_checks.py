#!/usr/bin/env python3
"""Validator checks of the committed Phase B audit receipts and of the three
restart-orphaned partial sessions (validator-written).

* every sessions/<session>.audit.json: ok, no problems, all checks true,
  complete, bound to the session id parsed by rederive.py, record count equal,
  auditor binary = the frozen binary; replay totals;
* runs/orphans/<spec>.orphan.tar.xz: every partial record compared with the
  same seq of the redone sessions/<spec>.r2.tar.xz (arm, workload, total_gae,
  recovered), and the spec ids compared.
Usage: python3 -I phase_b_checks.py STUDY_DIR SCRATCH_DIR (needs SCRATCH/rows/sessions.jsonl)
Writes SCRATCH/phase_b_audits.json and SCRATCH/orphans.json."""
import json
import subprocess
import sys

STUDY, SCR = sys.argv[1:3]
BIN = "ac4bc5b26f858a4d8a374208297157840af25fb1dc25ea1c19dd41360735fb35"
L = {d["session"]: d for d in map(json.loads, open(f"{SCR}/rows/sessions.jsonl"))}
bad, reps, ident, recs, env, hw = [], 0, 0, 0, set(), set()
for s, d in L.items():
    a = json.load(open(f"{STUDY}/sessions/{s}.audit.json"))
    ok = (a["ok"] and not a["problems"] and all(a["checks"].values()) and a["session_status"] == "complete"
          and a["session_id"] == d["session_ids"][0] and a["records"] == d["records"]
          and a["verified_records"] == d["records"] and a["auditor_binary_sha256"] == BIN)
    reps += len(a["replays"])
    ident += sum(1 for r in a["replays"] if r["reproduced"] and r["detail"] == "identical")
    recs += a["records"]
    env.add(a["auditor_env_class_id"])
    hw.add(a["auditor_hardware_class_id"])
    if not ok:
        bad.append(s)
json.dump({"audits": len(L), "bad": bad, "replays": reps, "replays_identical": ident, "records": recs,
           "env_classes": sorted(env), "hw_classes": sorted(hw)}, open(f"{SCR}/phase_b_audits.json", "w"))


def member(arc, name):
    p = subprocess.run(["tar", "-xJOf", arc, "--wildcards", f"*/{name}"], capture_output=True, text=True, check=True)
    return p.stdout


out = {}
for spec in ["V6-D-L1-c0", "V6-D-L2-c0", "V6-N-L1-c0"]:
    oa, ra = f"{STUDY}/runs/orphans/{spec}.orphan.tar.xz", f"{STUDY}/sessions/{spec}.r2.tar.xz"
    o = {json.loads(l)["seq"]: json.loads(l) for l in member(oa, "records.jsonl").splitlines()}
    r = {json.loads(l)["seq"]: json.loads(l) for l in member(ra, "records.jsonl").splitlines()}
    so, sr = json.loads(member(oa, "session.json")), json.loads(member(ra, "session.json"))
    same = sum(1 for k in o if k in r and o[k]["cost"]["total_gae"] == r[k]["cost"]["total_gae"]
               and o[k]["outcome"]["recovered"] == r[k]["outcome"]["recovered"] and o[k]["arm"] == r[k]["arm"]
               and o[k]["workload"]["workload_id"] == r[k]["workload"]["workload_id"])
    out[spec] = {"orphan_records": len(o), "r2_records": len(r), "orphan_seq_identical_in_r2": same,
                 "spec_id_equal": so.get("spec_id") == sr.get("spec_id"), "orphan_status": so.get("status"),
                 "r2_status": sr.get("status")}
json.dump(out, open(f"{SCR}/orphans.json", "w"))
print(json.dumps({"audits_bad": bad, "replays": reps, "identical": ident}), json.dumps(out))
