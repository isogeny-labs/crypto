import json, sys
from math import gcd, isqrt

def phi(n): return sum(1 for k in range(1, n+1) if gcd(k, n) == 1)
def primes_of(n):
    s, d = set(), 2
    while d*d <= n:
        while n % d == 0: s.add(d); n //= d
        d += 1
    if n > 1: s.add(n)
    return s
def genus1(N):  # genus of X_1(N), N>=5
    num, den = N*N, 2
    for q in primes_of(N): num = num*(q*q-1); den = den*q*q
    mu = num//den
    cusps2 = sum(phi(d)*phi(N//d) for d in range(1, N+1) if N % d == 0)  # = 2*cusps
    return 1 + mu/12 - cusps2/4
def isprime(n): return n > 1 and all(n % d for d in range(2, isqrt(n)+1))

def ypoints(p, N):
    """#Y_1(N)(F_p) for prime N = #{(a,b): #E=N}*(N-1)/(p-1); also returns curve count."""
    sq = [0]*p
    for x in range(1, p): sq[x*x % p] = 1
    chi = [0]+[1 if sq[x] else -1 for x in range(1, p)]
    cnt = 0
    for a in range(p):
        for b in range(p):
            if (4*a**3+27*b*b) % p == 0: continue
            n = p+1+sum(chi[(x*x*x+a*x+b) % p] for x in range(p))
            if n == N: cnt += 1
    return cnt, cnt*(N-1)/(p-1)

def main():
    smoke = "--smoke" in sys.argv
    # positive control: known genera of X_1(N)
    known = {5: 0, 7: 0, 10: 0, 11: 1, 13: 2, 17: 5, 19: 7, 23: 12, 29: 22, 37: 40}
    pc = all(abs(genus1(N)-g) < 1e-9 for N, g in known.items())
    # negative control: Y_1(N) with N below Hasse range has no points
    ps = [31, 53] if smoke else [31, 53, 79, 101, 127]
    rows, ok = [], True
    for p in ps:
        lo, hi = p+1-2*isqrt(p), p+1+2*isqrt(p)
        for N in [n for n in range(max(lo, 5), hi+1) if isprime(n)]:
            c, y = ypoints(p, N); g = genus1(N)
            hw = p+1+2*g*p**0.5
            rows.append(dict(p=p, N=N, curves=c, Y1pts=y, genus=g, HW_bound=hw, ratio_slack=hw/max(y, 1e-9)))
            if N > p/2 and not (2*g*p**0.5 > p): ok = False
    asym = [genus1(N)/(N*N/24) for N in range(29, 48) if isprime(N)]
    asym_ok = all(0.45 < r < 1.0 for r in asym)
    nc = all(ypoints(p, N)[0] == 0 for p, N in [(31, 5), (31, 11)] if not (p+1-2*isqrt(p) <= N <= p+1+2*isqrt(p)))
    # A/A: repeat deterministic computation
    aa = (ypoints(ps[0], rows[0]["N"])[0] == rows[0]["curves"])
    if not (pc and nc and aa): verdict = "inconclusive"
    elif ok and asym_ok: verdict = "consistent_with_prediction"
    else: verdict = "falsified"
    print(json.dumps(dict(lead="LEAD-STR-level-structure-torsor", smoke=smoke, seeds="none (deterministic)",
        primes_p=ps, positive_control_genus_table=pc, negative_control_no_points_outside_hasse=nc, aa_repeat_equal=aa,
        genus_over_N2_24_range=[min(asym), max(asym)], hw_vacuous_for_N_gt_p_half=ok, rows=rows, verdict=verdict)))
main()
