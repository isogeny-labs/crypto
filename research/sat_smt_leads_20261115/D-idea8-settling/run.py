"""Settling run. usage: python3 run.py l seed_index MODE out.jsonl [target_seed_offset] [max_targets]
Arms (CaDiCaL 1.5.3 via python-sat, default options, one core; counters are deterministic):
  INC   one solver for the whole sequence, parametric CNF, target bits as assumptions
  SPEC  new solver per target, CNF regenerated with the target as a constant (the fresh arm)
  FP    FRESH-PARAM: new solver per target, the parametric CNF, same assumptions as INC
        (emulation of INC with learnt clauses dropped; python-sat cannot delete learnts)
  AA    SPEC with the clause list shuffled (A/A arm for SPEC)
MODE: A   = INC on every target, nothing else (stops at the wall guard; see PROTOCOL.md)
      BL  = scheduled SPEC, FP, AA solves at last-third positions (no INC)
      BE  = scheduled SPEC, FP solves at earlier positions (no INC)
      ALL = every scheduled solve in one process
max_targets truncates the target list to its first max_targets entries (the INC prefix).
The schedule (which arm solves which position) is common.schedule()."""
import random, sys, time, json
from pysat.solvers import Cadical153
from pdp import Setup, build2, assumptions, lift_model
from common import cell_seed, schedule, CELLS

KEYS = ('conflicts', 'decisions', 'propagations')
WALL_GUARD = 565.0


def exact_decomposable(S):
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
    """nu non-decomposable (random rational points, verified by the real-point 2+1 MITM) + nd
    planted; all distinct, x(R) not in V, x(R) != 0; one seeded shuffle."""
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


def twist_pairs(S):
    """nonzero x in V with x and 1/x both in V and both non-liftable (the unsound case)."""
    return sum(1 for x in S.Vset if x and S.F.inv(x) in S.Vset
               and S.E.lift(x) is None and S.E.lift(S.F.inv(x)) is None)


def main(l, sidx, mode, out, off=1, maxt=None):
    t_start = time.time()
    c = CELLS[l]; n = c['n']
    seed = cell_seed(l, sidx)
    S = Setup(n, l, seed)
    ndec = len(exact_decomposable(S))
    nd = min(c['nd'], ndec // 2)            # never plant more than half the decomposable table
    T = make_targets(S, c['nu'] + c['nd'] - nd, nd, seed + off)
    if maxt:
        T = T[:maxt]
    sch = schedule(T, l)
    L0 = (2 * len(T)) // 3
    if mode == 'A':
        todo = {'INC': sch['INC']}
    elif mode == 'BL':
        todo = {a: {i for i in sch[a] if i >= L0} for a in ('SPEC', 'FP', 'AA')}
    elif mode == 'BE':
        todo = {a: {i for i in sch[a] if i < L0} for a in ('SPEC', 'FP')}
    else:
        todo = sch
    I = build2(S)
    f = open(out, 'w')
    meta = dict(kind='meta', n=n, l=l, seed_index=sidx, mode=mode, setup_seed=seed, target_seed=seed + off,
                shuffle_seed=seed + 2, mod=S.F.mod, V=S.V, signed_base=len(S.base), vars=I['nv'],
                clauses=len(I['clauses']), ands=I['nand'], nonrational_in_V=I['nonrational_in_V'],
                twist_pair_elements=twist_pairs(S), n_targets=len(T), last_third_from=L0,
                arms={a: sorted(v) for a, v in todo.items()},
                targets=[[R[0], d] for R, d in T])
    if l <= 5:
        meta['decomposable_abscissae_exact'] = ndec
        meta['rational_abscissae_outside_V'] = sum(
            1 for x in range(1, 1 << n) if x not in S.Vset and S.E.lift(x) is not None)
    f.write(json.dumps(meta) + '\n'); f.flush()
    inc = Cadical153(bootstrap_with=I['clauses']) if 'INC' in todo else None
    done = 0
    for idx, (R, truth) in enumerate(T):
        asm = assumptions(I, R[0])
        for arm in ('INC', 'SPEC', 'FP', 'AA'):
            if idx not in todo.get(arm, ()):
                continue
            t0 = time.perf_counter()
            if arm == 'INC':
                s, inst = inc, I
                t1 = time.perf_counter()
                a = s.accum_stats(); res = s.solve(assumptions=asm); b = s.accum_stats()
                d = {k: b[k] - a[k] for k in KEYS}
            else:
                if arm == 'FP':
                    inst = I; s = Cadical153(bootstrap_with=I['clauses'])
                    t1 = time.perf_counter(); res = s.solve(assumptions=asm)
                else:
                    inst = build2(S, R[0]); cl = inst['clauses']
                    if arm == 'AA':
                        cl = list(cl); random.Random(seed + 2 + 7919 * idx).shuffle(cl)
                    s = Cadical153(bootstrap_with=cl)
                    t1 = time.perf_counter(); res = s.solve()
                st = s.accum_stats(); d = {k: st[k] for k in KEYS}
            t2 = time.perf_counter()
            d['secs_solve'] = round(t2 - t1, 4); d['secs_build'] = round(t1 - t0, 4)
            err = None
            if res != truth:
                err = 'verdict'
            elif res and not lift_model(S, inst, s.get_model(), R)[0]:
                err = 'lift'
            if arm != 'INC':
                s.delete()
            f.write(json.dumps(dict(kind='row', l=l, n=n, seed_index=sidx, idx=idx, xR=R[0],
                                    decomposable=truth, arm=arm, err=err, **d)) + '\n'); f.flush()
        done = idx + 1
        if time.time() - t_start > WALL_GUARD:
            break
    f.write(json.dumps(dict(kind='end', l=l, seed_index=sidx, mode=mode, targets_done=done,
                            truncated=done < len(T), wall=round(time.time() - t_start, 1))) + '\n')
    f.close()
    print('done', l, sidx, mode, done, round(time.time() - t_start, 1))


if __name__ == '__main__':
    a = sys.argv
    main(int(a[1]), int(a[2]), a[3], a[4], int(a[5]) if len(a) > 5 else 1, int(a[6]) if len(a) > 6 else None)
