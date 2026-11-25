#!/usr/bin/env python3
"""AMENDMENT.md replication R2: arm N, s = 3, 120k steps on the given g = 3 curve seeds."""
import json, sys
sys.path.insert(0, __file__.rsplit('/', 1)[0])
import sibwalk as S
cells = json.load(open(sys.argv[1])); seeds = [int(x) for x in sys.argv[3].split(',')]
out = open(sys.argv[2], 'a')
for c in cells:
    if c['g'] == 3 and c['curve_seed'] in seeds:
        st = S.run_cell(c, 'N', 3, 120000)
        st['arm_label'] = 'N_rep'
        out.write(json.dumps(st) + '\n'); out.flush()
