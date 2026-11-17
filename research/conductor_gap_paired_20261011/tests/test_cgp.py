"""Unit tests for the paired conductor-gap smoke test.

Run:  python3 -m unittest discover -s research/conductor_gap_paired_20261011/tests -v
(or `python3 -m pytest` if pytest is installed).  Pure Python + numpy.

Covers the three items the brief requires -- point counting against a naive count, the
f_E certificate on a hand-checked example, relation verification -- plus the regressions
that protect the selection stage (class-count identity, S_3, the Groebner solver, the
isomorphic-model control, the statistics).
"""
import itertools
import os
import random
import sys
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))

from cgp_field import Fq, factor_squarefree, pdeg, peval, pmul, pderiv, pgcd, pmonic, proots, ptrim  # noqa: E402
from cgp_curve import (  # noqa: E402
    Counter, Curve, DivPolys, class_number, disc_decomposition, enumerate_classes, frobenius_x,
    j_invariant, kronecker_class_number, naive_count, rational_kernels, velu_image,
)
from cgp_certify import certify_curve  # noqa: E402
from cgp_engine import (  # noqa: E402
    Buchberger, Echelon, Instance, decompose_exhaustive, decompose_weil_gb, relation_row,
    s3_value, solve_zero_dim, verify_relation, weil_restrict,
)
from cgp_stats import bootstrap_ci, holm, sign_flip_pvalue  # noqa: E402


def random_curve(F, rng):
    while True:
        try:
            return Curve(F, rng.randrange(F.q), rng.randrange(F.q))
        except ValueError:
            pass


class TestField(unittest.TestCase):
    def test_laws_and_log_tables_against_slow_multiplication(self):
        rng = random.Random(1)
        for p in (5, 7, 11):
            F = Fq(p)
            for _ in range(500):
                x, y, z = (rng.randrange(F.q) for _ in range(3))
                self.assertEqual(F.mul(x, y), F.mul_slow(x, y))
                self.assertEqual(F.mul(x, F.add(y, z)), F.add(F.mul(x, y), F.mul(x, z)))
                if x:
                    self.assertEqual(F.mul(x, F.inv(x)), 1)
            self.assertEqual(sum(1 for x in range(1, F.q) if F.is_square(x)), (F.q - 1) // 2)

    def test_factorisation_recombines(self):
        rng = random.Random(2)
        F = Fq(7)
        n = 0
        while n < 15:
            f = pmonic(F, ptrim([rng.randrange(F.q) for _ in range(rng.randrange(2, 9))] + [1]))
            if pdeg(pgcd(F, f, pderiv(F, f))) > 0:
                continue
            prod = [1]
            for g, d in factor_squarefree(F, f, rng):
                self.assertEqual(pdeg(g), d)
                prod = pmul(F, prod, g)
            self.assertEqual(prod, f)
            n += 1


class TestPointCounting(unittest.TestCase):
    def test_fast_count_equals_naive_count_and_enumeration(self):
        rng = random.Random(3)
        for p in (5, 7):
            F = Fq(p)
            C = Counter(F)
            for _ in range(10):
                E = random_curve(F, rng)
                n_naive = naive_count(F, E.a, E.b)  # all (x, y) pairs, slow arithmetic
                self.assertEqual(C.count(E.a, E.b), n_naive)
                self.assertEqual(len(E.points()), n_naive)

    def test_group_order_annihilates_points_and_hasse(self):
        rng = random.Random(4)
        F = Fq(11)
        C = Counter(F)
        for _ in range(6):
            E = random_curve(F, rng)
            N = C.count(E.a, E.b)
            self.assertLessEqual(abs(F.q + 1 - N), 2 * F.p)
            for P in rng.sample([P for P in E.points() if P], 5):
                self.assertIsNone(E.mul(N, P))

    def test_class_numbers_known_values(self):
        self.assertEqual([class_number(d) for d in (-3, -4, -23, -75, -195)], [1, 1, 3, 2, 4])

    def test_enumeration_class_counts_equal_kronecker_class_numbers(self):
        """Regression: an earlier enumeration used b = 2 j (1728-j)^2 * (1728-j) and was caught
        by exactly this identity (number of classes with trace t = H(t^2 - 4q))."""
        for p in (5, 7, 11, 13):
            F = Fq(p)
            C = Counter(F)
            q = F.q
            classes = enumerate_classes(F)  # also asserts every model has the j it is labelled with
            self.assertEqual(len(classes), 2 * q + 6)
            by = {}
            for (j, w, a, b) in classes:
                by.setdefault(q + 1 - C.count(a, b), []).append((j, w))
            for t, v in by.items():
                if t % p == 0:
                    continue
                _, f, dk = disc_decomposition(t, q)
                self.assertEqual(len(v), sum(kronecker_class_number(f, dk).values()), (p, t))


class TestConductorCertificate(unittest.TestCase):
    """Hand-checked example: q = 7^2, t = 11.  t^2 - 4q = -75 = 5^2 * (-3), so f_pi = 5 is
    prime and f_E is 1 or 5, decided by the 5-volcano floor test.  Class numbers:
    h(-3) = 1 (End = Z[zeta_3], the crater, j = 0) and h(-75) = 2 (the floor), so exactly
    three classes: one with 5 + 1 = 6 rational 5-isogenies, two with exactly one."""

    @classmethod
    def setUpClass(cls):
        cls.F = Fq(7)
        cls.C = Counter(cls.F)
        cls.t, cls.N = 11, 49 + 1 - 11
        cls.curves = [c for c in enumerate_classes(cls.F) if cls.C.count(c[2], c[3]) == cls.N]

    def test_discriminant_and_prediction(self):
        self.assertEqual(disc_decomposition(self.t, 49), (-75, 5, -3))
        self.assertEqual(self.N, 39)
        self.assertEqual(kronecker_class_number(5, -3), {1: 1, 5: 2})
        self.assertEqual(frobenius_x(11, 5, -3), 13)
        self.assertEqual(len(self.curves), 3)

    def test_kernel_counts_decide_floor_and_crater(self):
        rng = random.Random(5)
        counts = {}
        for (j, w, a, b) in self.curves:
            ks, _ = rational_kernels(self.F, a, b, 5, rng)
            counts[j] = len(ks)
            for h in ks:  # every Velu image is isogenous: same group order
                self.assertEqual(self.C.count(*velu_image(self.F, a, b, h, 5)), self.N)
        self.assertEqual(sorted(counts.values()), [1, 1, 6])
        self.assertEqual(counts[0], 6)  # j = 0: the crater, End = Z[zeta_3]

    def test_floor_edges_lead_back_to_the_crater(self):
        rng = random.Random(6)
        for (j, w, a, b) in self.curves:
            if j == 0:
                continue
            (h,) = rational_kernels(self.F, a, b, 5, rng)[0]  # the unique rational kernel
            a2, b2 = velu_image(self.F, a, b, h, 5)
            self.assertEqual(j_invariant(self.F, a2, b2), 0)

    def test_full_certificate_both_methods(self):
        out = {}
        for (j, w, a, b) in self.curves:
            rec = certify_curve(self.F, self.C, a, b, self.t, self.N, 5, -3)
            self.assertEqual(rec["status"], "ok", rec.get("reason"))
            out[j] = rec["fE"]
            # method B must not contradict method A (here n1 = 1 for both signs, so it is not decisive)
            self.assertIn(rec["v_fE"][5], rec["method_B"]["per_prime"][5]["consistent_v_fE"])
        self.assertEqual(sorted(out.values()), [1, 5, 5])
        self.assertEqual(out[0], 1)

    def test_division_polynomial_vanishes_on_real_torsion(self):
        F = Fq(7)
        rng = random.Random(7)
        for _ in range(8):
            E = random_curve(F, rng)
            dp = DivPolys(F, E.a, E.b)
            N = Counter(F).count(E.a, E.b)
            for ell in (3, 5, 7):
                if N % ell:
                    continue
                for P in E.points():
                    if P and E.mul(ell, P) is None:
                        self.assertEqual(peval(F, dp.psi_x(ell), P[0]), 0)

    def test_certified_distribution_equals_class_numbers_whole_fields(self):
        """Volcano-certified f_E over every supported group of F_{11^2} and F_{13^2} equals the
        class-number prediction {f : h(f^2 D_K)} (depths up to 4 for l = 2 occur here)."""
        for p in (11, 13):
            F = Fq(p)
            C = Counter(F)
            q = F.q
            groups = {}
            for (j, w, a, b) in enumerate_classes(F):
                N = C.count(a, b)
                t = q + 1 - N
                if t > 0 and t % p:
                    groups.setdefault(t, []).append((a, b, N))
            checked = 0
            for t, v in groups.items():
                _, f, dk = disc_decomposition(t, q)
                if f == 1:
                    continue
                obs = {}
                for (a, b, N) in v:
                    rec = certify_curve(F, C, a, b, t, N, f, dk)
                    self.assertEqual(rec["status"], "ok", (p, t, rec.get("reason")))
                    obs[rec["fE"]] = obs.get(rec["fE"], 0) + 1
                self.assertEqual(obs, kronecker_class_number(f, dk), (p, t))
                checked += 1
            self.assertGreater(checked, 5)


class TestSemaevAndRelations(unittest.TestCase):
    def test_s3_zero_iff_a_sign_combination_sums_to_O(self):
        F = Fq(5)
        rng = random.Random(8)
        for _ in range(3):
            E = random_curve(F, rng)
            by_x = {}
            for P in E.points():
                if P:
                    by_x.setdefault(P[0], P)
            for x1, x2, x3 in itertools.product(sorted(by_x), repeat=3):
                P1, P2, P3 = by_x[x1], by_x[x2], by_x[x3]
                exists = any(
                    E.add(E.add(P1 if e1 > 0 else E.neg(P1), P2 if e2 > 0 else E.neg(P2)),
                          P3 if e3 > 0 else E.neg(P3)) is None
                    for e1 in (1, -1) for e2 in (1, -1) for e3 in (1, -1))
                self.assertEqual(s3_value(F, E.a, E.b, x1, x2, x3) == 0, exists)

    def test_weil_restriction_is_the_coordinates_of_s3(self):
        F = Fq(11)
        rng = random.Random(9)
        for _ in range(50):
            E = random_curve(F, rng)
            X3, x1, x2 = rng.randrange(F.q), rng.randrange(F.p), rng.randrange(F.p)
            f0, f1 = weil_restrict(F, E.a, E.b, X3)
            e1, e2 = (x1 + x2) % F.p, (x1 * x2) % F.p
            ev = lambda f: sum(c * pow(e1, i, F.p) * pow(e2, j, F.p) for (i, j), c in f.items()) % F.p
            self.assertEqual((ev(f0), ev(f1)), F.comps(s3_value(F, E.a, E.b, x1, x2, X3)))

    def test_buchberger_lex_solutions_equal_brute_force(self):
        rng = random.Random(10)
        mons = [(2, 0), (1, 1), (0, 2), (1, 0), (0, 1), (0, 0)]
        for p in (7, 11):
            for _ in range(150):
                f0 = {m: c for m in mons if (c := rng.randrange(p))}
                f1 = {m: c for m in mons if (c := rng.randrange(p))}
                if not f0 or not f1:
                    continue
                ev = lambda f, a, b: sum(c * pow(a, i, p) * pow(b, j, p) for (i, j), c in f.items()) % p
                brute = {(a, b) for a in range(p) for b in range(p) if ev(f0, a, b) == 0 == ev(f1, a, b)}
                G = Buchberger(p, "lex").groebner([f0, f1])
                sols = solve_zero_dim(G, p)
                if sols is not None:
                    self.assertEqual(set(sols), brute)

    def test_engines_agree_on_in_base_solutions(self):
        rng = random.Random(11)
        for p in (13, 19):
            F = Fq(p)
            E = random_curve(F, rng)
            full = Instance(F, E.a, E.b, 1).full_size
            inst = Instance(F, E.a, E.b, max(2, 2 * full // 3))
            pts = [P for P in E.points() if P]
            for _ in range(40):
                R = rng.choice(pts)
                a, _ = decompose_exhaustive(inst, R[0])
                b, met = decompose_weil_gb(inst, R[0])
                self.assertEqual(a["in_base"], b["in_base"])
                self.assertFalse(met["non_zero_dim"])
                self.assertLessEqual(met["quotient_dim"], 4)  # Bezout: two quadrics

    def _tiny_setup(self):
        """A prime-order-subgroup setup on a toy curve for the relation tests."""
        F = Fq(13)
        rng = random.Random(12)
        C = Counter(F)
        while True:
            E = random_curve(F, rng)
            N = C.count(E.a, E.b)
            from cgp_field import factorint
            r = max(factorint(N))
            if r > 30 and N % (r * r):
                break
        h = N // r
        inst = Instance(F, E.a, E.b, Instance(F, E.a, E.b, 1).full_size)
        E = inst.E
        while True:
            P0 = rng.choice([P for P in E.points() if P])
            G = E.mul(h, P0)
            if G is not None:
                break
        return F, E, inst, N, r, h, G

    def test_relation_verification_accepts_true_and_rejects_false(self):
        F, E, inst, N, r, h, G = self._tiny_setup()
        d = 7 % r
        H = E.mul(d, G)
        table = {None: 0}
        cur = None
        for k in range(1, r):
            cur = E.add(cur, G)
            table[cur] = k
        colinfo, colpts, keys = {}, [], {}
        for (x, y) in inst.base:
            Q = E.mul(h, (x, y))
            if Q is None:
                colinfo[x] = (None, 0)
                continue
            ny = F.neg(Q[1])
            canon, sg = (Q, 1) if Q[1] <= ny else ((Q[0], ny), -1)
            if canon not in keys:
                keys[canon] = len(colpts)
                colpts.append(canon)
            colinfo[x] = (keys[canon], sg)
        logC = [table[Q] for Q in colpts]
        ncols = len(colpts) + 2
        colG, colH = len(colpts), len(colpts) + 1
        found = 0
        rng = random.Random(13)
        for _ in range(60):
            aj, bj = rng.randrange(1, r), rng.randrange(1, r)
            R = E.add(E.mul(aj, G), E.mul(bj, H))
            if R is None:
                continue
            sols, _ = decompose_exhaustive(inst, R[0])
            for (x1, x2) in sols["in_base"]:
                P1, P2 = (x1, inst.base_y[x1]), (x2, inst.base_y[x2])
                ok, e1, e2, row = relation_row(E, R, P1, P2, x1, x2, colinfo, logC, ncols, colG,
                                               colH, h, aj, bj, d, r)
                self.assertTrue(ok)
                self.assertEqual(E.add(P1 if e1 > 0 else E.neg(P1), P2 if e2 > 0 else E.neg(P2)), R)
                # a wrong target scalar breaks the discrete-log identity ...
                bad = relation_row(E, R, P1, P2, x1, x2, colinfo, logC, ncols, colG, colH, h,
                                   (aj + 1) % r or 1, bj, d, r)
                self.assertFalse(bad[0])
                # ... a wrong second point breaks the group-law check ...
                other = next(x for x in inst.base_y if x not in (x1, x2)) if len(inst.base_y) > 2 else None
                if other is not None:
                    self.assertFalse(relation_row(E, R, P1, (other, inst.base_y[other]), x1, other,
                                                  colinfo, logC, ncols, colG, colH, h, aj, bj, d, r)[0])
                # ... and an off-curve point is rejected outright
                self.assertIsNone(verify_relation(E, R, P1, (P2[0], F.add(P2[1], 1))))
                found += 1
        self.assertGreater(found, 3)

    def test_echelon_rank_duplicate_dependent(self):
        ech = Echelon(101, 4)
        self.assertEqual(ech.add([1, 0, 0, 0]), "independent")
        self.assertEqual(ech.add([0, 1, 0, 0]), "independent")
        self.assertEqual(ech.add([1, 0, 0, 0]), "duplicate")
        self.assertEqual(ech.add([1, 1, 0, 0]), "dependent")
        self.assertEqual(ech.add([100, 2, 1, 0]), "independent")
        self.assertEqual(ech.rank, 3)


class TestIsoControl(unittest.TestCase):
    def test_scaled_model_is_isomorphic_and_maps_the_factor_base(self):
        F = Fq(13)
        rng = random.Random(14)
        C = Counter(F)
        E = random_curve(F, rng)
        u = F.p  # theta
        u2 = F.mul(u, u)
        self.assertTrue(F.in_fp(u2))
        u3, u4 = F.mul(u2, u), F.mul(u2, u2)
        u6 = F.mul(u4, u2)
        E2 = Curve(F, F.mul(u4, E.a), F.mul(u6, E.b))
        self.assertEqual(j_invariant(F, E.a, E.b), j_invariant(F, E2.a, E2.b))
        self.assertEqual(C.count(E.a, E.b), C.count(E2.a, E2.b))
        for P in rng.sample([P for P in E.points() if P], 10):
            Q = (F.mul(u2, P[0]), F.mul(u3, P[1]))
            self.assertTrue(E2.on_curve(Q))
        i1, i2 = Instance(F, E.a, E.b, 1).full_size, Instance(F, E2.a, E2.b, 1).full_size
        self.assertEqual(i1, i2)


class TestSelectedPairDecisive(unittest.TestCase):
    """The trial pair of F_{43^2}, t = 12: f_pi = 7, D_K = -148, N = 1838.  The crater has
    8 = 7 + 1 rational 7-isogenies (psi_7 splits completely), the floor exactly one, there is
    a direct 7-isogeny between them, and method B (exact n1 of the curve and its twist) is
    decisive for both curves independently of the volcano."""

    def test_pair(self):
        F = Fq(43)
        C = Counter(F)
        q, t = 43 * 43, 12
        N = q + 1 - t
        self.assertEqual(disc_decomposition(t, q), (-7252, 7, -148))  # 144 - 4*1849 = -7^2 * 148
        by_j = {c[0]: c for c in enumerate_classes(F) if C.count(c[2], c[3]) == N}
        recs = {}
        for j in (596, 386):
            _, w, a, b = by_j[j]
            recs[j] = certify_curve(F, C, a, b, t, N, 7, -148)
            self.assertEqual(recs[j]["status"], "ok", recs[j].get("reason"))
            self.assertTrue(recs[j]["method_B"]["per_prime"][7]["decisive"])
        self.assertEqual((recs[596]["fE"], recs[386]["fE"]), (1, 7))
        self.assertEqual(recs[596]["method_A"][7]["kernel_count_at_curve"], 8)
        self.assertEqual(recs[386]["method_A"][7]["kernel_count_at_curve"], 1)
        # method B alone separates them: the crater's twist has full 7-torsion, the floor has none
        self.assertEqual(recs[596]["method_B"]["per_prime"][7]["v_n1_twist"], 1)
        self.assertEqual(recs[386]["method_B"]["per_prime"][7]["v_n1_twist"], 0)
        from enumerate_pairs import find_path
        _, _, a, b = by_j[386]
        path = find_path(F, a, b, 7, 596, N, C)
        self.assertEqual(len(path), 1)


class TestStats(unittest.TestCase):
    def test_known_answers(self):
        self.assertAlmostEqual(sign_flip_pvalue([1, 1, 1, 1])["p"], 2 / 16)
        self.assertEqual(sign_flip_pvalue([1, -1])["p"], 1.0)
        self.assertEqual(sign_flip_pvalue([0, 0, 0])["p"], 1.0)
        self.assertEqual(holm([0.01, 0.04, 0.03]), [0.03, 0.06, 0.06])
        ci = bootstrap_ci([2.0] * 8)
        self.assertEqual((ci["lo"], ci["hi"]), (2.0, 2.0))

    def test_null_data_is_not_called_significant(self):
        rng = random.Random(15)
        ps = [sign_flip_pvalue([rng.gauss(0, 1) for _ in range(12)])["p"] for _ in range(200)]
        self.assertLess(sum(p < 0.05 for p in ps) / len(ps), 0.12)


if __name__ == "__main__":
    unittest.main()
