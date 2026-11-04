"""Two-query leading generators. Derived source; not numerically validated."""


def principal(source, cut, c, d):
    n = len(source.G)
    m = source.m
    assert 0 < cut < n and 0 <= c < m.r and 0 <= d < m.r and c != d
    # A(e_cut-d e_0) and A^t(c e_(cut-1)-a e_(n-1)).
    column_input = [0] * n
    column_input[cut] = 1
    column_input[0] = m.sub(0, d)
    row_input = [0] * n
    row_input[cut - 1] = c
    row_input[n - 1] = m.sub(0, source.a)
    column = source.apply(column_input)[:cut]
    row = source.apply(row_input, True)[:cut]
    g = [list(source.G[i]) + [int(i == 0), column[i]] for i in range(cut)]
    k = [list(source.K[i]) + [row[i], int(i == cut - 1)] for i in range(cut)]
    return g, k
