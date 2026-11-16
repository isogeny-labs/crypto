#!/usr/bin/env python3
"""Tests for tools/loss_ledger.py. Run: python3 -m unittest tools.test_loss_ledger"""
import contextlib
import io
import json
import os
import sys
import tempfile
import unittest

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import loss_ledger as ll  # noqa: E402


class LedgerTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.d = self.tmp.name

    def add(self, slug, cause, day="2026-11-09", hours=None, usd=None, route="other"):
        argv = ["add", "--slug", slug, "--route", route, "--cause", cause, "--what", "x",
                "--source", "docs/session-logs/d.md#h", "--observed-at", day]
        if hours is not None:
            argv += ["--lost-cpu-hours", str(hours)]
        if usd is not None:
            argv += ["--lost-usd", str(usd)]
        with contextlib.redirect_stdout(io.StringIO()):
            return ll.main(argv, loss_dir=self.d)

    def test_add_writes_record(self):
        self.assertEqual(self.add("a", "quota", hours=2), 0)
        r = json.load(open(os.path.join(self.d, "20261109-a.json")))
        self.assertEqual((r["schema"], r["pcc"], r["lost_usd"]), (ll.SCHEMA, "[pcc:unverified]", None))
        self.assertEqual(r["lost_usd_derivation"], "not recorded")

    def test_unknown_cause_refused(self):
        with contextlib.redirect_stderr(io.StringIO()):
            self.assertEqual(self.add("a", "gremlins"), 2)
        self.assertEqual(os.listdir(self.d), [])

    def test_unknown_route_and_duplicate_refused(self):
        with contextlib.redirect_stderr(io.StringIO()):
            self.assertEqual(self.add("a", "quota", route="mars"), 2)
            self.assertEqual(self.add("a", "quota"), 0)
            self.assertEqual(self.add("a", "quota"), 2)

    def test_report_and_top_cause(self):
        self.add("a", "spot_preemption", hours=3, usd=4)
        self.add("b", "spot_preemption", hours=1)
        self.add("c", "quota", usd=100)
        rows = ll.report(ll.load(self.d))
        sp = rows[0]
        self.assertEqual((sp["cause"], sp["count"], sp["lost_cpu_hours"], sp["lost_usd"], sp["unquantified"]),
                         ("spot_preemption", 2, 4.0, 4.0, 0))
        self.assertEqual(ll.top_cause(rows)["cause"], "spot_preemption")

    def test_since_filter(self):
        self.add("a", "quota", day="2026-11-01")
        self.add("b", "other", day="2026-11-09")
        rows = ll.report(ll.load(self.d), "2026-11-05")
        self.assertEqual([r["cause"] for r in rows], ["other"])
        self.assertIsNone(ll.top_cause(ll.report(ll.load(self.d), "2027-01-01")))

    def test_json_output(self):
        self.add("a", "quota")
        buf = io.StringIO()
        with contextlib.redirect_stdout(buf):
            ll.main(["report", "--json"], loss_dir=self.d)
        self.assertEqual(json.loads(buf.getvalue())[0]["cause"], "quota")


if __name__ == "__main__":
    unittest.main()
