#!/usr/bin/env python3
"""CT59: exact residue-degree floors for single ell-torsion on y^2+y=x^3/F2.
Pure stdlib. Group orders and modular-order checks are independently derived.
"""
PRIMES = (3, 5, 7, 11, 13, 17, 19, 23, 31, 37, 41, 43, 47, 59,
          61, 67, 73, 89, 97, 101, 127, 131, 163, 257, 521,
          65537, 2147483647, 4294967291)
FIXTURES = {3: (1, 2), 5: (8, 8), 11: (5, 10), 19: (9, 18),
            37: (72, 72), 43: (7, 14), 97: (96, 96),
            163: (81, 162), 257: (32, 32), 521: (520, 520),
            65537: (64, 64), 2147483647: (124, 124),
            4294967291: (2147483645, 4294967290)}

def require(c, msg):
    if not c:
        raise AssertionError(msg)

def prime(n):
    if n < 2: return False
    d = 2
    while d*d <= n:
        if n % d == 0: return False
        d += 1 if d == 2 else 2
    return True

def factors(n):
    ans = []
    d = 2
    while d*d <= n:
        if n % d == 0:
            ans.append(d)
            while n % d == 0: n //= d
        d += 1 if d == 2 else 2
    if n > 1: ans.append(n)
    return ans

def minus_two_order(ell):
    m = ell-1
    a = (-2) % ell
    for r in factors(m):
        while m % r == 0 and pow(a, m//r, ell) == 1:
            m //= r
    require(pow(a, m, ell) == 1, "not an order")
    require(all(pow(a, m//r, ell) != 1 for r in factors(m)), "nonminimal order")
    return m

def group_size(f):
    # Frobenius trace recurrence S_0=2,S_1=0,S_f=-2*S_(f-2).
    s0, s1 = 2, 0
    if f == 0: return 0  # not a finite field; explicit rejected sentinel
    if f == 1: return 3
    for _ in range(2, f+1):
        s0, s1 = s1, -2*s0
    return (1<<f) + 1 - s1

def audit():
    require(group_size(1)==3 and group_size(2)==9 and group_size(3)==9,
            "Frobenius trace recurrence wrong")
    for ell in PRIMES:
        require(prime(ell), f"nonprime {ell}")
        m = minus_two_order(ell)
        some = m if m % 2 else 2*m
        full = 2*m
        if ell <= 521:
            first = next(f for f in range(1, full+1) if group_size(f)%ell==0)
            require(first == some, f"group-order test mismatch at {ell}")
            require(group_size(full)%ell**2==0, "full torsion not in point count")
            if some != full:
                require(group_size(some)%ell**2!=0, "unexpected full torsion")
        if ell in FIXTURES:
            require((some, full)==FIXTURES[ell], f"fixture mismatch at {ell}")
        print(f"ell={ell} single_f={some} full_f={full} ord_minus2={m}")
    require(1 + sum(1 for x in range(2) for y in range(2)
                    if (y*y+y-x*x*x-x)%2 == 0) == 5,
            "twist countercontrol missing")
    print("PASS: fixed-twist only; five-point twist defeats universal single-point f>=8 at ell=5")

if __name__ == '__main__':
    audit()
