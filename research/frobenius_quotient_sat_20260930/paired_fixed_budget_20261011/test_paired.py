"""Tests for the paired fixed-budget comparison.  Run from this directory:

    python3 -m unittest test_paired -v
"""
import builtins
import copy
import itertools
import json
import os
import tempfile
import unittest

import paired_common as pc
import freeze_targets as ft
import rank as rk
import run_paired as rp

MANIFEST = pc.load_manifest()
CTX = pc.Ctx(MANIFEST)


def signed_point(ctx, rep, phase, sign):
    P = pc.frob_pt(ctx.E, ctx.reps_pt[rep], phase)
    return ctx.E.neg(P) if sign else P


def planted(ctx, spec):
    pts = [signed_point(ctx, *s) for s in spec]
    R = None
    for P in pts:
        R = ctx.E.add(R, P)
    return pts, R


SPEC = [(0, 3, 0), (1, 0, 1), (2, 7, 0), (3, 12, 1)]  # (rep, phase, sign)


class LambdaCoefficientCheck(unittest.TestCase):
    def test_lambda_is_the_frobenius_eigenvalue(self):
        c = CTX
        t = MANIFEST["curve"]["trace_t"]
        self.assertEqual((c.lam ** 2 - t * c.lam + 2) % c.r, 0)
        self.assertEqual(c.E.mul(c.lam, c.g), c.E.frob(c.g))
        for P in c.reps_pt:  # pi = [lambda] on every representative too
            self.assertEqual(c.E.mul(c.lam, P), c.E.frob(P))

    def test_planted_relation_passes(self):
        pts, R = planted(CTX, SPEC)
        rel = rk.relation_from_points(CTX, pts)
        self.assertEqual([(t["rep"], t["phase"], t["sign"]) for t in rel["terms"]],
                         [(0, 3, 1), (1, 0, -1), (2, 7, 1), (3, 12, -1)])
        chk = rk.check_relation(CTX, rel, R, pts)
        self.assertTrue(chk["all"], chk)
        # the coefficient on a repeated representative is the sum of signed powers
        for j, (rep, phase, sign) in enumerate(SPEC):
            want = (-1 if sign else 1) * pow(CTX.lam, phase, CTX.r) % CTX.r
            self.assertEqual(rel["coeffs"][rep], want)

    def test_perturbed_coefficient_fails(self):
        pts, R = planted(CTX, SPEC)
        rel = rk.relation_from_points(CTX, pts)
        for j in range(len(rel["coeffs"])):
            bad = copy.deepcopy(rel)
            bad["coeffs"][j] = (bad["coeffs"][j] + 1) % CTX.r
            chk = rk.check_relation(CTX, bad, R, pts)
            self.assertFalse(chk["all"])
            self.assertFalse(chk["lambda_log"], j)       # a nonzero shift of a nonzero log
            self.assertFalse(chk["coeffs_consistent"], j)

    def test_wrong_phase_sign_or_target_fails(self):
        pts, R = planted(CTX, SPEC)
        rel = rk.relation_from_points(CTX, pts)
        bad = copy.deepcopy(rel)
        bad["terms"][0]["phase"] = (bad["terms"][0]["phase"] + 1) % CTX.n
        self.assertFalse(rk.check_relation(CTX, bad, R, pts)["all"])
        bad = copy.deepcopy(rel)
        bad["terms"][1]["sign"] = -bad["terms"][1]["sign"]
        self.assertFalse(rk.check_relation(CTX, bad, R, pts)["curve_sum"])
        other = CTX.E.add(R, CTX.g)
        self.assertFalse(rk.check_relation(CTX, rel, other, pts)["all"])

    def test_the_other_root_of_the_characteristic_polynomial_fails(self):
        # lambda' = t - lambda is the second root; coefficients built from it
        # are not the Frobenius action on this subgroup.
        pts, R = planted(CTX, SPEC)
        rel = rk.relation_from_points(CTX, pts)
        lam2 = (MANIFEST["curve"]["trace_t"] - CTX.lam) % CTX.r
        self.assertNotEqual(lam2, CTX.lam)
        bad = copy.deepcopy(rel)
        bad["coeffs"] = [0] * 5
        for t in bad["terms"]:
            bad["coeffs"][t["rep"]] = (bad["coeffs"][t["rep"]]
                                       + t["sign"] * pow(lam2, t["phase"], CTX.r)) % CTX.r
        self.assertFalse(rk.check_relation(CTX, bad, R, pts)["lambda_log"])

    def test_point_outside_the_orbits_is_rejected(self):
        pts, _ = planted(CTX, SPEC)
        shift = CTX.g
        while CTX.E.add(pts[0], shift) is None or CTX.E.add(pts[0], shift)[0] in CTX.orbit:
            shift = CTX.E.add(shift, CTX.g)   # skip shifts that land on another admissible x
        pts[0] = CTX.E.add(pts[0], shift)
        with self.assertRaises(rk.RelationError):
            rk.relation_from_points(CTX, pts)


class RankTracker(unittest.TestCase):
    def test_incremental_rank_mod_r(self):
        r = CTX.r
        tr = rk.RankTracker(r, 5)
        self.assertTrue(tr.add([1, 0, 0, 0, 0]))
        self.assertFalse(tr.add([2, 0, 0, 0, 0]))
        self.assertFalse(tr.add([r, 0, 0, 0, 0]))
        self.assertTrue(tr.add([1, 1, 0, 0, 0]))
        self.assertFalse(tr.add([3, 5, 0, 0, 0]))
        self.assertTrue(tr.add([0, 0, 1, 0, 0]))
        self.assertTrue(tr.add([0, 0, 0, 1, 7]))
        self.assertTrue(tr.add([0, 0, 0, 0, 1]))
        self.assertEqual(tr.rank, 5)
        self.assertFalse(tr.add([4, 3, 2, 1, 0]))   # saturated at the column count


class Manifest(unittest.TestCase):
    def test_regeneration_is_byte_identical_and_matches_the_frozen_file(self):
        m1, s1 = ft.build_artifacts()
        m2, s2 = ft.build_artifacts()
        self.assertEqual(m1, m2)
        self.assertEqual(s1, s2)
        self.assertEqual(m1, open(pc.MANIFEST, "rb").read())
        self.assertEqual(s1, open(pc.SEALED, "rb").read())
        self.assertEqual(pc.sha256_bytes(m1), open(pc.MANIFEST_SHA).read().split()[0])

    def test_another_target_seed_changes_targets_not_factor_space(self):
        m1 = json.loads(open(pc.MANIFEST).read())
        m2 = json.loads(ft.build_artifacts(target_seed=pc.TARGET_SEED + 1)[0])
        self.assertEqual(m1["factor_space"], m2["factor_space"])
        self.assertNotEqual(m1["target_sets"], m2["target_sets"])

    def test_manifest_shape_and_no_witness_leak(self):
        sets = MANIFEST["target_sets"]
        self.assertEqual(sorted(sets), ["a", "b", "c"])
        self.assertTrue(all(len(v["targets"]) == pc.PER_SET for v in sets.values()))
        ids = [t["id"] for v in sets.values() for t in v["targets"]]
        self.assertEqual(len(ids), len(set(ids)))
        text = json.dumps(MANIFEST)
        for word in ('"phase":', "summands", '"sign"', "multiplier", "brute_force",
                     "decomposable"):
            self.assertNotIn(word, text)
        for t in pc.target_list(MANIFEST):
            self.assertTrue(CTX.E.on(t["R"]))
            self.assertIsNone(CTX.E.mul(CTX.r, t["R"]))
        self.assertEqual(MANIFEST["factor_space"]["admissible_x_phases_free"], 5 * 13)

    def test_sealed_file_agrees_with_the_manifest(self):
        sealed = json.load(open(pc.SEALED))
        self.assertEqual(sealed["manifest_sha256"], pc.sha256_file(pc.MANIFEST))
        for t in pc.target_list(MANIFEST):
            w = sealed["targets"][t["id"]]
            if w["construction"]["kind"] == "planted":
                R = None
                for s in w["construction"]["summands"]:
                    P = (pc.unhex(s["point"][0]), pc.unhex(s["point"][1]))
                    R = CTX.E.add(R, P)
                self.assertEqual(R, t["R"])
                phases = [s["phase"] for s in w["construction"]["summands"]]
                self.assertEqual(any(phases), t["set"] == "a")


class Schedule(unittest.TestCase):
    def test_abba_alternation(self):
        seq = pc.abba_schedule(pc.target_list(MANIFEST))
        arms = "".join(s["arm"] for s in seq)
        self.assertEqual(arms, "ABBA" * 6)
        self.assertEqual(len(seq), 24)
        for i in range(12):
            self.assertEqual({seq[2 * i]["target"]["id"], seq[2 * i + 1]["target"]["id"]},
                             {seq[2 * i]["target"]["id"]})
            self.assertEqual({seq[2 * i]["arm"], seq[2 * i + 1]["arm"]}, {"A", "B"})


class Runner(unittest.TestCase):
    def _assume(self, spec_rows, basis):
        coords = {}
        for c in itertools.product((0, 1), repeat=len(basis)):
            v = 0
            for b, bit in zip(basis, c):
                if bit:
                    v ^= b
            coords[v] = c

        def fn(cvars, evars):
            lits = []
            for (rep, phase, _), ci in zip(spec_rows, cvars):
                c = coords[CTX.reps_x[rep]]
                lits += [v if bit else -v for v, bit in zip(ci, c)]
            if evars is not None:
                for (rep, phase, _), ev in zip(spec_rows, evars):
                    lits += [e if i == phase else -e for i, e in enumerate(ev)]
            return lits
        return fn

    def test_verified_path_arm_b_and_arm_a(self):
        basis = [pc.unhex(b) for b in MANIFEST["factor_space"]["basis"]]
        for spec, phases in ((SPEC, True), ([(0, 0, 0), (1, 0, 1), (2, 0, 0), (3, 0, 1)], False)):
            pts, R = planted(CTX, spec)
            rec = rp.run_attempt(CTX.E, basis, R, phases, "line_subgroup", 60.0,
                                 assumptions=self._assume(spec, basis))
            self.assertEqual(rec["status"], "verified", rec)
            self.assertEqual(rec["rejected_tuples"], 0)
            got = [(pc.unhex(p[0]), pc.unhex(p[1])) for p in rec["found_points"]]
            rel = rk.relation_from_points(CTX, got)
            self.assertTrue(rk.check_relation(CTX, rel, R, got)["all"])
            self.assertEqual(len(rec["solver_summands"]), 4)
            for key in ("build_wall_s", "build_cpu_s", "load_wall_s", "load_cpu_s",
                        "solve_wall_s", "solve_cpu_s", "verify_wall_s", "verify_cpu_s",
                        "total_wall_s", "total_cpu_s", "size", "solver_instances"):
                self.assertIn(key, rec)
            self.assertLessEqual(rec["solve_cpu_s"], rec["total_cpu_s"] + 1e-6)

    def test_budget_is_cpu_and_timeout_is_recorded(self):
        basis = [pc.unhex(b) for b in MANIFEST["factor_space"]["basis"]]
        c0 = next(t for t in pc.target_list(MANIFEST) if t["id"] == "c0")
        rec = rp.run_attempt(CTX.E, basis, c0["R"], True, "line_subgroup", 0.3)
        self.assertEqual(rec["status"], "timeout")
        self.assertGreaterEqual(rec["solve_cpu_s"], 0.27)
        self.assertLess(rec["solve_cpu_s"], 1.0)

    def test_runner_never_opens_the_sealed_file(self):
        opened = []
        real_open = builtins.open

        def spy(path, *a, **k):
            opened.append(os.fspath(path) if not isinstance(path, int) else str(path))
            return real_open(path, *a, **k)

        with tempfile.TemporaryDirectory() as d:
            out = os.path.join(d, "r.jsonl")
            builtins.open = spy
            try:
                rp.main(["--out", out, "--budget", "0.2", "--forms", "line_subgroup",
                         "--targets", "c0"])
            finally:
                builtins.open = real_open
            with real_open(out) as fh:
                lines = [json.loads(l) for l in fh]
        self.assertFalse([p for p in opened if "witness_sealed" in p], opened)
        self.assertEqual([l["type"] for l in lines], ["header", "attempt", "attempt", "end"])
        self.assertEqual([l["arm"] for l in lines if l["type"] == "attempt"], ["A", "B"])
        with real_open(rp.__file__) as fh:
            src = fh.read()
        self.assertNotIn("witness_sealed", src)
        self.assertNotIn("SEALED", src)

    def test_cpu_cap_writes_skipped_receipts(self):
        with tempfile.TemporaryDirectory() as d:
            out = os.path.join(d, "r.jsonl")
            rp.main(["--out", out, "--budget", "0.2", "--forms", "line_subgroup",
                     "--targets", "c0,c1", "--cpu-cap", "0.1"])
            with open(out) as fh:
                rows = [json.loads(l) for l in fh]
        att = [r for r in rows if r["type"] == "attempt"]
        self.assertEqual([r["status"] for r in att], ["timeout", "timeout", "skipped", "skipped"])
        self.assertTrue(all("reason" in r for r in att if r["status"] == "skipped"))


if __name__ == "__main__":
    unittest.main()
