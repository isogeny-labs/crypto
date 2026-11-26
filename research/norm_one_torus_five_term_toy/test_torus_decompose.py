import unittest
from itertools import product

from torus_decompose import (
    Curve, EliminationSolver, PairSumSolver, coset_factor_base, power, torus_parameter,
)


class DecompositionTests(unittest.TestCase):
    def test_coset_matches_exhaustive_enumeration(self):
        for curve, order, seed, d in [(Curve(43, 1, 0), 4, 1, 2),
                                       (Curve(1019, 2, 18), 30, 1, 2),
                                       (Curve(17, 2, 2), 3, 0, 3)]:
            p = curve.p
            eta = power(torus_parameter(seed, p, d), order, p, d)
            expected = {
                (x, y) for x, y in product(range(p), repeat=2)
                if y*y % p == curve.rhs(x)
                and power(torus_parameter(x, p, d), order, p, d) == eta
            }
            with self.subTest(p=p, order=order):
                self.assertEqual(set(coset_factor_base(curve, order, seed)), expected)

    def test_solvers_match_exhaustive_sumsets(self):
        for curve, order, seed in [(Curve(43, 1, 0), 4, 1), (Curve(31, 2, 7), 4, 2)]:
            p = curve.p
            base = coset_factor_base(curve, order, seed)
            self.assertTrue(base)
            all_points = [None] + [(x, y) for x, y in product(range(p), repeat=2)
                                   if y*y % p == curve.rhs(x)]
            expected = {None}
            for terms in range(1, 6):
                expected = {curve.add(point, factor) for point in expected for factor in base}
                if terms == 4:
                    four = expected
            for solver in (PairSumSolver(curve, base), PairSumSolver(curve, base, cache=True),
                           EliminationSolver(curve, base),
                           EliminationSolver(curve, base, block_size=2)):
                with self.subTest(p=p, solver=type(solver).__name__):
                    for target in all_points:
                        witnesses = solver.four_terms(target)
                        self.assertEqual(witnesses is not None, target in four)
                        if witnesses is not None:
                            left, right = witnesses
                            self.assertEqual(curve.sum((*left, *right)), target)
                        result = solver.solve(target)
                        self.assertEqual(result is not None, target in expected)
                        if result is not None:
                            self.assertTrue(all(point in base for point in result))
                            self.assertEqual(curve.sum(result), target)


if __name__ == "__main__":
    unittest.main()
