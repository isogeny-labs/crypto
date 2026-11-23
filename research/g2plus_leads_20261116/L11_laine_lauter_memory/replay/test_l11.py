import unittest
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import l11
from l11 import *

_CACHE = {}


def curve80():
    if "c80" not in _CACHE:
        _CACHE["c80"] = gen_curve(8, 0)
    return _CACHE["c80"]


class T(unittest.TestCase):
    def test01_splitmix(self):
        s = SM(0)
        self.assertEqual(s.next(), 0xE220A8397B1DCDAF)
        self.assertEqual(s.next(), 0x6E789E6AA1B965F4)

    def test02_pairs(self):
        self.assertEqual([pair_of(t) for t in range(6)], [(1, 0), (2, 0), (2, 1), (3, 0), (3, 1), (3, 2)])
        b = 50
        got = {pair_of(t) for t in range(b * (b - 1) // 2)}
        self.assertEqual(len(got), b * (b - 1) // 2)
        self.assertEqual(got, {(i, j) for i in range(b) for j in range(i)})

    def test03_fermat31(self):
        cv = fermat(31)
        self.assertTrue(cv.is_smooth())
        pts = cv.points()
        self.assertEqual(len(pts), 32)
        self.assertEqual(pts[:4], [(1, 2, 7), (1, 2, 24), (1, 5, 6), (1, 5, 25)])
        fp = Fp(31)
        L = Lines(cv, fp)
        L.taylor(pts[0])
        self.assertEqual(fp.m, 82)
        for P in pts:
            L.taylor(P)

        def run(a, b):
            fp.m = 0
            fp.i = 0
            r = L.line(a, b)
            return r, fp.m, fp.i

        r, m, i = run((1, 2, 7), (1, 5, 25))
        self.assertEqual(r[0], "SPLIT")
        self.assertEqual({r[1], r[2]}, {(1, 9, 18), (1, 19, 16)})
        self.assertEqual((m, i), (32, 2))
        r, m, i = run((1, 2, 7), (1, 7, 2))
        self.assertEqual(r[0], "SPLIT")
        self.assertEqual({r[1], r[2]}, {(1, 18, 22), (1, 22, 18)})
        self.assertEqual((m, i), (32, 2))
        for q in ((1, 2, 24), (1, 5, 6)):
            r, m, i = run((1, 2, 7), q)
            self.assertEqual(r[0], "NOSPLIT")
            self.assertEqual((m, i), (19, 0))

    def test04_fermat263_and_N2(self):
        cv = fermat(263)
        self.assertTrue(cv.is_smooth())
        self.assertEqual(len(cv.points()), 264)
        self.assertEqual(n2_fermat(31), 1148)

    def test05_nodal_rejected(self):
        f = [0] * 15
        f[M4I[(0, 2, 2)]] = 1
        f[M4I[(4, 0, 0)]] = 262
        f[M4I[(2, 0, 2)]] = 262
        self.assertFalse(Curve(263, f).is_smooth())

    def _brute(self, cv):
        p = cv.p
        s = set()
        for x in range(p):
            for y in range(p):
                if cv.F_eval((x, y, 1)) == 0:
                    s.add(normalize((x, y, 1), p))
        for x in range(p):
            if cv.F_eval((x, 1, 0)) == 0:
                s.add(normalize((x, 1, 0), p))
        if cv.F_eval((1, 0, 0)) == 0:
            s.add((1, 0, 0))
        return sorted(s)

    def test06_enumeration(self):
        cv, att, sc = curve80()
        self.assertEqual(cv.points(), self._brute(cv))
        fc = fermat(263)
        self.assertEqual(fc.points(), self._brute(fc))

    def test07_line_vs_brute(self):
        cv, att, sc = curve80()
        p = cv.p
        pts = cv.points()
        ptset = set(pts)
        fp = Fp(p)
        L = Lines(cv, fp)
        sm = SM(1)
        nsplit = 0
        for _ in range(300):
            i = sm.next() % len(pts)
            j = sm.next() % len(pts)
            if i == j:
                continue
            Pi, Pj = pts[i], pts[j]
            kind, R1, R2 = L.line(Pi, Pj)
            coll = {q for q in pts if det_mod([list(Pi), list(Pj), list(q)], p) == 0}
            self.assertTrue({Pi, Pj} <= coll)
            if kind == "SPLIT":
                nsplit += 1
                self.assertEqual(coll - {Pi, Pj}, {R1, R2})
                self.assertEqual(len(coll), 4)
            elif kind == "NOSPLIT":
                self.assertEqual(len(coll), 2)
            else:
                self.assertIn(len(coll), (2, 3))
        self.assertGreater(nsplit, 50)

    def test08_dsu(self):
        def cnt(edges, nn):
            g = Graph()
            for _ in range(nn):
                g.new_node()
            for u, v in edges:
                g.add_edge(u, v)
            return g.cyc()
        self.assertEqual(cnt([(1, 0), (1, 0)], 1), 1)
        self.assertEqual(cnt([(1, 0), (2, 1), (2, 0)], 2), 1)
        self.assertEqual(cnt([(1, 2)], 2), 0)

    def test09_index(self):
        self.assertEqual(int_echelon([[2, 0], [0, 3]], 2), (2, 6))
        self.assertEqual(int_echelon([[1, 1], [1, -1]], 2), (2, 2))
        self.assertEqual(int_echelon([[1, 0], [1, 0]], 2)[1], None)

    def test10_af1_equals_u(self):
        cv, att, sc = curve80()
        a = run_cell(cv, "U", 8, 0, sc)
        b = run_cell(cv, "AF1", 8, 0, sc)
        for r in (a, b):
            pass
        a2 = dict(a)
        b2 = dict(b)
        for key in ("arm", "alpha"):
            a2.pop(key)
            b2.pop(key)
        self.assertEqual(a2, b2)
        self.assertEqual(a["g1_failures"], 0)

    def test11_g_admit(self):
        vid = {}
        self.assertFalse(g_admit("R", "R2", vid))
        vid["R"] = 1
        self.assertTrue(g_admit("R", "R2", vid))

    def test12_rank(self):
        self.assertEqual(rank_rows([{0: 1, 1: 1}, {0: 1, 1: -1}]), 2)
        self.assertEqual(rank_rows([{0: 1}, {0: 2}]), 1)


if __name__ == "__main__":
    unittest.main()
