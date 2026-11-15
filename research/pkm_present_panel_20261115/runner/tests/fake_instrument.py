#!/usr/bin/env python3
"""Stub of pkm_present_panel for runner self-tests: `solve --system F --max-degree N --deadline S --out OUT --extract`.
The behaviour is read from the "fake" object of the system JSON: sleep (s), ops, solve, extract, decomps, drop (field
names to omit from the output, to test the loud failure), cpu_s."""
import json, sys, time
a = sys.argv[1:]
assert a[0] == "solve", a
g = lambda f: a[a.index(f) + 1]
sysj = json.load(open(g("--system")))
fk = sysj.get("fake", {})
out = g("--out")
if fk.get("sleep"):
    json.dump({"ops": fk.get("ops", 1000), "degree": 4, "step": 7, "elapsed_s": 0.5}, open(out + ".progress.json", "w"))
    time.sleep(fk["sleep"])
if fk.get("exit"):
    sys.stderr.write(fk.get("stderr", ""))
    sys.exit(fk["exit"])
doc = {"solve_field_multiplications": fk.get("solve", 100), "extract_field_multiplications": fk.get("extract", 10),
       "solving_degree": 3, "degree_reached": 4, "basis_size": 5, "threads_used": 1, "cpu_s": 0.01,
       "censored": False, "verified_real_decompositions": fk.get("decomps", []), "unlifted_solutions": 0,
       "fp_rational_solutions": len(fk.get("decomps", []))}
for d in fk.get("drop", []):
    doc.pop(d, None)
json.dump(doc, open(out, "w"))
