"""Decide one Boolean polynomial system (the JSON format of bool_f4_file.rs)
with CryptoMiniSat through pycryptosat, using native XOR clauses.

    python3 sat_solve.py SYSTEM.json BUDGET_SECONDS

Encoding: every monomial of degree >= 2 gets one Tseitin variable t with
t <-> AND(vars) (k + 1 clauses); every polynomial becomes one XOR clause over
its linear variables and its monomial variables, right-hand side its constant
term.  Nothing else is added: no symmetry breaking, no domain clauses.  The
SAT arm therefore sees exactly the equations the F4 arm sees.

Output: one JSON line with status (sat | unsat | undecided), wall seconds,
the solver's own conflict/decision/propagation counters parsed from its
verbose report, the encoding size, and on sat the model as a hex bitmask of
the system variables (bit i = v_i) for independent verification.
"""

import json
import os
import re
import sys
import tempfile
import time


def encode(system):
    import pycryptosat
    nVars = system['n_vars']
    budget = float(sys.argv[2]) if len(sys.argv) > 2 else 1e9
    solver = pycryptosat.Solver(verbose=1, time_limit=budget, threads=1)
    nxt = nVars + 1  # DIMACS variables are 1-based; v_i -> i + 1
    monoVar = {}
    clauses = 0
    xors = 0
    for poly in system['polys']:
        lits = []
        rhs = False
        for mono in poly:
            if not mono:
                rhs = not rhs
            elif len(mono) == 1:
                lits.append(mono[0] + 1)
            else:
                key = tuple(mono)
                t = monoVar.get(key)
                if t is None:
                    t = nxt
                    nxt += 1
                    monoVar[key] = t
                    for v in key:
                        solver.add_clause([-t, v + 1])
                    solver.add_clause([t] + [-(v + 1) for v in key])
                    clauses += len(key) + 1
                lits.append(t)
        if lits:
            solver.add_xor_clause(lits, rhs)
            xors += 1
        elif rhs:
            # constant 1 = 0: inconsistent
            solver.add_clause([1])
            solver.add_clause([-1])
            clauses += 2
    return solver, nVars, nxt - 1, clauses, xors


def main():
    system = json.load(open(sys.argv[1]))
    solver, nVars, totalVars, clauses, xors = encode(system)
    # capture the C++ verbose report written to fd 1
    fd = tempfile.TemporaryFile(mode='w+b')
    saved = os.dup(1)
    sys.stdout.flush()
    os.dup2(fd.fileno(), 1)
    t0 = time.time()
    try:
        sat, model = solver.solve()
    finally:
        wall = time.time() - t0
        sys.stdout.flush()
        os.dup2(saved, 1)
        os.close(saved)
    fd.seek(0)
    report = fd.read().decode('utf-8', 'replace')
    stats = {}
    # pycryptosat 5.18 verbose=1 prints one summary line per solve call:
    #   c [solve] 1 ret: l_True confl: 614 assumps: 0 T: 0.02
    m = re.search(r'^c \[solve\].*?confl:\s*([0-9]+)', report, re.M)
    if m:
        stats['conflicts'] = int(m.group(1))
    for name in ('decisions', 'propagations'):
        m = re.search(r'^c\s+%s\s*:\s*([0-9]+)' % name, report, re.M)
        if m:
            stats[name] = int(m.group(1))
    stats['report_bytes'] = len(report)
    out = {
        'status': 'sat' if sat is True else ('unsat' if sat is False else 'undecided'),
        'wall_s': wall, 'system_vars': nVars, 'cnf_vars': totalVars,
        'and_clauses': clauses, 'xor_clauses': xors,
    }
    out.update(stats)
    if sat is True:
        bits = 0
        for i in range(nVars):
            if model[i + 1]:
                bits |= 1 << i
        out['model_hex'] = hex(bits)
    print(json.dumps(out))


if __name__ == '__main__':
    main()
