"""Independent algebraic and exhaustive small-subgroup checks."""
import unittest
from sage.all import PolynomialRing, factor
from common import seed_all, bsgs_ec, min_field_degree
from quadratic_cover import make_cover
from cubic_cover import make_cubic_cover
from trace_zero import choose_trace_instance


class AlgebraicChecks(unittest.TestCase):
    def setUp(self):
        seed_all(20261024)

    def test_quadratic_map_identity_as_rational_functions(self):
        d=make_cover(11)
        K,E,s,L=d['K'],d['E'],d['s'],d['L']
        R=PolynomialRing(K,'x'); x=R.gen(); F=R.fraction_field()
        f=R([K(c) for c in d['sextic'].list()])
        X=F(L*x)/(x+s)**2
        Y2=F(L*L*f)/(x+s)**6
        self.assertEqual(Y2,X**3+E.a2()*X*X+E.a4()*X+E.a6())

    def test_quadratic_transfer_on_entire_small_subgroup(self):
        d=make_cover(11); E=d['E']; phi=d['transfer']
        N=int(E.cardinality()); r=int(max(factor(N))[0])
        while True:
            P=(N//r)*E.random_point()
            if not P.is_zero() and not phi(P).is_zero(): break
        S=phi(P)
        for k in range(r):
            self.assertEqual(phi(k*P),k*S)
        self.assertEqual(r*S,0*S)

    def test_cubic_transfer_on_entire_small_subgroup(self):
        d=make_cubic_cover(5); E=d['E']; phi=d['transfer']; r=31
        while True:
            P=4*E.random_point()
            if not P.is_zero(): break
        S=phi(P)
        self.assertFalse(S.is_zero())
        for k in range(r):
            self.assertEqual(phi(k*P),k*S)

    def test_bsgs_zero_and_upper_boundary(self):
        E0,E,P,r,torder=choose_trace_instance(7,3)
        self.assertEqual(bsgs_ec(P,E(0),r)['scalar'],0)
        self.assertEqual(bsgs_ec(P,(r-1)*P,r)['scalar'],r-1)
        with self.assertRaises(ValueError): bsgs_ec(P,P,1)

    def test_ambient_degree_can_overstate_point_field(self):
        E0,E,P,r,torder=choose_trace_instance(5,6)
        self.assertEqual(r,37)
        self.assertEqual(min_field_degree(P,5,6),3)

    def test_noncyclic_primary_projection_regression(self):
        from towers import tower_controls
        # The initial N/r projection annihilated every candidate here.
        record=tower_controls(13,6,2)
        self.assertTrue(record['extension_trace_is_zero'])
        self.assertTrue(record['base_subgroup_survives'])


if __name__=='__main__': unittest.main(verbosity=2)
