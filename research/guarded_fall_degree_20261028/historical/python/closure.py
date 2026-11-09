"""Exact bounded-degree closure over a specified prime field.

Uses the ordinary polynomial ring, total degree, and explicit input equations.
Boolean reduction is NOT implicit. Input JSON: {"p": 3, "n": 2,
"polynomials": [[[1,[1,1]],[1,[1,0]],[1,[0,1]],[1,[0,0]]], ...]}.
Each term is [coefficient, exponent_vector]. This is a small reference engine,
not a scalable Semaev solver or an implementation of a first-fall invariant.
The certificate proves 1 is not in V_(F,D); unsatisfiability must be established
separately to turn this into a last-fall-degree lower bound. The supplied
modulus must be prime. Upper-bound derivations are handled by proof_trace.py.
"""

import argparse
import itertools
import json
from math import comb
from pathlib import Path
from instance import normalized_problem, problem_digest


def monomials(n, d):
    """Descending graded lexicographic order, including the constant."""
    def exact(k, total):
        if k == 0:
            if total == 0:
                yield ()
        else:
            for a in range(total, -1, -1):
                for rest in exact(k - 1, total - a):
                    yield (a,) + rest
    return [e for t in range(d, -1, -1) for e in exact(n, t)]


def vector(terms, columns, p):
    index = {e: j for j, e in enumerate(columns)}
    row = [0] * len(columns)
    for c, exp in terms:
        row[index[tuple(exp)]] = (row[index[tuple(exp)]] + c) % p
    return row


def inputs_at_degree(problem, d, columns):
    problem = normalized_problem(problem)
    return [vector(f, columns, problem['p']) for f in problem['polynomials']
            if max((sum(e) for c,e in f), default=-1) <= d]


def degree(row, columns):
    return max((sum(e) for a, e in zip(row, columns) if a), default=-1)


def multiply(row, variable, columns):
    index = {e: j for j, e in enumerate(columns)}
    result = [0] * len(columns)
    for a, exp in zip(row, columns):
        if a:
            e = list(exp)
            e[variable] += 1
            result[index[tuple(e)]] = a
    return result


def insert_rref(basis, row, p):
    """Incremental full RREF; basis maps leading columns to rows."""
    row = list(row)
    for pivot in sorted(basis):
        if row[pivot]:
            c = row[pivot]
            row = [(a - c*b) % p for a, b in zip(row, basis[pivot])]
    if not any(row):
        return False
    pivot = next(j for j, a in enumerate(row) if a)
    inv = pow(row[pivot], -1, p)
    row = [(a*inv) % p for a in row]
    for j, old in list(basis.items()):
        if old[pivot]:
            c = old[pivot]
            basis[j] = [(a - c*b) % p for a, b in zip(old, row)]
    basis[pivot] = row
    return True


def closure(problem, d):
    problem = normalized_problem(problem)
    p, n = problem['p'], problem['n']
    if type(d) is not int or d < 0:
        raise ValueError('degree must be a nonnegative integer')
    if comb(n+d, n) > 20000:
        raise ValueError('reference engine limit: more than 20000 columns')
    columns = monomials(n, d)
    basis = {}
    for row in inputs_at_degree(problem, d, columns):
        insert_rref(basis, row, p)
    rounds = 0
    while True:
        rounds += 1
        low = [list(row) for row in basis.values()
               if degree(row, columns) < d]
        changed = False
        for row in low:
            for i in range(n):
                changed |= insert_rref(basis, multiply(row, i, columns), p)
        if not changed:
            break
    rows = [basis[j] for j in sorted(basis)]
    contradiction = len(columns)-1 in basis
    dual = None
    if not contradiction:
        # A linear functional lambda with lambda(1)=1 and lambda(W)=0.
        dual = [0] * len(columns)
        dual[-1] = 1
        for j, row in basis.items():
            dual[j] = -row[-1] % p
    return {'problem': problem, 'problem_sha256': problem_digest(problem),
            'degree': d, 'basis': rows, 'dual': dual,
            'contains_one': contradiction, 'rounds': rounds,
            'column_count': len(columns), 'rank': len(rows)}


def rank_mod(rows, p):
    """Separate batch forward elimination, used only by the checker."""
    a = [list(row) for row in rows]
    if not a:
        return 0
    rank = 0
    for col in range(len(a[0])):
        pivot = next((i for i in range(rank, len(a)) if a[i][col] % p), None)
        if pivot is None:
            continue
        a[rank], a[pivot] = a[pivot], a[rank]
        inv = pow(a[rank][col] % p, -1, p)
        for i in range(rank+1, len(a)):
            c = a[i][col] * inv % p
            if c:
                a[i] = [(x-c*y) % p for x, y in zip(a[i], a[rank])]
        rank += 1
        if rank == len(a):
            break
    return rank


def check_lower_certificate(cert, expected_problem=None):
    """Verify V_(F,D) is contained in W and 1 is not in W.

    Does not trust the producer's closure flag, dimensions, or computation log.
    Certifies a closed separating superspace, not exact closure dimensions.
    For monomials in descending degree order, echelon rows of degree < D
    span W intersect R_(<=D-1). Checking their variable multiples is enough.
    The prime modulus is a mathematical precondition, not certified here.
    """
    try:
        return _check_lower_certificate(cert, expected_problem)
    except (KeyError, TypeError, ValueError, IndexError, OverflowError):
        return False


def _check_lower_certificate(cert, expected_problem):
    problem, d = normalized_problem(cert['problem']), cert['degree']
    p, n = problem['p'], problem['n']
    if type(d) is not int or d < 0 or comb(n+d,n) > 20000:
        return False
    if expected_problem is not None and problem != normalized_problem(expected_problem):
        return False
    if cert.get('problem_sha256', problem_digest(problem)) != problem_digest(problem):
        return False
    columns = monomials(n, d)
    rows, dual = cert['basis'], cert['dual']
    if (dual is None or len(dual) != len(columns) or
            any(type(a) is not int or not 0 <= a < p for a in dual) or
            dual[-1] != 1):
        return False
    if any(len(r) != len(columns) or not any(a % p for a in r) for r in rows):
        return False
    if any(any(type(a) is not int or not 0 <= a < p for a in r) for r in rows):
        return False
    pivots = [next(j for j, a in enumerate(r) if a) for r in rows]
    if pivots != sorted(set(pivots)):
        return False
    # An independent rank check confirms the supplied echelon representation.
    rank = rank_mod(rows, p)
    if rank != len(rows):
        return False
    if any(sum(a*b for a, b in zip(r, dual)) % p for r in rows):
        return False
    queries = inputs_at_degree(problem, d, columns)
    for row in rows:
        if degree(row, columns) < d:
            queries.extend(multiply(row, i, columns) for i in range(n))
    return all(rank_mod(rows + [query], p) == rank for query in queries)


def self_test():
    # Over F_3, (x+1)(y+1) is nonzero on the Boolean square.
    # The complete degree-2 space cannot refute it; degree 3 can.
    f = [[1,[1,1]], [1,[1,0]], [1,[0,1]], [1,[0,0]]]
    problem = {'p': 3, 'n': 2, 'polynomials':
               [f, [[1,[2,0]],[-1,[1,0]]], [[1,[0,2]],[-1,[0,1]]]]}
    assert all((x+1)*(y+1) % 3 for x,y in itertools.product((0,1), repeat=2))
    low, high = closure(problem, 2), closure(problem, 3)
    assert not low['contains_one'] and check_lower_certificate(low)
    assert high['contains_one']
    bad = json.loads(json.dumps(low))
    bad['dual'][-1] = 0
    assert not check_lower_certificate(bad)
    # An initial Macaulay span is generally not a closed space.
    bad = json.loads(json.dumps(high))
    cols = monomials(2, 3)
    basis = {}
    for row in inputs_at_degree(problem, 3, cols):
        insert_rref(basis, row, 3)
    bad['basis'] = [basis[j] for j in sorted(basis)]
    bad['dual'] = [0] * len(cols)
    bad['dual'][-1] = 1
    for j, row in basis.items():
        bad['dual'][j] = -row[-1] % 3
    assert not check_lower_certificate(bad)
    # Cancellation must be normalized before deciding which inputs enter V_D.
    cancellation = {'p': 3, 'n': 1, 'polynomials':
                    [[[1,[20]],[-1,[20]],[1,[0]]]]}
    assert closure(cancellation, 0)['contains_one']
    return {'self_test': 'passed', 'example': problem,
            'degree_2_lower_certificate': low,
            'degree_3_closure': high}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('input', nargs='?', type=Path)
    parser.add_argument('--degree', type=int)
    parser.add_argument('--verify', action='store_true')
    parser.add_argument('--expected', type=Path,
                        help='expected input instance; required with --verify')
    parser.add_argument('--self-test', action='store_true')
    args = parser.parse_args()
    if args.self_test:
        result = self_test()
    elif args.verify:
        if args.expected is None:
            parser.error('--verify requires --expected to bind the claimed instance')
        result = {'valid_lower_certificate': check_lower_certificate(
            json.loads(args.input.read_text()), json.loads(args.expected.read_text()))}
    else:
        result = closure(json.loads(args.input.read_text()), args.degree)
    print(json.dumps(result, indent=2))


if __name__ == '__main__':
    main()
