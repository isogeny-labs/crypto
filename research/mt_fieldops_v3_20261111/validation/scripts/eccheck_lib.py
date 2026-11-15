"""Validator EC helper (affine, own code): full double-and-add with explicit O handling; returns None for O."""
def aadd(P, Q, a, p):
    if P is None: return Q
    if Q is None: return P
    (x1, y1), (x2, y2) = P, Q
    if x1 == x2:
        if (y1 + y2) % p == 0: return None
        lam = (3 * x1 * x1 + a) * pow(2 * y1, -1, p) % p
    else:
        lam = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (lam * lam - x1 - x2) % p
    return (x3, (lam * (x1 - x3) - y1) % p)
def smul_full(k, G, a, p):
    R = None; B = G
    while k:
        if k & 1: R = aadd(R, B, a, p)
        B = aadd(B, B, a, p); k >>= 1
    return R
