"""Weighted-symmetry helpers; derived, no numerical validation.

Symmetry must come from the public compiler or a separately verified source.
Checking metric inverses alone does not certify D A is symmetric.
"""
def geometric_metric(n, gamma, meter):
    if type(n) is not int or n < 1 or type(gamma) is not int or not 0 < gamma < meter.r:
        raise ValueError('positive dimension and nonzero canonical gamma required')
    inverse = meter.inv(gamma)
    d, di = [1], [1]
    for _ in range(1, n):
        d.append(meter.mul(d[-1], gamma))
        di.append(meter.mul(di[-1], inverse))
    return tuple(d), tuple(di)


def validate_metric(d, di, meter):
    if not d or len(d) != len(di):
        raise ValueError('metric dimensions')
    for x, y in zip(d, di):
        if any(type(v) is not int or not 0 < v < meter.r for v in (x, y)):
            raise ValueError('canonical nonzero metric entries required')
        if meter.mul(x, y) != 1:
            raise ValueError('metric inverse mismatch')


def transpose_from_columns(columns, d, di, meter):
    """Return leading row vectors from leading columns using D A = A^T D.

    This reconstruction is conditional on weighted symmetry, not a check of it.
    Charges 2*n*k products. It allocates the reconstructed rows explicitly.
    """
    n, k = len(d), len(columns)
    validate_metric(d, di, meter)
    if not 0 < k <= n or any(len(c) != n for c in columns):
        raise ValueError('complete leading columns required')
    if any(type(v) is not int or not 0 <= v < meter.r for c in columns for v in c):
        raise ValueError('canonical columns required')
    return tuple(tuple(meter.mul(di[j], meter.mul(d[i], columns[j][i]))
                       for i in range(n)) for j in range(k))
