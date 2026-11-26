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


# ── Canned M2 stdout fixtures. These tests need NO M2 installation. ──────────
_SPEC_OK = "MODE specialized\nONE_IN_IDEAL 0\nDIM 0\nGBSIZE 3\nGB x3\nGB x2\nGB x1\nDONE\n"
_SPEC_TRUNC = "MODE specialized\nONE_IN_IDEAL 0\nDIM 0\nGBSIZE 3\nGB x3\nGB x2\nGB x1\n"
_VERIFY_PASS = "NBRANCHES 9\nNMATCH 1\nVERIFY PASS\nDONE\n"
_VERIFY_FAIL = "NBRANCHES 9\nNMATCH 1\nVERIFY FAIL\nDONE\n"
_VERIFY_NOBRANCH = "NBRANCHES 9\nNMATCH 0\nVERIFY NOBRANCH\nDONE\n"
_VERIFY_NMATCH2 = "NBRANCHES 9\nNMATCH 2\nVERIFY PASS\nDONE\n"
_PARAM_OK = ("MODE parametric\nNBRANCHES 2\nBRANCH\nE 0\nN t-1\nG x+t*y\n"
             "BRANCH\nE t-1\nN 1\nG 1\nDONE\n")
_PARAM_COUNT_MISMATCH = "MODE parametric\nNBRANCHES 2\nBRANCH\nE 0\nN t-1\nG x+t*y\nDONE\n"


class ParseAndValidate(unittest.TestCase):
    """Semantic validation of M2 output. Runs without M2 (fixtures are canned),
    so the fail-closed contract is a permanent regression guard in CI."""

    def _v(self, text, mode):
        return drv.validate_result(drv.parse(text), mode)

    def test_valid_outputs_accepted(self):
        self.assertTrue(self._v(_SPEC_OK, "specialized")[0])
        self.assertTrue(self._v(_VERIFY_PASS, "verify")[0])
        self.assertTrue(self._v(_PARAM_OK, "parametric")[0])

    def test_invalid_outputs_rejected(self):
        for text, mode in [
            (_SPEC_TRUNC, "specialized"),            # no DONE (truncated)
            (_VERIFY_FAIL, "verify"),                # branch != direct solve
            (_VERIFY_NOBRANCH, "verify"),            # no matching stratum
            (_VERIFY_NMATCH2, "verify"),             # NMATCH != 1
            (_PARAM_COUNT_MISMATCH, "parametric"),   # declared != emitted branches
        ]:
            ok, reason = self._v(text, mode)
            self.assertFalse(ok, f"{mode}: expected reject, got accept")
            self.assertTrue(reason)

    def test_main_exit_codes_without_m2(self):
        import contextlib
        import io
        saved = (drv.locate_m2, drv.locate_package, drv.run_m2)
        drv.locate_m2 = lambda: "M2-stub"
        drv.locate_package = lambda: "pkg-stub"
        try:
            def run(mode, canned):
                drv.run_m2 = lambda *a, **k: canned
                with contextlib.redirect_stdout(io.StringIO()), \
                     contextlib.redirect_stderr(io.StringIO()):
                    return drv.main([mode, "--p", "101", "--k", "3"])
            ok = {"timeout": False, "rc": 0, "stderr": "", "wall": 0.1}
            self.assertEqual(run("verify", {**ok, "stdout": _VERIFY_PASS}), 0)
            self.assertEqual(run("verify", {**ok, "stdout": _VERIFY_FAIL}), 3)
            self.assertEqual(run("verify", {**ok, "stdout": _VERIFY_NMATCH2}), 3)
            self.assertEqual(run("verify", {**ok, "stdout": _VERIFY_NOBRANCH}), 3)
            self.assertEqual(run("specialized", {**ok, "stdout": _SPEC_TRUNC}), 3)
            self.assertEqual(run("specialized", {"timeout": False, "rc": 1,
                                                 "stdout": "", "stderr": "boom", "wall": 0.1}), 1)
            self.assertEqual(run("verify", {"timeout": True, "wall": 600}), 2)
        finally:
            drv.locate_m2, drv.locate_package, drv.run_m2 = saved

    def test_large_p_refused(self):
        with self.assertRaises(SystemExit):
            drv.base_set_and_target(drv.TOY_P_MAX + 1, 1, 1, 3)


if __name__ == "__main__":
    unittest.main()
