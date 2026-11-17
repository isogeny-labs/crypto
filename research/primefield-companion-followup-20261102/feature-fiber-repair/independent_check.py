"""Fixed small-certificate checker, with independent square-set characters."""
from math import isqrt


def verify(packet):
    r, q = packet['r'], packet['q']
    # This reference checker is explicitly bounded; it is not a large-field oracle.
    if type(r) is not int or not 3 < r <= 10000 or any(r % d == 0 for d in range(2, isqrt(r)+1)):
        raise ValueError('reference checker requires a prime in (3,10000]')
    if type(q) is not int or not 1 <= q < r:
        raise ValueError('window')
    pairs = tuple(tuple(pair) for pair in packet['pairs'])
    if len(pairs) != 2 or any(len(pair) != 2 for pair in pairs):
        raise ValueError('pair dimensions')
    pts = tuple(x for pair in pairs for x in pair)
    if len(set(pts)) != 4 or any(type(x) is not int or not 0 <= x < r for x in pts):
        raise ValueError('points')
    squares = {j*j % r for j in range(1, (r+1)//2)}
    def chi(x):
        z = x % r
        return 0 if z == 0 else 1 if z in squares else r-1
    rows = [[chi(x+j) for j in range(q)] for x in pts]
    if rows[0] != rows[1] or rows[2] != rows[3]:
        raise ValueError('old pair signatures differ')
    d, e = packet['distinguishing_offset'], packet['new_offset']
    if type(d) is not int or type(e) is not int or not 0 <= d < q <= e < r:
        raise ValueError('offsets')
    if rows[0][d] == rows[2][d]:
        raise ValueError('old offset does not distinguish fibers')
    columns = tuple(tuple(c) for c in packet['columns'])
    if columns != ((e,), (d, e)):
        raise ValueError('columns')
    U = [[chi(x+e), chi(x+d)*chi(x+e) % r] for x in pts]
    matrix = tuple(tuple((U[2*i][j]-U[2*i+1][j]) % r for j in range(2)) for i in range(2))
    if matrix != tuple(tuple(row) for row in packet['projection']):
        raise ValueError('projection')
    if (matrix[0][0]*matrix[1][1]-matrix[0][1]*matrix[1][0]) % r == 0:
        raise ValueError('singular quotient projection')
    target = tuple((x-y) % r for x, y in pairs)
    if target != tuple(packet['target_pairings']):
        raise ValueError('target pairing')
    c = packet['coefficients']
    if len(c) != 2 or any(type(v) is not int or not 0 <= v < r for v in c):
        raise ValueError('coefficients')
    if any(sum(a*b for a,b in zip(row,c)) % r != y for row,y in zip(matrix,target)):
        raise ValueError('nonzero residual pairing')
    return dict(prime_verified=True, old_pair_singletons_verified=2*q,
                independent_characters='square set', quotient_rank=2,
                residual_pairings=[0,0], global_old_span_rank='not rechecked')
