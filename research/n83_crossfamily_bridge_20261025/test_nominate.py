"""Controls for bridge nomination from complete paired panel rows."""

import copy
import unittest

from research.n83_crossfamily_bridge_20261025 import nominate
from research.n83_crossfamily_current_20261025 import arms as wide_arms
from research.n83_nearby_grid_confirm16_20261024 import aggregate as nearby_aggregate
from research.n83_nearby_grid_confirm16_20261024 import protocol as nearby_protocol
from research.n83_nearby_grid_tier1_20261024 import protocol as nearby_catalog


def nearby_panel():
    selected = [1107, 1110, 1115]
    _, points = nearby_protocol.fixture()
    online = {k: {i: float(value) for i in range(16)}
              for k, value in ((1107, 10), (1110, 20), (1115, 30))}
    cold = {k: {i: float(value) for i in range(16)}
            for k, value in ((1107, 30), (1110, 20), (1115, 10))}
    alpha = 0.05 / 3
    return {
        "schema": "n83.nearby-grid-confirm16-aggregate/v1",
        "status": "PASS_TIER1_CONFIRM16", "selected_ks": selected,
        "stratum_statuses": ["STRATUM_ADMITTED"] * 2,
        "host_instance_ids": ["host-0", "host-1"],
        "runner_names": ["runner-0", "runner-1"],
        "one_target_rows_verified": 16 * len(selected),
        "alpha_per_unordered_pair": alpha,
        "online": nearby_aggregate.select_metric(selected, online,
                                                  "online", alpha),
        "cold": nearby_aggregate.select_metric(selected, cold,
                                                "cold", alpha),
        "rows": [{"point_index": i, "k": k, "stratum": i // 8,
                  "status": "PASS_INDEPENDENT_REPLAY",
                  "candidate_id": nearby_catalog.CANDIDATES[k][0],
                  "workload_id": points[i][3],
                  "run_id": f"{nearby_catalog.CANDIDATES[k][0]}W{points[i][3]}R1",
                  "ic_online_ms": online[k][i],
                  "ic_cold_process_ms": cold[k][i]}
                 for i in range(16) for k in selected],
    }


def wide_panel():
    ids = [arm[3] for arm in wide_arms.ARMS]
    return {
        "schema": "n83.crossfamily-current-validated/v1",
        "source_sha256": wide_arms.SOURCE_SHA256,
        "targets_sha256":
        "fd56a59079fc6cded14539320e0cc7ab88deda0f76fa4fbaf6b663c0cd7ac1b6",
        "evidence_status": "PASS_INDEPENDENT_REPLAY",
        "different_cloud_instances": True,
        "same_candidate_bases_across_strata": True,
        "candidate_ids": ids, "choice": wide_arms.choose()[1],
        "host_strata": [
            {"point_indices": list(range(start, start + 8)),
             "tier_status": "PASS_TIER1",
             "aa_status": "PASS_10_PERCENT_PER_PAIR"}
            for start in (0, 8)],
        "points": [
            {"point_index": index,
             "arms": [{"candidate_id": candidate,
                       "status": "PASS_INDEPENDENT_REPLAY",
                       "resource_status": "PASS",
                       "one_target_online_ms": float(40 - 10 * j),
                       "cold_process_ms": float(10 + 10 * j)}
                      for j, candidate in enumerate(ids)],
             "rho_by_candidate": {
                 candidate: {"status": "PASS_INDEPENDENT_REPLAY",
                             "resource_status": "PASS", "online_ms": 1000.0}
                 for candidate in ids},
             "calibration_by_candidate": {
                 candidate: {"status": "PASS_10_PERCENT"}
                 for candidate in ids}}
            for index in range(16)],
    }


class NominationTests(unittest.TestCase):
    def test_complete_panels_nominate_exact_union(self):
        result = nominate.nominate(nearby_panel(), wide_panel())
        self.assertEqual(result["status"], "READY_FOR_BRIDGE_DISPATCH")
        self.assertEqual(len(result["candidate_ids"]), 4)
        self.assertEqual(set(result["candidate_ids"]), {
            nearby_catalog.CANDIDATES[1107][0],
            nearby_catalog.CANDIDATES[1115][0],
            wide_arms.ARMS[0][3], wide_arms.ARMS[3][3]})

    def test_changed_nearby_row_fails_closed(self):
        panel = nearby_panel()
        panel["rows"][0]["ic_online_ms"] *= 2
        self.assertEqual(nominate.nominate(panel, wide_panel())["status"],
                         "BOUNDED_UNKNOWN")

    def test_wide_source_and_inconclusive_selection_fail_closed(self):
        wide = wide_panel()
        wide["source_sha256"] = "0" * 64
        self.assertEqual(nominate.nominate(nearby_panel(), wide)["status"],
                         "BOUNDED_UNKNOWN")
        nearby = copy.deepcopy(nearby_panel())
        nearby["online"]["status"] = "BOUNDED_UNKNOWN"
        self.assertEqual(nominate.nominate(nearby, wide_panel())["status"],
                         "BOUNDED_UNKNOWN")


if __name__ == "__main__":
    unittest.main()
