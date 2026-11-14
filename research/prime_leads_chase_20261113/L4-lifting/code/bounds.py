"""Bounds for Q1/Q2: Cramer-form Model X bound, pigeonhole (closure-note style) bound, Model F bound.

All quantities in log2. q ~ 2^qbits; rho is target = 2^(qbits/2) successes-per-trial threshold.
"""
import json, math, sys

# Hermite constants: exact for s<=8; for s>=9 use the rigorous Minkowski-type bound gamma_s <= 1 + s/4
GAMMA = {1: 1.0, 2: (4 / 3) ** 0.5, 3: 2 ** (1 / 3), 4: 2 ** 0.5, 5: 8 ** (1 / 5), 6: (64 / 3) ** (1 / 6), 7: 64 ** (1 / 7), 8: 2.0}


def gamma(s, rigorous=True):
    if s in GAMMA:
        return GAMMA[s]
    return 1 + s / 4


def lg2_comb(n, k):
    return (math.lgamma(n + 1) - math.lgamma(k + 1) - math.lgamma(n - k + 1)) / math.log(2)


def log2_sum(xs):
    m = max(xs)
    return m + math.log2(sum(2 ** (x - m) for x in xs))


def modelX_cramer(k, rho):
    """log2 |M|/2 : number of candidate relation vectors (up to sign), k points, any rank s<k."""
    terms = []
    for s in range(0, k):
        C = math.floor((gamma(s) * rho) ** (s / 2)) if s > 0 else 1
        n = s + 1
        terms.append(lg2_comb(k, n) + n * math.log2(2 * C + 1) - 1)
    return log2_sum(terms)


def modelX_cramer_top(k, rho):
    s = k - 1
    C = math.floor((gamma(s) * rho) ** (s / 2))
    return k * math.log2(2 * C + 1) - 1


def modelX_pigeon(k, rho, T=16):
    """closure-note style: (C+1)^k > T (1 + 2 k C sqrt(rho))^(k-1); bound ((2C+1)^k-1)/2."""
    r = k - 1
    lo, hi = 1, 1
    f = lambda C: k * math.log2(C + 1) > math.log2(T) + r * math.log2(1 + 2 * k * C * math.sqrt(rho))
    while not f(hi):
        hi *= 2
    lo = hi // 2
    while hi - lo > 1:
        mid = (lo + hi) // 2
        if f(mid):
            hi = mid
        else:
            lo = mid
    return k * math.log2(2 * hi + 1) - 1


def modelF(r, rho, T=16, L=1):
    return math.log2(L * T) + r * math.log2(1 + 2 * math.sqrt(rho))


def rho_star_X(k, half):
    """largest rho at which Model X (Cramer, all s) bound still <= 2^half."""
    lo, hi = 1.0, 1.0
    if modelX_cramer(k, 1.0) > half:
        return None
    while modelX_cramer(k, hi) <= half:
        hi *= 2
    lo = hi / 2
    for _ in range(60):
        mid = (lo + hi) / 2
        if modelX_cramer(k, mid) <= half:
            lo = mid
        else:
            hi = mid
    return lo


def kstar_X(qbits, rho):
    for k in range(2, 400):
        if modelX_cramer(k, rho) >= qbits / 2:
            return k
    return None


def rstar_F(qbits, rho, T=16, L=1):
    for r in range(1, 100000):
        if modelF(r, rho, T, L) >= qbits / 2:
            return r
    return None


def petsche_rho(beta_bits):
    """Unconditional worst-case rho for lifts whose coefficients/points have beta bits (cited constants;
    H <= beta*ln2 + O(log|Delta|) taken as 2*beta*ln2, log|Delta| <= 13*beta*ln2, sigma <= log|Delta|/log 11)."""
    logD = 13 * beta_bits * math.log(2)
    sigma = max(1.0, logD / math.log(11))
    lam = logD / (1e15 * sigma ** 6 * math.log(104613 * sigma ** 2) ** 2)
    lam_floor = 1 / (1e15 * sigma ** 5 * math.log(11) ** -1 * math.log(104613 * sigma ** 2) ** 2)
    H = 2 * beta_bits * math.log(2)
    return H / lam


if __name__ == '__main__':
    out = {}
    print('## Model X margins at 256 bits (q ~ 2^256, need numerator < 2^128 to exclude beating rho)')
    print('k=s+1 points, s=rank | rho | log2 |M| (Cramer, all s) | margin=128-log2|M| | pigeonhole T=16 log2 | margin')
    for s in range(2, 10):
        k = s + 1
        for rho in (1.1, 1.3, 1.8, 2.7, 5, 10):
            c = modelX_cramer(k, rho)
            pg = modelX_pigeon(k, rho)
            print(f'{k} {s} {rho} {c:.1f} {128 - c:+.1f} {pg:.1f} {128 - pg:+.1f}')
    print('\n## rho*(s): largest height ratio with Model X bound <= 2^128 at 256 bits')
    for s in range(2, 13):
        print(s, rho_star_X(s + 1, 128))
    print('\n## growing rank thresholds')
    for qb in (64, 128, 256, 384, 512, 1024, 2048, 4096):
        row = [qb]
        for rho in (1.5, 10, 100, 1e4):
            row.append((kstar_X(qb, rho), rstar_F(qb, rho)))
        print(row)
    print('\n## Petsche worst-case rho for beta-bit lifts (cited constants, see PROOF)')
    for b in (256, 1024, 4096):
        print(b, '%.3e' % petsche_rho(b), 'kstar_X(256)=', kstar_X(256, petsche_rho(b)), 'rstar_F(256)=', rstar_F(256, petsche_rho(b)))
