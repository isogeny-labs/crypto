"""Boolean semi-regular degree (same series as src/cryptanalysis/ic_descent_degrees.rs
`semi_regular_degree`): first k >= 1 with a non-positive coefficient of
(1+t)^N / prod_i (1+t^{d_i})."""


def semi_regular_degree(n_vars, degrees):
    top = n_vars + 1
    num = [0] * (top + 1)
    num[0] = 1
    for _ in range(n_vars):
        for k in range(top, 0, -1):
            num[k] += num[k - 1]
    den = [0] * (top + 1)
    den[0] = 1
    for d in degrees:
        if d == 0 or d > top:
            continue
        for k in range(top, d - 1, -1):
            den[k] += den[k - d]
    quot = [0] * (top + 1)
    for k in range(top + 1):
        acc = num[k] - sum(den[j] * quot[k - j] for j in range(1, k + 1))
        quot[k] = acc
        if k >= 1 and acc <= 0:
            return k
    return None


if __name__ == "__main__":
    for N, e in [(12, 11), (12, 12), (15, 15), (18, 18), (21, 21), (24, 24), (27, 27), (33, 33)]:
        print(N, e, semi_regular_degree(N, [3] * e))
