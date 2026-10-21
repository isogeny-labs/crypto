#!/usr/bin/env python3
"""Check frozen six-arm identities, fixture, schedule, and flags."""

import hashlib
import json
from pathlib import Path

from research.n83_online_finalists_20261018 import run, select, verify

HERE = Path(__file__).resolve().parent


def main():
    _, lines = run.fixture()
    _, points, scalars = verify.fixture_points()
    arms, choice = run.choose()
    assert len(lines) == len(points) == len(scalars) == 32
    assert tuple(arm[3] for arm in arms) == select.IDS
    assert choice["schema"] == "n83.online-finalists-choice/v1"
    events = [event for stratum in range(4)
              for event in run.schedule(stratum, arms)]
    assert len(events) == 1536
    for index in range(32):
        point_events = [event for event in events if event["point_index"] == index]
        assert len(point_events) == 48
        assert [event["candidate"] for event in point_events
                if event["kind"] == "ic"] == [
                    arm[0] for arm in arms[index % 6:] + arms[:index % 6]]
        assert len([event for event in point_events if event["kind"] == "rho"]) == 6
        assert len([event for event in point_events
                    if event["kind"] == "calibration"]) == 36
    for name, k, layout, candidate_id in arms:
        manifest = json.loads((HERE / f"candidate_{candidate_id}.json").read_text())
        stripped = {key: value for key, value in manifest.items()
                    if key != "candidate_id"}
        canonical = hashlib.sha256(json.dumps(
            stripped, sort_keys=True, separators=(",", ":"),
            ensure_ascii=False).encode()).hexdigest()[:12]
        flags = run.ic_environment(HERE / "scratch", layout, name)
        assert candidate_id.endswith("h" + canonical)
        assert manifest["factor_base"]["actual_usable_points"] == 166 * k
        assert manifest["factor_base"]["effective_columns"] == k
        assert manifest["implementation"]["source_sha256"]["ic"] == run.SOURCE_SHA256
        assert manifest["implementation"]["algorithm_flags"]["target_threads"] == 2
        assert manifest["implementation"]["algorithm_flags"]["target_cancel"] is True
        assert manifest["implementation"]["algorithm_flags"]["root_batch"] is True
        assert flags["KIC_TARGET_THREADS"] == "2"
        assert flags["KIC_TARGET_CANCEL"] == "1"
        assert flags["KIC_INDEXED5_ID30"] == "0"
        assert flags["KIC_INDEX_ROOT_BATCH"] == "1"
        if layout == "root11_dynamic":
            assert (flags["KIC_INDEXED_ROOT5"], flags["KIC_INDEXED5_LOAD75"],
                    flags["KIC_ROOT_BLOOM"], flags["KIC_ROOT_ONLY11"]) == (
                        "1", "1", "0", "1")
        elif layout == "root11_wide3":
            assert (flags["KIC_INDEXED_ROOT5"], flags["KIC_INDEXED5_FIXED30"],
                    flags["KIC_ROOT_BLOOM"], flags["KIC_ROOT_BLOOM_PROFILE"]) == (
                        "1", "1", "1", "wide3")
        elif layout == "root4_standard":
            assert (flags["KIC_INDEXED_ROOT4"], flags["KIC_INDEXED4_FIXED31"],
                    flags["KIC_ROOT_BLOOM"], flags["KIC_ROOT_BLOOM_PROFILE"]) == (
                        "1", "1", "1", "standard")
        else:
            raise AssertionError(f"unsupported layout: {layout}")
    audit = json.loads((HERE / "fresh_exclusion_audit.json").read_text())
    assert audit["status"] == "PASS_ZERO_OVERLAP"
    assert audit["fresh_targets_sha256"] == run.TARGETS_SHA256
    print(json.dumps({"status": "PASS_ONLINE_FINALISTS_PREFLIGHT",
                      "points": 32, "arms": 6, "events": len(events),
                      "candidate_ids": list(select.IDS)}, sort_keys=True))


if __name__ == "__main__":
    main()
