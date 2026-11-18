#!/usr/bin/env python3
"""Assessor's independent checks for L06 (stdlib only, written from scratch,
shares no code with l06/ or analyze.py). [pcc:unverified]

Usage: python3 -I assessor_check.py <lead_dir>

1. Recompute from raw census JSONL (only f, p, i, nJ, s1..s3 are trusted inputs):
   prime test of nJ, PRIME/ALL counts, and fire_l by an independent criterion
   (brute force over monic cubics h, product h*h^* == chi mod l, gcd == 1).
2. Independent chi for a seeded sample of PRIME curves: s1 from N1 and s2 from N2
   by enumerating F_p and F_{p^2}; s3 then follows from nJ; nJ itself checked by a
   from-scratch Cantor implementation (nJ * D == 0 for random D != 0, and
   (nJ/r) * D != 0 is moot since nJ is prime) and twist order nJtw likewise.
3. Re-derive the coefficients of the census curves from the SplitMix64 seed rule.
"""
import json, os, random, sys

M64 = (1 << 64) - 1

def splitmix(seed):
    s = seed & M64
    while True:
        s = (s + 0x9E3779B97F4A7C15) & M64
        z = s
        z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & M64
        z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & M64
        yield z ^ (z >> 31)

def uniform(g, n):
    zone = M64 - (M64 % n)
    while True:
        x = next(g)
        if x < zone:
            return x % n

def coeffs(p, i):
    g = splitmix(0x4C06000000000000 ^ (p << 24) ^ i)
    return [uniform(g, p) for _ in range(7)]

def is_prime(n):
    if n < 2:
        return False
    for q in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        if n % q == 0:
            return n == q
    d, r = n - 1, 0
    while d % 2 == 0:
        d //= 2; r += 1
    for a in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        x = pow(a, d, n)
        if x in (1, n - 1):
            continue
        for _ in range(r - 1):
            x = x * x % n
            if x == n - 1:
                break
        else:
            return False
    return True

# ---- polynomials over F_q, lists low->high, trimmed ----
def trim(a):
    while a and a[-1] == 0:
        a.pop()
    return a

def padd(a, b, q):
    n = max(len(a), len(b)); r = [0] * n
    for i, x in enumerate(a): r[i] = x
    for i, x in enumerate(b): r[i] = (r[i] + x) % q
    return trim(r)

def pneg(a, q):
    return [(-x) % q for x in a]

def psub(a, b, q):
    return padd(a, pneg(b, q), q)

def pmul(a, b, q):
    if not a or not b:
        return []
    r = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        if x:
            for j, y in enumerate(b):
                r[i + j] = (r[i + j] + x * y) % q
    return trim(r)

def pdivmod(a, b, q):
    a = list(a); inv = pow(b[-1], q - 2, q); out = [0] * max(0, len(a) - len(b) + 1)
    while len(a) >= len(b) and a:
        c = a[-1] * inv % q; s = len(a) - len(b); out[s] = c
        for j, y in enumerate(b):
            a[s + j] = (a[s + j] - c * y) % q
        trim(a)
    return trim(out), a

def monic(a, q):
    if not a:
        return a
    inv = pow(a[-1], q - 2, q)
    return [x * inv % q for x in a]

def pgcd(a, b, q):
    while b:
        a, b = b, pdivmod(a, b, q)[1]
    return monic(a, q)

def xgcd(a, b, q):
    # returns d, s, t with d = s a + t b, d monic
    r0, r1, s0, s1, t0, t1 = a, b, [1], [], [], [1]
    while r1:
        qq, r = pdivmod(r0, r1, q)
        r0, r1 = r1, r
        s0, s1 = s1, psub(s0, pmul(qq, s1, q), q)
        t0, t1 = t1, psub(t0, pmul(qq, t1, q), q)
    inv = pow(r0[-1], q - 2, q)
    return ([x * inv % q for x in r0], [x * inv % q for x in s0], [x * inv % q for x in t0])

# ---- Cantor, genus 3, y^2 = F ----
def cantor_add(D1, D2, F, p):
    u1, v1 = D1; u2, v2 = D2
    d0, e1, e2 = xgcd(u1, u2, p)
    d, c1, c2 = xgcd(d0, padd(v1, v2, p), p)
    s1, s2, s3 = pmul(c1, e1, p), pmul(c1, e2, p), c2
    dd = pmul(d, d, p)
    u, r = pdivmod(pmul(u1, u2, p), dd, p); assert not r
    num = padd(padd(pmul(pmul(s1, u1, p), v2, p), pmul(pmul(s2, u2, p), v1, p), p),
               pmul(s3, padd(pmul(v1, v2, p), F, p), p), p)
    v, r = pdivmod(num, d, p); assert not r
    v = pdivmod(v, u, p)[1]
    while len(u) - 1 > 3:
        un, r = pdivmod(psub(F, pmul(v, v, p), p), u, p); assert not r
        u = monic(un, p)
        v = pdivmod(pneg(v, p), u, p)[1]
    u = monic(u, p); v = pdivmod(v, u, p)[1]
    return (u, v)

def cantor_mul(n, D, F, p):
    R = ([1], [])
    for bit in bin(n)[2:]:
        R = cantor_add(R, R, F, p)
        if bit == '1':
            R = cantor_add(R, D, F, p)
    return R

def peval(a, x, p):
    r = 0
    for c in reversed(a):
        r = (r * x + c) % p
    return r

def sqrt_mod(a, p):
    for y in range(p):
        if y * y % p == a:
            return y

def rand_class(F, p, rnd):
    while True:
        D = ([1], [])
        for _ in range(3):
            while True:
                x = rnd.randrange(p); w = peval(F, x, p)
                if w == 0 or pow(w, (p - 1) // 2, p) == 1:
                    y = sqrt_mod(w, p)
                    if rnd.random() < 0.5: y = (-y) % p
                    break
            D = cantor_add(D, ([(-x) % p, 1], [y] if y else []), F, p)
        if D != ([1], []):
            return D

# ---- point counts over F_p and F_{p^2} ----
def counts(f, p):
    nu = next(z for z in range(2, p) if pow(z, (p - 1) // 2, p) == p - 1)
    N1 = 1
    for x in range(p):
        w = peval(f, x, p)
        N1 += 1 if w == 0 else (2 if pow(w, (p - 1) // 2, p) == 1 else 0)
    # F_{p^2} = F_p[t]/(t^2 - nu); element (a, b) = a + b t; square test via norm
    N2 = 1
    for a in range(p):
        for b in range(p):
            ra, rb = 0, 0
            for c in reversed(f):  # Horner
                ra, rb = (ra * a + rb * b * nu + c) % p, (ra * b + rb * a) % p
            if ra == 0 and rb == 0:
                N2 += 1
            else:
                nm = (ra * ra - nu * rb * rb) % p
                N2 += 2 if pow(nm, (p - 1) // 2, p) == 1 else 0
    return N1, N2, nu

# ---- criterion, independent ----
def fire(chi_int, p, l):
    c = [x % l for x in chi_int]
    q = p % l
    n_h = 0
    for a0 in range(1, l):
        for a1 in range(l):
            for a2 in range(l):
                h = [a0, a1, a2, 1]
                inv0 = pow(a0, l - 2, l)
                hs = [h[3 - i] * pow(q, 3 - i, l) * inv0 % l for i in range(4)]
                if trim(pmul(h, hs, l)) == trim(list(c)) and len(pgcd(h, hs, l)) == 1:
                    n_h += 1
    return n_h

def chi_of(p, s1, s2, s3):
    return [p ** 3, -p * p * s1, p * s2, -s3, s2, -s1, 1]

def main(d):
    rows = []
    for p in (257, 521, 1031, 2053, 4099):
        for line in open(os.path.join(d, 'raw', f'census_p{p}.jsonl')):
            r = json.loads(line)
            if r.get('status') == 'ok':
                rows.append(r)
    seedbad = sum(1 for r in rows if coeffs(r['p'], r['i']) != r['f'])
    prime = [r for r in rows if is_prime(r['nJ'])]
    flagmis = sum(1 for r in rows if is_prime(r['nJ']) != r['prime'])
    out = {'ok_rows': len(rows), 'coeff_seed_mismatch': seedbad,
           'prime_flag_mismatch': flagmis, 'PRIME': len(prime)}
    for l in (3, 5, 7):
        k = 0; mis = 0
        for r in prime:
            nh = fire(chi_of(r['p'], r['s1'], r['s2'], r['s3']), r['p'], l)
            k += nh > 0
            mis += (nh > 0) != r['crit'][str(l)]['fire'] or nh != r['crit'][str(l)]['n_h']
        out[f'F_{l}'] = f'{k}/{len(prime)}'; out[f'crit_mismatch_{l}'] = mis
    allu = [r for r in rows if r['block'] == 'U']
    for l in (3, 5):
        k = sum(fire(chi_of(r['p'], r['s1'], r['s2'], r['s3']), r['p'], l) > 0 for r in allu)
        out[f'ALL_F_{l}'] = f'{k}/{len(allu)}'
    # independent chi on a seeded sample of PRIME curves at the three smaller p
    rnd = random.Random(20261116)
    checks = []
    for p in (257, 521, 1031):
        pr = [r for r in prime if r['p'] == p]
        for r in rnd.sample(pr, 2):
            f = list(r['f']) + [1]
            N1, N2, nu = counts(f, p)
            S1 = p + 1 - N1; S2 = p * p + 1 - N2
            s1 = S1; s2 = (s1 * S1 - S2) // 2
            ok_s = (s1 == r['s1'] and s2 == r['s2'])
            chi = chi_of(p, r['s1'], r['s2'], r['s3'])
            nJ = sum(chi); nJtw = sum(c * (-1) ** i for i, c in enumerate(chi))
            Ft = [c * nu % p for c in f]
            ann = all(cantor_mul(nJ, rand_class(f, p, rnd), f, p) == ([1], []) for _ in range(3))
            annt = all(cantor_mul(nJtw, rand_class(Ft, p, rnd), Ft, p) == ([1], []) for _ in range(3))
            checks.append({'p': p, 'i': r['i'], 's1s2_match': ok_s, 'nJ_match': nJ == r['nJ'],
                           'nJtw_match': nJtw == r['nJtw'], 'ann_J': ann, 'ann_twist': annt})
    out['independent_chi'] = checks
    print(json.dumps(out, indent=1))

if __name__ == '__main__':
    main(sys.argv[1])
