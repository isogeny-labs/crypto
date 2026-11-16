#!/usr/bin/env python3
"""Independent F128 sigma-annihilator check for the selected Hess parameter."""

from math import gcd

def mul(a: int, b: int) -> int:
    out = 0
    while b:
        if b & 1:
            out ^= a
        b >>= 1
        a <<= 1
        if a & 0x80:
            a ^= 0x83  # z^7 + z + 1
    return out


def power(a: int, n: int) -> int:
    out = 1
    while n:
        if n & 1:
            out = mul(out, a)
        a = mul(a, a)
        n >>= 1
    return out


def sigma(a: int) -> int:
    return mul(a, a)


def trace(a: int) -> int:
    out = 0
    for _ in range(7):
        out ^= a
        a = sigma(a)
    return out


def annihilator(a: int) -> int:
    orbit = [a]
    for _ in range(7):
        orbit.append(sigma(orbit[-1]))
    for degree in range(1, 8):
        for lower in range(1 << degree):
            mask = (1 << degree) | lower
            value = 0
            for i in range(degree + 1):
                if (mask >> i) & 1:
                    value ^= orbit[i]
            if value == 0:
                return mask
    raise AssertionError("Frobenius order must divide T^7+1")


z = 2
assert power(z, 127) == 1
assert all(power(z, i) != 1 for i in range(1, 127))
r = power(z, 65)
c = power(z, 94)
aux = power(z, 98)
j = power(z, 124)
assert mul(r, r) == power(z, 3)
assert mul(c, aux) == r
assert j not in (0, 1)
assert trace(c) == 1
mc, mb = annihilator(c), annihilator(aux)
ma = 0b11  # T+1: trace of the a2=1 term is odd
print(f"z=0x{z:02x} c=0x{c:02x} r=0x{r:02x} aux=0x{aux:02x} j=0x{j:02x}")
print(f"trace_c={trace(c)} trace_aux={trace(aux)} j_in_F2={j in (0, 1)}")
print(f"m_c=0b{mc:b} degree={mc.bit_length()-1}")
print(f"m_aux=0b{mb:b} degree={mb.bit_length()-1}")
print(f"m_a=0b{ma:b}")
# A common annihilator is the first monic polynomial divisible by all three.
for degree in range(1, 8):
    found = []
    for lower in range(1 << degree):
        mask = (1 << degree) | lower
        values = []
        for a in (c, aux, 1):
            x = a
            value = 0
            for i in range(degree + 1):
                if (mask >> i) & 1:
                    value ^= x
                x = sigma(x)
            values.append(value)
        if values == [0, 0, 0]:
            found.append(mask)
    if found:
        mf = min(found)
        break
print(f"m_f=0b{mf:b} degree={mf.bit_length()-1} kernel_exponent_bound={1 << (mf.bit_length()-2)}")
assert mf.bit_length() - 1 == 4
assert gcd(23, 1 << (4 - 1)) == 1
