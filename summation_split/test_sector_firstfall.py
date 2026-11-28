"""Deterministic negative control: artificial first fall in sector circuits.

The cancellation is a presentation artifact, not a bound on F4/Macaulay degree.
"""
import unittest
import sympy as sp
from .sector_complete import circuit

def leaf_split_certificate(system, i):
    """Identify two *actual* generator polynomials with degree-two fall."""
    x, y, s = system["leaves"][i]
    pos = system["plus"][i]
    neg = system["minus"][i]
    equations = system["equations"]
    gx = sp.expand(pos.x-(1-s)*x)
    hx = sp.expand(neg.x-s*x)
    gy = sp.expand(pos.y-(1-s)*y)
    hy = sp.expand(neg.y-s*y)
    expressions = {sp.expand(z) for z in equations}
    for candidate in (gx, hx, gy, hy):
        if candidate not in expressions:
            raise AssertionError(f"missing generator {candidate}")
    return ((gx, hx, sp.expand(pos.x+neg.x-x)),
            (gy, hy, sp.expand(pos.y+neg.y-y)))

class FirstFallArtifactTest(unittest.TestCase):
    def test_degree_two_first_fall_for_every_leaf(self):
        for p, a, b, m in ((5,0,1,3), (7,1,1,3),
                            (11,1,1,4), (2**31-1,1,1,5)):
            for field_lift in (False, True):
                system=circuit(p,a,b,m,field_lift=field_lift)
                self.assertEqual(system["degree"],4)
                for i in range(m+1):
                    for f, g, h in leaf_split_certificate(system, i):
                        # Coefficient 1 for each source generator: total
                        # combination degree 2, nonzero output degree 1.
                        self.assertEqual(sp.Poly(f).total_degree(),2)
                        self.assertEqual(sp.Poly(g).total_degree(),2)
                        self.assertEqual(sp.expand(f+g-h),0)
                        self.assertEqual(sp.Poly(h).total_degree(),1)
                        self.assertNotEqual(h,0)

if __name__ == "__main__":
    unittest.main()
