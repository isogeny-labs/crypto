#!/usr/bin/env python3
"""Check all fifteen frozen contrasts and fail-closed Tier 1 admission."""

import copy
import json
import math

from research.n83_online_finalists_20261018 import select as subject


def fixture():
    points = []
    for index in range(subject.N):
        online = (60.0, 50.0, 40.0, 30.0, 20.0, 10.0)
        cold = (120_000.0, 110_000.0, 100_000.0, 90_000.0, 80_000.0, 70_000.0)
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
                {"point_indices": list(range(i * 8, (i + 1) * 8)),
                 "tier_status": "PASS_TIER1",
                 "aa_status": "PASS_10_PERCENT_PER_PAIR"}
                for i in range(4)],
            "points": points}


def main():
    minimum = 2 / (1 << subject.N)
    assert len(set(subject.IDS)) == 6
    assert subject.sign_flip_p([1.0] * subject.N) == minimum
    assert subject.sign_flip_p([-1.0] * subject.N) == minimum
    assert subject.sign_flip_p([1.0, -1.0] * 16) == 1.0
    data = fixture()
    result = subject.select(data)
    assert result["admission_status"] == "PASS"
    assert result["online"]["selected_candidate_id"] == subject.IDS[5]
    assert result["cold"]["selected_candidate_id"] == subject.IDS[5]
    assert len(result["online"]["comparisons"]) == 15
    assert len(result["cold"]["comparisons"]) == 15
    assert all(item["two_sided_sign_flip_p"] == minimum
               for item in result["online"]["comparisons"])
    assert math.isclose(result["online"]["mean_by_candidate_ms"][subject.IDS[5]],
                        10.0)
    assert all(item["status"] == "PASS_ALL_EIGHT_SAME_POINT"
               for host in result["tier1_speedup_by_host"].values()
               for item in host.values())
    tied = copy.deepcopy(data)
    for point in tied["points"]:
        point["arms"][5]["one_target_online_ms"] = 20.0
    assert subject.select(tied)["online"]["selected_candidate_id"] is None
    noisy = copy.deepcopy(data)
    noisy["host_strata"][2]["aa_status"] = "NOISY_OR_INCOMPLETE"
    assert subject.select(noisy)["online"]["selected_candidate_id"] is None
    failed = copy.deepcopy(data)
    failed["points"][0]["arms"][5]["resource_status"] = "OOM"
    assert subject.select(failed)["admission_status"] == (
        "IC_REPLAY_RESOURCE_OR_TIMING_GATE")
    original = subject.sign_flip_p
    try:
        def unstable(_differences):
            raise ValueError("half-list counts differ")
        subject.sign_flip_p = unstable
        numerical = subject.select(data)
        assert numerical["admission_status"] == "STATISTICAL_REPLAY_NUMERIC_MISMATCH"
        assert numerical["online"]["selected_candidate_id"] is None
        assert numerical["cold"]["selected_candidate_id"] is None
    finally:
        subject.sign_flip_p = original
    print(json.dumps({"status": "PASS_ONLINE_FINALISTS_SELECTOR",
                      "cases": 13, "exact_minimum_p": minimum}, sort_keys=True))


if __name__ == "__main__":
    main()
