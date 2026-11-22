"""Tests for run_local.py (amendment-1 PA-4, M4 harness defaults).

python3 -m unittest research/groebner_solver_shootout_20261116/test_run_local.py -v
(from the repository root). Correctness only; nothing here is performance evidence.
"""

import collections
import hashlib
import importlib.util
from pathlib import Path
import signal
import sys
import tempfile
import unittest

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("run_local", HERE / "run_local.py")
rl = importlib.util.module_from_spec(spec)
sys.modules["run_local"] = rl
spec.loader.exec_module(rl)

from groebner_compare import semaev  # noqa: E402
from groebner_compare.certificate import terms, vanishes  # noqa: E402


class SeedRule(unittest.TestCase):
    def test_literal_v1_hash_and_domain(self):
        self.assertTrue(rl.MAIN_DOMAIN.startswith(rl.V1_CANONICAL + ":amend1:K17-m3-plain-l5-a1:"))
        raw = hashlib.sha256((rl.MAIN_DOMAIN + "natural:0").encode()).digest()
        self.assertEqual(rl.seed(rl.MAIN_DOMAIN, "natural", 0), int.from_bytes(raw[:8], "big"))

    def test_pilot_domain_disjoint(self):
        self.assertNotEqual(rl.PILOT_DOMAIN, rl.MAIN_DOMAIN)
        self.assertNotEqual(rl.seed(rl.PILOT_DOMAIN, "natural", 0),
                            rl.seed(rl.MAIN_DOMAIN, "natural", 0))


class HarnessDefaults(unittest.TestCase):
    """M4: max_solutions, the --random passthrough, thread and Gauss defaults."""

    def test_cms_argv(self):
        argv = rl.cms_argv("cms", "in.cnf")
        self.assertEqual(argv[argv.index("--maxsol") + 1], "1000000")
        self.assertEqual(argv[argv.index("--random") + 1], "0")
        self.assertEqual(argv[argv.index("--threads") + 1], "1")
        self.assertEqual(argv[argv.index("--verb") + 1], "1")
        for gauss in ("--maxmatrixrows", "--maxmatrixcols", "--autodisablegauss"):
            self.assertNotIn(gauss, argv)  # M5: defaults kept

    def test_msolve_argv(self):
        argv = rl.msolve_argv("msolve", "in.ms", "out.gb")
        self.assertEqual(argv[1:7], ["-g", "2", "-v", "2", "-t", "1"])

    def test_caps(self):
        self.assertEqual(rl.CPU_CAP_S, 60)
        self.assertEqual(rl.MEM_CAP_BYTES, 4 << 30)
        self.assertEqual(rl.WORKERS, 3)

    def test_separate_timers_and_charge(self):
        res = {"outcome": "ok", "child": {"cpu_s": 2.0}, "extract_cpu_s": 0.5}
        self.assertEqual(rl.charged_cpu(res), 2.5)  # projection never charged
        res["outcome"] = "censored_cpu"
        self.assertEqual(rl.charged_cpu(res), 60.0)
        res["outcome"] = "censored_budget"
        self.assertEqual(rl.charged_cpu(res), 60.0)


class Cell(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.oracle = rl.build_oracle()  # asserts 1757/65572 itself

    def test_oracle_weights(self):
        self.assertEqual((self.oracle["sat_abscissae"], self.oracle["liftable_outside_V"]),
                         (1757, 65572))
        self.assertEqual(len(self.oracle["factor_base"]), 14)
        self.assertNotIn(0, self.oracle["factor_base"])

    def test_build_instance_matches_generate(self):
        inst, ans = semaev.generate(3, 17, 5, variant="plain", a=1, b=1, seed=123, index=0,
                                    modulus=rl.MODULUS)
        mine = rl.build_instance(ans["target"][0], "t")
        self.assertEqual(mine["equations"], inst["equations"])
        self.assertEqual(mine["semaev"]["equations_sha256"], inst["semaev"]["equations_sha256"])

    def test_projection(self):
        inst, ans = semaev.generate(3, 17, 5, variant="plain", a=1, b=1, seed=7, index=0,
                                    modulus=rl.MODULUS)
        n = inst["semaev"]["layout"]["nvars"]
        self.assertTrue(vanishes(terms(inst["equations"], n), ans["assignment"]))
        proj = rl.project_roots(inst, [ans["assignment"], ans["assignment"]])
        self.assertEqual(proj["verified"], [tuple(sorted(ans["abscissae"]))])
        self.assertEqual(proj["roots_per_multiset"], {",".join(map(str, sorted(ans["abscissae"]))): 2})
        self.assertEqual(proj["readditions"], 1)  # memoised
        # x = 0 counted separately; a non-relation multiset is spurious.
        zero = ans["assignment"] & ~((1 << 5) - 1)
        self.assertEqual(rl.project_roots(inst, [zero])["zero_abscissa_roots"], 1)
        oset = self.oracle["table"][ans["target"][0]]
        self.assertIn(tuple(sorted(ans["abscissae"])), oset)
        for ms in [(1, 2, 3), (2, 3, 5), (4, 6, 7)]:
            if all(x in self.oracle["factor_base"] for x in ms) and ms not in oset:
                a = sum(x << o for x, o in zip(ms, inst["semaev"]["layout"]["x"]))
                self.assertEqual(rl.project_roots(inst, [a])["spurious_roots"], 1)
                break

    def test_m1_planted_multiset_is_root(self):
        p = rl.planted_targets(rl.PILOT_DOMAIN, 1)[0]
        check = rl.oracle_roots_check(p, self.oracle)
        self.assertTrue(check[",".join(map(str, p["planted_multiset"]))])

    def test_natural_stream(self):
        sat, unsat, _, draws = rl.natural_targets(rl.PILOT_DOMAIN, self.oracle, 2, 2)
        self.assertEqual(len(sat), 2)
        self.assertEqual(len(unsat), 2)
        for t in sat:
            self.assertIn(t["x_R"], self.oracle["table"])
            self.assertGreaterEqual(t["x_R"], 32)
        for t in unsat:
            self.assertNotIn(t["x_R"], self.oracle["table"])


class Scheduling(unittest.TestCase):
    def test_williams_balance(self):
        targets = [{"target_id": f"t{i}"} for i in range(160)]
        order, seqs = rl.assign_sequences(rl.MAIN_DOMAIN, targets)
        counts = collections.Counter(seqs)
        self.assertEqual(len(counts), 6)
        self.assertEqual(sorted(set(counts.values())), [26, 27])
        # Each pipeline appears equally often at each position in pass A.
        pos = collections.Counter((p, k) for s in seqs for k, p in enumerate(s))
        self.assertLessEqual(max(pos.values()) - min(pos.values()), 2)  # 26/27 split

    def test_cost_guard(self):
        n, total, reduced = rl.cost_guard(60.0, 1.0, 3)
        self.assertEqual((n, reduced), (80, False))
        n, total, reduced = rl.cost_guard(60.0, 120.0, 3)  # 120 s/target
        self.assertTrue(reduced)
        self.assertGreaterEqual(n, 40)
        self.assertTrue(total <= rl.COST_GUARD_S or n == 40)


class Caps(unittest.TestCase):
    def test_rlimit_cpu_kills_and_is_censored(self):
        old = (rl.CPU_CAP_S, rl.CPU_HARD_S)
        rl.CPU_CAP_S, rl.CPU_HARD_S = 1, 2
        try:
            with tempfile.TemporaryDirectory() as d:
                d = Path(d)
                child = rl.run_capped([sys.executable, "-c", "while True: pass"],
                                      d / "o", d / "e")
            self.assertEqual(child["signal"], signal.SIGXCPU)
            self.assertGreaterEqual(child["cpu_s"], 0.9)
            self.assertEqual(rl.classify_kill(child, ""), "censored_cpu")
        finally:
            rl.CPU_CAP_S, rl.CPU_HARD_S = old

    def test_rlimit_as_blocks_large_allocation(self):
        with tempfile.TemporaryDirectory() as d:
            d = Path(d)
            child = rl.run_capped([sys.executable, "-c", "x = bytearray(5 << 30)"],
                                  d / "o", d / "e")
            err = (d / "e").read_text()
        self.assertNotEqual(child["exit_code"], 0)
        self.assertEqual(rl.classify_kill(child, err), "censored_mem")


if __name__ == "__main__":
    unittest.main()
