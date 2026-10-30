"""Produce and replay bounded-degree ideal derivations.

The replay checker uses sparse polynomials and no row-reduction code. Every
non-input node is a scalar multiple, a sum, or a permitted variable multiple.
Certificates are bound to a separately supplied expected problem. A successful
replay establishes 1 in V_(F,D), independently of the producer's closure flags.
It is exact Python arithmetic, not a Lean-kernel proof. The dense producer is
only a small reference implementation.
"""

import argparse
import json
from pathlib import Path

from closure import degree, monomials, multiply, vector
from instance import normalized_problem, problem_digest


def traced_refutation(problem, d):
    problem = normalized_problem(problem)
    p, n = problem['p'], problem['n']
    if type(d) is not int or d < 0:
        raise ValueError('degree must be a nonnegative integer')
    from math import comb
    if comb(n+d, n) > 20000:
        raise ValueError('reference engine limit: more than 20000 columns')
    columns = monomials(n, d)
    basis, nodes = {}, []

    def node(op, **data):
        nodes.append({'op': op, **data})
        return len(nodes)-1

    def insert(row, source):
        row = list(row)
        for pivot in sorted(basis):
            old, old_source = basis[pivot]
            c = row[pivot]
            if c:
                row = [(a-c*b) % p for a,b in zip(row, old)]
                source = node('add', left=source, right=old_source, coefficient=-c % p)
        if not any(row):
            return False
        pivot = next(i for i,a in enumerate(row) if a)
        inv = pow(row[pivot], -1, p)
        if inv != 1:
            row = [a*inv % p for a in row]
            source = node('scale', parent=source, coefficient=inv)
        for j, (old, old_source) in list(basis.items()):
            c = old[pivot]
            if c:
                old = [(a-c*b) % p for a,b in zip(old,row)]
                old_source = node('add', left=old_source, right=source, coefficient=-c % p)
                basis[j] = old, old_source
        basis[pivot] = row, source
        return True

    for i, f in enumerate(problem['polynomials']):
        if max((sum(e) for c,e in f), default=-1) <= d:
            insert(vector(f, columns, p), node('input', index=i))
    while len(columns)-1 not in basis:
        low = [(list(row), source) for row,source in basis.values()
               if degree(row, columns) < d and any(row)]
        changed = False
        for row, source in low:
            for i in range(n):
                new_source = node('multiply', parent=source, variable=i)
                changed |= insert(multiply(row, i, columns), new_source)
        if not changed:
            return None
    goal = basis[len(columns)-1][1]
    # Retain only the ancestry of the contradiction.
    keep, pending = set(), [goal]
    while pending:
        j = pending.pop()
        if j in keep:
            continue
        keep.add(j)
        item = nodes[j]
        pending.extend(item[k] for k in ('parent','left','right') if k in item)
    ordered = sorted(keep)
    remap = {j:i for i,j in enumerate(ordered)}
    compact = []
    for j in ordered:
        item = dict(nodes[j])
        for key in ('parent','left','right'):
            if key in item:
                item[key] = remap[item[key]]
        compact.append(item)
    return {'schema': 'bounded-degree-refutation-v1', 'degree': d,
            'problem_sha256': problem_digest(problem), 'nodes': compact,
            'goal': remap[goal]}


def verify_refutation(expected_problem, certificate):
    """Independent sparse replay. Invalid encodings return False.

    No primality test is needed for this upper certificate: the replayed
    operations establish a valid ideal identity even over Z/pZ. Interpreting
    it as a field last-fall bound presumes the expected modulus is prime.
    """
    try:
        expected = normalized_problem(expected_problem)
        p, n = expected['p'], expected['n']
        d = certificate['degree']
        if (certificate['schema'] != 'bounded-degree-refutation-v1' or
                type(d) is not int or d < 0 or
                certificate['problem_sha256'] != problem_digest(expected)):
            return False
        values = []

        def prior(j):
            if type(j) is not int or not 0 <= j < len(values):
                raise ValueError('invalid DAG reference')
            return values[j]

        def coefficient(c):
            if type(c) is not int or not 0 <= c < p:
                raise ValueError('invalid coefficient')
            return c

        for item in certificate['nodes']:
            op = item['op']
            if op == 'input':
                i = item['index']
                if type(i) is not int or not 0 <= i < len(expected['polynomials']):
                    return False
                result = {tuple(e):c for c,e in expected['polynomials'][i]}
            elif op == 'scale':
                c = coefficient(item['coefficient'])
                result = {e:a*c % p for e,a in prior(item['parent']).items()}
            elif op == 'add':
                result = dict(prior(item['left']))
                c = coefficient(item['coefficient'])
                for e,a in prior(item['right']).items():
                    result[e] = (result.get(e,0) + c*a) % p
            elif op == 'multiply':
                i = item['variable']
                if type(i) is not int or not 0 <= i < n:
                    return False
                result = {}
                for e,a in prior(item['parent']).items():
                    exp = list(e)
                    exp[i] += 1
                    result[tuple(exp)] = a
            else:
                return False
            result = {e:a for e,a in result.items() if a}
            if any(sum(e) > d for e in result):
                return False
            values.append(result)
        goal = certificate['goal']
        if type(goal) is not int or not 0 <= goal < len(values):
            return False
        return values[goal] == {(0,)*n:1}
    except (KeyError, TypeError, ValueError, IndexError, OverflowError):
        return False


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('problem', type=Path)
    parser.add_argument('--degree', type=int)
    parser.add_argument('--verify', type=Path)
    args = parser.parse_args()
    problem = json.loads(args.problem.read_text())
    if args.verify:
        result = {'valid_refutation': verify_refutation(problem, json.loads(args.verify.read_text()))}
    else:
        result = traced_refutation(problem, args.degree)
    print(json.dumps(result, indent=2))


if __name__ == '__main__':
    main()
