"""Pick a prime-order curve y^2=x^3+ax+b over F_p for each small prime p."""
from prime_field import Fp, CurveP

def isprime(n):
    return n > 1 and all(n % q for q in range(2, int(n ** 0.5) + 1))

def prime_order_curve(p):
    F = Fp(p)
    best = None
    for a in range(p):
        for b in range(1, p):
            if (4 * a ** 3 + 27 * b * b) % p == 0: continue
            C = CurveP(F, a, b)
            n = len(C.points()) + 1
            if isprime(n) and n > 3:
                return C, n
    return None, None

if __name__ == "__main__":
    for p in [5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127]:
        C, n = prime_order_curve(p)
        print(p, Fp(p).d, (C.a, C.b) if C else None, n)
