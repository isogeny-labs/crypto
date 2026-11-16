#!/usr/bin/env python3
"""Documentation step (no new runs): did any unit at q >= 262111 in the committed records of the earlier studies need a
retry?  A retry = more than one 'ic' record for one (unit file, IC arm), or attempt > 1, or ok == false.
usage: audit_retries.py  (run with python3 -I); writes retries.json next to this file."""
import glob, io, json, os, sys, tarfile
HERE = os.path.dirname(os.path.abspath(__file__))
R = os.path.join(HERE, "..")
QMIN = 262111
roots = ["quartic_dl1_cd_sqrt_20261113/runs", "quartic_dl1_cd_sqrt_20261113/cd_sweep/runs",
         "quartic_dl1_cd_sqrt_20261113/combo/runs", "quartic_n4v2_20261020"]
sources = []  # (label, text)
for root in roots:
    for f in sorted(glob.glob(os.path.join(R, root, "**", "*.jsonl"), recursive=True)):
        sources.append((os.path.relpath(f, R), open(f).read()))
    for t in sorted(glob.glob(os.path.join(R, root, "**", "*.tar.gz"), recursive=True)):
        try:
            with tarfile.open(t, errors="replace") as tf:
                for m in tf.getmembers():
                    if m.isfile() and m.name.endswith(".jsonl") and not os.path.basename(m.name).startswith("._"):
                        sources.append((os.path.relpath(t, R) + "!" + m.name, tf.extractfile(m).read().decode()))
        except Exception as e:
            print("cannot open", t, e, file=sys.stderr)
units = {}
for label, txt in sources:
    for l in txt.splitlines():
        try:
            r = json.loads(l)
        except Exception:
            continue
        if r.get("rec") != "ic" or r.get("q", 0) < QMIN:
            continue
        units.setdefault((label, r["arm"]), []).append(r)
retried = [{"unit": k[0], "arm": k[1], "records": len(v), "attempts": [r.get("attempt") for r in v],
            "ok": [r["ok"] for r in v], "q": v[0]["q"], "seed": v[0]["seed"], "c": [r.get("c") for r in v]}
           for k, v in sorted(units.items()) if len(v) > 1 or any(r.get("attempt", 1) > 1 or not r["ok"] for r in v)]
by_q = {}
for (label, arm), v in units.items():
    by_q.setdefault(v[0]["q"], 0)
    by_q[v[0]["q"]] += 1
d17 = [r for r in retried if r["arm"] in ("D", "DL1", "DL") and min(c for c in r["c"] if c is not None) >= 1.7]
out = {"d_family_retries_at_c_ge_1.7": len(d17), "files_scanned": len(sources), "arm_units_at_q_ge_262111": len(units), "arm_units_by_q": by_q,
       "retried_or_failed": retried}
json.dump(out, open(os.path.join(HERE, "retries.json"), "w"), indent=1)
print(json.dumps({k: v for k, v in out.items() if k != "retried_or_failed"}), "d-family retries at c>=1.7:", len(d17), "retried/failed:", len(retried))
for r in retried:
    print(r)
