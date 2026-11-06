#!/usr/bin/env python3
"""Validator provenance check of every done session (validator-written):
from each committed archive, session.json and spec.json are read (streamed);
checks: spec.json equals the committed specs/<spec>.json (sha256), session.json
spec_sha256 is reported only (it hashes ecbench's canonical spec form, not the
file bytes; ecbench verify checks spec identity itself), binary_sha256 is the frozen binary, status is
complete, records_written equals the progress line's runs; records git_commit
and git_dirty. Usage: python3 -I session_provenance.py STUDY_DIR OUT_JSON"""
import collections, hashlib, json, os, subprocess, sys
STUDY, OUT = sys.argv[1:3]
BIN = "ac4bc5b26f858a4d8a374208297157840af25fb1dc25ea1c19dd41360735fb35"
prog = [json.loads(l) for l in open(f"{STUDY}/runs/progress.jsonl")]
res = {"sessions": 0, "spec_file_equal_committed": 0, "spec_sha256_field_equal_file_bytes_hash_(informational)": 0, "binary_ok": 0, "complete": 0,
       "records_written_equal_progress_runs": 0, "git_commits": collections.Counter(), "git_dirty": collections.Counter(), "bad": []}
for d in prog:
    if d.get("status") != "done":
        continue
    s = os.path.basename(d["out"]); arc = f"{STUDY}/sessions/{s}.tar.xz"
    def mem(n):
        return subprocess.run(["tar", "-xJOf", arc, "--wildcards", f"*/{n}"], capture_output=True, check=True).stdout
    sj = json.loads(mem("session.json")); sp = mem("spec.json")
    committed = open(f"{STUDY}/specs/{d['spec']}.json", "rb").read()
    h = hashlib.sha256(sp).hexdigest()
    ok = [sp == committed, sj.get("spec_sha256") == h, sj.get("binary_sha256") == BIN, sj.get("status") == "complete",
          sj.get("records_written") == d["runs"]]
    for k, v in zip(["spec_file_equal_committed", "spec_sha256_field_equal_file_bytes_hash_(informational)", "binary_ok", "complete", "records_written_equal_progress_runs"], ok):
        res[k] += int(v)
    res["sessions"] += 1
    res["git_commits"][sj.get("git_commit")] += 1
    res["git_dirty"][str(sj.get("git_dirty"))] += 1
    if not (ok[0] and all(ok[2:])):
        res["bad"].append({"session": s, "checks": ok})
res["git_commits"] = dict(res["git_commits"]); res["git_dirty"] = dict(res["git_dirty"])
res["distinct_git_commits"] = len(res["git_commits"])
json.dump(res, open(OUT, "w"), indent=1)
print(json.dumps({k: v for k, v in res.items() if k != "git_commits"}))
