#!/usr/bin/env python3
"""C1: frozen files hash to frozen.sha256 at the freeze commit and at the validated commit; the freeze
commit holds no data; every data file first appears in a later commit; no src/ or Cargo change after the
freeze. Usage: freeze_check.py REPO OUT_JSON"""
import hashlib, json, subprocess, sys
REPO, OUT = sys.argv[1:3]
FZ, HEAD = "ae36cdc6e47d374389a65b9d93e4c2f31a2a3741", "6213ecbc0da0ed4917ab75805258201d6629b477"
D = "research/genus4_followup_20261118"
g = lambda *a: subprocess.run(["git", "-C", REPO, *a], check=True, capture_output=True).stdout
res = {"freeze_commit": FZ, "validated_commit": HEAD, "files": {}}
frozen = [l.split() for l in g("show", f"{FZ}:{D}/frozen.sha256").decode().splitlines() if l.strip()]
ok = True
for h, name in frozen:
    a = hashlib.sha256(g("show", f"{FZ}:{D}/{name}")).hexdigest()
    b = hashlib.sha256(g("show", f"{HEAD}:{D}/{name}")).hexdigest()
    res["files"][name] = {"frozen": h, "at_freeze": a, "at_validated": b, "ok": h == a == b}
    ok &= h == a == b
res["frozen_sha256_unchanged"] = g("show", f"{FZ}:{D}/frozen.sha256") == g("show", f"{HEAD}:{D}/frozen.sha256")
res["binary_sha256_unchanged"] = g("show", f"{FZ}:{D}/binary.sha256") == g("show", f"{HEAD}:{D}/binary.sha256")
res["binary_sha256"] = g("show", f"{FZ}:{D}/binary.sha256").decode().strip()
at_fz = set(g("ls-tree", "-r", "--name-only", FZ, D + "/").decode().splitlines())
res["files_at_freeze"] = sorted(at_fz)
res["no_data_at_freeze"] = not any(".jsonl" in f or f.endswith(".json") for f in at_fz)
first = {}
for line in g("log", "--reverse", "--format=%H", f"{FZ}^..{HEAD}", "--", D).decode().split():
    for f in g("show", "--name-only", "--format=", line).decode().split():
        first.setdefault(f, line)
data = [f for f in first if ".jsonl" in f]
res["data_first_commit"] = {f: first[f] for f in sorted(data)}
order = g("rev-list", "--reverse", f"{FZ}..{HEAD}").decode().split()
res["data_after_freeze"] = all(first[f] in order for f in data)
res["src_or_cargo_changed_after_freeze"] = g("diff", "--name-only", FZ, HEAD, "--", "src", "Cargo.toml", "Cargo.lock").decode().split()
res["freeze_commit_src_diff"] = g("show", "--stat", "--format=", FZ, "--", "src").decode().strip()
res["parent_has_study"] = bool(g("ls-tree", "-r", "--name-only", FZ + "^", D + "/").decode().strip())
res["pass"] = ok and res["frozen_sha256_unchanged"] and res["binary_sha256_unchanged"] and res["no_data_at_freeze"] \
    and res["data_after_freeze"] and not res["src_or_cargo_changed_after_freeze"] and not res["parent_has_study"]
json.dump(res, open(OUT, "w"), indent=1)
print(json.dumps(res, indent=1))
