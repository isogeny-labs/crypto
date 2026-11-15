"""Powered reuse run. One process = one group of arms, interleaved per target.
usage: python3 run.py n l n_unsat n_sat ARM[,ARM...] out.jsonl [target_seed_offset]
Arms (all CaDiCaL 1.5.3 via python-sat, default options, conflicts are deterministic):
  INC      one solver for the whole sequence, parametric CNF, target by assumptions
  FRESH-A  new solver per target, same CNF, same assumptions
  FRESH-U  new solver per target, parametric CNF + target bits as unit clauses
  SPEC     new solver per target, CNF regenerated with constant target
  FRESH-A2 as FRESH-A on the clause-shuffled copy of the CNF (A/A arm)
  INC2     as INC on the clause-shuffled copy (replicate of the treatment)
"""
import random, sys, time, json
from pysat.solvers import Cadical153
from pdp import Setup, build2, assumptions, lift_model
from common import cell_seed

KEYS = ('conflicts', 'decisions', 'propagations')


def exact_decomposable(S):
    """exact set of decomposable abscissae (signed base, m=3), outside V, nonzero."""
    E = S.E; xs = set(); s2 = {}
    for i, P in enumerate(S.base):
        for Q in S.base[i:]:
            T = E.add(P, Q)
            if T is not None:
                s2[T] = 1
    for T in s2:
        for P in S.base:
            R = E.add(T, P)
            if R is not None and R[0] and R[0] not in S.Vset:
                xs.add(R[0])
    return xs


def make_targets(S, nu, nd, seed):
    """nu non-decomposable (random rational points, verified) + nd planted; all distinct,
    x(R) not in V, x(R) != 0; one seeded shuffle."""
    rng = random.Random(seed); used = set(); T = []
    while sum(1 for t in T if t[1]) < nd:
        R = S.planted(rng)
        if R is None or R[0] == 0 or R[0] in S.Vset or R[0] in used:
            continue
        assert S.decomposable(R)
        used.add(R[0]); T.append((R, True))
    while sum(1 for t in T if not t[1]) < nu:
        R = S.random_point(rng)
        if R[0] in S.Vset or R[0] in used or S.decomposable(R):
            continue
        used.add(R[0]); T.append((R, False))
    rng.shuffle(T)
    return T


def main(n, l, nu, nd, arms, out, off=1):
    seed = cell_seed(l)
    S = Setup(n, l, seed)
    T = make_targets(S, nu, nd, seed + off)
    I = build2(S)
    I2 = dict(I); cl = list(I['clauses']); random.Random(seed + 2).shuffle(cl); I2['clauses'] = cl
    f = open(out, 'w')
    meta = dict(kind='meta', n=n, l=l, setup_seed=seed, target_seed=seed + off, shuffle_seed=seed + 2,
                mod=S.F.mod, V=S.V, signed_base=len(S.base), vars=I['nv'], clauses=len(I['clauses']),
                ands=I['nand'], arms=arms, n_unsat=nu, n_sat=nd,
                targets=[[R[0], d] for R, d in T])
    if l <= 5:
        meta['decomposable_abscissae_exact'] = len(exact_decomposable(S))
    f.write(json.dumps(meta) + '\n'); f.flush()
    inc = {}
    if 'INC' in arms:
        inc['INC'] = (Cadical153(bootstrap_with=I['clauses']), I)
    if 'INC2' in arms:
        inc['INC2'] = (Cadical153(bootstrap_with=I2['clauses']), I2)
    for idx, (R, truth) in enumerate(T):
        asm = assumptions(I, R[0])
        for arm in arms:
            t0 = time.time()
            if arm in inc:
                s, inst = inc[arm]
                a = s.accum_stats(); res = s.solve(assumptions=asm); b = s.accum_stats()
                d = {k: b[k] - a[k] for k in KEYS}
            else:
                if arm == 'FRESH-A':
                    inst = I; s = Cadical153(bootstrap_with=I['clauses']); res = s.solve(assumptions=asm)
                elif arm == 'FRESH-A2':
                    inst = I2; s = Cadical153(bootstrap_with=I2['clauses']); res = s.solve(assumptions=asm)
                elif arm == 'FRESH-U':
                    inst = I; s = Cadical153(bootstrap_with=I['clauses'] + [[x] for x in asm]); res = s.solve()
                elif arm == 'SPEC':
                    inst = build2(S, R[0]); s = Cadical153(bootstrap_with=inst['clauses']); res = s.solve()
                else:
                    raise ValueError(arm)
                st = s.accum_stats(); d = {k: st[k] for k in KEYS}
            d['secs'] = round(time.time() - t0, 3)
            err = None
            if res != truth:
                err = 'verdict'
            elif res and not lift_model(S, inst, s.get_model(), R)[0]:
                err = 'lift'
            if arm not in inc:
                s.delete()
            row = dict(kind='row', l=l, n=n, idx=idx, xR=R[0], decomposable=truth, arm=arm, err=err, **d)
            f.write(json.dumps(row) + '\n'); f.flush()
        print(l, idx, truth, flush=True)
    f.close()


if __name__ == '__main__':
    a = sys.argv
    main(int(a[1]), int(a[2]), int(a[3]), int(a[4]), a[5].split(','), a[6], int(a[7]) if len(a) > 7 else 1)
