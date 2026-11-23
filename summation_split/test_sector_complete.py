"""Producer-only finite-field proof-obligation tests for sector circuits."""
import itertools
import random
import unittest
import sympy as sp
from .sector import add, classify, nonsquare, ordinates, f
from .core import specialized_summation, eval_poly
from .sector_complete import (
    Point, add_gate, point_equations, circuit, certificate, gate_witness,
    point_assignment, compile_checker,
)

def linear_oracle(p, a, b, xs):
    coeff = specialized_summation(p, a, b, xs)
    return {x for x in range(p) if eval_poly(coeff, x, p) == 0}

def sem_witness(sys, xs):
    """Try all signed lifts; this is NOT a Gröbner basis solver."""
    for signs in itertools.product((1, -1), repeat=len(xs)):
        env = certificate(sys, xs, signs)
        az, bz = env[sys['A'].z], env[sys['B'].z]
        if az != bz:
            continue
        if az and (env[sys['A'].x] != env[sys['B'].x] or
                   env[sys['A'].y] != 0 or env[sys['B'].y] != 0):
            continue
        return env
    return None

class CompleteSectorTest(unittest.TestCase):
    def test_local_chart_exhaustive(self):
        """All ordered pairs, including O/tangent/inversion, over three fields."""
        for p, a, b in ((5, 0, 1), (7, 1, 1), (11, 1, 1)):
            for scale in (1, nonsquare(p)):
                coords = []
                for x in range(p):
                    v = f(x, p, a, b)*pow(scale, -1, p) % p
                    coords.extend((x, y) for y in range(p) if y*y % p == v)
                states = [None]+coords
                P = Point(*sp.symbols('px py pz'))
                Q = Point(*sp.symbols('qx qy qz'))
                g = add_gate(P, Q, 'verify', a, b, scale)
                all_eq = point_equations(P, a, b, scale) + point_equations(
                    Q, a, b, scale) + g.equations
                vv = sorted(set().union(*(e.free_symbols for e in all_eq)), key=str)
                fast = sp.lambdify(vv, all_eq, 'math')
                for x, y in itertools.product(states, repeat=2):
                    expected = add(x, y, p, a, scale)
                    z, env = gate_witness(g, x, y, p, a)
                    self.assertEqual(z, expected)
                    env.update(point_assignment(P, x, p))
                    env.update(point_assignment(Q, y, p))
                    results = fast(*(env[k] for k in vv))
                    self.assertTrue(all(int(v) % p == 0 for v in results),
                                    (p, a, b, scale, x, y))
                    self.assertEqual(sum(env[c] for c in g.selectors), 1)
                    case = next(i for i, c in enumerate(g.selectors) if env[c])
                    if case == 0:
                        self.assertIsNone(x)
                    if case == 1:
                        self.assertIsNotNone(x)
                        self.assertIsNone(y)
                    if case == 2:
                        self.assertIsNotNone(x)
                        self.assertIsNotNone(y)
                        self.assertNotEqual(x[0], y[0])
                    if case == 3:
                        self.assertEqual(x, y)
                        self.assertNotEqual(x[1], 0)
                    if case == 4:
                        self.assertEqual(x[0], y[0])
                        self.assertEqual((x[1]+y[1]) % p, 0)

    def test_prime31_mixed_sector_positive_certificates(self):
        """50 exact resultant-positive mixed-sector fixtures for each (curve,m)."""
        p = 2**31-1
        for a, b in ((1, 1), (1, 4)):
            T = None
            if (a, b) == (1, 1):
                zz = sp.Symbol('zz')
                poly = sp.Poly(zz**3+a*zz+b, zz, modulus=p)
                for factor, _ in poly.factor_list()[1]:
                    if factor.degree() == 1:
                        u, v = map(int, factor.all_coeffs())
                        T = (-v*pow(u % p, -1, p) % p, 0)
                        break
                self.assertIsNotNone(T)
            for m in (3, 4, 5):
                sys = circuit(p, a, b, m, field_lift=True)
                fast = compile_checker(sys)
                rr = random.Random(p+9349*a+6151*b+731*m)
                d = nonsquare(p)
                for k in range(50):
                    target = T if T is not None and k % 2 else None
                    all_leaves = []
                    for scale, count in ((1, (m+1)//2),
                                         (d, (m+1)-(m+1)//2)):
                        while True:
                            pts, S = [], None
                            for j in range(count-1):
                                while True:
                                    x = rr.randrange(p)
                                    rhs = f(x, p, a, b)*pow(scale, -1, p) % p
                                    if pow(rhs, (p-1)//2, p) in (0, 1):
                                        y = ordinates(x, p, a, b, scale)[0]
                                        break
                                if rr.randrange(2):
                                    y = (-y) % p
                                P = (x, y)
                                pts.append(P)
                                S = add(S, P, p, a, scale)
                            minusS = None if S is None else (S[0], (-S[1]) % p)
                            last = add(target, minusS, p, a, scale)
                            if last is not None and (scale == 1 or last[1] != 0):
                                break
                        pts.append(last)
                        all_leaves.extend((x, y, scale) for x, y in pts)
                    rr.shuffle(all_leaves)
                    xs = [x for x, _, _ in all_leaves]
                    signs = []
                    for x, y, scale in all_leaves:
                        self.assertEqual(
                            classify(x, p, a, b),
                            'minus' if scale == d else 'plus')
                        reference = ordinates(x, p, a, b, scale)[0]
                        signs.append(1 if reference == y else -1)
                    env = certificate(sys, xs, signs)
                    self.assertTrue(fast(env), (a, b, m, k, xs))
                    self.assertEqual(
                        eval_poly(specialized_summation(p, a, b, xs[:-1]),
                                  xs[-1], p), 0, (a, b, m, k, xs))
                print('prime31 mixed positive', a, b, m, '50/50',
                      'vars', len(sys['variables']),
                      'gens', len(sys['equations']))

    def test_projected_50_instances(self):
        """Full each-X root-set comparison after 50 random slices per cell."""
        for p, a, b, m in ((5, 0, 1, 3), (7, 1, 1, 3),
                            (11, 1, 1, 3), (11, 1, 1, 4)):
            sys = circuit(p, a, b, m, field_lift=True)
            fast = compile_checker(sys)
            self.assertEqual(sys['degree'], 4)
            rr = random.Random(6052026+1337*p+100*m)
            good = 0
            for sample in range(50):
                xs = [rr.randrange(p) for _ in range(m)]
                roots = linear_oracle(p, a, b, xs)
                found = {X for X in range(p)
                         if sem_witness(sys, (*xs, X)) is not None}
                self.assertEqual(roots, found, (p, a, b, m, sample, xs))
                for X in roots:
                    env = sem_witness(sys, (*xs, X))
                    self.assertTrue(fast(env))
                    good += 1
            print('gate', p, a, b, m, '50/50',
                  'positive certificates', good, 'vars', len(sys['variables']),
                  'gens', len(sys['equations']))

if __name__ == '__main__':
    unittest.main()
