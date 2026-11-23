"""Validator EC arithmetic (short Weierstrass y^2 = x^3 + a x + b over F_p), written for this validation;
shares no code with examples/mt_fieldops.rs.  Jacobian coordinates, Fermat inversion only at the end."""
def is_prime(n):
    if n < 2: return False
    for q in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        if n % q == 0: return n == q
    d, s = n - 1, 0
    while d % 2 == 0: d //= 2; s += 1
    for a in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):   # deterministic for n < 3.3e24
        x = pow(a, d, n)
        if x in (1, n - 1): continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1: break
        else: return False
    return True
class Curve:
    def __init__(s, p, a, b): s.p, s.a, s.b = p, a % p, b % p
    def on(s, P):
        if P is None: return True
        x, y = P; return (y * y - (x * x * x + s.a * x + s.b)) % s.p == 0
    def _dbl(s, X, Y, Z):
        p = s.p
        if Y == 0 or Z == 0: return (1, 1, 0)
        YY = Y * Y % p; S = 4 * X * YY % p; ZZ = Z * Z % p
        M = (3 * X * X + s.a * ZZ * ZZ) % p
        X3 = (M * M - 2 * S) % p
        return (X3, (M * (S - X3) - 8 * YY * YY) % p, 2 * Y * Z % p)
    def _add(s, P, Q):
        p = s.p; X1, Y1, Z1 = P; X2, Y2, Z2 = Q
        if Z1 == 0: return Q
        if Z2 == 0: return P
        Z1Z1 = Z1 * Z1 % p; Z2Z2 = Z2 * Z2 % p
        U1 = X1 * Z2Z2 % p; U2 = X2 * Z1Z1 % p
        S1 = Y1 * Z2 * Z2Z2 % p; S2 = Y2 * Z1 * Z1Z1 % p
        if U1 == U2:
            return s._dbl(X1, Y1, Z1) if S1 == S2 else (1, 1, 0)
        H = (U2 - U1) % p; R = (S2 - S1) % p
        HH = H * H % p; HHH = H * HH % p; V = U1 * HH % p
        X3 = (R * R - HHH - 2 * V) % p
        return (X3, (R * (V - X3) - S1 * HHH) % p, Z1 * Z2 * H % p)
    def mul(s, k, P):
        if P is None or k == 0: return None
        R = (1, 1, 0); B = (P[0], P[1], 1)
        for bit in bin(k)[2:]:
            R = s._dbl(*R)
            if bit == "1": R = s._add(R, B)
        X, Y, Z = R
        if Z == 0: return None
        zi = pow(Z, s.p - 2, s.p); zi2 = zi * zi % s.p
        return (X * zi2 % s.p, Y * zi2 * zi % s.p)
    def neg(s, P): return None if P is None else (P[0], (-P[1]) % s.p)
    def add(s, P, Q):
        if P is None: return Q
        if Q is None: return P
        R = s._add((P[0], P[1], 1), (Q[0], Q[1], 1))
        if R[2] == 0: return None
        zi = pow(R[2], s.p - 2, s.p); zi2 = zi * zi % s.p
        return (R[0] * zi2 % s.p, R[1] * zi2 * zi % s.p)
def load(path):
    import json
    d = json.load(open(path))
    C = Curve(int(d["field"]["modulus"]), int(d["a"]), int(d["b"]))
    G = (int(d["generator"]["x"]), int(d["generator"]["y"]))
    return d, C, G, int(d["subgroup_order"])
