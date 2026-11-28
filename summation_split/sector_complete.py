"""Branch-complete degree-four Fp-point certificate for a Semaev relation.

Five guarded Weierstrass addition charts on E and its quadratic twist.
No claim of F4 speedup or unspecialized elimination-ideal equality.
"""
from dataclasses import dataclass
import sympy as sp
from .sector import add, classify, nonsquare, ordinates

@dataclass(frozen=True)
class Point:
    x: object
    y: object
    z: object  # zero means group identity and x=y=0

@dataclass
class Gate:
    p: Point
    q: Point
    r: Point
    selectors: tuple
    slope: object
    inv: object
    scale: int
    equations: list

def point_equations(P, a, b, scale):
    x, y, z = P.x, P.y, P.z
    return [z*(z-1), (1-z)*x, (1-z)*y,
            z*(scale*y*y-x*x*x-a*x-b)]

def add_gate(P, Q, name, a, b, scale):
    """Mutually exclusive charts: identity-left/right, chord, tangent, inverse.

    The one-hot selectors and inverse guards prevent fake tangent and
    zero-denominator solutions. All generator degrees are at most four.
    """
    rx, ry, rz = sp.symbols(f'{name}_x {name}_y {name}_z')
    lam, inv = sp.symbols(f'{name}_slope {name}_inv')
    cs = sp.symbols(' '.join(f'{name}_case{i}' for i in range(5)))
    R = Point(rx, ry, rz)
    c0, c1, c2, c3, c4 = cs
    eq = [sum(cs)-1, *(c*(c-1) for c in cs)]
    def when(c, *conditions):
        eq.extend(sp.expand(c*v) for v in conditions)
    when(c0, P.z, rx-Q.x, ry-Q.y, rz-Q.z)
    when(c1, P.z-1, Q.z, rx-P.x, ry-P.y, rz-P.z)
    finite = (P.z-1, Q.z-1, rz-1)
    xdiff = Q.x-P.x
    when(c2, *finite, xdiff*inv-1, lam*xdiff-(Q.y-P.y),
         rx-scale*lam*lam+P.x+Q.x, ry-lam*(P.x-rx)+P.y)
    when(c3, *finite, P.x-Q.x, P.y-Q.y, P.y*inv-1,
         2*scale*P.y*lam-3*P.x*P.x-a,
         rx-scale*lam*lam+P.x+Q.x, ry-lam*(P.x-rx)+P.y)
    when(c4, P.z-1, Q.z-1, P.x-Q.x, P.y+Q.y, rx, ry, rz)
    eq.extend(point_equations(R, a, b, scale))
    return Gate(P, Q, R, cs, lam, inv, scale, eq)

def frobenius_lift(v, p):
    """Quadratic addition chain for v^p=v over algebraic closure.

    All extra power variables are uniquely determined by v; final degree <=2.
    """
    powers = [v]
    eq = []
    update = []
    for i in range(1, p.bit_length()):
        u = sp.Symbol(f'{v}_pow{2**i}')
        eq.append(u-powers[-1]*powers[-1])
        powers.append(u)
        update.append((u, 2**i))
    bits = [i for i in range(p.bit_length()) if (p >> i) & 1]
    acc = powers[bits[0]]
    exponent = 2**bits[0]
    for j, i in enumerate(bits[1:]):
        next_exp = exponent+2**i
        if j == len(bits)-2:
            eq.append(acc*powers[i]-v)
        else:
            w = sp.Symbol(f'{v}_pow{next_exp}')
            eq.append(w-acc*powers[i])
            acc = w
            update.append((w, next_exp))
        exponent = next_exp
    return eq, update

def circuit(p, a, b, m, *, field_lift=False):
    """Unspecialized input x_0,...,x_m, with optional quadratic Fp constraints.

    field_lift=True enforces each leaf x_i,y_i in Fp even over Fpbar.
    This recovers the Fp zero SET of S_(m+1), not its principal ideal.
    """
    if not sp.isprime(p) or p <= 3 or (4*a**3+27*b*b) % p == 0 or m < 2:
        raise ValueError('requires nonsingular odd-prime curve and m>=2')
    d = nonsquare(p)
    leaves, eq, plus, minus, gates = [], [], [], [], []
    field_helpers = {}
    for i in range(m+1):
        x, y, s = sp.symbols(f'x{i} y{i} sector{i}')
        ap, bp, cp, am, bm, cm = sp.symbols(
            f'plus{i}_x plus{i}_y plus{i}_z minus{i}_x minus{i}_y minus{i}_z')
        u, v = Point(ap, bp, cp), Point(am, bm, cm)
        eq.extend([s*(s-1), (1+(d-1)*s)*y*y-x*x*x-a*x-b,
                   cp-(1-s), ap-(1-s)*x, bp-(1-s)*y,
                   cm-s, am-s*x, bm-s*y])
        eq.extend(point_equations(u, a, b, 1))
        eq.extend(point_equations(v, a, b, d))
        if field_lift:
            for value in (x, y):
                checks, helpers = frobenius_lift(value, p)
                eq.extend(checks)
                field_helpers[value] = helpers
        leaves.append((x, y, s))
        plus.append(u)
        minus.append(v)
    def reduce_chain(points, prefix, scale):
        P = points[0]
        for j, Q in enumerate(points[1:], 1):
            g = add_gate(P, Q, f'{prefix}{j}', a, b, scale)
            gates.append(g)
            eq.extend(g.equations)
            P = g.r
        return P
    A = reduce_chain(plus, 'A', 1)
    B = reduce_chain(minus, 'B', d)
    # Both sums agree at O or at the same rational 2-torsion point.
    eq.extend([A.z-B.z, A.z*(A.x-B.x), A.z*A.y, B.z*B.y])
    vars_ = sorted(set().union(*(e.free_symbols for e in eq)), key=str)
    degree = max(sp.total_degree(e) for e in eq)
    assert degree <= 4, degree
    return dict(equations=eq, variables=vars_, leaves=leaves,
                gates=gates, plus=plus, minus=minus, A=A, B=B,
                degree=degree, prime=p, a=a, b=b, d=d,
                field_helpers=field_helpers)

def point_assignment(P, value, p):
    if value is None:
        return {P.x: 0, P.y: 0, P.z: 0}
    return {P.x: value[0] % p, P.y: value[1] % p, P.z: 1}

def gate_witness(g, P, Q, p, a):
    """Build chart assignment with independent sector.add point arithmetic."""
    R = add(P, Q, p, a, g.scale)
    if P is None:
        case = 0
    elif Q is None:
        case = 1
    elif P[0] != Q[0]:
        case = 2
    elif (P[1]+Q[1]) % p == 0:
        case = 4
    else:
        case = 3
    slope = inv = 0
    if case == 2:
        denom = (Q[0]-P[0]) % p
        inv = pow(denom, -1, p)
        slope = (Q[1]-P[1])*inv % p
    if case == 3:
        inv = pow(P[1], -1, p)
        slope = (3*P[0]*P[0]+a)*pow(2*g.scale*P[1] % p, -1, p) % p
    assignment = point_assignment(g.r, R, p)
    assignment.update({g.slope: slope, g.inv: inv})
    assignment.update({s: int(i == case) for i, s in enumerate(g.selectors)})
    return R, assignment

def certificate(sys, xs, signs=None):
    """Full witness for given leaf x-coordinates and signed lifts."""
    p, a, b, d = sys['prime'], sys['a'], sys['b'], sys['d']
    if len(xs) != len(sys['leaves']):
        raise ValueError('wrong arity')
    if signs is None:
        signs = [1]*len(xs)
    out, plus, minus = {}, [], []
    for i, (x, (sx, sy, ss), positive, negative) in enumerate(
            zip(xs, sys['leaves'], sys['plus'], sys['minus'])):
        sector = int(classify(x, p, a, b) == 'minus')
        y = (signs[i]*ordinates(x, p, a, b, d if sector else 1)[0]) % p
        out.update({sx: x % p, sy: y, ss: sector})
        P = (x % p, y)
        qplus = P if sector == 0 else None
        qminus = P if sector == 1 else None
        plus.append(qplus)
        minus.append(qminus)
        out.update(point_assignment(positive, qplus, p))
        out.update(point_assignment(negative, qminus, p))
    states = {'A': plus[0], 'B': minus[0]}
    ix = {'A': 1, 'B': 1}
    for g in sys['gates']:
        sect = g.selectors[0].name[0]
        Q = (plus if sect == 'A' else minus)[ix[sect]]
        states[sect], update = gate_witness(g, states[sect], Q, p, a)
        ix[sect] += 1
        out.update(update)
    for value, helpers in sys['field_helpers'].items():
        out.update({u: pow(out[value], exp, p) for u, exp in helpers})
    return out

def check_assignment(sys, values):
    p = sys['prime']
    missing = set(sys['variables'])-set(values)
    if missing:
        raise KeyError(missing)
    return all(int(e.subs(values)) % p == 0 for e in sys['equations'])

def compile_checker(sys):
    """Compiled numeric generator-checking oracle, not a GB solver."""
    variables = sys['variables']
    fast = sp.lambdify(variables, sys['equations'], 'math')
    p = sys['prime']
    def check(values):
        return all(int(v) % p == 0 for v in fast(*(values[k] for k in variables)))
    return check
