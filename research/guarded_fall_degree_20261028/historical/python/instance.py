"""Canonical, instance-bound representation of polynomial inputs."""

import hashlib
import json


def normalized_problem(problem):
    p, n = problem['p'], problem['n']
    if type(p) is not int or p < 2 or type(n) is not int or n < 1:
        raise ValueError('invalid modulus or variable count')
    polynomials = []
    for f in problem['polynomials']:
        terms = {}
        for c, exp in f:
            if (type(c) is not int or len(exp) != n or
                    any(type(e) is not int or e < 0 for e in exp)):
                raise ValueError('invalid polynomial term')
            e = tuple(exp)
            terms[e] = (terms.get(e, 0) + c) % p
        polynomials.append([[terms[e], list(e)] for e in sorted(terms) if terms[e]])
    # Preserve polynomial order so derivation input indices retain their meaning.
    return {'p': p, 'n': n, 'polynomials': polynomials}


def problem_digest(problem):
    raw = json.dumps(normalized_problem(problem), sort_keys=True,
                     separators=(',', ':')).encode()
    return hashlib.sha256(raw).hexdigest()
