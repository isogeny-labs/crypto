# Independent SageMath 10.10.rc0 group-order check for the four exact sources.
from sage.all import GF, PolynomialRing, EllipticCurve

for n, poly, a, b, expected in [
    (6, 0x43, 0, 32, 76),
    (8, 0x11b, 0, 32, 268),
    (10, 0x409, 0, 32, 1032),
    (12, 0x1009, 1, 512, 3980),
]:
    F2 = GF(2)
    x = PolynomialRing(F2, 'x').gen()
    modulus = sum(x**i for i in range(n+1) if poly >> i & 1)
    field = GF(2**n, 'z', modulus=modulus)
    z = field.gen()
    element = lambda word: sum(z**i for i in range(n) if word >> i & 1)
    curve = EllipticCurve(field, [field.one(), element(a), field.zero(), field.zero(), element(b)])
    order = curve.cardinality()
    assert order == expected, (n, order, expected)
    print(f'n={n} modulus=0x{poly:x} order={order} independently_verified=true')
