"""Correctness check: S_4, S_5 (and S_3) vanish on x-coordinates of random sums; random points do not."""
import random, sys, json
sys.path.insert(0, '/dev/shm/bkk/code')
from bkklib import *

rng = random.Random('bkk-check')
res = {}
for trial in range(20):
    p = rand_prime(14, rng); a, b = rand_curve(p, rng)
    Ps = [rand_point(p, a, b, rng) for _ in range(4)]
    R3 = ec_add(ec_add(Ps[0], Ps[1], p, a), Ps[2], p, a)
    R4 = ec_add(R3, Ps[3], p, a)
    if R3 is None or R4 is None:
        continue
    s4, v4 = special(S4_GEN, p, a, b, ['x1', 'x2', 'x3', R3[0]])
    s5, v5 = S5_special(p, a, b, R4[0])
    asg = {f'x{i+1}': Ps[i][0] for i in range(4)}
    ok4 = eval_poly(s4, v4, asg, p) == 0
    ok5 = eval_poly(s5, v5, asg, p) == 0
    # negative: random xR
    s5n, _ = S5_special(p, a, b, rng.randrange(p))
    neg5 = eval_poly(s5n, v5, asg, p) != 0
    res.setdefault('ok4', []).append(ok4); res.setdefault('ok5', []).append(ok5); res.setdefault('neg5', []).append(neg5)
s5, v5 = S5_special(p, a, b, 5)
s4, v4 = special(S4_GEN, p, a, b, ['x1', 'x2', 'x3', 'x4'])
print(json.dumps({k: f'{sum(v)}/{len(v)}' for k, v in res.items()}))
print('S4 terms', len(s4), 'degs', [max(m[i] for m in s4) for i in range(4)], 'tot', max(sum(m) for m in s4))
print('S5(xR fixed) terms', len(s5), 'degs', [max(m[i] for m in s5) for i in range(4)], 'tot', max(sum(m) for m in s5))
