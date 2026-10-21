#!/usr/bin/env python3
"""Check the preregistered factorial selector and its admission gates."""

import copy
import json
import math

from research.n83_highk_cancel_pilot_20261017 import select_fresh as subject


def fixture():
    points = []
    for index in range(subject.N):
        online = (20.0, 10.0, 15.0, 5.0)
        cold = (100_000.0, 100_000.0, 120_000.0, 120_000.0)
        arms = [{"candidate_id": candidate,
                 "status": "PASS_INDEPENDENT_REPLAY",
                 "resource_status": "PASS",
                 "one_target_online_ms": online[arm],
                 "cold_process_ms": cold[arm]}
                for arm, candidate in enumerate(subject.IDS)]
        points.append({"point_index": index, "arms": arms,
                       "rho_by_candidate": {
                           candidate: {"status": "PASS_INDEPENDENT_REPLAY",
                                       "resource_status": "PASS",
                                       "online_ms": 100.0}
                           for candidate in subject.IDS},
                       "calibration_by_candidate": {
                           candidate: {"status": "PASS_10_PERCENT"}
                           for candidate in subject.IDS}})
    return {"schema": subject.SCHEMA,
            "source_sha256": subject.SOURCE_SHA256,
            "targets_sha256": subject.TARGETS_SHA256,
            "candidate_ids": list(subject.IDS),
            "evidence_status": "PASS_INDEPENDENT_REPLAY",
            "different_cloud_instances": True,
            "same_candidate_bases_across_strata": True,
            "host_strata": [
                {"point_indices": list(range(i * 6, (i + 1) * 6)),
                 "tier_status": "PASS_TIER1",
                 "aa_status": "PASS_10_PERCENT_PER_PAIR"}
                for i in range(4)],
            "points": points}


def main():
    minimum = 2 / (1 << subject.N)
    assert subject.sign_flip_p([1.0] * subject.N) == minimum
    assert subject.sign_flip_p([-1.0] * subject.N) == minimum
    assert subject.sign_flip_p([1.0, -1.0] * 12) == 1.0
    data = fixture()
    result = subject.select(data)
    assert result["admission_status"] == "PASS"
    assert result["online"]["selected_candidate_id"] == subject.IDS[3]
    assert result["cold"]["selected_candidate_id"] == subject.IDS[1]
    assert result["online"]["two_sided_sign_flip_p"] == minimum
    assert math.isclose(result["online"]["geometric_second_over_first"], 0.5)
    assert all(item["status"] == "PASS_THRESHOLD"
               for item in result["policy_contrasts"].values())
    assert all(item["status"] == "PASS_ALL_SIX_SAME_POINT"
               for host in result["tier1_speedup_by_host"].values()
               for item in host.values())
    noisy = copy.deepcopy(data)
    noisy["host_strata"][2]["aa_status"] = "NOISY_OR_INCOMPLETE"
    assert subject.select(noisy)["online"]["selected_candidate_id"] is None
    failed = copy.deepcopy(data)
    failed["points"][0]["arms"][3]["resource_status"] = "OOM"
    assert subject.select(failed)["admission_status"] == (
        "IC_REPLAY_RESOURCE_OR_TIMING_GATE")
    print(json.dumps({"status": "PASS_FROZEN_FACTORIAL_SELECTOR",
                      "cases": 10, "exact_minimum_p": minimum}, sort_keys=True))


if __name__ == "__main__":
    main()
