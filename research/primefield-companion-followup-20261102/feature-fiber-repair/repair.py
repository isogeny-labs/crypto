"""Repair two supplied character-fiber obstructions with two new columns.

Does not find colliding pairs, solve the old span, or certify its rank.
"""
def character(x, r):
    return pow(x % r, (r-1)//2, r)


def construct(r, q, pairs, new_offset):
    if (type(r) is not int or type(q) is not int or r <= 3 or not 1 <= q < r or
            type(new_offset) is not int or not q <= new_offset < r):
        raise ValueError('caller-certified prime, window, or new offset')
    pairs = tuple(tuple(pair) for pair in pairs)
    if len(pairs) != 2 or any(len(pair) != 2 for pair in pairs):
        raise ValueError('two pairs required')
    points = tuple(x for pair in pairs for x in pair)
    if (any(type(x) is not int or not 0 <= x < r for x in points) or
            len(set(points)) != 4):
        raise ValueError('four distinct canonical points required')
    signatures = [tuple(character(x+d, r) for d in range(q)) for x in points]
    if signatures[0] != signatures[1] or signatures[2] != signatures[3]:
        raise ValueError('supplied pairs are not old signature fibers')
    d = next((j for j in range(q) if signatures[0][j] != signatures[2][j]), None)
    if d is None:
        raise ValueError('the two fibers have the same signature')
    values = [character(x+new_offset, r) for x in points]
    delta = tuple((values[2*i]-values[2*i+1]) % r for i in range(2))
    if not all(delta):
        raise ValueError('new character does not separate both supplied pairs')
    a, b = signatures[0][d], signatures[2][d]
    M = ((delta[0], delta[0]*a % r), (delta[1], delta[1]*b % r))
    target = tuple((x-y) % r for x, y in pairs)
    det = (M[0][0]*M[1][1]-M[0][1]*M[1][0]) % r
    inverse = pow(det, -1, r)
    c0 = (target[0]*M[1][1]-M[0][1]*target[1])*inverse % r
    c1 = (M[0][0]*target[1]-target[0]*M[1][0])*inverse % r
    return dict(schema='two-fiber-span-repair/v1', r=r, q=q, pairs=pairs,
                new_offset=new_offset, distinguishing_offset=d,
                columns=((new_offset,), (d, new_offset)),
                projection=M, target_pairings=target, coefficients=(c0, c1),
                status='two_obstructions_repaired',
                remaining='solve old-span residual; full rank premise not checked here')


def residual(packet, x):
    """Evaluate X minus the two-column correction; no field-sized residual array."""
    r = packet['r']
    if type(x) is not int or not 0 <= x < r:
        raise ValueError('canonical residual coordinate required')
    v = character(x+packet['new_offset'], r)
    z = character(x+packet['distinguishing_offset'], r)
    c0, c1 = packet['coefficients']
    return (x-c0*v-c1*v*z) % r
