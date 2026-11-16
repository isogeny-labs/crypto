"""Derived same-field row-pivot transport; syntax checked, no numerical trials.

Uses companion-recursive-inverse.Operator and its charged fused source calls.
This module supplies a pivot primitive, not a complete rank-profile algorithm.
"""
from recursive import Operator


def permute(vector, i, j):
    out = list(vector)
    out[i], out[j] = out[j], out[i]
    return out


def swap_rows(source, i, j):
    n = len(source.G)
    assert 0 <= i < n and 0 <= j < n
    if i == j:
        return source, dict(kind='identity', rows=[i, j], source_queries=0,
                            input_width=len(source.G[0]), output_width=len(source.G[0]))
    meter = source.m
    u = [0] * n
    u[i], u[j] = 1, meter.sub(0, 1)
    zu = [meter.mul(source.a, u[-1])] + u[:-1]
    ztu = u[1:] + [meter.mul(source.a, u[0])]
    v1 = source.apply(ztu, True)
    v2 = source.apply(u, True)
    pg = permute(source.G, i, j)
    g = [list(pg[k]) + [u[k], meter.sub(0, zu[k])] for k in range(n)]
    h = [list(source.K[k]) + [v1[k], v2[k]] for k in range(n)]
    target = Operator(g, h, source.a, source.b, meter, source.events,
                      source.path + '/row-swap-' + str(i) + '-' + str(j))
    receipt = dict(kind='row_swap', rows=[i, j], source_queries=2,
                   query_directions=['transpose', 'transpose'],
                   input_width=len(source.G[0]), output_width=len(g[0]),
                   field_products_outside_queries=2,
                   field_additions_outside_queries=n + 1)
    return target, receipt


def first_column_pivot(source):
    """One charged column query, then swap or exact right-null certificate.

    Any nonsingular source has a nonzero first column, so the pivot branch
    covers all nonsingular inputs at this one elimination position.
    """
    n = len(source.G)
    unit = [1] + [0] * (n - 1)
    column = source.apply(unit)
    row = None
    inspected = 0
    for i, value in enumerate(column):
        inspected += 1
        if value:
            row = i
            break
    source.m.add('pivot_entries_inspected', inspected)
    if row is None:
        return None, dict(kind='right_null', witness=unit, source_product=column,
                          source_queries=1, inspected_entries=inspected,
                          classification='singular source; no target RHS conclusion')
    target, transport = swap_rows(source, 0, row)
    return target, dict(kind='nonzero_first_pivot', row=row, pivot=column[row],
                        inspected_entries=inspected, column=column,
                        source_queries=1 + transport['source_queries'],
                        transport=transport)


def transport_rhs(rhs, receipt):
    """For B=PA, solve Bx=P rhs. Permutations use no field arithmetic."""
    i, j = receipt['rows']
    return permute(rhs, i, j)


def transport_left_witness(w, receipt):
    """w^t(PA)=0 transports to (P^t w)^t A=0; P^t=P."""
    i, j = receipt['rows']
    return permute(w, i, j)
