import sys, os, random, itertools, unittest
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from fp import Ops, Fp
from curve import Curve, setup_points, eadd, eneg, emul, free_field
import counts
from arms import column_map
from section import section, verify_relation, make_row
import la


def toy():
    cv = Curve(23, 1, 2, 0, 3, 5)
    ops = Ops()
    F = Fp(23, ops)
    S = setup_points(F, cv)
    return cv, F, ops, S


def brute_p2(cv):
    """Brute force #C and #E over F_{p^2} (small p)."""
    p = cv.p
    T = counts.sqrt_table(p)
    d = counts.nonresidue(p, T)
    els = [(a, b) for a in range(p) for b in range(p)]

    def mul(u, v):
        return ((u[0] * v[0] + d * u[1] * v[1]) % p, (u[0] * v[1] + u[1] * v[0]) % p)

    def add(u, v):
        return ((u[0] + v[0]) % p, (u[1] + v[1]) % p)
    nE = nC = 0
    sqcount = {}
    for y in els:
        sqcount[mul(y, y)] = sqcount.get(mul(y, y), 0) + 1
    for x in els:
        hx = add(mul((cv.a1, 0), x), (cv.a3, 0))
        x2 = mul(x, x)
        x3 = mul(x2, x)
        fx = add(add(add(x3, mul((cv.a2, 0), x2)), mul((cv.a4, 0), x)), (cv.a6, 0))
        for w in els:
            if add(add(mul(w, w), mul(hx, w)), ((-fx[0]) % p, (-fx[1]) % p)) == (0, 0):
                nE += 1
                nC += sqcount.get(w, 0)
    return nE + 1, nC + 1


class T1(unittest.TestCase):
    def test_invariants(self):
        cv = toy()[0]
        self.assertEqual((cv.disc_f, cv.DeltaE, cv.c4, cv.c6), (22, 7, 16, 2))

    def test_setup(self):
        cv, F, ops, S = toy()
        exp = [(5, 6), (5, 7), (5, 16), (5, 17), (6, 1), (6, 22), (7, 3), (7, 20), (10, 0), (14, 10), (14, 13)]
        self.assertEqual([(q[0], q[1]) for q in S.points], exp)
        self.assertEqual((S.N_aff, S.N_ram, S.nE), (11, 1, 18))
        self.assertEqual((ops.mul, ops.inv), (120, 1))

    def test_counts(self):
        cv = toy()[0]
        c = counts.full_counts(cv)
        self.assertEqual((c["nC1"], c["nE1"], c["nC2"], c["nE2"]), (12, 18, 564, 540))
        self.assertEqual((c["s1"], c["T2"], c["s2"], c["nP"]), (6, -24, 30, 416))
        for (p, a1, a3, a2, a4, a6) in [(23, 1, 2, 0, 3, 5), (7, 1, 1, 0, 1, 3), (11, 0, 1, 2, 3, 4)]:
            cv = Curve(p, a1, a3, a2, a4, a6)
            if cv.disc_f == 0 or cv.DeltaE == 0:
                continue
            nE, nC = brute_p2(cv)
            self.assertEqual((nE, nC), (counts.count_Fp2(cv)[0], counts.count_Fp2(cv)[1]), (p,))
            # F_p
            n1 = sum(1 for x in range(p) for y in range(p)
                     if (pow(y, 4, p) + cv.h(x) * y * y - cv.f(x)) % p == 0) + 1
            self.assertEqual(counts.count_Fp(cv)[1], n1)

    def test_columns(self):
        cv, F, ops, S = toy()
        for arm, n in (("A", 11), ("B", 6), ("C", 5)):
            col, sign, reps = column_map(arm, S.points, cv, 1)
            self.assertEqual(len(reps), n)
        col, sign, reps = column_map("B", S.points, cv, 1)
        self.assertEqual(col, [0, 1, 1, 0, 2, 2, 3, 3, 4, 5, 5])
        col, sign, reps = column_map("C", S.points, cv, 1)
        self.assertEqual(col, [0, 0, 0, 0, 1, 1, 2, 2, 3, 4, 4])
        self.assertEqual(sign, [-1, 1, 1, -1, 1, 1, -1, -1, 1, 1, 1])
        self.assertEqual(reps, [(5, 3), (6, 1), (7, 5), (10, 0), (14, 8)])

    def sec(self, a, b):
        cv, F, ops, S = toy()
        Fc = Fp(23, Ops())
        Fc.SQRT = F.SQRT
        i1 = S.index[a]
        i2 = S.index[b]
        st, terms = section(Fc, cv, S.points, S.index, i1, i2)
        return cv, S, Fc, i1, i2, st, terms

    def test_section_split(self):
        cv, S, Fc, i1, i2, st, terms = self.sec((5, 7), (6, 1))
        self.assertEqual(st, "split")
        self.assertEqual({(S.points[k][0], S.points[k][1]): v for k, v in terms.items()},
                         {(5, 7): 1, (6, 1): 2, (10, 0): 1})
        self.assertEqual((Fc.ops.mul, Fc.ops.inv), (34, 2))
        v = verify_relation(cv, S.points, i1, i2, terms)
        self.assertTrue(v["G2"] and v["G3"])
        rows = {}
        for arm in "ABC":
            col, sign, _ = column_map(arm, S.points, cv, 1)
            rows[arm] = make_row(terms, col, sign)
        self.assertEqual(rows["A"], ((1, 1), (4, 2), (8, 1)))
        self.assertEqual(rows["B"], ((1, 1), (2, 2), (4, 1)))
        self.assertEqual(rows["C"], ((0, 1), (1, 2), (3, 1)))
        # tamper
        nt = dict(terms)
        nt.pop(S.index[(10, 0)])
        nt[S.index[(14, 10)]] = 1
        v = verify_relation(cv, S.points, i1, i2, nt)
        self.assertFalse(v["G2"])

    def test_section_nonsplit(self):
        cv, S, Fc, i1, i2, st, terms = self.sec((5, 6), (6, 1))
        self.assertEqual(st, "nonsplit")
        self.assertEqual((Fc.ops.mul, Fc.ops.inv), (30, 1))

    def test_section_V(self):
        cv, S, Fc, i1, i2, st, terms = self.sec((5, 6), (5, 7))
        self.assertEqual(st, "split")
        self.assertEqual((Fc.ops.mul, Fc.ops.inv), (14, 1))
        rows = {}
        for arm in "ABC":
            col, sign, _ = column_map(arm, S.points, cv, 1)
            rows[arm] = make_row(terms, col, sign)
        self.assertEqual(rows["A"], ((0, 1), (1, 1), (2, 1), (3, 1)))
        self.assertEqual(rows["B"], ((0, 2), (1, 2)))
        self.assertEqual(rows["C"], ())

    def test_eadd(self):
        cv, F, ops, S = toy()
        FF = Fp(23, Ops())
        P = eadd(FF, cv, (6, 1), (6, 1))
        self.assertEqual(P, (5, 3))
        self.assertEqual((FF.ops.mul, FF.ops.inv), (10, 1))
        self.assertIsNone(eadd(FF, cv, (5, 3), eneg(FF, cv, (5, 3))))
        FF = free_field(23)
        pts = {(q[0], q[2]) for q in S.points}
        for P in pts:
            self.assertIsNone(eadd(FF, cv, P, eneg(FF, cv, P)))
            self.assertIsNone(emul(FF, cv, P, 18))

    def test_pushforward_all_pairs(self):
        cv, F, ops, S = toy()
        Fc = Fp(23, Ops())
        Fc.SQRT = F.SQRT
        nsplit_L = 0
        for i1, i2 in itertools.combinations(range(S.N_aff), 2):
            st, terms = section(Fc, cv, S.points, S.index, i1, i2)
            if st == "split":
                v = verify_relation(cv, S.points, i1, i2, terms)
                self.assertTrue(v["G2"] and v["G3"], (i1, i2))
                if S.points[i1][0] != S.points[i2][0]:
                    nsplit_L += 1
        self.assertEqual(nsplit_L, 6)

    def test_la(self):
        r = 101
        rng = random.Random(4242)
        while True:
            v = [1] + [rng.randrange(1, 101) for _ in range(24)]
            rows = []
            while len(rows) < 40:
                cols = rng.sample(range(25), 4)
                co = [rng.randrange(1, 101) for _ in range(3)]
                s = sum(c * v[k] for c, k in zip(co, cols[:3])) % r
                last = (-s) * pow(v[cols[3]], r - 2, r) % r
                if last == 0:
                    continue
                co.append(last)
                rows.append(tuple(sorted(zip(cols, co))))
            if len(set(k for row in rows for k, _ in row)) == 25:
                break
        o1, o2 = Ops(), Ops()
        x, a = la.solve_fixed(rows, 25, 0, r, 99, o1)
        self.assertEqual(x, v[1:])
        x2, a2 = la.solve_fixed(rows, 25, 0, r, 99, o2)
        self.assertEqual(o1.as_dict(), o2.as_dict())
        self.assertEqual(a, a2)

    def test_bm(self):
        s = [pow(2, n, 101) for n in range(10)]
        C, L = la.bm(s, 101, Ops())
        self.assertEqual(L, 1)
        self.assertEqual([C[L - k] for k in range(L + 1)], [(-2) % 101, 1])

    def test_stream(self):
        from ic_run import stream_pairs
        a = stream_pairs(12345, 11, 20)
        b = stream_pairs(12345, 11, 20)
        self.assertEqual(a, b)
        for t, (i, j) in enumerate(a):
            self.assertNotEqual(i, j)
            self.assertEqual(i, t % 11)

    def test_end_to_end(self):
        import select_instances as si
        insts, _ = si.find("F1", 263, 1)
        inst = insts[0]
        import ic_run
        res = {arm: ic_run.run(inst, 1, arm) for arm in "ABCN"}
        for arm in "ABC":
            self.assertEqual(res[arm]["status"], "ok", arm)
            self.assertTrue(res[arm]["certified"], arm)
        self.assertEqual(len({res[a]["stream_digest"] for a in "ABCN"}), 1)
        self.assertEqual(res["A"]["sigma_equal"], 1.0)
        self.assertTrue((not res["N"]["certified"]) or res["N"]["status"] == "la_fail")
        self.assertTrue(res["A"]["tamper_rejected"])


if __name__ == "__main__":
    unittest.main()
