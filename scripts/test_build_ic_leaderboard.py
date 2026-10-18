#!/usr/bin/env python3
"""Tests for the GAE-pin flag in scripts/build_ic_leaderboard.py.

A row whose source records an empty `calibration_pinned.pinned` list, or no pin
record, converted its native counters to GAE at host-measured ratios; it is
flagged and no ranking sentence may name it.  Rows priced in a clock unit make
no GAE conversion and are not flagged.  Stdlib only; reads no research file."""
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import build_ic_leaderboard as b  # noqa: E402


def row(slug, regime, log2_r, ratio, pinned=("ns_per_add",), gae=True, artefact=False):
    best = {"ratio_rho": ratio, "s": 1.0, "label": "x"}
    if artefact:
        best["construction_artefact"] = True
    return {"slug": slug, "regime": regime, "log2_r": log2_r, "best": best,
            "gae_pin": b.gae_pin_state(pinned, priced_in_gae=gae)}


class PinState(unittest.TestCase):
    def test_empty_list_is_unpinned(self):
        self.assertTrue(b.gae_pin_state([])["unpinned"])

    def test_missing_record_is_unpinned(self):
        self.assertTrue(b.gae_pin_state(None)["unpinned"])

    def test_nonempty_list_is_pinned(self):
        s = b.gae_pin_state(["ns_per_double"])
        self.assertFalse(s["unpinned"])
        self.assertEqual(s["pinned"], ["ns_per_double"])

    def test_clock_priced_row_is_not_flagged(self):
        s = b.gae_pin_state(None, priced_in_gae=False)
        self.assertFalse(s["unpinned"])
        self.assertFalse(s["priced_in_gae"])


class Leaders(unittest.TestCase):
    def board(self, flagged_pinned):
        return [row("close-unpinned", "prime", 10.0, 1.5, pinned=flagged_pinned),
                row("mid-pinned", "prime", 12.0, 3.0),
                row("far-pinned", "prime", 14.0, 9.0),
                row("big-unpinned", "prime", 30.0, 2.0, pinned=flagged_pinned),
                row("c2", "char2", 10.0, 4.0), row("k", "koblitz", 10.0, 5.0, gae=False),
                row("kb", "koblitz_batch", 10.0, 6.0, gae=False)]

    def test_unpinned_rows_are_never_named(self):
        for flagged in ([], None):
            lead = b.leaders_of(self.board(flagged))["prime"]
            self.assertEqual(lead["slug"], "mid-pinned")
            self.assertEqual(lead["ratio_rho"], 3.0)
            self.assertEqual(lead["largest"]["slug"], "far-pinned")

    def test_pinned_rows_rank_as_before(self):
        lead = b.leaders_of(self.board(["ns_per_add"]))["prime"]
        self.assertEqual(lead["slug"], "close-unpinned")
        self.assertEqual(lead["largest"]["slug"], "big-unpinned")

    def test_all_unpinned_regime_refuses_a_ranking(self):
        board = [row("a", "prime", 10.0, 1.5, pinned=[]), row("c2", "char2", 10.0, 4.0),
                 row("k", "koblitz", 10.0, 5.0, gae=False), row("kb", "koblitz_batch", 10.0, 6.0, gae=False)]
        with self.assertRaises(SystemExit):
            b.leaders_of(board)

    def test_clock_rows_rank_without_pins(self):
        lead = b.leaders_of(self.board(["ns_per_add"]))
        self.assertEqual(lead["koblitz"]["slug"], "k")
        self.assertEqual(lead["koblitz_batch"]["slug"], "kb")


if __name__ == "__main__":
    unittest.main()
