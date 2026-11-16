"""Minimal assumption cores over target bits for UNSAT targets (deletion-based, two orders)."""
import random, sys, time, json
from pysat.solvers import Glucose4
from pdp import Setup, build2, assumptions
from run import make_targets, MASTER

n, l, nd, nu, k, out = int(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4]), int(sys.argv[5]), sys.argv[6]
seed = MASTER + 1000 * l
S = Setup(n, l, seed)
T, _, _ = make_targets(S, nd, nu, seed + 1)
I = build2(S)
s = Glucose4(bootstrap_with=I['clauses'])
f = open(out, 'w')
done = 0
for idx, (R, truth) in enumerate(T):
    if truth or done >= k:
        continue
    done += 1
    asm = assumptions(I, R[0])
    assert not s.solve(assumptions=asm)
    core0 = list(s.get_core())
    row = dict(kind='core', n=n, l=l, idx=idx, xR=R[0], first_core=len(core0), orders={})
    for oname in ('ascending', 'shuffled'):
        core = list(core0)
        order = sorted(core, key=abs)
        if oname == 'shuffled':
            random.Random(seed + idx).shuffle(order)
        t0 = time.time(); calls = 0; sat_calls = 0
        for lit in order:
            if lit not in core:
                continue
            trial = [x for x in core if x != lit]
            calls += 1
            if s.solve(assumptions=trial):
                sat_calls += 1
            else:
                c2 = set(s.get_core() or [])
                core = [x for x in trial if x in c2]
        assert not s.solve(assumptions=core)
        row['orders'][oname] = dict(min_core=len(core), calls=calls, sat_calls=sat_calls, secs=round(time.time() - t0, 2),
                                    bits=sorted(I['r'].index(abs(x)) for x in core))
    f.write(json.dumps(row) + '\n'); f.flush()
    print(row, flush=True)
