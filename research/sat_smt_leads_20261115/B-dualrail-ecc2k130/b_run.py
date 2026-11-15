"""Measured runs of PROTOCOL.md.  python3 run.py OUT.jsonl D [arm,arm,..]"""
import json
import random
import sys
import time

import dr
from dr import weil

SEED0 = 2026111500
ALL = ['a', 'b', 'c', 'a-scan', 'L-scan', 'L-plain-scan', 'L-free', 'L-rail-free']


def P(*a):
    print(*a, file=sys.stderr, flush=True)


def main():
    out, d = sys.argv[1], int(sys.argv[2])
    arms = sys.argv[3].split(',') if len(sys.argv) > 3 else ALL
    nUni = int(sys.argv[4]) if len(sys.argv) > 4 else 8
    nPla = int(sys.argv[5]) if len(sys.argv) > 5 else 4
    pb, poly = dr.repoField()
    F = dr.FastField(dr.N, poly)
    C = dr.FastCurve(F)
    chk = random.Random(1)
    for _ in range(100):
        a, b = chk.getrandbits(131), chk.getrandbits(131)
        assert F.mul(a, b) == pb.mul(a, b)
    base = [p for p in (C.lift(x) for x in range(1, 1 << d)) if p]
    assert [p[0] for p in base] == weil.admissibleAbscissae(pb, d)
    A = len(base)
    pairs = A * (A - 1) // 2
    rng = random.Random(SEED0 + d)
    targets = []
    for i in range(8):
        targets.append(('uniform', i, weil.uniformTarget(pb, rng), None))
    for i in range(4):
        R, xs, _ = weil.plantedTarget(pb, d, rng)
        targets.append(('planted', i, R, sorted(xs)))
    targets = targets[:nUni] + targets[8:8 + nPla]
    fh = open(out, 'a')

    def emit(row):
        row.update({'d': d, 'A': A, 'pairs': pairs, 'leaves_total': (1 << d) - 1})
        fh.write(json.dumps(row, sort_keys=True) + '\n')
        fh.flush()
        P({k: row.get(k) for k in ('d', 'stratum', 'index', 'arm', 'status', 'conflicts', 'leaves', 'capped_leaves', 'wall')})

    def check(R, truth, sat, model):
        """Gates 3 and 4."""
        if sat is None:
            return None
        assert sat == bool(truth), 'answer disagrees with pair enumeration'
        if sat:
            xs = dr.modelXs(model, d)
            assert dr.liftsToTarget(C, xs, R), 'model does not lift (own arithmetic)'
            v = weil.verifyProjected(pb, R, xs)
            assert v['relation'] and not v['degenerate'], v   # repository check is up to the sign of R; exact sum is the own-arithmetic check above
            assert tuple(sorted(xs)) in truth
            return sorted(xs)
        return None

    def free(rendered, cap):
        s = dr.CMS(rendered, timeLimit=cap)
        t0 = time.time()
        sat, model, cf = s.solve()
        return sat, model, cf, time.time() - t0

    def scan(rendered, leaves, leafCap, wallCap=240.0):
        s = dr.CMS(rendered)
        t0 = time.time()
        per = []
        capped = 0
        res = (False, None)
        censored = False
        for x1 in leaves:
            asm = [(1 + k) if (x1 >> k) & 1 else -(1 + k) for k in range(d)]
            sat, model, cf = s.solve(asm, conflLimit=leafCap)
            per.append(cf)
            if sat is None:
                capped += 1
            elif sat:
                res = (True, model)
                break
            if time.time() - t0 > wallCap:
                censored = True
                break
        return res, per, capped, censored, time.time() - t0

    for stratum, idx, R, planted in targets:
        t0 = time.time()
        truth = dr.pairDecision(C, d, R, base)
        if planted is None:
            assert truth == []
        else:
            assert tuple(planted) in truth
        common = {'stratum': stratum, 'index': idx, 'R': [str(R[0]), str(R[1])], 'planted': planted,
                  'truth_triples': [list(t) for t in truth], 'truth_wall': round(time.time() - t0, 2)}
        s4 = weil.buildS4(pb, d, R[0]).cmsClauses()
        s4r = dr.railify(*s4)
        ll = llr = llsel = None
        if any(a.startswith('L') for a in arms):
            ll = dr.buildLinearLeaf(pb, d, R[0]).cmsClauses()
            sel = set(range(1, d + 1)) | set(range(6 * d, 6 * d + dr.N))
            llsel = dr.railify(*ll, selectors=sel)
        allLeaves = list(range(1, 1 << d))
        for arm in arms:
            row = dict(common)
            row['arm'] = arm
            if arm in ('a', 'b', 'L-free', 'L-rail-free'):
                if arm.startswith('L') and not (stratum == 'uniform' and idx < 2):
                    continue
                if arm == 'L-rail-free':
                    llr = dr.railify(*ll)
                inst = {'a': s4, 'b': s4r, 'L-free': ll, 'L-rail-free': llr}[arm]
                sat, model, cf, wall = free(inst, 20.0 if arm.startswith('L') else 45.0)
                xs = check(R, truth, sat, model)
                row.update({'status': 'CENSORED' if sat is None else ('SAT' if sat else 'UNSAT'),
                            'conflicts': cf, 'wall': round(wall, 3), 'xs': xs,
                            'cnf': [inst[0], len(inst[1]), len(inst[2])]})
            else:
                inst = {'c': s4r, 'a-scan': s4, 'L-scan': llsel, 'L-plain-scan': ll}[arm]
                leaves = allLeaves
                leafCap = 20000
                if arm == 'L-scan' and d >= 9 and not (stratum == 'uniform' and idx < 2):
                    continue
                if arm == 'L-plain-scan':
                    leaves = sorted(random.Random(SEED0 * 100 + d * 16 + idx).sample(allLeaves, 8))
                    leafCap = 5000
                if arm == 'a-scan' and d >= 9:
                    leaves = sorted(random.Random(SEED0 * 100 + d * 16 + idx).sample(allLeaves, 64))
                (sat, model), per, capped, censored, wall = scan(inst, leaves, leafCap)
                sampled = len(leaves) < len(allLeaves)
                if sat:
                    xs = check(R, truth, True, model)
                    status = 'SAT'
                elif censored:
                    xs, status = None, 'CENSORED'
                elif capped:
                    xs, status = None, 'UNDECIDED_CAPPED_LEAVES'
                elif sampled:
                    xs, status = None, 'SAMPLE_ALL_UNSAT'
                else:
                    xs = check(R, truth, False, None)
                    status = 'UNSAT'
                row.update({'status': status, 'conflicts': sum(per), 'leaves': len(per),
                            'capped_leaves': capped, 'leaf_cap': leafCap, 'sampled': sampled,
                            'per_leaf': per, 'wall': round(wall, 3), 'xs': xs,
                            'cnf': [inst[0], len(inst[1]), len(inst[2])]})
            emit(row)


if __name__ == '__main__':
    main()
