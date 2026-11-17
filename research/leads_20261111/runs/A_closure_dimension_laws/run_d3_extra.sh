#!/bin/bash
# ADDED after the preregistered D=3 cells (exploratory, NOT preregistered; reported separately):
# (a) unsatisfiable seeds 3rd..6th per odd n = 23..33 (fresh A/A instances),
# (b) odd n = 35..43 at D=3 (outside the stated range 15..33), (c) even n = 24..34 at D=3, k=(n+1)/2 rounded down by bash = n/2.
cd "$(dirname "$0")"
export INSTFILE=extra_instances.jsonl SCANFILE=extra_scan.jsonl
for n in 23 25 27 29 31 33; do ./select.sh R $n 6; done
for n in 35 37 39 41 43; do ./select.sh R $n 2; done
for n in 24 26 28 30 32 34; do ./select.sh R $n 2; done
python3 - <<'PY'
import json, subprocess
for l in open('out/extra_instances.jsonl'):
    d = json.loads(l)
    res = subprocess.run(['./fallclose', '3', '8192', '1'], input=d['line'], capture_output=True, text=True).stdout
    r = json.loads(res); r.update(tag='extraD3', type=d['type'], seed=d['seed'], sols=d['sols'], Dreq=3, line=d['line'])
    with open('out/cells_extra.jsonl', 'a') as f: f.write(json.dumps(r) + '\n')
PY
