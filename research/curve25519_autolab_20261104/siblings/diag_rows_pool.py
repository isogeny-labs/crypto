"""Raw collisions per distinct row (up to sign) for SP(m, M); r = 131203, C = 1."""
import random
r = 131203
def run(m, M, seed):
    rnd = random.Random(seed)
    d = rnd.randrange(r); s = rnd.randrange(r)
    vec = lambda: [rnd.randrange(r) for _ in range(3)]
    J = [vec() for _ in range(M)]; P = vec()
    sc = lambda v: (v[0] + v[1] * d + v[2] * s) % r
    seen = {}; rows = set(); coll = 0; n = 0
    def emit(v):
        nonlocal coll, n
        n += 1
        c = sc(v)
        if c == 0: return
        k = min(c, r - c)
        if k in seen:
            u = seen[k]
            if u != v:  # identical vectors are the same point, not a collision
                coll += 1
                same = sc(u) == c
                row = tuple((v[i] - u[i]) % r if same else (v[i] + u[i]) % r for i in range(3))
                rows.add(min(row, tuple((-x) % r for x in row)))
        else:
            seen[k] = v
    emit(P)
    while n < 12 * int(r ** .5):
        S = rnd.sample(range(M), m); h = rnd.choice(S)
        for a in S:
            for pl in (1, -1):
                emit([(P[i] + pl * J[a][i]) % r for i in range(3)])
        P = [(P[i] + J[h][i]) % r for i in range(3)]
    return coll, len(rows)
for m, M in ((4, 64), (8, 64), (8, 256), (16, 256)):
    tot = [0, 0]
    for sd in range(20):
        c, u = run(m, M, sd); tot[0] += c; tot[1] += u
    print(f"m={m} M={M} raw_collisions={tot[0]} distinct_rows={tot[1]} ratio={tot[0]/tot[1]:.3f} pred={1+(m/M)**2:.3f}")
