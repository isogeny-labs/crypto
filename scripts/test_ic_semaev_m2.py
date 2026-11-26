"""Correctness tests for the Macaulay2 ComprehensiveGBs Semaev backend.

Correctness controls only -- nothing here is performance evidence.

These exercise `scripts/ic_semaev_m2.py`. They require the external tools the
backend shells out to:

  * ``M2`` on PATH (e.g. ``nix profile install nixpkgs#macaulay2``);
  * ``CGB_PACKAGE_PATH`` -> the ComprehensiveGBs.m2 package file (from the
    Macaulay2/M2 repository; not vendored here).

When either is absent the tests SKIP (they do not fail), mirroring the other
optional-tool tests in this repository.

The core check (`verify` mode): the comprehensive Groebner system computed once
over ALL targets, when specialized to the actual target `t = xQ`, generates the
same ideal as the direct per-target Groebner basis. If that held only sometimes,
the parametric backend would be wrong; it must hold on every instance.
"""

import os
import shutil
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import ic_semaev_m2 as drv  # noqa: E402


def _deps():
    m2 = shutil.which("M2")
    pkg = os.environ.get("CGB_PACKAGE_PATH")
    pkg = pkg if (pkg and os.path.isfile(pkg)) else None
    return m2, pkg


class ComprehensiveGBsBackend(unittest.TestCase):
    def setUp(self):
        self.m2, self.pkg = _deps()
        if not self.m2 or not self.pkg:
            self.skipTest("M2 and/or CGB_PACKAGE_PATH not available")

    def test_specialized_gb_is_zero_dimensional(self):
        xset, xq = drv.base_set_and_target(101, 1, 1, 3, None)
        script = drv.gen_specialized(101, 1, 1, xset, xq, self.pkg)
        res = drv.run_m2(script, self.m2, 300)
        self.assertFalse(res.get("timeout"), "M2 timed out on specialized solve")
        self.assertEqual(res["rc"], 0, res.get("stderr", ""))
        out = drv.parse(res["stdout"])
        self.assertTrue(out.get("done"))
        self.assertEqual(out["mode"], "specialized")
        self.assertEqual(out["dim"], 0, f"expected 0-dim variety, got {out}")
        self.assertGreaterEqual(out["gbsize"], 1)

    def test_parametric_branch_matches_direct_solve(self):
        # (prime, k, method). Both ComprehensiveGBs algorithms, two primes.
        cases = [
            (101, 3, "parametricGBMain"),
            (197, 3, "parametricGBMain"),
            (101, 3, "comprehensiveGBMain"),
        ]
        for p, k, method in cases:
            with self.subTest(p=p, k=k, method=method):
                xset, xq = drv.base_set_and_target(p, 1, 1, k, None)
                script = drv.gen_verify(p, 1, 1, xset, xq, self.pkg, method)
                res = drv.run_m2(script, self.m2, 400)
                self.assertFalse(res.get("timeout"), f"timeout p={p} k={k}")
                self.assertEqual(res["rc"], 0, res.get("stderr", ""))
                out = drv.parse(res["stdout"])
                # Exactly one disjoint stratum contains the target.
                self.assertEqual(out.get("nmatch"), 1, f"p={p} k={k}: {out}")
                # Its specialized basis == the direct Groebner basis.
                self.assertEqual(out.get("verify"), "PASS", f"p={p} k={k}: {out}")

    def test_missing_package_env_exits_cleanly(self):
        # Dependency detection: no CGB_PACKAGE_PATH -> clean SystemExit, not a crash.
        saved = os.environ.pop("CGB_PACKAGE_PATH", None)
        try:
            with self.assertRaises(SystemExit):
                drv.locate_package()
        finally:
            if saved is not None:
                os.environ["CGB_PACKAGE_PATH"] = saved


if __name__ == "__main__":
    unittest.main()
