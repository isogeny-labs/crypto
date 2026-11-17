#!/usr/bin/env python3
"""Validator negative control: for one 22-bit holdout row per arm, rerun the instrumented build with
MTFO_CORRUPT=1 (found[0] += 1 before verification) and MTFO_DUMP; the binary's counted check must report
ok = targets - 1 and the validator's independent EC check (eccheck.check) must flag exactly target 0.
usage: negctl.py <dbg binary> <rows.jsonl> <frozen instances dir> <dump dir> <out.json>"""
import json, subprocess, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
BIN, ROWS, INST, DD, OUT = sys.argv[1:6]
os.makedirs(DD, exist_ok=True)
rows = [json.loads(l) for l in open(ROWS)]
pick = {}
for i, r in enumerate(rows):
    if r["pb"] == 22 and r["T"] == 1024 and r["arm"] not in pick: pick[r["arm"]] = i
sys.argv = [sys.argv[0], INST, "/dev/null", ROWS, "/dev/null"]  # eccheck parses argv at import
import eccheck
res = []
for arm, i in sorted(pick.items()):
    r = rows[i]; argv = list(r["argv"]); argv[1] = os.path.join(INST, os.path.basename(argv[1]))
    dump = os.path.join(DD, f"neg_{arm}.txt")
    env = dict(os.environ, MTFO_CORRUPT="1", MTFO_DUMP=dump)
    p = subprocess.run([BIN] + argv, capture_output=True, text=True, env=env)
    o = json.loads(p.stdout.strip().splitlines()[-1])
    c = eccheck.check((i, dump, argv[1], arm))
    res.append(dict(arm=arm, row=i, binary_ok=o["ok"], targets=o["targets"], binary_caught=o["ok"] == o["targets"] - 1,
                    validator_failures=c["failures"], validator_failed=c["failed"], validator_caught=c["failures"] == 1 and c["failed"] == [0]))
json.dump(res, open(OUT, "w"), indent=1)
for x in res: print(x)
print("all caught:", all(x["binary_caught"] and x["validator_caught"] for x in res))
