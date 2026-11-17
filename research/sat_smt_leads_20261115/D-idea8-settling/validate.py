"""Label validation at the smallest cell (l=3, n=11), all three V seeds, exhaustive over every
rational abscissa outside V. Checks: independent triple enumeration = 2+1 MITM ground truth;
parametric CNF (with and without the rationality clauses) = ground truth; constant-target CNF =
ground truth on every decomposable abscissa and every third other; every SAT model lifts.
usage: python3 validate.py [out.json]"""
import sys, json
from pysat.solvers import Cadical153
from pdp import Setup, build2, assumptions, lift_model
from common import cell_seed, NSEEDS, CELLS
from run import twist_pairs


def brute_triples(S):
    E = S.E; xs = set()
    for P in S.base:
        for Q in S.base:
            PQ = E.add(P, Q)
            for T in S.base:
                R = E.add(PQ, T)
                if R is not None:
                    xs.add(R[0])
    return xs


def one(l, sidx):
    n = CELLS[l]['n']
    S = Setup(n, l, cell_seed(l, sidx))
    dec = brute_triples(S)
    I = build2(S); I0 = build2(S, rational=False)
    inc = Cadical153(bootstrap_with=I['clauses']); inc0 = Cadical153(bootstrap_with=I0['clauses'])
    rat = [x for x in range(1, 1 << n) if S.E.lift(x) is not None and x not in S.Vset]
    o = dict(n=n, l=l, seed_index=sidx, V=S.V, rational_abscissae_outside_V=len(rat), signed_base=len(S.base),
             nonrational_in_V=I['nonrational_in_V'], twist_pair_elements=twist_pairs(S),
             decomposable_outside_V=sum(1 for x in rat if x in dec),
             mitm_vs_brute=0, param_mismatch=0, param_norational_mismatch=0, label_changed_by_rationality=0,
             const_checked=0, const_mismatch=0, lift_fail=0)
    for k, x in enumerate(rat):
        R = S.E.lift(x); truth = x in dec
        o['mitm_vs_brute'] += S.decomposable(R) != truth
        asm = assumptions(I, x)
        res = inc.solve(assumptions=asm); res0 = inc0.solve(assumptions=asm)
        o['param_mismatch'] += res != truth
        o['param_norational_mismatch'] += res0 != truth
        o['label_changed_by_rationality'] += res != res0
        if res and res == truth and not lift_model(S, I, inc.get_model(), R)[0]:
            o['lift_fail'] += 1
        if truth or k % 3 == 0:
            J = build2(S, x); s = Cadical153(bootstrap_with=J['clauses']); r2 = s.solve()
            o['const_checked'] += 1
            if r2 != truth:
                o['const_mismatch'] += 1
            elif r2 and not lift_model(S, J, s.get_model(), R)[0]:
                o['lift_fail'] += 1
            s.delete()
    o['ok'] = o['mitm_vs_brute'] == o['param_mismatch'] == o['const_mismatch'] == o['lift_fail'] == 0
    return o


if __name__ == '__main__':
    res = [one(3, s) for s in range(NSEEDS)]
    print(json.dumps(res, indent=1))
    json.dump(res, open(sys.argv[1] if len(sys.argv) > 1 else 'validation_l3.json', 'w'), indent=1)
    sys.exit(0 if all(r['ok'] for r in res) else 1)
