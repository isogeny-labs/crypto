#!/usr/bin/env python3
"""Exercise selected, fallback and changed-evidence holdout choices."""

import copy
import json
from pathlib import Path
from tempfile import TemporaryDirectory

from research.n83_fresh_cold_binary_retest_20261021 import arms
from research.n83_online_finalists_20261018 import select as six_select
from research.n83_online_finalists_20261018.test_select import fixture


def files(directory, validated):
    validated["run_id"] = 39160284543
    validated["head_sha"] = "f152eb7d5e5de660fb16902d2abfe7cbd3ff331a"
    selection = six_select.select(validated)
    first, second = directory / "validated.json", directory / "selection.json"
    first.write_text(json.dumps(validated, sort_keys=True) + "\n")
    second.write_text(json.dumps(selection, sort_keys=True) + "\n")
    return first, second


def main():
    with TemporaryDirectory() as temporary:
        directory = Path(temporary)
        chosen, choice = arms.choose(*files(directory, fixture()))
        assert [arm[3] for arm in chosen] == [
            arms.SMALL[0][3], arms.SMALL[1][3], arms.MID[0][3],
            six_select.IDS[5]]
        assert not choice["online_fallback_used"]
        assert not choice["cold_fallback_used"]
        for candidate in choice["ordered_candidate_ids"]:
            assert candidate in choice["candidate_manifest_sha256"]

        mixed = fixture()
        for point in mixed["points"]:
            for index, arm in enumerate(point["arms"]):
                arm["cold_process_ms"] = 70_000 + 10_000 * index
        chosen, choice = arms.choose(*files(directory, mixed))
        assert [arm[3] for arm in chosen] == [
            arms.SMALL[0][3], arms.SMALL[1][3], arms.MID[0][3],
            six_select.IDS[5], six_select.IDS[0]]
        assert not choice["online_fallback_used"]
        assert not choice["cold_fallback_used"]

        fallback = fixture()
        for point in fallback["points"]:
            for arm in point["arms"]:
                arm["one_target_online_ms"] = 50.0
                arm["cold_process_ms"] = 100_000.0
        chosen, choice = arms.choose(*files(directory, fallback))
        assert [arm[3] for arm in chosen] == [
            arms.SMALL[0][3], arms.SMALL[1][3], arms.MID[0][3],
            arms.ONLINE_FALLBACK, arms.COLD_FALLBACK]
        assert choice["online_fallback_used"] and choice["cold_fallback_used"]

        noisy = fixture()
        noisy["host_strata"][0]["aa_status"] = "NOISY_OR_INCOMPLETE"
        chosen, choice = arms.choose(*files(directory, noisy))
        assert choice["upstream_admission_status"] == "HOST_STRATA_GATE"
        assert [arm[3] for arm in chosen] == [
            arms.SMALL[0][3], arms.SMALL[1][3], arms.MID[0][3],
            arms.ONLINE_FALLBACK, arms.COLD_FALLBACK]

        bad = copy.deepcopy(fallback)
        bad["different_cloud_instances"] = False
        try:
            arms.choose(*files(directory, bad))
        except ValueError as error:
            assert "not admitted" in str(error)
        else:
            raise AssertionError("failed provider gate chose arms")

        first, second = files(directory, fallback)
        selected = json.loads(second.read_text())
        selected["online"]["selected_candidate_id"] = six_select.IDS[5]
        second.write_text(json.dumps(selected) + "\n")
        try:
            arms.choose(first, second)
        except ValueError as error:
            assert "selector differs" in str(error)
        else:
            raise AssertionError("changed selector chose arms")
    print(json.dumps({"status": "PASS_FINAL_EXTENDED_ARMS", "cases": 6}))


if __name__ == "__main__":
    main()
