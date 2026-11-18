#!/usr/bin/env python3
"""Applies pilot/SELECTION_RULE.md: ecbench compare (ls vs each candidate arm) at w = 0, 0.1, 1 on the pilot session of
each curve; writes pilot/selection.json. Usage: ECBENCH=<pinned binary> python3 pilot_select.py PILOT_SESS_DIR"""
import json, os, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
SESS = sys.argv[1]; EC = os.environ["ECBENCH"]; W = ["0", "0.1", "1"]
out = {}
for n in (23, 29, 37):
    d = os.path.join(SESS, f"pilot-n{n}")
    arms = [a["name"] for a in json.load(open(os.path.join(d, "spec.json")))["arms"] if a["name"] != "ls"]
    rows = {}
    for a in arms:
        r = {}
        for w in W:
            c = json.loads(subprocess.run([EC, "compare", "--dir", d, "--a", "ls", "--b", a, "--weight", w, "--resamples", "4000",
                                          "--seed", "20261002", "--json"], capture_output=True, text=True, check=True).stdout)
            o = c.get("ops", c); r[w] = {"ratio": o["ratio_b_over_a"], "ci95": o["ci95"], "ci95_bootstrap": o["ci95_bootstrap"], "pairs": o["pairs"]}
        rows[a] = r
    def km(a):
        k, m = a[1:].split("-m"); return int(k), int(m)
    ok = [a for a in arms if all(rows[a][w]["ratio"] < 1 for w in W)]
    if ok:
        pick = min(ok, key=lambda a: (round(sum(rows[a][w]["ratio"] for w in W) / 3, 3), *km(a)))
        why = "all three point ratios < 1; smallest mean"
    else:
        pick = min(arms, key=lambda a: (round(max(rows[a][w]["ratio"] for w in W), 3), *km(a)))
        why = "no arm below 1 at all three weights; smallest maximum"
    out[n] = {"arms": rows, "pick": pick, "k_m": km(pick), "why": why}
    print(f"n{n}: pick {pick} ({why})")
    for a in arms:
        print(f"   {a:8s} " + "  ".join(f"{rows[a][w]['ratio']:.3f}" for w in W))
json.dump(out, open(os.path.join(HERE, "pilot", "selection.json"), "w"), indent=1)
