#!/usr/bin/env python3
"""join.py OUT.jsonl SCALE_G3.jsonl FILE...: concatenate the study files and append the genus-3
rho runs of the scaling study (R_rho_neg lines) whose prime appears in the study files."""
import json, sys
out, scale, files = sys.argv[1], sys.argv[2], sys.argv[3:]
primes3 = set()
with open(out, "w") as o:
    for f in files:
        for l in open(f):
            r = json.loads(l)
            if r.get("kind") == "instance" and r["g"] == 3:
                primes3.add(r["p"])
            if r.get("kind") in ("instance", "run", "cell"):
                o.write(l if l.endswith("\n") else l + "\n")
    for l in open(scale):
        r = json.loads(l)
        if r.get("kind") == "run" and r["arm"] == "R_rho_neg" and r["p"] in primes3:
            o.write(l if l.endswith("\n") else l + "\n")
