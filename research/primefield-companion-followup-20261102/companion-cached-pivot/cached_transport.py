"""Reuse the first-nonzero-column search in same-field column transport.

Derived source; no numerical validation. SearchCache is an internal one-use
record, not an independently verified certificate. The source must not be
mutated between search and immediate transport.
"""
from dataclasses import dataclass
from recursive import Operator
from pivot import permute


@dataclass
class SearchCache:
    source: object
    signature: tuple
    column_index: int
    pivot_row: int
    column: tuple
    consumed: bool = False


def find_pivot(source):
    n = len(source.G)
    for j in range(n):
        unit = [0] * n
        unit[j] = 1
        column = source.apply(unit)
        source.m.add('rank_solver_pivot_column_queries')
        for i, value in enumerate(column):
            source.m.add('rank_solver_pivot_entry_tests')
            if value != 0:
                cache = SearchCache(source, (n, source.a, source.b, source.m.r),
                                    j, i, tuple(column))
                return i, j, value, j+1, cache
    raise ArithmeticError('nonzero displacement but all queried columns are zero')


def swap_from_search(source, cache):
    """Swap columns 0,j, relying on this search's zero prefix and pivot column."""
    n, meter = len(source.G), source.m
    if (not isinstance(cache, SearchCache) or cache.source is not source or
            cache.signature != (n, source.a, source.b, meter.r) or cache.consumed):
        raise ValueError('wrong, changed-signature or consumed search cache')
    j, row, column = cache.column_index, cache.pivot_row, cache.column
    if (type(j) is not int or not 0 <= j < n or type(row) is not int or
            not 0 <= row < n or len(column) != n or
            any(type(x) is not int or not 0 <= x < meter.r for x in column) or
            column[row] == 0 or any(column[:row])):
        raise ValueError('malformed search cache')
    cache.consumed = True
    if j == 0:
        return source, dict(kind='identity', source_queries=0,
                            cached_column_index=0, zero_prefix_columns=0)
    minus_one = meter.sub(0, 1)
    u = [0] * n
    u[0], u[j] = 1, minus_one
    # Z_b^T(e0-ej): its only nonzero entries are -1 at j-1 and b at n-1.
    # Assignment of the existing canonical residue b needs no field product.
    ztu = [0] * n
    ztu[j-1], ztu[-1] = minus_one, source.b
    au = [meter.sub(0, x) for x in column]
    extra_additions = 0
    if j < n-1:
        unit = [0] * n
        unit[j+1] = 1
        following = source.apply(unit)
        source_queries = 1
        if j == 1:
            correction = [meter.sub(x, y) for x, y in zip(following, column)]
            extra_additions = n
        else:
            correction = list(following)
    else:
        source_queries = 0
        correction = list(au) if j == 1 else [0] * n
    qk = permute(source.K, 0, j)
    G = [list(source.G[k]) + [au[k], correction[k]] for k in range(n)]
    K = [list(qk[k]) + [ztu[k], u[k]] for k in range(n)]
    moved = Operator(G, K, source.a, source.b, meter, source.events,
                     source.path + '/cached-column-swap')
    return moved, dict(kind='cached_column_swap', columns=[0, j],
                       source_queries=source_queries,
                       reference_source_queries=2,
                       field_products_outside_queries=0,
                       field_additions_outside_queries=n+1+extra_additions,
                       zero_prefix_columns=j, cached_column=list(column),
                       output_width=len(G[0]),
                       cache_independently_verified=False)
