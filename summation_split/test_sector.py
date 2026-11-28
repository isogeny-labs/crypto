"""Regression gates for the Frobenius-sign Semaev root extractor."""
import itertools
import random
import unittest
import sympy as sp
from .core import specialized_summation, eval_poly
from .sector import (
    frobenius_sector_roots, zero_only_roots, nonsquare, legendre,
    f, add, ordinates, classify, sqrt_mod,
)
from .sector_polynomial import (
    leaf_constraints, generic_add_chart, generic_double_chart,
    max_generator_degree,
)


def exact_roots(p, a, b, xs):
    """Semaev fixed-degree resultant roots, enumerated or factored over Fp."""
    coeffs = specialized_summation(p, a, b, xs)
    if p < 1000:
        return {z for z in range(p) if eval_poly(coeffs, z, p) == 0}
    if not any(coeffs):
        raise ArithmeticError("zero polynomial requires all-Fp root branch")
    X = sp.Symbol("X")
    poly = sp.Poly.from_list(list(reversed(coeffs)), gens=X, modulus=p)
    roots = set()
    for g, _ in poly.factor_list()[1]:
        if g.degree() == 1:
            u, v = map(int, g.all_coeffs())
            r = -v * pow(u % p, -1, p) % p
            assert eval_poly(coeffs, r, p) == 0
            roots.add(r)
    return roots


CELLS = (
    (5,0,1,3), (7,1,1,3), (11,1,1,3), (11,1,1,4),
    (19,2,3,4), (19,2,3,5), (23,2,3,3), (29,1,1,4),
    (2147483647,1,1,3), (2147483647,1,1,4), (2147483647,1,1,5),
    (2147483647,1,4,3), (2147483647,1,4,4), (2147483647,1,4,5),
)


class SectorTest(unittest.TestCase):
    def test_50_independent_cases_per_parameter(self):
        for p, a, b, m in CELLS:
            rr = random.Random(12042026 + p*10007 + m*733)
            for i in range(50):
                xs = tuple(rr.randrange(p) for _ in range(m))
                self.assertEqual(exact_roots(p,a,b,xs),
                                 frobenius_sector_roots(p,a,b,xs),
                                 (p,a,b,m,i,xs))

    def test_exhaustive_tiny_fields(self):
        for p,a,b,m in ((5,0,1,3), (7,1,1,3),
                        (11,1,1,3), (11,1,1,4)):
            for xs in itertools.product(range(p), repeat=m):
                self.assertEqual(exact_roots(p,a,b,xs),
                                 frobenius_sector_roots(p,a,b,xs))

    def test_rational_two_torsion_is_necessary(self):
        self.assertEqual({3}, frobenius_sector_roots(5,0,1,(0,1,2)))
        self.assertEqual(set(), zero_only_roots(5,0,1,(0,1,2)))

    def test_leaf_circuit_and_tonelli(self):
        x,y,s=sp.symbols("x y s")
        for p,a,b in ((5,0,1),(7,1,1),(11,1,1),(19,2,3)):
            d=nonsquare(p)
            gate=leaf_constraints(x,y,s,a,b,d)
            self.assertLessEqual(max_generator_degree(gate),3)
            for t in range(p):
                select=int(classify(t,p,a,b)=="minus")
                scale=d if select else 1
                for v in ordinates(t,p,a,b,scale):
                    self.assertTrue(all(int(q.subs({x:t,y:v,s:select}))%p==0
                                        for q in gate))
                self.assertIsNotNone(sqrt_mod(f(t,p,a,b)*pow(scale,-1,p),p))

    def test_finite_addition_charts(self):
        X,Y,U,V,Z,W,L=sp.symbols("x y u v z w ell")
        for p,a,b in ((5,0,1),(7,1,1),(11,1,1)):
            for scale in (1,nonsquare(p)):
                generic=generic_add_chart(X,Y,U,V,Z,W,L,scale)
                doubling=generic_double_chart(X,Y,Z,W,L,scale,a)
                self.assertLessEqual(max_generator_degree(generic),2)
                self.assertLessEqual(max_generator_degree(doubling),2)
                points=[]
                for x in range(p):
                    if legendre(f(x,p,a,b)*pow(scale,-1,p),p)==-1:
                        continue
                    points.extend((x,y) for y in ordinates(x,p,a,b,scale))
                for P in points:
                    for Q in points:
                        R=add(P,Q,p,a,scale)
                        if R is None:
                            continue
                        if P[0]!=Q[0]:
                            lam=(Q[1]-P[1])*pow((Q[0]-P[0])%p,-1,p)%p
                            gate=generic
                            assignment={X:P[0],Y:P[1],U:Q[0],V:Q[1],
                                        Z:R[0],W:R[1],L:lam}
                        else:
                            lam=(3*P[0]**2+a)*pow(2*scale*P[1]%p,-1,p)%p
                            gate=doubling
                            assignment={X:P[0],Y:P[1],Z:R[0],W:R[1],L:lam}
                        self.assertTrue(all(int(q.subs(assignment))%p==0
                                            for q in gate),assignment)


if __name__=="__main__":
    unittest.main()
