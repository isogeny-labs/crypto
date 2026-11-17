from metric_views import require_view

def transpose_from_columns(columns, d, di, meter, view):
    """Return leading row vectors from leading columns using D A = A^T D.

    This reconstruction is conditional on weighted symmetry, not a check of it.
    Charges 2*n*k products. It allocates the reconstructed rows explicitly.
    """
    n, k = len(d), len(columns)
    require_view(view, d, di, meter)
    if not 0 < k <= n or any(len(c) != n for c in columns):
        raise ValueError('complete leading columns required')
    if any(type(v) is not int or not 0 <= v < meter.r for c in columns for v in c):
        raise ValueError('canonical columns required')
    return tuple(tuple(meter.mul(di[j], meter.mul(d[i], columns[j][i]))
                       for i in range(n)) for j in range(k))
