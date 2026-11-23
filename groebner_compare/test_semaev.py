"""Generator, certificate and adapter tests for the Semaev solver comparison.

Correctness controls only: nothing here is performance evidence.
"""

import json
import os
from pathlib import Path
import shutil
import sys
import tempfile
import unittest

from .certificate import basis_roots, certify, vanishes
from .external_worker import (parse_cms, parse_msolve_basis, parse_msolve_log,
                              run_measured, solve)
from .runner import run, validate
from .semaev import (Field, Symbolic, build_manifest, decode_abscissae, generate,
                     irreducible, is_irreducible, readdition, s3_value,
                     s4_symmetrised_value, setup, symmetric3, to_dimacs_xor, to_msolve)
from .semaev_verifier import verify


def lucas_order(n, a):
    """#E(GF(2^n)) for the Koblitz curve y^2+xy=x^3+ax^2+1 (Frobenius trace)."""
    mu = 1 if a == 1 else -1
    t0, t1 = 2, mu
    for _ in range(n - 1):
        t0, t1 = t1, mu * t1 - 2 * t0
    return 2 ** n + 1 - t1


def brute_roots(equations, n):
    """Exhaustive zeros (n <= 12); independent of every solver under test."""
    return [a for a in range(1 << n) if vanishes(equations, a)]


TINY_PLAIN3 = dict(m=3, n=5, l=2, variant="plain", a=0)        # 11 variables
TINY_EONLY3 = dict(m=3, n=5, l=2, variant="symmetrised-e", a=0)  # 9 variables


class FieldAndCurve(unittest.TestCase):
    def test_irreducible_search_matches_known_moduli(self):
        # z^5+z^2+1, z^7+z+1, z^11+z^2+1, z^19+z^5+z^2+z+1 (ic corpus n19).
        self.assertEqual(irreducible(5), 0b100101)
        self.assertEqual(irreducible(7), 0b10000011)
        self.assertEqual(irreducible(11), (1 << 11) | 0b101)
        self.assertEqual(irreducible(19), (1 << 19) | 0b100111)
        self.assertFalse(is_irreducible(0b101))  # z^2+1 = (z+1)^2

    def test_point_counts_match_frobenius_trace(self):
        # Independent of the lift code: Lucas sequence of the Koblitz trace.
        for n in (5, 7, 8, 11):
            for a in (0, 1):
                _, curve = setup(n, a, 1)
                self.assertEqual(curve.order(), lucas_order(n, a), (n, a))

    def test_group_law_and_semaev_polynomials_on_points(self):
        field, curve = setup(11, 1, 1)
        points = [p for x in range(1, 60) for p in curve.lift(x)]
        self.assertTrue(all(curve.on_curve(p) for p in points))
        for p, q, r in ((points[0], points[3], points[7]), (points[2], points[9], points[11])):
            s = curve.add(p, q)
            self.assertTrue(curve.on_curve(s))
            self.assertEqual(s3_value(field, p[0], q[0], s[0], 1), 0)
            self.assertIsNone(curve.add(p, curve.neg(p)))
            t = curve.add(s, r)
            e = symmetric3(field, [p[0], q[0], r[0]])
            self.assertEqual(s4_symmetrised_value(field, *e, t[0]), 0)
            # Associativity spot check.
            self.assertEqual(curve.add(curve.add(p, q), r), curve.add(p, curve.add(q, r)))
        self.assertEqual(curve.add(points[0], points[0]),
                         curve.add(curve.add(points[0], points[1]),
                                   curve.add(points[0], curve.neg(points[1]))))

    def test_symbolic_descent_agrees_with_field_arithmetic(self):
        field = Field(7)
        sym = Symbolic(field)
        x1, x2 = sym.variables(0, 7), sym.variables(7, 7)
        product = sym.mul(x1, x2)
        square = sym.square(x1)
        for a, b in ((3, 77), (100, 5), (127, 127)):
            assignment = a | b << 7
            value = lambda element: sum(1 << d for d, poly in enumerate(element)
                                        if sum((m & assignment) == m for m in poly) % 2)
            self.assertEqual(value(product), field.mul(a, b))
            self.assertEqual(value(square), field.sq(a))


class Generator(unittest.TestCase):
    def test_deterministic_and_planted(self):
        for spec in (TINY_PLAIN3, TINY_EONLY3, dict(m=4, n=5, l=3, variant="plain", a=0),
                     dict(m=3, n=7, l=3, variant="symmetrised", a=1)):
            with self.subTest(**spec):
                one, planted = generate(**spec, seed=3, index=1)
                two, again = generate(**spec, seed=3, index=1)
                self.assertEqual(one, two)
                self.assertEqual(planted, again)
                self.assertNotIn("planted", json.dumps(one))
                self.assertTrue(vanishes(one["equations"], planted["assignment"]))
                self.assertIsNotNone(readdition(one["semaev"], planted["assignment"]))
                self.assertGreaterEqual(one["semaev"]["x_R"], 1 << spec["l"])
                other, _ = generate(**spec, seed=4, index=1)
                self.assertEqual(other["semaev"]["layout"], one["semaev"]["layout"])

    def test_variable_counts_and_degrees(self):
        cases = {("plain", 3): (3 * 2 + 5, 3), ("symmetrised-e", 3): (6 * 2 - 3, 2),
                 ("plain", 4): (4 * 3 + 2 * 5, 3)}
        for (variant, m), (nvars, degree) in cases.items():
            l = 3 if m == 4 else 2
            instance, _ = generate(m, 5, l, variant=variant, a=0)
            self.assertEqual(instance["semaev"]["layout"]["nvars"], nvars)
            self.assertEqual(max(x.bit_count() for row in instance["equations"] for x in row),
                             degree)
        sym, _ = generate(3, 7, 3, variant="symmetrised", a=1)
        self.assertEqual(sym["semaev"]["layout"]["nvars"], 9 * 3 - 3)
        self.assertEqual(len(sym["equations"]), 7 + 6 * 3 - 3)

    def test_same_planting_across_variants(self):
        plain, a = generate(**{**TINY_PLAIN3}, seed=2)
        eonly, b = generate(**{**TINY_EONLY3}, seed=2)
        self.assertEqual(a["abscissae"], b["abscissae"])
        self.assertEqual(plain["semaev"]["x_R"], eonly["semaev"]["x_R"])
        self.assertIn(sorted(a["abscissae"]),
                      [sorted(t) for t in decode_abscissae(eonly["semaev"], b["assignment"])])

    def test_unsupported_cells_refuse(self):
        with self.assertRaises(NotImplementedError):
            generate(4, 7, 2, variant="symmetrised")
        with self.assertRaises(ValueError):
            generate(3, 7, 2, variant="symmetrised", b=3)
        with self.assertRaises(ValueError):
            generate(3, 5, 1, variant="plain", a=0)  # too few factor-base points

    def test_readdition_rejects_wrong_roots(self):
        instance, planted = generate(**TINY_PLAIN3)
        meta = instance["semaev"]
        good = planted["assignment"]
        self.assertIsNotNone(readdition(meta, good))
        # Flip a factor-base bit: generally no longer a decomposition of R.
        rejected = [readdition(meta, good ^ (1 << bit)) is None for bit in range(6)]
        self.assertTrue(any(rejected))

    def test_all_exhaustive_roots_readd_or_are_rejected_consistently(self):
        instance, planted = generate(**TINY_PLAIN3)
        n = instance["semaev"]["layout"]["nvars"]
        roots = brute_roots(instance["equations"], n)
        self.assertIn(planted["assignment"], roots)
        verified = [s for s in roots if readdition(instance["semaev"], s) is not None]
        self.assertIn(planted["assignment"], verified)
        # e-only: spurious e-triples that do not split over the factor base
        # are algebraic roots but must fail re-addition.
        eonly, answer = generate(**TINY_EONLY3)
        roots = brute_roots(eonly["equations"], 9)
        verified = [s for s in roots if readdition(eonly["semaev"], s) is not None]
        self.assertIn(answer["assignment"], verified)
        self.assertLess(len(verified), len(roots))

    def test_exports(self):
        instance, _ = generate(**TINY_EONLY3)
        n = instance["semaev"]["layout"]["nvars"]
        text = to_msolve(instance["equations"], n)
        lines = text.splitlines()
        self.assertEqual(lines[0], ",".join(f"x{i}" for i in range(n)))
        self.assertEqual(lines[1], "2")
        self.assertTrue(text.rstrip().endswith(f"x{n - 1}^2+x{n - 1}"))
        dimacs, shape = to_dimacs_xor([[3, 4], [1, 0], [2]], 3)
        self.assertEqual(shape, {"tseitin_gates": 1, "xor_rows": 3})
        self.assertIn("x-4 3 0", dimacs)   # x0*x1 + x2 = 0 -> XOR(g, x2) = 0
        self.assertIn("x1 0", dimacs)      # x0 + 1 = 0 -> x0 = 1
        self.assertIn("x-2 0", dimacs)     # x1 = 0


class RootsAndRunner(unittest.TestCase):
    def test_basis_roots_matches_exhaustive(self):
        instance, planted = generate(**TINY_PLAIN3)
        n = instance["semaev"]["layout"]["nvars"]
        # The input generators have the same zeros as any basis of the ideal.
        roots, info = basis_roots(n, instance["equations"])
        self.assertTrue(info["complete"])
        self.assertEqual(roots, brute_roots(instance["equations"], n))
        self.assertIn(planted["assignment"], roots)
        self.assertEqual(basis_roots(3, [[0]])[0], [])
        self.assertEqual(basis_roots(2, [])[0], [0, 1, 2, 3])
        with self.assertRaises(TimeoutError):
            basis_roots(20, [], limit=10)

    def test_validation_bounds(self):
        manifest = build_manifest(4, 5, 3, variant="plain", a=0, count=1, backends=[
            {"id": "x", "mode": "cold", "command": ["true"], "provenance": "test"}])
        self.assertEqual(manifest["certificate"], "witness-replay")
        validate(manifest)
        manifest["certificate"] = "exhaustive"
        with self.assertRaises(ValueError):
            validate(manifest)
        manifest["certificate"] = "witness-replay"
        manifest["instances"][0]["planted"] = 1
        with self.assertRaises(ValueError):
            validate(manifest)

    def test_verifier_rejects_tampered_instance(self):
        instance, planted = generate(**TINY_PLAIN3)
        request = {"operation": "verify_witnesses", "ring": {"nvars": 11},
                   "instance": instance, "solutions": [planted["assignment"]]}
        self.assertEqual(verify(request)["verified_solutions"], [planted["assignment"]])
        tampered = json.loads(json.dumps(instance))
        tampered["equations"][0] = tampered["equations"][0][1:]
        request["instance"] = tampered
        self.assertEqual(verify(request)["status"], "error")

    def run_manifest(self, manifest):
        directory = tempfile.TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        return run(manifest, Path(directory.name) / "receipt")

    @unittest.skipUnless(shutil.which("msolve"), "msolve not installed")
    def test_runner_with_msolve_and_unavailable_arms(self):
        py = sys.executable
        backends = [
            {"id": "msolve-f4", "mode": "cold",
             "command": [py, "-m", "groebner_compare.external_worker", "msolve"],
             "provenance": "msolve on PATH"},
            {"id": "magma-f4", "mode": "cold",
             "command": [py, "-m", "groebner_compare.external_worker", "magma"],
             "provenance": "licensed Magma only"},
            {"id": "julia-missing", "mode": "learn_apply",
             "command": ["/no-such-julia-for-test", "groebner_compare/worker.jl"],
             "provenance": "missing executable"}]
        env = os.environ.pop("MAGMA_BINARY", None)
        self.addCleanup(lambda: env is not None and os.environ.__setitem__("MAGMA_BINARY", env))
        if shutil.which("magma"):
            self.skipTest("a magma binary is on PATH; unavailable path not testable")
        f4, magma, julia = self.run_manifest(build_manifest(**TINY_PLAIN3, count=2,
                                                            backends=backends))
        self.assertEqual(f4["verified_instances"], 2)
        self.assertEqual(f4["verified_curve_witness_instances"], 2)
        self.assertEqual(f4["planted_recovered_instances"], 2)
        self.assertEqual(f4["availability"], "available")
        self.assertIsNotNone(f4["backend_reported_highest_degree"])
        self.assertIsNotNone(f4["backend_reported_largest_matrix_rows"])
        self.assertGreater(f4["backend_reported_solver_peak_rss_bytes"], 0)
        for summary in (magma, julia):
            self.assertEqual(summary["availability"], "UNAVAILABLE")
            self.assertEqual(summary["verified_instances"], 0)
            self.assertTrue(summary["unavailable_reasons"])
        self.assertIn("UNAVAILABLE", magma["unavailable_reasons"][0])

    @unittest.skipUnless(shutil.which("msolve") and shutil.which("cryptominisat5"),
                         "msolve and cryptominisat5 required")
    def test_runner_witness_replay_mode(self):
        py = sys.executable
        manifest = build_manifest(3, 7, 3, variant="symmetrised", a=1, count=1, backends=[
            {"id": "msolve-f4", "mode": "cold",
             "command": [py, "-m", "groebner_compare.external_worker", "msolve"],
             "provenance": "msolve on PATH"},
            {"id": "cryptominisat5-xor", "mode": "cold",
             "command": [py, "-m", "groebner_compare.external_worker", "cryptominisat5"],
             "provenance": "cryptominisat5 on PATH"}], watchdog=300)
        msolve_summary, cms_summary = self.run_manifest(manifest)
        for summary in (msolve_summary, cms_summary):
            self.assertEqual(summary["results"][0]["status"], "verified_witness")
            self.assertEqual(summary["results"][0]["curve_witness"], "verified")
            self.assertTrue(summary["results"][0]["planted_in_certified_solutions"])
        self.assertEqual(sorted(msolve_summary["results"][0]["curve_verified_solutions"]),
                         sorted(cms_summary["results"][0]["curve_verified_solutions"]))
        summary = msolve_summary
        self.assertEqual(summary["results"][0]["status"], "verified_witness")
        self.assertIsNone(summary["results"][0]["root_count"])
        self.assertEqual(summary["certificate_mode"], "witness-replay")
        self.assertTrue(summary["results"][0]["planted_in_certified_solutions"])


class ExternalParsers(unittest.TestCase):
    LOG = """
deg     sel   pairs        mat          density            new data         time(rd) in sec (real|cpu)
------------------------------------------------------------------------------------------------------
  3       8       8      21 x 17         15.13%        1 new       7 zero         0.00 | 0.00
  2       3       3      10 x 10         27.00%        0 new       3 zero         0.00 | 0.00
------------------------------------------------------------------------------------------------------
reduce final basis        4 x 7          35.71%        4 new       0 zeroDimension of quotient: 3
"""
    BASIS = """#Reduced Groebner basis data
#---
[1*x0^1+1*x1^1+1*x2^1,
1*x2^2+1*x2^1,
1*x1^1*x2^1+1*x1^1+1*x2^1+1,
1*x1^2+1*x1^1]:
"""

    def test_msolve_log_and_basis(self):
        metrics = parse_msolve_log(self.LOG)
        self.assertEqual(metrics["highest_degree"], 3)
        self.assertEqual((metrics["largest_matrix_rows"], metrics["largest_matrix_columns"]),
                         (21, 17))
        self.assertEqual(metrics["f4_rounds"], 2)
        self.assertEqual(metrics["msolve_claimed_quotient_dimension"], 3)
        basis = parse_msolve_basis(self.BASIS, 3)
        self.assertEqual(basis, [[1, 2, 4], [], [0, 2, 4, 6], []])
        equations = [[3, 4, 0], [6, 1, 0], [5, 2, 0]]
        self.assertTrue(certify(3, equations, basis)["verified"])
        self.assertEqual(parse_msolve_basis("[1]:", 2), [[0]])

    @unittest.skipUnless(Path("/proc/self/status").is_file(), "Linux /proc required")
    def test_solver_rss_is_not_the_parent_rss(self):
        ballast = bytearray(96 * 1024 * 1024)  # inflate this (parent) process
        ballast[::4096] = b"x" * len(ballast[::4096])
        with tempfile.TemporaryDirectory() as directory:
            code, wall, _, rss = run_measured(
                ["sh", "-c", "sleep 0.1"], Path(directory) / "o", Path(directory) / "e")
        self.assertEqual(code, 0)
        self.assertGreater(wall, 0.05)
        self.assertLess(rss["solver_peak_rss_bytes"], 32 * 1024 * 1024)
        self.assertGreater(rss["solver_ru_maxrss_upper_bound_bytes"], 64 * 1024 * 1024)
        del ballast

    def test_cms_parse(self):
        text = "s SATISFIABLE\nv -1 2 3 0\ns SATISFIABLE\nv 1 -2 -3 0\ns UNSATISFIABLE\n"
        masks, verdicts, _ = parse_cms(text, 2)
        self.assertEqual(masks, [2, 1])
        self.assertEqual(verdicts[-1], "UNSATISFIABLE")

    @unittest.skipUnless(shutil.which("msolve"), "msolve not installed")
    def test_msolve_adapter_on_planted_instance(self):
        instance, planted = generate(**TINY_EONLY3)
        n = instance["semaev"]["layout"]["nvars"]
        response = solve("msolve", {"operation": "solve", "ring": {"nvars": n},
                                    "instance": instance}, {"threads": 1})
        self.assertEqual(response["status"], "ok", response)
        certificate = certify(n, instance["equations"], response["basis_terms"])
        self.assertTrue(certificate["verified"])
        self.assertIn(planted["assignment"], certificate["solutions"])
        self.assertGreater(response["metrics"]["solver_peak_rss_bytes"], 0)
        self.assertIsNotNone(response["metrics"]["highest_degree"])

    @unittest.skipUnless(shutil.which("cryptominisat5"), "cryptominisat5 not installed")
    def test_cryptominisat_adapter_enumerates_all_roots(self):
        instance, planted = generate(**TINY_EONLY3)
        n = instance["semaev"]["layout"]["nvars"]
        response = solve("cryptominisat5", {"operation": "solve", "ring": {"nvars": n},
                                            "instance": instance},
                         {"threads": 1, "max_solutions": 4096})
        self.assertEqual(response["status"], "ok", response)
        exhaustive = brute_roots(instance["equations"], n)
        self.assertEqual(sorted(response["solutions"]), exhaustive)
        self.assertTrue(response["metrics"]["solver_claimed_enumeration_complete"])


if __name__ == "__main__":
    unittest.main()
