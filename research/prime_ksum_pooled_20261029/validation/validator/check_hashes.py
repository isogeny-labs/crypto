#!/usr/bin/env python3
"""Validator: bind recovered records.jsonl to the repository (stdlib only).
For each session: sha256 of recovered file vs .artifacts.json (repo, HEAD tree),
vs session.json records_sha256 (repo copy), vs git blob at a0ff77014f
(git cat-file blob content hashed), and git blob id of the recovered bytes."""
import hashlib, json, subprocess, sys
from pathlib import Path
REPO = Path("/home/user/crypto")
STUDY = "research/prime_ksum_pooled_20261029"
REC = Path(sys.argv[1])  # recovered sessions dir
COMMIT = "a0ff77014f"
out = {"commit_full": subprocess.check_output(["git", "-C", str(REPO), "rev-parse", COMMIT + "^{commit}"], text=True).strip(),
       "head": subprocess.check_output(["git", "-C", str(REPO), "rev-parse", "HEAD"], text=True).strip(),
       "dirty_lines": len(subprocess.check_output(["git", "-C", str(REPO), "status", "--porcelain"], text=True).splitlines()),
       "sessions": {}}
ok_all = True
for s in ["train-P20", "train-P24", "train-P28", "train-P32", "holdout-P20", "holdout-P24", "holdout-P28", "holdout-P32"]:
    rel = f"{STUDY}/sessions/{s}/records.jsonl"
    data = (REC / s / "records.jsonl").read_bytes()
    sha = hashlib.sha256(data).hexdigest()
    gitblob_local = hashlib.sha1(b"blob %d\0" % len(data) + data).hexdigest()
    man = json.loads((REPO / STUDY / "sessions" / s / ".artifacts.json").read_text())
    man_sha = man["files"]["records.jsonl"]["sha256"]; man_bytes = man["files"]["records.jsonl"]["bytes"]
    sess = json.loads((REPO / STUDY / "sessions" / s / "session.json").read_text())
    blob = subprocess.check_output(["git", "-C", str(REPO), "rev-parse", f"{COMMIT}:{rel}"], text=True).strip()
    blob_bytes = subprocess.check_output(["git", "-C", str(REPO), "cat-file", "blob", blob])
    blob_sha = hashlib.sha256(blob_bytes).hexdigest()
    # other session files: recovered vs repo HEAD
    side = {}
    for f in ["session.json", "spec.json", "plan.json", "host.json"]:
        a = hashlib.sha256((REC / s / f).read_bytes()).hexdigest()
        b = hashlib.sha256((REPO / STUDY / "sessions" / s / f).read_bytes()).hexdigest()
        side[f] = {"sha256_repo_head": b, "recovered_equal": a == b,
                   "git_blob_head": subprocess.check_output(["git", "-C", str(REPO), "rev-parse", f"HEAD:{STUDY}/sessions/{s}/{f}"], text=True).strip()}
    r = {"repo_path_at_commit": rel, "sha256": sha, "bytes": len(data),
         "artifacts_json_sha256": man_sha, "artifacts_json_bytes": man_bytes,
         "session_json_records_sha256": sess["records_sha256"],
         "git_blob_at_a0ff77014f": blob, "git_blob_of_recovered_bytes": gitblob_local,
         "git_blob_content_sha256": blob_sha,
         "records_written": sess["records_written"], "status_counts": sess["status_counts"],
         "session_id": sess["session_id"], "binary_sha256": sess["binary_sha256"],
         "git_commit_in_session": sess["git_commit"], "git_dirty_in_session": sess["git_dirty"],
         "side_files": side}
    r["ok"] = (sha == man_sha == sess["records_sha256"] == blob_sha and len(data) == man_bytes and blob == gitblob_local
               and all(v["recovered_equal"] for v in side.values()))
    ok_all &= r["ok"]
    out["sessions"][s] = r
    print(s, sha[:16], blob[:12], "ok" if r["ok"] else "MISMATCH")
out["all_ok"] = ok_all
json.dump(out, open(Path(sys.argv[2]), "w"), indent=1)
print("all_ok", ok_all)
