"""Conditional recursive inverse generators; syntax checked, zero trials.

No supplied solve oracle, dense inverse, random pivoting, or source extension.
Caller supplies paths for companion-fused, companion-schur, companion-blocks,
structured-block-kernel and this directory. All arithmetic uses one Meter.
"""
from multiply import apply as fused_apply
from schur import Schur
from principal import principal


class PivotTreeObstruction(Exception):
    """The chosen tree failed; no root-matrix rank conclusion follows."""


class Operator:
    def __init__(self, G, K, a, b, meter, events, path='root', role='source'):
        assert G and K and len(G) == len(K) and G[0]
        assert a != b and 0 <= a < meter.r and 0 <= b < meter.r
        self.G = [list(row) for row in G]
        self.K = [list(row) for row in K]
        self.a, self.b, self.m = a, b, meter
        self.events, self.path, self.role = events, path, role

    def apply(self, x, transpose=False):
        self.m.add('recursive_kernel_calls')
        out = fused_apply(self.G, self.K, self.a, self.b, x, self.m, transpose)
        self.events.append(dict(path=self.path, role=self.role, transpose=transpose,
                                dimension=len(self.G), width=len(self.G[0])))
        return out


class SourceSchur(Schur):
    def __init__(self, source, cut, inverse):
        self.source = source
        super().__init__(source.G, source.K, source.a, source.b, cut,
                         inverse.apply, lambda x: inverse.apply(x, True), source.m)

    def _source(self, x, transpose=False):
        self.m.add('source_transpose_calls' if transpose else 'source_forward_calls')
        return self.source.apply(x, transpose)


def prepare(source):
    """Return inverse Operator on success, or a located pivot-tree obstruction.

    Only arithmetic inversion is a nonzero scalar field inversion at a leaf.
    The returned inverse stores width-s generators (width one at scalar leaves).
    """
    n = len(source.G)
    width = len(source.G[0])
    meter = source.m
    meter.add('recursive_prepare_nodes')
    if n == 1:
        value = source.apply([1])[0]
        if value == 0:
            raise PivotTreeObstruction(source.path + ': zero leaf; root rank undetermined')
        inverse = meter.inv(value)
        g = [[meter.mul(meter.sub(source.b, source.a), inverse)]]
        return Operator(g, [[1]], source.b, source.a, meter, source.events,
                        source.path, 'inverse')
    cut = n // 2
    # Fixed distinct wraps in the same prime field at every child.
    c, d = 2, 1
    g, k = principal(source, cut, c, d)
    leading = prepare(Operator(g, k, c, d, meter, source.events, source.path + '/E'))
    schur = SourceSchur(source, cut, leading)
    g, k = schur.generators(c, d)
    trailing = prepare(Operator(g, k, c, d, meter, source.events, source.path + '/S'))

    def solve(rhs, transpose=False):
        meter.add('recursive_block_solves')
        first = leading.apply(rhs[:cut], transpose)
        correction = source.apply(first + [0] * (n - cut), transpose)[cut:]
        reduced = [meter.sub(x, y) for x, y in zip(rhs[cut:], correction)]
        tail = trailing.apply(reduced, transpose)
        correction = source.apply([0] * cut + tail, transpose)[:cut]
        head_rhs = [meter.sub(x, y) for x, y in zip(rhs[:cut], correction)]
        return leading.apply(head_rhs, transpose) + tail

    inverse_g, inverse_k = [[] for _ in range(n)], [[] for _ in range(n)]
    for j in range(width):
        left = solve([row[j] for row in source.G])
        right = solve([row[j] for row in source.K], True)
        for i in range(n):
            inverse_g[i].append(meter.sub(0, left[i]))
            inverse_k[i].append(right[i])
    # Swapped wraps are essential: Z_b A^-1 - A^-1 Z_a.
    return Operator(inverse_g, inverse_k, source.b, source.a, meter,
                    source.events, source.path, 'inverse')
