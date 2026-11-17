"""Independent finite-field identity/minor checker, without producer imports.

Does not certify primality, elliptic transport, or actual runtime meter calls.
Derived source; numerical and corruption-control validation is pending.
"""


def _require(condition, message):
    if not condition:
        raise ValueError(message)


def _matrix(value, rows, cols, prime):
    _require(type(value) is list and len(value) == rows, 'matrix rows')
    _require(all(type(row) is list and len(row) == cols for row in value), 'matrix columns')
    _require(all(type(x) is int and 0 <= x < prime for row in value for x in row),
             'canonical field entries')


def _indices(value, count, bound):
    _require(type(value) is list and len(value) == count, 'pivot count')
    _require(all(type(x) is int and 0 <= x < bound for x in value), 'pivot index')
    _require(len(set(value)) == count, 'duplicate pivot')


def _factor(matrix, width, record, prime):
    n = len(matrix)
    U, R = record['basis'], record['coordinates']
    k = len(R)
    _require(k <= min(n, width), 'factor rank range')
    _matrix(U, n, k, prime)
    _matrix(R, k, width, prime)
    rows, cols = record['pivot_rows'], record['pivot_columns']
    _indices(rows, k, n)
    _indices(cols, k, width)
    _require(cols == sorted(cols), 'pivot columns order')
    for i in range(k):
        _require(U[rows[i]][i] == 1 and R[i][cols[i]] != 0, 'minor diagonal')
        for j in range(i + 1, k):
            _require(U[rows[i]][j] == 0 and R[j][cols[i]] == 0, 'minor triangularity')
    for i in range(n):
        for j in range(width):
            value = sum(U[i][q] * R[q][j] for q in range(k)) % prime
            _require(value == matrix[i][j], 'rank factor identity')
    # Every earlier pivot contributes one elimination pair for each later column.
    expected_pairs = sum(width - 1 - j for j in cols)
    _require(type(record['elimination_pairs']) is int and
             record['elimination_pairs'] == expected_pairs, 'elimination pair count')
    return U, R, k, expected_pairs


def verify(G, K, result, prime):
    """Check preservation and exact minimal width using skinny products only."""
    _require(type(prime) is int and prime > 3, 'caller-certified prime > 3 required')
    c = result['certificate']
    _require(c['schema'] == 'companion-generator-compression/v1', 'schema')
    n, s, t = c['dimension'], c['input_width'], c['rank']
    _require(all(type(x) is int for x in (n, s, t)) and n > 0 and s >= 0 and t >= 0,
             'dimensions')
    _matrix(G, n, s, prime)
    _matrix(K, n, s, prime)
    U, R, g, q_left = _factor(G, s, c['left'], prime)
    H = c['intermediate']
    _matrix(H, n, g, prime)
    for i in range(n):
        for j in range(g):
            _require(H[i][j] == sum(K[i][q] * R[j][q] for q in range(s)) % prime,
                     'intermediate identity')
    V, S, actual_t, q_right = _factor(H, g, c['right'], prime)
    _require(t == actual_t, 'reported minimal rank')
    _matrix(result['G'], n, t, prime)
    _matrix(result['K'], n, t, prime)
    _require(result['K'] == V, 'right output')
    for i in range(n):
        for j in range(t):
            _require(result['G'][i][j] == sum(U[i][q] * S[j][q] for q in range(g)) % prime,
                     'left output')
    # Recomputed schedule totals are separate from validation of actual Meter use.
    expected = dict(field_products=n*(q_left+g+q_right+t+s*g+g*t),
                    field_additions=n*(q_left+q_right+s*g+g*t),
                    field_inversions=g+t)
    return dict(status='identities_and_minimal_rank_verified', rank=t,
                input_width=s, first_rank=g, expected_producer_arithmetic=expected,
                producer_runtime_counts_verified=False)
