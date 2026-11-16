"""Exact supplied-pivot primitives. Prime certification and pivot search are external."""
from dataclasses import dataclass, field


@dataclass
class Meter:
    r: int
    products: int = 0
    additions: int = 0
    inversions: int = 0

    def mul(self, a, b):
        self.products += 1
        return a*b % self.r

    def add(self, a, b):
        self.additions += 1
        return (a+b) % self.r

    def sub(self, a, b):
        self.additions += 1
        return (a-b) % self.r

    def inv(self, a):
        if not a:
            raise ValueError('singular pivot')
        self.inversions += 1
        return pow(a, -1, self.r)

    def dot(self, a, b):
        if len(a) != len(b):
            raise ValueError('dot dimensions')
        out = 0
        for x, y in zip(a, b):
            out = self.add(out, self.mul(x, y))
        return out


def power(a, e, meter):
    out = 1
    while e:
        if e & 1:
            out = meter.mul(out, a)
        e //= 2
        if e:
            a = meter.mul(a, a)
    return out


def canonical(x, r):
    return type(x) is int and 0 <= x < r


@dataclass(frozen=True)
class Basis:
    r: int
    q: int
    labels: tuple

    def evaluate(self, x, meter):
        if meter.r != self.r or not canonical(x, self.r):
            raise ValueError('field or coordinate')
        chars = tuple(power((x+d) % self.r, (self.r-1)//2, meter)
                      for d in range(self.q))
        values = []
        for label in self.labels:
            value = 1
            for d in label:
                value = meter.mul(value, chars[d])
            values.append(value)
        return tuple(values)


@dataclass(frozen=True)
class State:
    basis: Basis
    C: tuple
    u: tuple
    used: tuple = ()
    token: object = field(default_factory=object, compare=False, repr=False)

    def validate(self, meter):
        r, n = self.basis.r, len(self.basis.labels)
        if meter.r != r or len(self.C) != n or len(self.u) != n:
            raise ValueError('state dimensions or field')
        if any(len(row) != n for row in self.C):
            raise ValueError('coefficient dimensions')
        if not all(canonical(x, r) for row in self.C for x in row):
            raise ValueError('coefficient residues')
        if not all(canonical(x, r) for x in self.u + self.used):
            raise ValueError('state residues')
        if len(set(self.used)) != len(self.used):
            raise ValueError('duplicate used coordinate')
        if any(self.C[i][j] != self.C[j][i] for i in range(n) for j in range(i)):
            raise ValueError('nonsymmetric coefficients')


def build(r, q, gamma, weights, meter):
    """Same labels and diagonal weights as the preserved public constructor."""
    if (type(r) is not int or type(q) is not int or r <= 3 or
            not 1 <= q < r or not canonical(gamma, r) or gamma <= 1 or meter.r != r):
        raise ValueError('parameters; r must be caller-certified prime')
    patterns=[(0,)]+[(0,d) for d in range(1,q)]+[(0,d,e) for d in range(1,q) for e in range(d+1,q)]
    weights = tuple(weights)
    if len(weights) != len(patterns)+1 or not all(canonical(w, r) and w for w in weights):
        raise ValueError('nonzero canonical weights')
    labels, diagonal = [()], [weights[0]]
    for P, w in zip(patterns, weights[1:]):
        ell = q-max(P)
        a = w
        for shift in range(ell):
            labels.append(tuple(shift+d for d in P))
            diagonal.append(a)
            if shift+1 < ell:
                a = meter.mul(a, gamma)
    n = len(labels)
    C = tuple(tuple(diagonal[i] if i == j else 0 for j in range(n)) for i in range(n))
    return State(Basis(r, q, tuple(labels)), C, (0,)*n)


def inverse_block(E, meter):
    if len(E) == 1:
        return ((meter.inv(E[0][0]),),)
    a, b = E[0]
    c, d = E[1]
    det = meter.sub(meter.mul(a, d), meter.mul(b, c))
    z = meter.inv(det)
    return ((meter.mul(d, z), meter.mul(meter.sub(0, b), z)),
            (meter.mul(meter.sub(0, c), z), meter.mul(a, z)))


@dataclass(frozen=True)
class Witness:
    support: tuple
    moment: tuple
    token: object = field(compare=False, repr=False)


@dataclass(frozen=True)
class Frame:
    points: tuple
    F: tuple
    H: tuple
    parent_token: object = field(compare=False, repr=False)
    child_token: object = field(compare=False, repr=False)
    r: int

    def lift(self, witness, meter):
        """Internal witnessed-state path; final checker distrusts cached moments."""
        if witness.token is not self.child_token or meter.r != self.r:
            raise ValueError('witness provenance or field')
        n, k = len(self.H), len(self.points)
        if len(witness.moment) != n:
            raise ValueError('moment dimension')
        if any(x in self.points for x, w in witness.support):
            raise ValueError('overlapping support')
        y = tuple(meter.sub(0, meter.dot(tuple(row[j] for row in self.H), witness.moment))
                  for j in range(k))
        moment = tuple(meter.add(witness.moment[i],
                       meter.dot(tuple(row[i] for row in self.F), y)) for i in range(n))
        return Witness(witness.support + tuple(zip(self.points, y)), moment, self.parent_token)


def pivot(state, points, meter):
    """Return a new state and lift frame; singular blocks fail without altering state."""
    state.validate(meter)
    points = tuple(points)
    if (len(points) not in (1, 2) or len(set(points)) != len(points) or
            any(not canonical(x, state.basis.r) or x in state.used for x in points)):
        raise ValueError('one or two distinct unused public coordinates required')
    n, k = len(state.u), len(points)
    F = tuple(state.basis.evaluate(x, meter) for x in points)
    W = tuple(tuple(meter.dot(row, f) for f in F) for row in state.C)
    E = tuple(tuple(meter.dot(f, tuple(row[j] for row in W)) for j in range(k)) for f in F)
    inverse = inverse_block(E, meter)
    H = tuple(tuple(meter.dot(row, tuple(v[j] for v in inverse)) for j in range(k)) for row in W)
    C = tuple(tuple(meter.sub(state.C[i][j], meter.dot(H[i], W[j]))
                    for j in range(n)) for i in range(n))
    h = tuple(meter.sub(x, meter.dot(f, state.u)) for x, f in zip(points, F))
    u = tuple(meter.add(state.u[i], meter.dot(H[i], h)) for i in range(n))
    child = State(state.basis, C, u, state.used + points)
    return child, Frame(points, F, H, state.token, child.token, state.basis.r)


def terminal(state, x, meter):
    """Sufficient supplied-point terminal: C*phi(x)=0 and residual RHS nonzero."""
    state.validate(meter)
    if x in state.used:
        raise ValueError('terminal point already eliminated')
    f = state.basis.evaluate(x, meter)
    image = tuple(meter.dot(row, f) for row in state.C)
    if any(image):
        raise ValueError('supplied point is not coefficient-annihilated')
    h = meter.sub(x, meter.dot(f, state.u))
    if not h:
        raise ValueError('zero residual RHS at supplied terminal point')
    w = meter.inv(h)
    return Witness(((x, w),), tuple(meter.mul(w, v) for v in f), state.token)
