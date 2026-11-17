import math
import os
import sys
import unittest
from fractions import Fraction

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from genus_table import genus_row
from split_law import splitfrac, eval_poly, mc_count, MC_TRIALS, MC_SEEDS


class T(unittest.TestCase):
    def test_g4(self):
        r = genus_row(4)
        self.assertEqual((r["h"], r["genus_D"]), (5, 9))
        self.assertEqual(r["prym_2lp_exp"], Fraction(16, 9))
        self.assertEqual(r["hyper_g_2lp_exp"], Fraction(3, 2))
        self.assertEqual(r["rho_prym_exp"], Fraction(2))
        self.assertEqual(r["prym_beats_hyper"], "false")

    def test_g5(self):
        r = genus_row(5)
        self.assertEqual((r["h"], r["genus_D"]), (6, 11))
        self.assertEqual(r["prym_2lp_exp"], Fraction(20, 11))
        self.assertEqual(r["hyper_g_2lp_exp"], Fraction(8, 5))
        self.assertEqual(r["rho_prym_exp"], Fraction(5, 2))
        self.assertEqual(r["prym_beats_hyper"], "false")
        self.assertEqual(r["dominant"], "true")

    def test_g6(self):
        self.assertEqual(genus_row(6)["dominant"], "false")

    def test_pos(self):
        self.assertEqual(splitfrac(101, 1), Fraction(1, 1))

    def test_d2(self):
        self.assertEqual(splitfrac(101, 2), Fraction(5050, 10201))
        self.assertEqual(splitfrac(101, 2), Fraction(50, 101))
        self.assertEqual(splitfrac(101, 2), Fraction(101 - 1, 2 * 101))

    def test_monotone(self):
        v = [splitfrac(1009, d) for d in range(1, 12)]
        for a, b in zip(v, v[1:]):
            self.assertGreater(a, b)

    def test_eval_poly(self):
        # x^2 - 1 mod 7: coeffs [c0, c1] = [-1 mod 7, 0]
        roots = [x for x in range(7) if eval_poly([6, 0], x, 7) == 0]
        self.assertEqual(roots, [1, 6])
        # x^2 + 1 at x=10 mod 101 -> 0
        self.assertEqual(eval_poly([1, 0], 10, 101), 0)

    def test_mc(self):
        T_ = 20000  # smaller than frozen MC_TRIALS for unit-test speed; same sampler
        for s in MC_SEEDS:
            self.assertEqual(mc_count(101, 1, s, T_), T_)
        p2 = float(splitfrac(101, 2))
        fr = [mc_count(101, 2, s, T_) / T_ for s in MC_SEEDS]
        tol = 4 * math.sqrt(p2 * (1 - p2) / T_)
        for f in fr:
            self.assertLess(abs(f - p2), tol)
        self.assertLess(abs(fr[0] - fr[1]), tol * 2)

    def test_determinism(self):
        self.assertEqual(mc_count(101, 3, 20261116, 5000), mc_count(101, 3, 20261116, 5000))


if __name__ == "__main__":
    unittest.main()
