"""Full-precision cross-check of j1_decision's method against the in-pod
control-comparison.json (the one instrument compare output in eks-results).
usage: xcheck_inpod.py OUT.json"""
import sys

import j1_decision as j
import vcommon as v

out = {f"P{b}": j.inpod_crosscheck(b) for b in (32, 24)}
v.dump(out, sys.argv[1])
for k, r in out.items():
    print(k, "max rel diff %.3e" % r["max_rel_diff"])
