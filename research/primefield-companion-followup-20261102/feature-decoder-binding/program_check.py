"""Independent full-field verifier, using square-set characters and no solver imports."""
from math import isqrt


def evaluate(program):
    r = program['r']
    if type(r) is not int or not 3 < r <= 10000 or any(r % d == 0 for d in range(2, isqrt(r)+1)):
        raise ValueError('bounded prime reference domain')
    if program.get('schema') != 'public-character-polynomial/v1':
        raise ValueError('program schema')
    terms = program['terms']
    labels = []
    for term in terms:
        S, c = term['offsets'], term['coefficient']
        if (type(S) is not list or any(type(d) is not int or not 0 <= d < r for d in S)
                or S != sorted(set(S)) or type(c) is not int or not 0 < c < r):
            raise ValueError('canonical distinct-offset term')
        labels.append(tuple(S))
    if len(labels) != len(set(labels)):
        raise ValueError('duplicate term')
    squares = {k*k % r for k in range(1, (r+1)//2)}
    chi = [0 if x == 0 else 1 if x in squares else r-1 for x in range(r)]
    values = []
    for x in range(r):
        total = 0
        for term in terms:
            value = term['coefficient']
            for d in term['offsets']:
                value = value*chi[(x+d) % r] % r
            total = (total+value) % r
        values.append(total)
    return values


def verify(program):
    if program['r'] != 479:
        raise ValueError('this repair certificate is scoped to F479')
    new = [t for t in program['terms'] if any(d >= 15 for d in t['offsets'])]
    if new != [{'offsets': [15], 'coefficient': 405}]:
        raise ValueError('expected exactly the declared new linear term')
    if any(len(t['offsets']) > 3 for t in program['terms']):
        raise ValueError('cubic family exceeded')
    values = evaluate(program)
    failures = [x for x, v in enumerate(values) if x != v]
    if failures:
        raise ValueError('identity fails at ' + repr(failures[:10]))
    return dict(verified=True, points_checked=479, new_shift_terms=1,
                new_linear_coefficient=405, max_degree=max(len(t['offsets']) for t in program['terms']),
                checker='independent square-set full-field evaluation')
