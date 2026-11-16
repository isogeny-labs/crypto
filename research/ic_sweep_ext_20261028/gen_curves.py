"""Prime-order curves for the sizes ecbench's prime_search does not cover (34..44 bits).

For each size: a prime p of that many bits (deterministic from the seed), random (a, b), and the
group order from a baby-step giant-step search of the Hasse interval on a random point P.  A
candidate is kept only if the order n found is prime and nP = O: then ord(P) = n, and since
n > 4 sqrt(p) is the only multiple of ord(P) in the Hasse interval, #E = n and E has prime
order.  Writes curves.json (the `prime_explicit` curve specs).  Standard library only.
"""
import json, math, pathlib, random

HERE = pathlib.Path(__file__).parent
SIZES = [34, 36, 38, 40, 42, 44]


def is_prime(n):
    if n < 2:
        return False
    for q in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        if n % q == 0:
            return n == q
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2; s += 1
    for a in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        x = pow(a, d, n)
        if x in (1, n - 1):
            continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1:
                break
        else:
            return False
    return True


def add(P, Q, a, p):
    if P is None: return Q
    if Q is None: return P
    x1, y1 = P; x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        l = (3 * x1 * x1 + a) * pow(2 * y1, -1, p) % p
    else:
        l = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (l * l - x1 - x2) % p
    return (x3, (l * (x1 - x3) - y1) % p)


def mul(k, P, a, p):
    R = None
    while k:
        if k & 1: R = add(R, P, a, p)
        P = add(P, P, a, p); k >>= 1
    return R


def sqrt_mod(n, p):
    if pow(n, (p - 1) // 2, p) != 1:
        return None
    # Tonelli-Shanks
    q, s = p - 1, 0
    while q % 2 == 0:
        q //= 2; s += 1
    z = 2
    while pow(z, (p - 1) // 2, p) != p - 1:
        z += 1
    m, c, t, r = s, pow(z, q, p), pow(n, q, p), pow(n, (q + 1) // 2, p)
    while t != 1:
        i, t2 = 0, t
        while t2 != 1:
            t2 = t2 * t2 % p; i += 1
        b = pow(c, 1 << (m - i - 1), p)
        m, c, t, r = i, b * b % p, t * b * b % p, r * b % p
    return r


def order_in_interval(P, a, p):
    """Some n in [p+1-2sqrt(p), p+1+2sqrt(p)] with nP = O, or None."""
    lo = p + 1 - 2 * math.isqrt(p) - 2
    width = 4 * math.isqrt(p) + 6
    m = math.isqrt(width) + 1
    baby = {}
    R = None
    for j in range(m + 1):
        if R is not None:
            baby.setdefault(R[0], []).append((j, R[1]))
        R = add(R, P, a, p)
    giant = mul(m, P, a, p)
    cur = mul(lo, P, a, p)
    for i in range(m + 2):
        if cur is None:
            return lo + i * m
        for j, y in baby.get(cur[0], []):
            if cur[1] == y:
                return lo + i * m - j       # cur = (lo + i m) P = jP  =>  (lo + i m - j) P = O
            if (cur[1] + y) % p == 0:
                return lo + i * m + j
        cur = add(cur, giant, a, p)
    return None


def find(bits):
    rng = random.Random(0x1C0DE + bits)
    p = (1 << (bits - 1)) | (rng.getrandbits(bits - 2) << 1) | 1
    while not (is_prime(p) and p % 4 == 3):
        p += 2
    while True:
        a, b = rng.randrange(1, p), rng.randrange(1, p)
        if (4 * a ** 3 + 27 * b * b) % p == 0:
            continue
        while True:
            x = rng.randrange(p)
            y = sqrt_mod((x ** 3 + a * x + b) % p, p)
            if y is not None:
                P = (x, y); break
        n = order_in_interval(P, a, p)
        if n is None or not is_prime(n) or mul(n, P, a, p) is not None:
            continue
        # a second random point must agree: prime order n kills every point
        while True:
            x = rng.randrange(p)
            y = sqrt_mod((x ** 3 + a * x + b) % p, p)
            if y is not None and mul(n, (x, y), a, p) is None:
                break
        return {"kind": "prime_explicit", "p": p, "a": a, "b": b, "group_order": n, "r": n, "gx": P[0], "gy": P[1]}


if __name__ == "__main__":
    out = {}
    for bits in SIZES:
        c = find(bits)
        out[str(bits)] = c
        print(bits, c["p"].bit_length(), c["r"].bit_length(), flush=True)
    (HERE / "curves.json").write_text(json.dumps(out, indent=1) + "\n")
