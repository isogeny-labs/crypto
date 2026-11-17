#!/usr/bin/env python3
"""For every record the replay-all receipt flags, re-execute it with `ecbench profile-input | ecbench exec` and report
which phase fields differ from the archived record (stdlib only; resumable).
usage: replay_fields.py ECBENCH SESSION_DIR RECEIPT.json OUT.json"""
import json, subprocess, sys
from collections import Counter, defaultdict
from pathlib import Path

eb, sess, rcp, outp = sys.argv[1:5]
sess = Path(sess)
recs = {}
for l in open(sess / "records.jsonl"):
    d = json.loads(l); recs[d["seq"]] = d
rc = json.load(open(rcp))
key = next(k for k, v in rc.items() if isinstance(v, list) and v and isinstance(v[0], dict) and "reproduced" in v[0])
bad = [r["seq"] for r in rc[key] if not r["reproduced"]]
done = json.load(open(outp)) if Path(outp).is_file() else {}
for seq in bad:
    if str(seq) in done: continue
    inp = subprocess.run([eb, "profile-input", "--spec", str(sess / "spec.json"), "--seq", str(seq)], capture_output=True, text=True, check=True).stdout
    out = subprocess.run([eb, "exec"], input=inp, capture_output=True, text=True, check=True).stdout
    rep = json.loads(out)["report"]
    a = recs[seq]
    diffs = []
    ph_new = {p["name"]: p for p in rep["phases"]}
    for p in a["phases"]:
        q = ph_new.get(p["name"])
        if q is None: diffs.append(f"{p['name']}: phase missing in replay"); continue
        for f in ("adds", "doubles", "scalar_mults"):
            if p.get(f) != q.get(f): diffs.append(f"{p['name']}.{f}: {p.get(f)} -> {q.get(f)}")
        for k in sorted(set(p["native"]) | set(q["native"])):
            if p["native"].get(k) != q["native"].get(k): diffs.append(f"{p['name']}.native.{k}: {p['native'].get(k)} -> {q['native'].get(k)}")
    for k in sorted(set(a["counters"]) | set(rep["counters"])):
        if a["counters"].get(k) != rep["counters"].get(k): diffs.append(f"counters.{k}: {a['counters'].get(k)} -> {rep['counters'].get(k)}")
    done[str(seq)] = {"arm": a["arm"], "recovered_equal": str(rep.get("recovered")) == a["outcome"]["recovered"],
                      "total_gae": [a["cost"]["total_gae"], rep["total_gae"]], "diffs": diffs}
    json.dump(done, open(outp, "w"), indent=1)
summ = defaultdict(Counter)
for s, v in done.items():
    for d in v["diffs"]:
        summ[v["arm"]][d.split(":")[0]] += 1
print({a: dict(c) for a, c in summ.items()})
print("recovered equal on all:", all(v["recovered_equal"] for v in done.values()), "total_gae differs on:", sorted({v["arm"] for v in done.values() if v["total_gae"][0] != v["total_gae"][1]}))
