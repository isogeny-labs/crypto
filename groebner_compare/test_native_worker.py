"""Tests for the native (Rust) arms: inherited-f4 and trace-f4.

Correctness controls only: nothing here is performance evidence. Tests that
need the built binary (cargo build --release --example gc_native_solve) skip
when it is absent; the unavailable-path test always runs.
"""

import os
import unittest
from unittest import mock

from . import native_worker
from .certificate import terms, vanishes
from .native_worker import charge_learning, from_tuple, learn, locate, solve, to_tuple
from .semaev import generate, readdition


def brute_roots(equations, n):
    """Exhaustive zeros; independent of every solver under test (small n only)."""
    return [a for a in range(1 << n) if vanishes(equations, a)]


TINY = [dict(m=3, n=5, l=2, variant="plain", a=0),          # 11 variables, cubic
        dict(m=3, n=5, l=2, variant="symmetrised-e", a=0),  # 9 variables, quadratic
        dict(m=3, n=5, l=2, variant="symmetrised", a=0)]    # 15 variables
BUILT = locate() is not None


def masks_of(result):
    return sorted(from_tuple(r) for r in result["roots"])


class Conversions(unittest.TestCase):
    def test_tuple_round_trip(self):
        for mask in (0, 1, 5, 0b1011001):
            self.assertEqual(from_tuple(to_tuple(mask, 9)), mask)
        self.assertEqual(to_tuple(0b110, 3), (0, 1, 1))  # x0 first

    def test_unavailable_without_binary(self):
        instance, _ = generate(**TINY[0], seed=1, index=0)
        with mock.patch.dict(os.environ, {"GC_NATIVE_BINARY": "/nonexistent/gc"}):
            with mock.patch.object(native_worker, "DEFAULT_BINARY",
                                   native_worker.Path("/nonexistent/gc2")):
                result = solve(instance, 5, arm="inherited-f4")
        self.assertEqual(result["status"], "unavailable")
        self.assertIsNone(result["roots"])
        for key in ("wall_s", "cpu_s", "peak_rss_bytes", "degree", "max_matrix", "notes"):
            self.assertIn(key, result)

    def test_unknown_arm(self):
        instance, _ = generate(**TINY[0], seed=1, index=0)
        self.assertEqual(solve(instance, 5, arm="magma")["status"], "unavailable")

    def test_trace_arm_requires_trace(self):
        instance, _ = generate(**TINY[0], seed=1, index=0)
        if BUILT:
            self.assertEqual(solve(instance, 5, arm="trace-f4")["status"], "unavailable")

    def test_run_child_kills_at_timeout(self):
        import tempfile
        with tempfile.TemporaryDirectory() as directory:
            script = os.path.join(directory, "slow")
            with open(script, "w") as handle:
                handle.write("#!/bin/sh\nexec sleep 30\n")
            os.chmod(script, 0o755)
            answer, wall, _, _, timed_out, _ = native_worker.run_child(script, {}, 0.3)
        self.assertTrue(timed_out)
        self.assertIsNone(answer)
        self.assertLess(wall, 5.0)

    def test_charge_learning_amortises(self):
        rows = [{"wall_s": 1.0, "cpu_s": 0.5, "notes": {}},
                {"wall_s": 2.0, "cpu_s": 1.5, "notes": {}}]
        charged = charge_learning(rows, {"learn_wall_s": 4.0, "learn_cpu_s": 2.0})
        self.assertEqual([c["wall_s"] for c in charged], [3.0, 4.0])
        self.assertEqual([c["cpu_s"] for c in charged], [1.5, 2.5])
        self.assertEqual(charged[0]["notes"]["call_wall_s"], 1.0)
        self.assertEqual(charged[0]["notes"]["learn_amortised_over"], 2)


@unittest.skipUnless(BUILT, "gc_native_solve not built")
class InheritedF4(unittest.TestCase):
    def test_root_sets_equal_brute_force_on_tiny_cells(self):
        for cell in TINY:
            for index in range(3):
                instance, planted = generate(**cell, seed=7, index=index)
                n = instance["semaev"]["layout"]["nvars"]
                equations = terms(instance["equations"], n)
                result = solve(instance, 60, arm="inherited-f4")
                self.assertEqual(result["status"], "ok", (cell, index, result["notes"]))
                self.assertEqual(masks_of(result), brute_roots(equations, n), (cell, index))
                self.assertIn(planted["assignment"], masks_of(result))
                self.assertIsNotNone(result["wall_s"])
                self.assertIsNotNone(result["cpu_s"])
                self.assertGreaterEqual(result["degree"], 1)

    def test_planted_root_readds_on_the_curve(self):
        instance, planted = generate(**TINY[0], seed=3, index=1)
        result = solve(instance, 60, arm="inherited-f4")
        readded = [m for m in masks_of(result) if readdition(instance["semaev"], m)]
        self.assertIn(planted["assignment"], readded)

    def test_non_root_from_child_is_rejected(self):
        instance, _ = generate(**TINY[1], seed=7, index=0)
        n = instance["semaev"]["layout"]["nvars"]
        equations = terms(instance["equations"], n)
        bogus = next(a for a in range(1 << n) if not vanishes(equations, a))
        fake = ({"status": "ok", "roots": [bogus], "degree_reached": 3}, 0.1, 0.1, None,
                False, "")
        with mock.patch.object(native_worker, "run_child", return_value=fake):
            result = solve(instance, 60, arm="inherited-f4")
        self.assertEqual(result["status"], "error")
        self.assertIsNone(result["roots"])

    def test_timeout_kills_child(self):
        instance, _ = generate(**TINY[0], seed=7, index=0)
        fake_timeout = (None, 0.5, 0.4, None, True, "")
        with mock.patch.object(native_worker, "run_child", return_value=fake_timeout):
            result = solve(instance, 0.5, arm="inherited-f4")
        self.assertEqual(result["status"], "timeout")
        self.assertIsNone(result["roots"])


@unittest.skipUnless(BUILT, "gc_native_solve not built")
class TraceF4(unittest.TestCase):
    def test_learn_replay_root_sets_equal_brute_force(self):
        for cell in TINY:
            training = [generate(**cell, seed=99, index=i)[0] for i in range(3)]
            trace = learn(training, 4, 60)
            self.assertEqual(trace["status"], "ok", trace)
            self.assertGreater(trace["learn_cpu_s"], 0.0)
            self.assertGreater(trace["survivors"], 0)
            for index in range(3):
                instance, planted = generate(**cell, seed=7, index=index)
                n = instance["semaev"]["layout"]["nvars"]
                equations = terms(instance["equations"], n)
                result = solve(instance, 60, arm="trace-f4", trace=trace, max_free=n)
                self.assertIn(result["notes"]["path"], ("replay", "fallback-full"))
                self.assertEqual(result["status"], "ok", (cell, index, result["notes"]))
                self.assertEqual(masks_of(result), brute_roots(equations, n), (cell, index))
                self.assertIn(planted["assignment"], masks_of(result))
                self.assertEqual(result["degree"], 4)

    def test_learning_cost_is_charged(self):
        cell = TINY[1]
        training = [generate(**cell, seed=99, index=i)[0] for i in range(2)]
        trace = learn(training, 3, 60)
        targets = [generate(**cell, seed=7, index=i)[0] for i in range(2)]
        raw = [solve(t, 60, arm="trace-f4", trace=trace, max_free=9) for t in targets]
        charged = charge_learning(raw, trace)
        for r, c in zip(raw, charged):
            self.assertAlmostEqual(c["cpu_s"], r["cpu_s"] + trace["learn_cpu_s"] / 2)
            self.assertAlmostEqual(c["wall_s"], r["wall_s"] + trace["learn_wall_s"] / 2)


if __name__ == "__main__":
    unittest.main()
