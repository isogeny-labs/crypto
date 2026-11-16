"""Reuse experiment: INC vs FRESH-A vs FRESH-U vs SPEC on the same targets, interleaved."""
import random, sys, time, json
from pysat.solvers import Cadical153, Glucose4
from pdp import Setup, build2, assumptions, lift_model

MASTER = 20261114


def make_targets(S, nd, nu, seed):
    rng = random.Random(seed)
    used = set()
    T = []
    while sum(1 for t in T if t[1]) < nd:
        R = S.planted(rng)
        if R is None or R[0] == 0 or R[0] in S.Vset or R[0] in used:
            continue
        assert S.decomposable(R)
        used.add(R[0]); T.append((R, True))
    drawn = dec = 0
    while sum(1 for t in T if not t[1]) < nu:
        R = S.random_point(rng)
        if R[0] in S.Vset or R[0] in used:
            continue
        drawn += 1
        if S.decomposable(R):
            dec += 1
            continue
        used.add(R[0]); T.append((R, False))
    rng.shuffle(T)
    return T, drawn, dec


def delta(a, b):
    return {k: b[k] - a[k] for k in ('conflicts', 'decisions', 'propagations')}


def proof_stats(proof, inst):
    rset = set(inst['r']); cone = inst['cone']
    hist_r = {}; hist_c = {}; nl = 0; tl = 0
    for line in proof:
        if line.startswith('d'):
            continue
        lits = [abs(int(x)) for x in line.split()[:-1]]
        if not lits:
            continue
        nl += 1; tl += len(lits)
        kr = sum(1 for v in lits if v in rset)
        kc = sum(1 for v in lits if v in cone)
        hist_r[kr] = hist_r.get(kr, 0) + 1
        hist_c[kc] = hist_c.get(kc, 0) + 1
    return dict(learnts=nl, mean_len=(tl / nl if nl else None), hist_r=hist_r, hist_cone=hist_c)


def main(n, l, nd, nu, out):
    seed = MASTER + 1000 * l
    S = Setup(n, l, seed)
    T, drawn, dec = make_targets(S, nd, nu, seed + 1)
    I = build2(S)
    meta = dict(kind='meta', n=n, l=l, seed=seed, mod=S.F.mod, V=S.V, signed_base=len(S.base),
                sum2x=len(S.sum2x), vars=I['nv'], clauses=len(I['clauses']), ands=I['nand'],
                random_drawn=drawn, random_decomposable=dec, targets=len(T))
    f = open(out, 'w'); f.write(json.dumps(meta) + '\n'); f.flush()
    inc = {'cadical153': Cadical153(bootstrap_with=I['clauses']), 'glucose4': Glucose4(bootstrap_with=I['clauses'])}
    cls = {'cadical153': Cadical153, 'glucose4': Glucose4}
    for idx, (R, truth) in enumerate(T):
        row = dict(kind='row', n=n, l=l, idx=idx, xR=R[0], decomposable=truth, arms={})
        asm = assumptions(I, R[0])
        errs = []

        def check(name, res, model, inst):
            if res != truth:
                errs.append(name + ':verdict')
            elif res:
                ok, _ = lift_model(S, inst, model, R)
                if not ok:
                    errs.append(name + ':lift')
        for sv in ('cadical153', 'glucose4'):
            s = inc[sv]
            a = s.accum_stats(); t0 = time.time()
            res = s.solve(assumptions=asm)
            d = delta(a, s.accum_stats()); d['secs'] = round(time.time() - t0, 3)
            if not res:
                d['core'] = len(s.get_core() or [])
            check(sv + '/INC', res, s.get_model() if res else None, I)
            row['arms'][sv + '/INC'] = d
            # FRESH-A
            wp = (sv == 'glucose4')
            s2 = cls[sv](bootstrap_with=I['clauses'], with_proof=wp) if wp else cls[sv](bootstrap_with=I['clauses'])
            t0 = time.time(); res = s2.solve(assumptions=asm)
            d = dict(s2.accum_stats()); d.pop('restarts', None); d['secs'] = round(time.time() - t0, 3)
            if not res:
                d['core'] = len(s2.get_core() or [])
            check(sv + '/FRESH-A', res, s2.get_model() if res else None, I)
            if wp:
                d['proof'] = proof_stats(s2.get_proof() or [], I)
            row['arms'][sv + '/FRESH-A'] = d
            s2.delete()
            if sv == 'cadical153':
                s3 = Cadical153(bootstrap_with=I['clauses'] + [[a_] for a_ in asm])
                t0 = time.time(); res = s3.solve()
                d = dict(s3.accum_stats()); d.pop('restarts', None); d['secs'] = round(time.time() - t0, 3)
                check('cadical153/FRESH-U', res, s3.get_model() if res else None, I)
                row['arms']['cadical153/FRESH-U'] = d; s3.delete()
                J = build2(S, R[0])
                s4 = Cadical153(bootstrap_with=J['clauses'])
                t0 = time.time(); res = s4.solve()
                d = dict(s4.accum_stats()); d.pop('restarts', None); d['secs'] = round(time.time() - t0, 3)
                d['vars'] = J['nv']; d['clauses'] = len(J['clauses'])
                check('cadical153/SPEC', res, s4.get_model() if res else None, J)
                row['arms']['cadical153/SPEC'] = d; s4.delete()
        row['errors'] = errs
        f.write(json.dumps(row) + '\n'); f.flush()
        print(l, idx, truth, {k: v['conflicts'] for k, v in row['arms'].items()}, errs, flush=True)
    f.close()


if __name__ == '__main__':
    n, l, nd, nu = map(int, sys.argv[1:5])
    main(n, l, nd, nu, sys.argv[5])
