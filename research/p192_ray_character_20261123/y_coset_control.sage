"""Screen fixed P-192 y-coordinate coset products.

Run: sage research/p192_ray_character_20261123/y_coset_control.sage
Long check: sage -python -u research/p192_ray_character_20261123/y_coset_control.sage --order-95560
"""

import sys

from sage.all import EllipticCurve, GF, ZZ, kronecker


p = 6277101735386680763835789423207666416083908700390324961279
r = 6277101735386680763835789423176059013767194773182842284081
n = (r - 1) // 2
E = EllipticCurve(
    GF(p), [-3, 0x64210519E59C80E70FA7E9AB72243049FEB8DEECC146B9B1]
)
G = E(
    0x188DA80EB03090F67CBF20EB43A18800F4FF0AFD82FF1012,
    0x07192B95FFC8DA78631011ED6B24CDD573F977A11E794811,
)
assert r * G == E(0)


class EDSWindow:
    """Evaluate division-polynomial values with logarithmic window space."""

    def __init__(self, P):
        x, y = int(P[0]), int(P[1])
        a = -3
        b = int(E.a6())
        x2 = x * x % p
        x3 = x2 * x % p
        x4 = x2 * x2 % p
        x6 = x3 * x3 % p
        self.inverse_two_y = pow(2 * y % p, -1, p)
        self.small = [
            0,
            1,
            2 * y % p,
            (3 * x4 + 6 * a * x2 + 12 * b * x - a * a) % p,
            4 * y * (x6 + 5 * a * x4 + 20 * b * x3 - 5 * a * a * x2
                     - 4 * a * b * x - 8 * b * b - a * a * a) % p,
        ]
        for j in range(5, 13):
            self.small.append(self._from_values(j, self.small, 0))

    def _from_values(self, j, values, first):
        def get(k):
            return values[k - first]

        q = j // 2
        if j & 1:
            return (get(q + 2) * pow(get(q), 3, p)
                    - get(q - 1) * pow(get(q + 1), 3, p)) % p
        return (get(q) * (get(q + 2) * pow(get(q - 1), 2, p)
                          - get(q - 2) * pow(get(q + 1), 2, p))
                * self.inverse_two_y) % p

    def _window(self, center):
        first = max(0, center - 4)
        if center <= 8:
            return first, self.small[first:center + 5]
        parent_first, parent = self._window(center // 2)
        return first, [self._from_values(j, parent, parent_first)
                       for j in range(first, center + 5)]

    def psi(self, j):
        assert 1 <= j < r
        if j < len(self.small):
            return self.small[j]
        first, values = self._window(j)
        return values[j - first]


def y_character(P, reps):
    value = E.base_field()(1)
    for j in reps:
        value *= (j * P)[1]
    return int(kronecker(ZZ(value), p))


def y_character_eds(P, reps):
    assert len(reps) % 2 == 0
    eds = EDSWindow(P)
    product = 1
    for j in reps:
        product = product * eds.psi(2 * j) % p
    # psi_(2j) = 2*y([j]P)*psi_j(P)^4; 2^len(reps) is a square.
    return int(kronecker(ZZ(product), p))


if "--order-95560" in sys.argv:
    m = 95560
    t = n // m
    assert t % 2 == 1
    h = pow(3, t, r)
    assert pow(h, m, r) == r - 1
    reps = sorted({min(pow(h, j, r), r - pow(h, j, r)) for j in range(m)})
    assert len(reps) == m
    for P in (G, 3 * G):
        eds = EDSWindow(P)
        for j in reps[:8]:
            assert eds.psi(2 * j) == int(2 * (j * P)[1]) * pow(eds.psi(j), 4, p) % p
    base = y_character_eds(G, reps)
    target = y_character_eds(3 * G, reps)
    assert (base, target) == (-1, -1)
    observed = base * target
    expected = int(kronecker(3, r))
    assert observed == 1 and expected == -1
    print(f"m={m}, d=3: y-coset ratio={observed}, ray ratio={expected}")
    sys.exit(0)


for m in (8, 40):
    t = n // m
    assert t % 2 == 1
    h = pow(3, t, r)
    assert pow(h, m, r) == r - 1
    reps = sorted({min(pow(h, j, r), r - pow(h, j, r)) for j in range(m)})
    assert len(reps) == m
    base = y_character(G, reps)
    assert y_character_eds(G, reps) == base
    assert y_character(h * G, reps) * base == -1
    for d in (1, 2, 3, 5):
        observed = y_character(d * G, reps) * base
        expected = int(kronecker(d, r))
        if d == 2:
            assert observed == -1 and expected == 1
        print(f"m={m}, d={d}: y-coset ratio={observed}, ray ratio={expected}")


m = 19112
t = n // m
assert t % 2 == 1
h = pow(3, t, r)
assert pow(h, m, r) == r - 1
reps = sorted({min(pow(h, j, r), r - pow(h, j, r)) for j in range(m)})
assert len(reps) == m
for P in (G, 3 * G):
    eds = EDSWindow(P)
    for j in reps[:8]:
        assert eds.psi(2 * j) == int(2 * (j * P)[1]) * pow(eds.psi(j), 4, p) % p
base = y_character_eds(G, reps)
target = y_character_eds(3 * G, reps)
assert (base, target) == (-1, -1)
observed = base * target
expected = int(kronecker(3, r))
assert observed == 1 and expected == -1
print(f"m={m}, d=3: y-coset ratio={observed}, ray ratio={expected}")
