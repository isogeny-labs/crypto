import os
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from l05.rng import mix64
from l05.field import Counter, F, batch_inv, inv_table, GateFailure
from l05.instance import random_f, random_f_even, squarefree, roots, select_curve, Tables
from l05.models import sqrt_part, odd_transform, OddMap, top_part
from l05.poly import psub, pmul, IDENT, cantor, smul, point
from l05 import verify as V
from l05.zeta import lpoly, order_of_delta, find_modulus
from l05.ctx import build_ctx
from l05.pencil import run_arm, arm_spec, sieve_pencil, interpolate, AnchorStream, ARM_NAMES

P = 101


def ctx1():
    return build_ctx(101, 2, "root", 1, need_odd_tab=True)


class T(unittest.TestCase):
    def test01_mix(self):
        self.assertEqual(mix64(0), 16294208416658607535)
        self.assertEqual(mix64(1), 10451216379200822465)

    def test02_random_f(self):
        self.assertEqual(random_f(101, 2, 1), [31, 46, 14, 30, 51, 1])

    def test03_random_f_even(self):
        self.assertEqual(random_f_even(101, 2, 1), [75, 29, 79, 35, 2, 13, 1])
        self.assertEqual(random_f_even(101, 2, 2), [2, 75, 95, 19, 21, 78, 1])
        self.assertEqual(random_f_even(31, 2, 1), [26, 19, 4, 19, 18, 20, 1])
        self.assertEqual(random_f_even(37, 3, 1), [9, 14, 25, 14, 17, 24, 0, 25, 1])

    def test04_sqrt(self):
        f1 = random_f_even(101, 2, 1)
        self.assertTrue(squarefree(f1, 101))
        self.assertEqual(roots(f1, 101), [16, 68])
        F_ = F(101, Counter())
        s = sqrt_part(f1, 2, 101, F_)
        self.assertEqual(s, [41, 43, 57, 1])
        self.assertEqual(psub(f1, pmul(s, s, 101), 101), [10, 38, 20])
        f2 = random_f_even(101, 2, 2)
        self.assertEqual(roots(f2, 101), [28])
        self.assertEqual(sqrt_part(f2, 2, 101, F_), [20, 58, 39, 1])

    def test05_odd_transform(self):
        f1 = random_f_even(101, 2, 1)
        c, Fo, cg = odd_transform(f1, 16, 2, 101)
        self.assertEqual(c, [0, 17, 99, 21, 34, 8, 1])
        self.assertEqual(Fo, [95, 15, 29, 54, 99, 1])
        self.assertEqual(cg, 87)
        f2 = random_f_even(101, 2, 2)
        c, Fo, cg = odd_transform(f2, 28, 2, 101)
        self.assertEqual(c[1], 24)
        self.assertEqual(Fo, [92, 34, 13, 43, 65, 1])

    def test06_map(self):
        om = OddMap(random_f_even(101, 2, 1), 16, 2, 101)
        self.assertEqual(om.map_point((36, 71)), (16, 20))
        self.assertEqual(om.map_point((42, 99)), (90, 14))
        self.assertEqual(om.map_point((88, 12)), (83, 54))
        self.assertEqual(om.map_point("inf+"), (0, 87))
        self.assertEqual(om.map_point("inf-"), (0, 14))

    def test07_zeta(self):
        c = ctx1()
        Le, Ne = lpoly(c["f"], 101, 2, c["tab_even"].chi, 2)
        self.assertEqual(Ne, {1: 114, 2: 10366})
        self.assertEqual(Le, [1, 12, 154, 1212, 10201])
        self.assertEqual(sum(Le), 11580)
        Lo, No = lpoly(c["Fodd"], 101, 2, c["tab_odd"].chi, 1)
        self.assertEqual(No, {1: 114, 2: 10366})
        self.assertEqual(sum(Lo), 11580)
        c2 = build_ctx(101, 2, "root", 2, need_odd_tab=True)
        self.assertEqual(c2["f"], random_f_even(101, 2, 2))
        L2e, _ = lpoly(c2["f"], 101, 2, c2["tab_even"].chi, 2)
        L2o, _ = lpoly(c2["Fodd"], 101, 2, c2["tab_odd"].chi, 1)
        self.assertEqual(L2e, [1, -5, 123, -505, 10201])
        self.assertEqual(sum(L2e), 9815)
        self.assertEqual(sum(L2o), 9815)
        om = c["omap"]
        Wp = point(*om.map_point("inf+"), 101)
        Dd = cantor(Wp, Wp, om.Fodd, 2, 101)
        self.assertEqual(order_of_delta(Dd, 11580, om.Fodd, 2, 101), 2895)

    def test08_base(self):
        c = ctx1()
        self.assertEqual(c["tab_even"].n, 55)
        self.assertEqual(c["tab_even"].B[:5], [4, 5, 6, 8, 11])
        c2 = build_ctx(101, 2, "root", 2)
        self.assertEqual(c2["tab_even"].n, 47)

    def test09_Eg(self):
        c = ctx1()
        rec, pens = run_arm("E_g", c, 2, 101, 1, 1, 540, "full")
        self.assertEqual(rec["candidates"], 17)
        self.assertEqual(rec["relations"], 17)
        self.assertEqual(pens[0]["mul"], 12)
        self.assertEqual(pens[0]["inv"], 0)
        F_ = F(101, Counter())
        spec = arm_spec("E_g", 2)
        cands, d, k = sieve_pencil(spec, 2, 101, c["tab_even"], [], F_, None, top_part(c["s"], 2, 2))
        self.assertEqual(cands[0]["lam"], 3)
        self.assertEqual(cands[0]["residual"], [(36, 71), (42, 99), (88, 12)])
        self.assertEqual(cands[0]["a"], [3, 43, 57, 1])
        self.assertEqual((spec["ip"], spec["im"]), (0, -3))
        c2 = build_ctx(101, 2, "root", 2)
        cands2, _, _ = sieve_pencil(spec, 2, 101, c2["tab_even"], [], F(101, Counter()), None, top_part(c2["s"], 2, 2))
        self.assertEqual(len(cands2), 15)
        self.assertEqual(cands2[0]["lam"], 2)
        self.assertEqual(cands2[0]["residual"], [(4, 13), (31, 87), (91, 100)])

    def test10_v2_mutations(self):
        c = ctx1()
        om = c["omap"]
        pts = [(36, 71), (42, 99), (88, 12)]
        self.assertEqual(V.v2_class(om.Fodd, om, pts, 0, -3, 2, 101, True), IDENT)
        m1 = V.v2_class(om.Fodd, om, pts, 1, -4, 2, 101, True)
        self.assertEqual(m1, ([0, 0, 1], [87, 68]))
        pts2 = pts[:2] + [(88, 101 - 12)]
        m2 = V.v2_class(om.Fodd, om, pts2, 0, -3, 2, 101, True)
        self.assertEqual(m2, ([21, 36, 1], [93, 25]))

    def test11_vinf(self):
        f1 = random_f_even(101, 2, 1)
        S = V.S_series(f1, 2, 101)
        self.assertEqual(S[:7], [1, 57, 43, 41, 10, 55, 76])
        for a, o in [([3, 43, 57, 1], (0, -3)), ([41, 43, 57, 1], (1, -3)), ([0, 0, 1], (-3, -3)),
                     ([5, 0, 0, 1], (-2, -3)), ([7, 9, 57, 1], (-1, -3))]:
            self.assertEqual(V.vinf_orders(f1, a, 2, 101), o)

    def test12_inv_table(self):
        ctr = Counter()
        t = inv_table(101, F(101, ctr))
        self.assertTrue(all(i * t[i] % 101 == 1 for i in range(1, 101)))
        self.assertEqual(ctr.mul, 99)
        self.assertEqual(ctr.inv, 0)

    def test13_batch_interp(self):
        ctr = Counter()
        F_ = F(101, ctr)
        vals = [3, 7, 11, 50, 99]
        out = batch_inv(F_, vals)
        self.assertEqual((ctr.mul, ctr.inv), (12, 1))
        self.assertTrue(all(a * b % 101 == 1 for a, b in zip(vals, out)))
        from l05.poly import ev
        for j, em in ((2, 10), (3, 27)):
            ctr = Counter()
            F_ = F(101, ctr)
            anchors = [(4, 9), (10, 33), (21, 77)][:j]
            c, Pi = interpolate(F_, anchors, None, 2)
            self.assertEqual((ctr.mul, ctr.inv), (em, 1))
            for x, y in anchors:
                self.assertEqual(ev(c, x, 101), y)

    def test14_one_pencil_each(self):
        c = ctx1()
        for arm in ARM_NAMES:
            spec = arm_spec(arm, 2)
            even = spec["model"] == "even"
            tab = c["tab_even"] if even else c["tab_odd"]
            t = top_part(c["s"], spec["e"], 2) if spec["e"] is not None else None
            F_ = F(101, Counter())
            invt = inv_table(101, F_) if spec["inv"] == "table" else None
            anchors = AnchorStream(spec, 2, 101, tab, 1, 1).next()
            cands, d, k = sieve_pencil(spec, 2, 101, tab, anchors, F_, invt, t)
            om = c["omap"]
            for cd in cands:
                pts = cd["anchors"] + cd["residual"]
                V.v1(tab.h, cd["a"], pts, spec["ip"], spec["im"], 2, 101, even)
                if even:
                    V.vinf(c["f"], cd["a"], spec["ip"], spec["im"], 2, 101)
                V.v2(om.Fodd, om, pts, spec["ip"], spec["im"], 2, 101, even)
                if even:
                    self.assertEqual(V.mutations(om.Fodd, om, cd["anchors"], cd["residual"], spec["ip"], spec["im"], 2, 101),
                                     (False, False))

    def test15_twins(self):
        c = ctx1()
        for a, b in (("E_0", "E_0_T"), ("E_1a", "E_1a_T"), ("O", "O_T")):
            r1, _ = run_arm(a, c, 2, 101, 1, 1, 540, "none")
            r2, _ = run_arm(b, c, 2, 101, 1, 1, 540, "none")
            self.assertEqual(r1["keys_sha256"], r2["keys_sha256"])
            self.assertEqual(r1["relations"], r2["relations"])

    def test16_g3(self):
        f = random_f_even(37, 3, 1)
        self.assertIsInstance(squarefree(f, 37), bool)
        chi37 = [0] + [1 if pow(a, 18, 37) == 1 else -1 for a in range(1, 37)]
        chi53 = [0] + [1 if pow(a, 26, 53) == 1 else -1 for a in range(1, 53)]
        self.assertIsNotNone(find_modulus(37, 3, chi37))
        self.assertIsNotNone(find_modulus(53, 3, chi53))
        sel = select_curve(37, 3, "root", 1)
        if sel is not None:
            c = build_ctx(37, 3, "root", 1, need_odd_tab=True)
            Le, _ = lpoly(c["f"], 37, 3, c["tab_even"].chi, 2)
            Lo, _ = lpoly(c["Fodd"], 37, 3, c["tab_odd"].chi, 1)
            self.assertEqual(sum(Le), sum(Lo))


if __name__ == "__main__":
    unittest.main()
