#!/usr/bin/env python3
"""Why ic5ks's retention helps only 0-2%: exact count, in logs mod a 32-bit prime n, of the target logs q'
that one retained (solved) target q_t can solve within K' paired trials, when its K retained trial classes
are x(Q_t +- W_k):  q' must lie in {e (q_t + s w_k) - s' w_k'} (e, s, s' = +-1, k < K, k' < K').
  AP (ic5ks): w_k = w_0 + k d          RW (validator ic5rw): w_(k+1) = w_k + d_(j(k)), 32 random d_j
A table entry, by contrast, covers 2 K' distinct q' (the paired trial classes of one target)."""
import random
n = 4294967291 * 0 + 4295042729  # the order of W32-0 (prime)
rnd = random.Random(20261112)
def cover(ws, K, Kp, qt):
    S = set()
    for e in (1, -1):
        for s in (1, -1):
            for sp in (1, -1):
                for k in range(K):
                    base = e * (qt + s * ws[k])
                    for kp in range(Kp):
                        S.add((base - sp * ws[kp]) % n)
    return len(S)
out = []
for K, Kp in [(200, 200), (500, 500), (1000, 1000)]:
    w0, d = rnd.randrange(n), rnd.randrange(n)
    ap = [(w0 + k * d) % n for k in range(max(K, Kp))]
    ds = [rnd.randrange(n) for _ in range(32)]; rw = [w0]
    while len(rw) < max(K, Kp):
        rw.append((rw[-1] + ds[((rw[-1] * 0x9E3779B97F4A7C15) % 2**64) >> 59]) % n)  # index from a mix of w (as x(W) in ic5rw)
    qt = rnd.randrange(n)
    a, r = cover(ap, K, Kp, qt), cover(rw, K, Kp, qt)
    out.append(dict(K=K, Kp=Kp, pairs=8 * K * Kp, distinct_AP=a, distinct_RW=r, ratio=r / a))
    print(out[-1])
