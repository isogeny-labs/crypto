"""Check Tian's published F257 genus-3 3-kernel in F_(257^6).

Usage: python3 tian_genus3_kernel_gate.py dlpiso.tex
The source TeX digest is pinned; this checks torsion and Frobenius stability,
not Weil-pairing isotropy or quotient-from-kernel construction.
"""

from __future__ import annotations

import functools
import hashlib
import json
import re
import sys
from pathlib import Path

P = 257
N = 6
Q = P ** N
TEX_SHA = "46092fc37bc6bac7e0f42c67e6764150557f63ac4292da86d739980268d3eaae"
MOD = (3, 138, 18, 62, 3, 0)
ZERO = (0,) * N
ONE = (1, 0, 0, 0, 0, 0)
B = (0, 1, 0, 0, 0, 0)


def c(a: int) -> tuple[int, ...]:
    return (a % P, 0, 0, 0, 0, 0)


def fa(a: tuple[int, ...], b: tuple[int, ...]) -> tuple[int, ...]:
    return tuple((x + y) % P for x, y in zip(a, b))


def fn(a: tuple[int, ...]) -> tuple[int, ...]:
    return tuple(-x % P for x in a)


def fs(a: tuple[int, ...], b: tuple[int, ...]) -> tuple[int, ...]:
    return fa(a, fn(b))


def fm(a: tuple[int, ...], b: tuple[int, ...]) -> tuple[int, ...]:
    t = [0] * 11
    for i, x in enumerate(a):
        if x:
            for j, y in enumerate(b):
                t[i + j] += x * y
    for i in range(10, 5, -1):
        high = t[i] % P
        if high:
            for j, m in enumerate(MOD):
                t[i - 6 + j] -= high * m
    return tuple(x % P for x in t[:6])


def fpow(a: tuple[int, ...], n: int) -> tuple[int, ...]:
    out = ONE
    while n:
        if n & 1:
            out = fm(out, a)
        n >>= 1
        if n:
            a = fm(a, a)
    return out


@functools.lru_cache(maxsize=None)
def fi(a: tuple[int, ...]) -> tuple[int, ...]:
    assert a != ZERO
    out = fpow(a, Q - 2)
    assert fm(a, out) == ONE
    return out


def trim(a: list[tuple[int, ...]]) -> list[tuple[int, ...]]:
    while a and a[-1] == ZERO:
        a.pop()
    return a


def pa(a: list, b: list) -> list:
    return trim([fa(a[i] if i < len(a) else ZERO,
                    b[i] if i < len(b) else ZERO) for i in range(max(len(a), len(b)))])


def pn(a: list) -> list:
    return [fn(v) for v in a]


def ps(a: list, b: list) -> list:
    return pa(a, pn(b))


def pm(a: list, b: list) -> list:
    if not a or not b:
        return []
    out = [ZERO] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            out[i + j] = fa(out[i + j], fm(x, y))
    return trim(out)


def scale(a: list, k: tuple[int, ...]) -> list:
    return trim([fm(x, k) for x in a])


def divmodp(a: list, b: list) -> tuple[list, list]:
    assert b
    r = trim(a.copy())
    q = [ZERO] * max(1, len(a) - len(b) + 1)
    lead = fi(b[-1]) if b[-1] != ONE else ONE
    while len(r) >= len(b):
        degree = len(r) - len(b)
        factor = fm(r[-1], lead)
        q[degree] = factor
        for j, term in enumerate(b):
            r[degree + j] = fs(r[degree + j], fm(factor, term))
        trim(r)
    return trim(q), r


def exact(a: list, b: list) -> list:
    q, r = divmodp(a, b)
    assert not r
    return q


def rem(a: list, b: list) -> list:
    return divmodp(a, b)[1]


def monic(a: list) -> list:
    return scale(a, fi(a[-1])) if a and a[-1] != ONE else a


def egcd(a: list, b: list) -> tuple[list, list, list]:
    r0, r1, s0, s1, t0, t1 = a, b, [ONE], [], [], [ONE]
    while r1:
        q, r = divmodp(r0, r1)
        r0, r1 = r1, r
        s0, s1 = s1, ps(s0, pm(q, s1))
        t0, t1 = t1, ps(t0, pm(q, t1))
    inv = fi(r0[-1])
    return scale(r0, inv), scale(s0, inv), scale(t0, inv)


F = [c(v) for v in (167, 104, 125, 138, 6, 13, 0, 1)]
DZERO = ((ONE,), ())


def divisor(u: list, v: list) -> tuple:
    u = monic(trim(u))
    v = rem(trim(v), u)
    assert 1 <= len(u) <= 4 and u[-1] == ONE and len(v) < len(u)
    assert not rem(ps(pm(v, v), F), u)
    return tuple(u), tuple(v)


def neg(d: tuple) -> tuple:
    return d[0], tuple(trim(pn(list(d[1]))))


def add(a: tuple, b: tuple) -> tuple:
    if a == DZERO:
        return b
    if b == DZERO:
        return a
    ua, va = map(list, a)
    ub, vb = map(list, b)
    d1, e1, e2 = egcd(ua, ub)
    d, c1, c2 = egcd(d1, pa(va, vb))
    s1, s2 = pm(c1, e1), pm(c1, e2)
    u = exact(pm(ua, ub), pm(d, d))
    vnum = pa(pa(pm(pm(s1, ua), vb), pm(pm(s2, ub), va)),
              pm(c2, pa(pm(va, vb), F)))
    v = rem(exact(vnum, d), u)
    u = monic(u)
    v = rem(v, u)
    while len(u) > 4:
        new_u = monic(exact(ps(F, pm(v, v)), u))
        assert len(new_u) < len(u)
        v = rem(pn(v), new_u)
        u = new_u
    return divisor(u, v)


def mul(d: tuple, n: int) -> tuple:
    out = DZERO
    while n:
        if n & 1:
            out = add(out, d)
        n >>= 1
        if n:
            d = add(d, d)
    return out


def frob(d: tuple) -> tuple:
    return tuple(tuple(fpow(v, P) for v in part) for part in d)


def base_gcd(a: list[int], b: list[int]) -> list[int]:
    def clean(poly: list[int]) -> list[int]:
        while poly and poly[-1] == 0:
            poly.pop()
        return poly

    a, b = clean(a.copy()), clean(b.copy())
    while b:
        rem_poly = a.copy()
        inverse_lead = pow(b[-1], -1, P)
        while len(rem_poly) >= len(b):
            top = rem_poly[-1] * inverse_lead % P
            offset = len(rem_poly) - len(b)
            for i, coefficient in enumerate(b):
                rem_poly[offset + i] = (rem_poly[offset + i] - top * coefficient) % P
            clean(rem_poly)
        a, b = b, rem_poly
    return [(value * pow(a[-1], -1, P)) % P for value in a]


def extract_exponents(tex: str, name: str) -> list[int]:
    start = tex.index(name + "=&")
    end = tex.index("\\\\", start + len(name) + 2)
    # Each T2/T3 occupies a second TeX line for v; stop at the next label.
    stop = tex.index("T_3=&" if name == "T_2" else "\\end{align*}", start)
    exponents = [int(x) for x in re.findall(r"b\^\{\s*(\d+)\s*\}", tex[start:stop])]
    assert len(exponents) == 6, (name, exponents, end)
    return exponents


def main() -> None:
    source = Path(sys.argv[1]).read_bytes()
    assert hashlib.sha256(source).hexdigest() == TEX_SHA
    tex = source.decode()
    assert fpow(B, Q) == B
    modulus = list(MOD) + [1]
    assert base_gcd(modulus, list(fs(fpow(B, P ** 2), B))) == [1]
    assert base_gcd(modulus, list(fs(fpow(B, P ** 3), B))) == [1]
    t1 = divisor([c(v) for v in (57, 224, 15, 1)], [c(v) for v in (53, 119, 168)])
    generators = [t1]
    for name in ("T_2", "T_3"):
        exponents = extract_exponents(tex, name)
        a = [fpow(B, value) for value in exponents]
        generators.append(divisor([a[2], a[1], a[0], ONE], [a[5], a[4], a[3]]))
        if name == "T_2":
            corrupted = a.copy()
            corrupted[0] = fm(corrupted[0], B)
            try:
                divisor([corrupted[2], corrupted[1], corrupted[0], ONE],
                        [corrupted[5], corrupted[4], corrupted[3]])
            except AssertionError:
                corrupted_generator_rejected = True
            else:
                corrupted_generator_rejected = False
            assert corrupted_generator_rejected
    assert all(d != DZERO and mul(d, 3) == DZERO for d in generators)
    multiples = [(DZERO, d, add(d, d)) for d in generators]
    elements = {}
    for i in range(3):
        for j in range(3):
            for k in range(3):
                value = add(add(multiples[0][i], multiples[1][j]), multiples[2][k])
                elements[value] = [i, j, k]
    assert len(elements) == 27
    action = []
    for d in generators:
        image = frob(d)
        assert image in elements
        action.append(elements[image])
    assert frob(t1) == t1
    print(json.dumps({
        "schema": "tian-genus3-f257-kernel-gate/v1",
        "source_tex_sha256": TEX_SHA,
        "field": "F_257[b]/(b^6+3b^4+62b^3+18b^2+138b+3)",
        "basis_frobenius_degree": 6,
        "field_modulus_irreducible_by_rabin": True,
        "generators_order_three": 3,
        "distinct_kernel_elements": len(elements),
        "frobenius_generator_images": action,
        "frobenius_stable": True,
        "corrupted_generator_rejected": corrupted_generator_rejected,
        "isotropy_checked": False,
        "quotient_constructed": False,
    }, indent=2))


if __name__ == "__main__":
    main()
