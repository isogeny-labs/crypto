"""Exact reusable row elimination; dense reference backend, not a cost claim."""
from dataclasses import dataclass


@dataclass(frozen=True)
class Factor:
    prime: int
    rows: int
    columns: int
    pivots: tuple
    upper: tuple
    steps: tuple

    def solve(self, rhs):
        r = self.prime
        v = list(rhs)
        if len(v) != self.rows or any(type(x) is not int or not 0 <= x < r for x in v):
            raise ValueError('canonical RHS dimensions')
        for i, (swap, inverse, factors) in enumerate(self.steps):
            v[i], v[swap] = v[swap], v[i]
            v[i] = v[i]*inverse % r
            for k, f in enumerate(factors, i+1):
                v[k] = (v[k]-f*v[i]) % r
        rank = len(self.pivots)
        if any(v[rank:]):
            raise ValueError('inconsistent rectangular system')
        solution = [0]*self.columns
        for i in range(rank-1, -1, -1):
            j = self.pivots[i]
            solution[j] = (v[i]-sum(self.upper[i][k]*solution[k]
                                    for k in range(j+1, self.columns))) % r
        return tuple(solution)


def factor(matrix, r):
    """Caller certifies prime r; retain transformations for subsequent RHS solves."""
    if type(r) is not int or r <= 2:
        raise ValueError('prime precondition')
    A = [list(row) for row in matrix]
    if not A or not A[0]:
        raise ValueError('nonempty matrix required')
    n, m = len(A), len(A[0])
    if any(len(row) != m or any(type(x) is not int or not 0 <= x < r for x in row) for row in A):
        raise ValueError('canonical rectangular matrix required')
    pivots, steps = [], []
    i = 0
    for j in range(m):
        pivot = next((k for k in range(i, n) if A[k][j]), None)
        if pivot is None:
            continue
        A[i], A[pivot] = A[pivot], A[i]
        inverse = pow(A[i][j], -1, r)
        tail = [x*inverse % r for x in A[i][j:]]
        A[i][j:] = tail
        factors = []
        for k in range(i+1, n):
            f = A[k][j]
            factors.append(f)
            if f:
                A[k][j:] = [(x-f*y) % r for x, y in zip(A[k][j:], tail)]
        pivots.append(j)
        steps.append((pivot, inverse, tuple(factors)))
        i += 1
        if i == n:
            break
    return Factor(r, n, m, tuple(pivots), tuple(tuple(row) for row in A[:i]), tuple(steps))
