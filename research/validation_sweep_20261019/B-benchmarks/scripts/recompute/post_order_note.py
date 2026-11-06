# Post-process SWB-RECOMPUTE-06.jsonl: add a factual git-order note to each row (read-only git log results already in the rows).
import json, os, re, sys
OUT = "/home/user/crypto/research/validation_sweep_20261019/B-benchmarks/recompute/SWB-RECOMPUTE-06.jsonl"
rows = [json.loads(l) for l in open(OUT) if l.strip()]
for r in rows:
    prot = re.findall(r"([0-9a-f]{40}) (\S+)", r["protocol_git_date"])
    raw = re.findall(r"([0-9a-f]{40}) (\S+)", r["first_raw_git_date"])
    if not prot:
        note = "git order: no protocol commit found in the row (see protocol_git_date)."
    else:
        ph = {h for h, _ in prot}
        rh = {h for h, _ in raw}
        if ph & rh:
            note = (f"git order: protocol and first raw record share add commit {sorted(ph & rh)[0]} "
                    f"(one commit: git history does not order them).")
        else:
            note = "git order: protocol add commit and first raw add commit differ (see the two commit hashes in the row)."
    r["notes"] = (r["notes"] + " " + note).strip()
with open(OUT, "w") as fh:
    for r in rows:
        fh.write(json.dumps(r, ensure_ascii=False) + "\n")
print("ok", len(rows))
