"""Amendment 2, S1: m=4, 18-bit, instance 0, arms D, B4, H4b sequentially, alone, 11 GB, 7200 s."""
import sys, json
sys.path.insert(0, '/dev/shm/bkk/code')
import bkklib, run
inst = bkklib.make_instance(4, 18, 0)
for arm in ('D', 'B4', 'H4b'):
    r = run.run_one(inst, arm, 7200, 11_000_000, 1)
    r['tag'] += '_S1'
    open('/dev/shm/bkk/runs/s1.jsonl', 'a').write(json.dumps(r) + '\n')
    print(r['tag'], r['status'], round(r['T'], 1), r['Dmax'], r['Msum'], flush=True)
