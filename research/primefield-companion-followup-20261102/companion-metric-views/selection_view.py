"""One/two-coordinate pivot selection for a trusted weighted-symmetric source."""
from dataclasses import dataclass
from metric_views import require_view


@dataclass
class Selection:
    source: object
    signature: tuple
    metric: tuple
    inverse_metric: tuple
    indices: tuple
    columns: tuple
    source_queries: int
    consumed: bool = False


def select(source, d, di, view):
    n, m = len(source.G), source.m
    require_view(view, d, di, m)
    if n != len(d) or m.r <= 3:
        raise ValueError('matching metric and caller-certified prime > 3 required')
    queries = 0
    def query(j):
        nonlocal queries
        unit = [0]*n
        unit[j] = 1
        out = source.apply(unit)
        queries += 1
        m.add('metric_pivot_column_queries')
        if len(out) != n or any(type(v) is not int or not 0 <= v < m.r for v in out):
            raise ArithmeticError('malformed source column')
        return tuple(out)
    def result(indices, columns):
        return Selection(source, (n, source.a, source.b, m.r), tuple(d), tuple(di),
                         tuple(indices), tuple(columns), queries)
    for j in range(n):
        c = query(j)
        m.add('metric_pivot_diagonal_tests')
        if c[j] != 0:
            return result((j,), (c,))
        i = None
        for k, value in enumerate(c):
            m.add('metric_pivot_entry_tests')
            if value != 0:
                i = k
                break
        if i is None:
            continue
        other = query(i)
        # Local consistency guard, not a full source-symmetry certificate.
        if m.mul(d[i], c[i]) != m.mul(d[j], other[j]) or other[j] == 0:
            raise ArithmeticError('weighted reciprocal entry mismatch')
        m.add('metric_pivot_diagonal_tests')
        if other[i] != 0:
            return result((i,), (other,))
        return result((j, i), (c, other))
    return result((), ())
