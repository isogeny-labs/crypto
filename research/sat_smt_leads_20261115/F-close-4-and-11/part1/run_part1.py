"""Part 1 runner. Writes raw rows incrementally to raw_l<L>.jsonl.
usage: run_part1.py --validate
       run_part1.py <l> <n> <nU> <nS> [--seeds 101,102,103] [--ks 1,2,3]
Each row: one (instance, arm, k, order_seed) with total conflicts over the 2^k cubes."""
import sys, json, time, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib4 import *

SEEDS = [101, 102, 103]
ARMS = ['lit', 'xor', 'sym']


def validate():
    F = Field(11)
    l = 3
    # planted SAT
    I = make_target(F, l, 1000 * l + 50, True)
    full, z = zero_set(F, I['basis'], I['xR'])
    tab = span_table(I['basis'])
    # planted tuple is a zero and lifts
    us = [coords(I['basis'], x) for x in I['planted']]
    assert full[us[0] | us[1] << l | us[2] << 2 * l] == 0, 'planted not a zero'
    from pdplib import lifts_to_points
    assert lifts_to_points(F, I['planted'], I['xR']), 'planted does not lift to +-R'
    print('planted SAT: zero and lifts OK')
    # CNF models == curve brute-force decomposition set, SAT and UNSAT
    for want in (True, False):
        J = make_target(F, l, 1000 * l + (60 if want else 0), want)
        tabJ = span_table(J['basis'])
        bf = curve_bruteforce(F, J['basis'], J['xR'])
        BX = build_x(F, J)
        BE = build_e(F, J)
        mx = all_models(BX)
        me = all_models(BE)
        # each model's x in V\{0}, lifts to +-R
        for u in mx:
            xs = [int(tabJ[u[i]]) for i in range(3)]
            assert all(xs), 'x=0 in a model'
            assert lifts_to_points(F, xs, J['xR']), 'model does not lift'
        assert mx == bf, ('lit CNF models != brute force', want, len(mx), len(bf))
        assert me == bf, ('sym CNF models != brute force', want, len(me), len(bf))
        print(('SAT' if want else 'UNSAT'), 'cell: lit & sym models == group-law brute force',
              '(', len(bf), 'decompositions )')
    # symbolic e-form numeric check (yesterday's)
    sj = json.load(open(os.path.join(HERE, 'symbolic_e.json')))
    assert sj['numeric_check_200_points'] is True
    print('symbolic e-form numeric check: True')
    print('VALIDATION OK')


def run_cell(l, n, nU, nS, seeds, ks, arms=None):
    arms = arms or ARMS
    F = Field(n)
    out = open(os.path.join(HERE, f'raw_l{l}.jsonl'), 'w')
    targets = [(i, False) for i in range(nU)] + [(50 + i, True) for i in range(nS)]
    for (idx, want) in targets:
        I = make_target(F, l, 1000 * l + idx, want)
        full, z = zero_set(F, I['basis'], I['xR'])
        truth = len(z) > 0 if want else True  # UNSAT instances have no (V\0)^3 zero by construction
        BX = build_x(F, I)
        BE = build_e(F, I)
        meta = dict(l=l, n=n, idx=idx, want_sat=want, nzeros=int(len(z)),
                    Xvars=BX['cnf'].nv, Xcl=len(BX['cnf'].cl), Evars=BE['cnf'].nv, Ecl=len(BE['cnf'].cl))
        for arm in arms:
            B = BX if arm in ('lit', 'xor') else BE
            for k in ks:
                for sd in seeds:
                    t0 = time.time()
                    r = run_cubes(B, arm, k, sd, 'cadical153', full)
                    row = dict(meta, arm=arm, k=k, order_seed=sd, wall=round(time.time() - t0, 3), **r)
                    out.write(json.dumps(row) + '\n')
                    out.flush()
                    print(l, idx, 'SAT' if want else 'UNS', arm, 'k', k, 'sd', sd,
                          'conf', r['conflicts'], 'nsat', r['n_sat_cubes'], round(time.time() - t0, 1), flush=True)
    out.close()


if __name__ == '__main__':
    if sys.argv[1] == '--validate':
        validate()
    else:
        l, n, nU, nS = int(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4])
        seeds = SEEDS
        ks = list(range(1, l + 1))
        arms = ARMS
        av = sys.argv[5:]
        for i, a in enumerate(av):
            if a == '--seeds':
                seeds = [int(x) for x in av[i + 1].split(',')]
            if a == '--ks':
                ks = [int(x) for x in av[i + 1].split(',')]
            if a == '--arms':
                arms = av[i + 1].split(',')
        run_cell(l, n, nU, nS, seeds, ks, arms)
