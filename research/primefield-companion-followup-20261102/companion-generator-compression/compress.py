"""Exact generator compression over a caller-certified prime field.

Derived implementation, not numerically validated. Meter is the existing
kernel.Meter interface. No operator query or dense displacement is formed.
"""


def _shape(matrix, rows, cols, prime):
    if len(matrix) != rows or any(len(row) != cols for row in matrix):
        raise ValueError('matrix shape')
    if any(type(x) is not int or not 0 <= x < prime for row in matrix for x in row):
        raise ValueError('canonical field entries required')


def _factor(matrix, meter):
    """Column rank factorization with triangular minor certificates."""
    n, width = len(matrix), len(matrix[0])
    columns, coordinates, pivot_rows, pivot_columns = [], [], [], []
    elimination_pairs = 0
    for j in range(width):
        residual = [matrix[i][j] for i in range(n)]
        elimination_pairs += len(columns)
        for k, column in enumerate(columns):
            coefficient = residual[pivot_rows[k]]
            coordinates[k][j] = coefficient
            # Charge the full update, including zero coefficients.
            residual = [meter.sub(x, meter.mul(coefficient, y))
                        for x, y in zip(residual, column)]
        pivot = None
        for i, value in enumerate(residual):
            meter.add('compression_pivot_entry_tests')
            if value != 0:
                pivot = i
                break
        if pivot is None:
            continue
        value = residual[pivot]
        inverse = meter.inv(value)
        columns.append([meter.mul(x, inverse) for x in residual])
        row = [0] * width
        row[j] = value
        coordinates.append(row)
        pivot_rows.append(pivot)
        pivot_columns.append(j)
    basis = [[column[i] for column in columns] for i in range(n)]
    return dict(basis=basis, coordinates=coordinates,
                pivot_rows=pivot_rows, pivot_columns=pivot_columns,
                elimination_pairs=elimination_pairs)


def compress(G, K, meter):
    """Return minimal-width generators plus exact factorization certificates.

    Preconditions: meter.r is prime > 3, G and K have equal positive row
    counts and equal widths, all entries canonical. Width zero is supported.
    Primality is a caller precondition, not tested by this routine.
    """
    n = len(G)
    if n == 0 or type(meter.r) is not int or meter.r <= 3:
        raise ValueError('positive dimension and caller-certified prime > 3 required')
    s = len(G[0])
    _shape(G, n, s, meter.r)
    _shape(K, n, s, meter.r)
    before = dict(meter.counts)
    left = _factor(G, meter)
    U, R = left['basis'], left['coordinates']
    g = len(R)
    H = [[meter.dot(K[i], R[j]) for j in range(g)] for i in range(n)]
    right = _factor(H, meter)
    V, S = right['basis'], right['coordinates']
    t = len(S)
    reduced_G = [[meter.dot(U[i], S[j]) for j in range(t)] for i in range(n)]
    reduced_K = [row[:] for row in V]
    counts = {key: value - before.get(key, 0) for key, value in meter.counts.items()
              if value != before.get(key, 0)}
    return dict(G=reduced_G, K=reduced_K,
                certificate=dict(schema='companion-generator-compression/v1',
                                 dimension=n, input_width=s, rank=t,
                                 left=left, intermediate=H, right=right,
                                 producer_counts=counts))


def nonempty_generators(result):
    """Explicit width-one zero encoding for legacy positive-width kernels.

    Requires a verified result. The rank certificate still says zero; encoding
    width is one. The returned copies do not change the original certificate.
    """
    if result['certificate']['rank'] == 0:
        n = result['certificate']['dimension']
        return [[0] for _ in range(n)], [[0] for _ in range(n)]
    return [row[:] for row in result['G']], [row[:] for row in result['K']]
