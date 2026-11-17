"""Independent global feature-annihilation check; no producer imports or caches."""
from itertools import combinations


def verify(r, q, support):
    """r is a caller-certified prime >3; accept only a normalized exact certificate."""
    if type(r) is not int or type(q) is not int or r <= 3 or not 1 <= q < r:
        raise ValueError('prime/window precondition')
    support = tuple(tuple(pair) for pair in support)
    if not support or any(len(pair) != 2 for pair in support):
        raise ValueError('support shape')
    if any(type(v) is not int or not 0 <= v < r for pair in support for v in pair):
        raise ValueError('noncanonical support')
    if len({x for x, w in support}) != len(support):
        raise ValueError('duplicate support')
    if sum(x*w for x, w in support) % r != 1:
        raise ValueError('normalization')
    checked = 0
    for size in range(4):
        for label in combinations(range(q), size):
            total = 0
            for x, w in support:
                value = w
                for d in label:
                    value = value * pow((x+d) % r, (r-1)//2, r) % r
                total = (total+value) % r
            if total:
                raise ValueError('nonzero feature moment at ' + repr(label))
            checked += 1
    return {'features_checked': checked, 'support_size': len(support),
            'normalized_pairing': 1, 'scope': 'all degree-at-most-three character features'}
