#!/usr/bin/env python3
"""Check frozen factorial identities, fixture, schedule, and worker flags."""

import hashlib
import json
from pathlib import Path

from research.n83_highk_cancel_pilot_20261017 import run_fresh, select_fresh
from research.n83_highk_cancel_pilot_20261017 import verify_fresh

HERE = Path(__file__).resolve().parent


def main():
    _, lines = run_fresh.fixture()
    _, points, scalars = verify_fresh.fixture_points()
    arms, choice = run_fresh.choose()
    assert len(lines) == len(points) == len(scalars) == 24
    assert tuple(arm[3] for arm in arms) == select_fresh.IDS
    assert choice["schema"] == "n83.highk-cancel-factorial-choice/v1"
    events = [event for stratum in range(4)
              for event in run_fresh.schedule(stratum, arms)]
    assert len(events) == 768
    for index in range(24):
        point_events = [event for event in events if event["point_index"] == index]
        assert len(point_events) == 32
        assert [event["candidate"] for event in point_events
                if event["kind"] == "ic"] == [
                    arm[0] for arm in arms[index % 4:] + arms[:index % 4]]
        assert len([event for event in point_events if event["kind"] == "rho"]) == 4
        assert len([event for event in point_events
                    if event["kind"] == "calibration"]) == 24
    for name, k, layout, candidate_id in arms:
        manifest = json.loads((HERE / f"candidate_{candidate_id}.json").read_text())
        stripped = {key: value for key, value in manifest.items()
                    if key != "candidate_id"}
        canonical = hashlib.sha256(json.dumps(
            stripped, sort_keys=True, separators=(",", ":"),
            ensure_ascii=False).encode()).hexdigest()[:12]
        flags = run_fresh.ic_environment(HERE / "scratch", layout, name)
        assert candidate_id.endswith("h" + canonical)
        assert manifest["factor_base"]["actual_usable_points"] == 166 * k
        assert manifest["factor_base"]["effective_columns"] == k
        assert manifest["implementation"]["source_sha256"]["ic"] == run_fresh.SOURCE_SHA256
        assert manifest["implementation"]["algorithm_flags"]["target_threads"] == 2
        assert manifest["implementation"]["algorithm_flags"]["target_cancel"] == name.endswith("_cancel")
        assert flags["KIC_TARGET_THREADS"] == "2"
        assert flags["KIC_TARGET_CANCEL"] == ("1" if name.endswith("_cancel") else "0")
    audit = json.loads((HERE / "fresh_exclusion_audit.json").read_text())
    assert audit["status"] == "PASS_ZERO_OVERLAP"
    assert audit["fresh_targets_sha256"] == run_fresh.TARGETS_SHA256
    print(json.dumps({"status": "PASS_FRESH_FACTORIAL_PREFLIGHT",
                      "points": 24, "arms": 4, "events": len(events),
                      "candidate_ids": list(select_fresh.IDS)}, sort_keys=True))


if __name__ == "__main__":
    main()
