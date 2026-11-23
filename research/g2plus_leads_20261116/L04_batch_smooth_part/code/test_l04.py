import os, random, subprocess, sys, unittest, json, tempfile
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import l04
from l04 import Ops
import run_cell


class T(unittest.TestCase):
    def setUp(self):
        self.chi = l04.chi_table(7, Ops())

    def test1(self):
        o = Ops()
        self.assertEqual(l04.chi_table(7, o), [0, 1, 1, -1, 1, -1, -1])
        self.assertEqual(o.mul, 3)

    def test2(self):
        o = Ops()
        self.assertEqual(l04.poly_mul([1, 1], [2, 1], 7, o), [2, 3, 1])
        self.assertEqual(o.mul, 4)

    def test3(self):
        o = Ops()
        self.assertEqual(l04.build_FS([1, 2], 7, o), [2, 4, 1])
        self.assertEqual(o.mul, 3)

    def test4(self):
        for u, v, m in [([1, 4, 1, 1], 1, 17), ([5, 0, 0, 1], 1, 12), ([3, 4, 6, 1], -1, 17),
                        ([4, 0, 2, 1], 0, 17), ([2, 4, 1], 1, 7)]:
            o = Ops()
            self.assertEqual(l04.disc_character(u, self.chi, 7, o), v, u)
            self.assertEqual(o.mul, m, u)

    def test5(self):
        o = Ops()
        self.assertTrue(l04.splits_completely([1, 4, 1, 1], 7, o))
        self.assertEqual(o.mul, 36)
        o = Ops()
        self.assertFalse(l04.splits_completely([5, 0, 0, 1], 7, o))
        self.assertEqual(o.mul, 36)

    def test6(self):
        o = Ops(); l04.splits_completely([1, 2, 3, 1], 65521, o); self.assertEqual(o.mul, 261)
        o = Ops(); l04.splits_completely([5, 7, 1], 65521, o); self.assertEqual(o.mul, 114)

    def test7(self):
        o = Ops(); l04.modu_mul([1, 2, 3], [3, 2, 1], [1, 2, 3], 3, 7, o); self.assertEqual(o.mul, 15)
        o = Ops(); l04.modu_mul([1, 2], [3, 2], [1, 2], 2, 7, o); self.assertEqual(o.mul, 6)

    def test8(self):
        o = Ops()
        r = l04.vrem([0] * 10 + [1], [1, 4, 1, 1], 7, o)
        self.assertEqual(l04.peval(r, 1, 7), 1)
        self.assertEqual(l04.peval(r, 2, 7), 2)
        self.assertEqual(l04.peval(r, 3, 7), 4)
        self.assertLessEqual(len(r), 3)
        self.assertEqual(o.inv, 0)
        self.assertEqual(o.mul % 3, 0)
        self.assertLessEqual(o.mul, 24)

    def test9(self):
        inS = bytearray(7); inS[1] = inS[2] = 1
        o = Ops()
        R = l04.vrem([2, 4, 1], [1, 4, 1, 1], 7, o)
        self.assertEqual(R, [2, 4, 1]); self.assertEqual(o.mul, 0)
        o = Ops()
        ok, G, c = l04.post_S([1, 4, 1, 1], R, 3, self.chi, inS, 7, o)
        self.assertTrue(ok); self.assertEqual(G, [2, 4, 1]); self.assertEqual(c, [4, 1])
        self.assertEqual(o.mul, 8); self.assertEqual(o.inv, 0)

    def test10(self):
        inS = bytearray(7); inS[1] = inS[2] = 1
        u = [4, 0, 2, 1]
        R = l04.vrem([2, 4, 1], u, 7, Ops())
        self.assertEqual(R, [2, 4, 1])
        self.assertEqual(l04.vrem(u, R, 7, Ops()), [1, 6])
        ok, G, c = l04.post_S(u, R, 3, self.chi, inS, 7, Ops())
        self.assertFalse(ok); self.assertEqual(G, [6, 1]); self.assertEqual(c, [3, 3, 1])
        self.assertEqual((c[1] * c[1] - 4 * c[0]) % 7, 4)
        self.assertEqual(len(l04.vgcd_monic(c, G, 7, Ops())) - 1, 1)

    def test11(self):
        r = l04.roots_of([1, 4, 1, 1], 7, random.Random(1), Ops())
        self.assertEqual(sorted(r), [1, 2, 3])

    def test12(self):
        o = Ops()
        self.assertEqual(l04.vgcd_monic([2, 4, 1], [1, 6], 7, o), [6, 1])
        self.assertEqual(o.inv, 2)

    def _cell(self, g):
        p, T, seed = 1009, 200, 99
        setup = Ops()
        chi = l04.chi_table(p, setup)
        f = l04.make_f(p, g, seed)
        N = l04.compute_N(p, g)
        S, inS = l04.make_S(p, g, f, chi, N, setup)
        FS = l04.build_FS(S, p, setup)
        rng = random.Random(seed + 50)
        us = [[rng.randrange(p) for _ in range(g)] + [1] for _ in range(T)]
        pu, exp = run_cell.make_planted(g, p, S, inS, chi, seed + 90)
        us += pu
        ctx = l04.Ctx(g, p, S, inS, FS, chi)
        orc = [run_cell.oracle(u, g, p, inS) for u in us]
        rs = {"A_pf": l04.run_arm_A(ctx, us, True, seed + 70),
              "A_nopf": l04.run_arm_A(ctx, us, False, seed + 70),
              "B": l04.run_arm_B(ctx, us, False, seed + 70),
              "B_pf": l04.run_arm_B(ctx, us, True, seed + 70),
              "C_m4": l04.run_arm_C(ctx, us, 4, False, seed + 70),
              "Cpf_m4": l04.run_arm_C(ctx, us, 4, True, seed + 70),
              "C_m1": l04.run_arm_C(ctx, us, 1, False, seed + 70)}
        for k, r in rs.items():
            self.assertEqual(r["dec_bits"], orc, k)
            self.assertEqual(r["dec_bits"][T:], exp, k)
            self.assertEqual(r["cz_fail"], 0, k)
        self.assertEqual(rs["C_m1"]["dec"].mul, rs["B"]["dec"].mul)
        self.assertEqual(rs["C_m1"]["dec"].inv, rs["B"]["dec"].inv)
        self.assertEqual(rs["C_m1"]["tprod"].mul, 0)
        self.assertEqual(rs["C_m1"]["trem"].mul, rs["B"]["dec"].mul - rs["C_m1"]["post"].mul)
        self.assertGreater(sum(orc), 0)

    def test13(self):
        self._cell(3)

    def test14(self):
        self._cell(2)

    def test15(self):
        self._cell(3)


if __name__ == "__main__":
    unittest.main()
