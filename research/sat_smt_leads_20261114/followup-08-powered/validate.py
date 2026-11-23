"""Validation at the smallest cell: exhaustive agreement of the fixed CNF with brute force.
usage: python3 validate.py [n l]   (default 11 3)"""
import sys, random, json
from pysat.solvers import Cadical153
from pdp import Setup, build2, assumptions, lift_model
from common import MASTER, cell_seed


def brute_triples(S):
    """independent of Setup.decomposable: every P1+P2+P3 over the signed base."""
    E = S.E; xs = set()
    for P in S.base:
        for Q in S.base:
            PQ = E.add(P, Q)
            for T in S.base:
                R = E.add(PQ, T)
                if R is not None:
                    xs.add(R[0])
    return xs


def main(n, l):
    S = Setup(n, l, cell_seed(l))
    dec = brute_triples(S)
    I = build2(S)
    inc = Cadical153(bootstrap_with=I['clauses'])
    rat = [x for x in range(1, 1 << n) if S.E.lift(x) is not None and x not in S.Vset]
    out = dict(n=n, l=l, rational_abscissae_outside_V=len(rat), signed_base=len(S.base),
               decomposable_outside_V=sum(1 for x in rat if x in dec),
               mitm_vs_brute=0, param_mismatch=0, const_mismatch=0, lift_fail=0, sat=0, unsat=0)
    for x in rat:
        R = S.E.lift(x); truth = x in dec
        if S.decomposable(R) != truth:
            out['mitm_vs_brute'] += 1
        res = inc.solve(assumptions=assumptions(I, x))
        if res != truth:
            out['param_mismatch'] += 1
        elif res and not lift_model(S, I, inc.get_model(), R)[0]:
            out['lift_fail'] += 1
        J = build2(S, x)
        s = Cadical153(bootstrap_with=J['clauses']); r2 = s.solve()
        if r2 != truth:
            out['const_mismatch'] += 1
        elif r2 and not lift_model(S, J, s.get_model(), R)[0]:
            out['lift_fail'] += 1
        s.delete()
        out['sat' if res else 'unsat'] += 1
    # planted targets
    rng = random.Random(cell_seed(l) + 7); pl = pl_ok = 0
    while pl < 20:
        R = S.planted(rng)
        if R is None or R[0] == 0 or R[0] in S.Vset:
            continue
        pl += 1
        if inc.solve(assumptions=assumptions(I, R[0])) and lift_model(S, I, inc.get_model(), R)[0]:
            pl_ok += 1
    out['planted'] = pl; out['planted_sat_and_lifted'] = pl_ok
    # the defect, shown: old encoding (nonzero=False) on the same abscissae
    I0 = build2(S, nonzero=False); s0 = Cadical153(bootstrap_with=I0['clauses'])
    out['old_encoding_mismatch'] = sum(1 for x in rat if s0.solve(assumptions=assumptions(I0, x)) != (x in dec))
    out['ok'] = (out['mitm_vs_brute'] == out['param_mismatch'] == out['const_mismatch'] == out['lift_fail'] == 0
                 and pl_ok == pl)
    print(json.dumps(out, indent=1))
    return out


if __name__ == '__main__':
    a = sys.argv[1:]
    o = main(int(a[0]), int(a[1])) if a else main(11, 3)
    json.dump(o, open('validation_l%d.json' % o['l'], 'w'), indent=1)
    sys.exit(0 if o['ok'] else 1)
