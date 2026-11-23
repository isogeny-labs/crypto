"""Part B: conflicts under XOR cubes (bits of e1) vs literal cubes (bits of x1), same CNF.
usage: run_sat.py out.json l n [ninst] [klist comma] [--fresh] [--cms]"""
import json, sys, time
import numpy as np
from pysat.solvers import Cadical153
from pdplib import *


def build(F, I):
    l, n = len(I['basis']), F.n
    full, _ = eval_table(F, I['basis'], I['xR'])
    anf = moebius(full)
    nb = 3 * l
    pc = popcounts(nb)
    nv = nb + l
    clauses, xors = [], []
    yv = []
    for j in range(l):
        y = nb + 1 + j
        yv.append(y)
        a, b, c = 1 + j, 1 + l + j, 1 + 2 * l + j
        xors.append(([a, b, c, y], 0))
    masks = np.nonzero(anf)[0]
    mono_var = {}
    for mk in masks:
        mk = int(mk)
        if mk == 0:
            continue
        vs = [b + 1 for b in range(nb) if mk >> b & 1]
        if len(vs) == 1:
            mono_var[mk] = vs[0]
        else:
            nv += 1
            mono_var[mk] = nv
            for v in vs:
                clauses.append([-nv, v])
            clauses.append([nv] + [-v for v in vs])
    c0 = int(anf[0])
    for i in range(n):
        lits = [mono_var[int(mk)] for mk in masks if mk and (int(anf[mk]) >> i & 1)]
        xors.append((lits, c0 >> i & 1))
    return dict(nvars=nv, clauses=clauses, xors=xors, yv=yv, full=full, nb=nb, l=l)


def xor_cnf(lits, rhs, nv, out):
    """XOR(lits) = rhs, chopped with aux vars; returns new nv."""
    lits = list(lits)
    if not lits:
        if rhs:
            out.append([])
        return nv
    while len(lits) > 3:
        nv += 1
        a, b = lits[0], lits[1]
        t = nv
        out += [[-a, -b, -t], [a, b, -t], [a, -b, t], [-a, b, t]]  # t = a^b
        lits = lits[2:] + [t]
    k = len(lits)
    for m in range(1 << k):
        if bin(m).count('1') % 2 != rhs:  # forbid assignments with wrong parity
            out.append([-lits[i] if m >> i & 1 else lits[i] for i in range(k)])
    return nv


def to_cnf(B):
    out = list(B['clauses'])
    nv = B['nvars']
    for lits, rhs in B['xors']:
        nv = xor_cnf(lits, rhs, nv, out)
    return out, nv


def cube_lits(B, arm, k, c):
    if arm == 'lit':
        return [(1 + j) if c >> j & 1 else -(1 + j) for j in range(k)]
    return [B['yv'][j] if c >> j & 1 else -B['yv'][j] for j in range(k)]


def check_model(B, model):
    idx = sum(1 << b for b in range(B['nb']) if model[b] > 0)
    assert B['full'][idx] == 0, 'model is not a zero of S4'
    return idx


def run_cadical(B, cnf, arm, k, fresh=False):
    tot = dict(conflicts=0, decisions=0, propagations=0, restarts=0)
    nsat = 0
    first_sat_conf = None
    t0 = time.time()
    s = None
    models = []
    for c in range(1 << k):
        if s is None or fresh:
            if s is not None:
                s.delete()
            s = Cadical153(bootstrap_with=cnf)
            prev = dict(conflicts=0, decisions=0, propagations=0, restarts=0)
        r = s.solve(assumptions=cube_lits(B, arm, k, c))
        st = s.accum_stats()
        for key in tot:
            tot[key] += st[key] - prev[key]
        prev = dict(st)
        if r:
            nsat += 1
            models.append(check_model(B, s.get_model()))
            if first_sat_conf is None:
                first_sat_conf = tot['conflicts']
    s.delete()
    tot.update(n_sat_cubes=nsat, time=time.time() - t0, conflicts_to_first_sat=first_sat_conf, models=models)
    return tot


def run_cms(B, arm, k):
    import pycryptosat
    s = pycryptosat.Solver(threads=1)
    s.add_clauses(B['clauses'])
    for lits, rhs in B['xors']:
        if lits:
            s.add_xor_clause(lits, bool(rhs))
        elif rhs:
            s.add_clause([1]); s.add_clause([-1])
    nsat = 0
    t0 = time.time()
    for c in range(1 << k):
        r, model = s.solve(cube_lits(B, arm, k, c))
        if r:
            nsat += 1
            idx = sum(1 << b for b in range(B['nb']) if model[b + 1])
            assert B['full'][idx] == 0
    return dict(n_sat_cubes=nsat, time=time.time() - t0)


if __name__ == '__main__':
    out, l, n = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
    args = [a for a in sys.argv[4:] if not a.startswith('--')]
    ninst = int(args[0]) if args else 8
    ks = [int(x) for x in args[1].split(',')] if len(args) > 1 else list(range(1, l + 1))
    fresh = '--fresh' in sys.argv
    cms = '--cms' in sys.argv
    F = Field(n)
    rows = []
    for i in range(ninst):
        seed = 1000 * l + i
        I = make_instance(F, l, seed, planted=(i < 4))
        B = build(F, I)
        cnf, nvc = to_cnf(B)
        zeros = np.nonzero(B['full'] == 0)[0]
        truth = len(zeros) > 0
        tab = span_table(I['basis'])
        lm = (1 << l) - 1
        nlift = sum(lifts_to_points(F, [int(tab[z & lm]), int(tab[z >> l & lm]), int(tab[z >> 2 * l & lm])], I['xR'])
                    for z in zeros.tolist())
        base = dict(l=l, n=n, seed=seed, planted=i < 4, truth_sat=truth, truth_nsol=len(zeros), truth_nsol_lifting=nlift,
                    cnf_vars=nvc, cnf_clauses=len(cnf), fresh=fresh)
        if I['planted']:
            c = [coords(I['basis'], x) for x in I['planted']]
            assert B['full'][c[0] | c[1] << l | c[2] << 2 * l] == 0
        jobs = [('plain', 0)]
        for k in ks:
            jobs += [('lit', k), ('xor', k)]
        for arm, k in jobs:
            r = run_cadical(B, cnf, arm, k, fresh)
            assert (r['n_sat_cubes'] > 0) == truth, 'SAT/UNSAT disagrees with brute force'
            models = r.pop('models')
            r['models_lifting'] = sum(
                lifts_to_points(F, [int(tab[z & lm]), int(tab[z >> l & lm]), int(tab[z >> 2 * l & lm])], I['xR']) for z in models)
            r.update(base, arm=arm, k=k, solver='cadical153')
            rows.append(r)
            print(l, seed, 'SAT' if truth else 'UNS', arm, k, r['conflicts'], r['decisions'], round(r['time'], 2), flush=True)
            if cms and (k in (0, l)):
                r2 = run_cms(B, arm, k)
                assert (r2['n_sat_cubes'] > 0) == truth
                r2.update(base, arm=arm, k=k, solver='cryptominisat-nativexor')
                rows.append(r2)
                print(l, seed, 'cms', arm, k, round(r2['time'], 2), flush=True)
        json.dump(rows, open(out, 'w'))
