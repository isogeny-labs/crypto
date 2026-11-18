"""Positive control for msolve -l 1 at p > 2^16: planted m=3 decompositions at 18 and 20 bits with
small L (8), every arm; the planted solution must satisfy the GB (verify_planted logic)."""
import sys, json
sys.path.insert(0, '/dev/shm/bkk/code')
import bkklib, run
orig = bkklib.L_of
bkklib.L_of = lambda b, m: 8
run.L_of = bkklib.L_of
for bits in (18, 20):
    inst = bkklib.make_instance(3, bits, 0, True)
    for arm in bkklib.ARMS[3]:
        r = run.run_one(inst, arm, 600, 4_500_000, 1)
        print(json.dumps({k: r[k] for k in ('tag', 'p', 'L', 'status', 'T', 'gb_is_one')}), flush=True)
        open('/dev/shm/bkk/runs/planted_bigp.jsonl', 'a').write(json.dumps(r) + '\n')
