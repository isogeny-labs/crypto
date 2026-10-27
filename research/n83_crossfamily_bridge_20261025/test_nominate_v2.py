"""Adaptive bridge nomination checks on admitted panel shapes."""

import copy
import unittest

from research.n83_crossfamily_bridge_20261025 import nominate_v2
from research.n83_crossfamily_bridge_20261025.test_nominate import (
    nearby_panel, wide_panel,
)
from research.n83_crossfamily_current_20261025 import arms as wide_arms
from research.n83_crossfamily_current_20261025 import select as wide_select
from research.n83_nearby_grid_confirm16_20261024 import aggregate
from research.n83_nearby_grid_tier1_20261024 import protocol as nearby_catalog


def unresolved_nearby():
    panel = copy.deepcopy(nearby_panel())
    selected = panel["selected_ks"]
    online = {k: {} for k in selected}
    cold = {k: {} for k in selected}
    for row in panel["rows"]:
        k, index = row["k"], row["point_index"]
        row["ic_online_ms"] = float(20 + (index % 3))
        row["ic_cold_process_ms"] = float(1000 + (index % 3))
        online[k][index] = row["ic_online_ms"]
        cold[k][index] = row["ic_cold_process_ms"]
    alpha = panel["alpha_per_unordered_pair"]
    panel["online"] = aggregate.select_metric(selected, online, "online", alpha)
    panel["cold"] = aggregate.select_metric(selected, cold, "cold", alpha)
    return panel


def unresolved_wide():
    panel = wide_panel()
    for point in panel["points"]:
        for arm in point["arms"]:
            arm["one_target_online_ms"] = 25.0
            arm["cold_process_ms"] = 1500.0
    return panel


class AdaptiveNominationTests(unittest.TestCase):
    def test_admitted_unresolved_panels_nominate_bounded_frontier(self):
        nearby = unresolved_nearby()
        wide = unresolved_wide()
        self.assertEqual(nearby["online"]["status"], "BOUNDED_UNKNOWN")
        self.assertEqual(wide_select.select(wide)["online"]["status"],
                         "BOUNDED_UNKNOWN")
        result = nominate_v2.nominate(nearby, wide)
        self.assertEqual(result["status"], "READY_FOR_BRIDGE_DISPATCH")
        self.assertEqual(result["small_shortlist_ks"], [1107, 1110, 1115])
        self.assertEqual(result["wide_shortlist_candidate_ids"],
                         sorted([wide_arms.ARMS[0][3], wide_arms.ARMS[1][3]]))
        self.assertEqual(len(result["candidate_ids"]), 8)
        self.assertEqual(result["candidate_manifest_sha256"][
            nearby_catalog.CANDIDATES[1107][0]],
            nearby_catalog.CANDIDATES[1107][1])

    def test_changed_row_fails_admission(self):
        nearby = unresolved_nearby()
        nearby["rows"][0]["ic_online_ms"] *= 2
        result = nominate_v2.nominate(nearby, unresolved_wide())
        self.assertEqual(result["status"], "BOUNDED_UNKNOWN")
        self.assertEqual(result["reason"],
                         "NEARBY_PANEL_NOT_ADMITTED_OR_CHANGED")

    def test_wide_identity_and_replay_gates(self):
        wide = unresolved_wide()
        wide["choice"]["candidate_manifest_sha256"][wide_arms.ARMS[0][3]] = "0" * 64
        result = nominate_v2.nominate(unresolved_nearby(), wide)
        self.assertEqual(result["reason"], "WIDE_PANEL_NOT_ADMITTED")
        wide = unresolved_wide()
        wide["points"][0]["arms"][0]["status"] = "TIMEOUT"
        result = nominate_v2.nominate(unresolved_nearby(), wide)
        self.assertEqual(result["reason"], "WIDE_PANEL_NOT_ADMITTED")

    def test_selected_panels_remain_eligible(self):
        result = nominate_v2.nominate(nearby_panel(), wide_panel())
        self.assertEqual(result["status"], "READY_FOR_BRIDGE_DISPATCH")
        self.assertEqual(result["nearby_online_selection_status"],
                         "PASS_SELECTED")
        self.assertEqual(result["wide_online_selection_status"], "SELECTED")


if __name__ == "__main__":
    unittest.main()
