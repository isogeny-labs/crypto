"""Derived structured solve-or-witness path; zero numerical trials.

Caller provides sibling module paths for recursive, pivot, lift and compress.
All arithmetic is in source.m.r, a caller-certified prime > 3. Existing
frozen modules are reused without modification. No matrix solve oracle.
"""
from recursive import Operator, SourceSchur
from pivot import swap_rows, permute
from lift import reduced_rhs, lift_primal, lift_dual
from compress import compress, nonempty_generators


class ScalarInverse:
    def __init__(self, inverse, meter):
        self.inverse, self.meter = inverse, meter

    def apply(self, rhs, transpose=False):
        if len(rhs) != 1:
            raise ValueError('scalar solve dimension')
        self.meter.add('rank_solver_scalar_solve_calls')
        return [self.meter.mul(self.inverse, rhs[0])]


def swap_columns(source, i, j):
    """Represent A Q using two forward queries and width s+2."""
    n, meter = len(source.G), source.m
    if not (0 <= i < n and 0 <= j < n):
        raise ValueError('column index')
    if i == j:
        return source, dict(kind='identity', source_queries=0)
    u = [0] * n
    u[i], u[j] = 1, meter.sub(0, 1)
    zu = [meter.mul(source.b, u[-1])] + u[:-1]
    ztu = u[1:] + [meter.mul(source.b, u[0])]
    au, azu = source.apply(u), source.apply(zu)
    qk = permute(source.K, i, j)
    G = [list(source.G[k]) + [au[k], meter.sub(0, azu[k])] for k in range(n)]
    K = [list(qk[k]) + [ztu[k], u[k]] for k in range(n)]
    result = Operator(G, K, source.a, source.b, meter, source.events,
                      source.path + '/column-swap')
    return result, dict(kind='column_swap', columns=[i, j], source_queries=2,
                        field_products_outside_queries=2,
                        field_additions_outside_queries=n+1)


def _find_pivot(source):
    """Search columns in order, retaining one queried column at a time."""
    n = len(source.G)
    for j in range(n):
        unit = [0] * n
        unit[j] = 1
        column = source.apply(unit)
        source.m.add('rank_solver_pivot_column_queries')
        for i, value in enumerate(column):
            source.m.add('rank_solver_pivot_entry_tests')
            if value != 0:
                return i, j, value, j+1
    raise ArithmeticError('nonzero displacement but all queried columns are zero')


def solve_or_witness(source, rhs):
    """Return checked solution or normalized exclusion witness.

    Nonrecursive control flow avoids a Python recursion-depth assumption.
    The trace retains compression certificates and frames retain source objects;
    total memory is not bounded by the final generator width alone.
    """
    if not __debug__:
        raise ValueError('inherited numerical assertions must remain enabled')
    n, meter = len(source.G), source.m
    if (n == 0 or meter.r <= 3 or source.a == source.b or
            not (0 <= source.a < meter.r and 0 <= source.b < meter.r)):
        raise ValueError('square distinct-wrap source over caller-certified prime > 3 required')
    if len(rhs) != n or any(type(x) is not int or not 0 <= x < meter.r for x in rhs):
        raise ValueError('canonical RHS required')
    original, original_rhs = source, list(rhs)
    b = list(rhs)
    frames, trace = [], []
    pivots = 0
    while True:
        size = len(source.G)
        reduced = compress(source.G, source.K, meter)
        trace.append(dict(kind='compression', dimension=size,
                          certificate=reduced['certificate']))
        rank = reduced['certificate']['rank']
        if rank == 0:
            # Distinct wraps make the square Sylvester operator invertible.
            # Therefore zero displacement implies the actual source is zero.
            index = None
            for i, value in enumerate(b):
                meter.add('rank_solver_rhs_entry_tests')
                if value != 0:
                    index = i
                    break
            vector = [0] * size
            if index is None:
                kind = 'solution'
            else:
                kind = 'exclusion_witness'
                vector[index] = meter.inv(b[index])
            trace.append(dict(kind='zero_source_terminal', dimension=size,
                              rhs_nonzero_index=index))
            break
        G, K = nonempty_generators(reduced)
        source = Operator(G, K, source.a, source.b, meter, source.events,
                          source.path + '/compressed')
        row, column, value, queried = _find_pivot(source)
        moved, column_receipt = swap_columns(source, 0, column)
        moved, row_receipt = swap_rows(moved, 0, row)
        b = permute(b, 0, row)
        inverse = ScalarInverse(meter.inv(value), meter)
        pivots += 1
        trace.append(dict(kind='pivot', dimension=size, row=row, column=column,
                          value=value, search_queries=queried,
                          column_transport=column_receipt, row_transport=row_receipt,
                          transported_width=len(moved.G[0])))
        if size == 1:
            vector = inverse.apply(b)
            kind = 'solution'
            break
        schur = SourceSchur(moved, 1, inverse)
        G, K = schur.generators(2, 1)
        h = reduced_rhs(schur, b)
        frames.append((schur, b, row, column))
        source = Operator(G, K, 2, 1, meter, moved.events,
                          moved.path + '/scalar-schur')
        b = h
    for schur, parent_rhs, row, column in reversed(frames):
        if kind == 'solution':
            vector = permute(lift_primal(schur, parent_rhs, vector), 0, column)
        else:
            vector = permute(lift_dual(schur, vector), 0, row)
    if kind == 'solution':
        product = original.apply(vector)
        meter.add('rank_solver_output_entry_tests', n)
        if product != original_rhs:
            raise ArithmeticError('root solution check failed')
    else:
        product = original.apply(vector, True)
        pairing = meter.dot(vector, original_rhs)
        meter.add('rank_solver_output_entry_tests', n+1)
        if any(product) or pairing != 1:
            raise ArithmeticError('root normalized witness check failed')
    return dict(kind=kind, vector=vector, root_equation_checked=True,
                derived_rank=pivots, rank_independently_checked=False,
                trace=trace, feature_annihilation_checked=False)
