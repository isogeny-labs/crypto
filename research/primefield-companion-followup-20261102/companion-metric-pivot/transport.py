"""Simultaneous generator permutations and explicit selected-block inverses.

Caller provides sibling module paths: solve is companion-rank-solver/solve.py;
pivot is companion-pivot-transport/pivot.py. No cache from an arbitrary row-only
pivot is accepted. The selector's weighted-symmetry contract remains required.
"""
from pivot import swap_rows
from solve import swap_columns


class SmallInverse:
    def __init__(self, block, meter):
        self.m = meter
        if any(type(x) is not int or not 0 <= x < meter.r for row in block for x in row):
            raise ValueError('canonical small block required')
        if len(block) == 1 and len(block[0]) == 1 and block[0][0] != 0:
            self.size = 1
            self.inverse = (meter.inv(block[0][0]),)
        elif (len(block) == 2 and all(len(row) == 2 for row in block)
              and block[0][0] == block[1][1] == 0
              and block[0][1] != 0 and block[1][0] != 0):
            self.size = 2
            self.inverse = (meter.inv(block[0][1]), meter.inv(block[1][0]))
        else:
            raise ValueError('nonzero scalar or invertible antidiagonal block required')

    def apply(self, rhs, transpose=False):
        if type(transpose) is not bool or len(rhs) != self.size or any(
                type(x) is not int or not 0 <= x < self.m.r for x in rhs):
            raise ValueError('canonical small-block RHS required')
        self.m.add('metric_small_inverse_calls')
        if self.size == 1:
            return [self.m.mul(self.inverse[0], rhs[0])]
        iu, iv = self.inverse
        return ([self.m.mul(iu, rhs[1]), self.m.mul(iv, rhs[0])] if transpose else
                [self.m.mul(iv, rhs[1]), self.m.mul(iu, rhs[0])])


def move(source, selection):
    n, m = len(source.G), source.m
    if (selection.source is not source or selection.consumed or
            selection.signature != (n, source.a, source.b, m.r)):
        raise ValueError('wrong, changed or consumed selection')
    if not selection.indices:
        raise ValueError('zero-source selection has no pivot to move')
    selection.consumed = True
    order = list(range(n))
    moved = source
    receipts = []
    for front, original_index in enumerate(selection.indices):
        position = order.index(original_index)
        if position == front:
            continue
        moved, rows = swap_rows(moved, front, position)
        moved, cols = swap_columns(moved, front, position)
        order[front], order[position] = order[position], order[front]
        receipts.append(dict(front=front, position=position, rows=rows, columns=cols))
    columns = tuple(tuple(c[i] for i in order) for c in selection.columns)
    k = len(columns)
    block = tuple(tuple(columns[j][i] for j in range(k)) for i in range(k))
    inverse = SmallInverse(block, m)
    return dict(source=moved, metric=tuple(selection.metric[i] for i in order),
                inverse_metric=tuple(selection.inverse_metric[i] for i in order),
                order=tuple(order), cut=k, columns=columns, inverse=inverse,
                trace=receipts, transport_source_queries=4*len(receipts),
                output_width=len(moved.G[0]))


def permute_rhs(rhs, order):
    return [rhs[i] for i in order]


def undo_vector(vector, order):
    if len(vector) != len(order) or sorted(order) != list(range(len(order))):
        raise ValueError('complete permutation required')
    result = [0]*len(order)
    for i, original_index in enumerate(order):
        result[original_index] = vector[i]
    return result
