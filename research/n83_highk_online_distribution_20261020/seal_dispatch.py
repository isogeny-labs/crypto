#!/usr/bin/env python3
"""Seal source, candidates, fixtures and previous result for one Pod run."""

import hashlib
import json
from pathlib import Path

from research.n83_highk_online_distribution_20261020 import arms, run
from research.n83_highk_online_distribution_20261020 import check_dispatch

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def build():
    run.fixture()  # Includes the exact pre-parallel IC source gate.
    chosen, _ = arms.choose()
    if (len(chosen) != 5 or len(run.screen_schedule()) != 262 or
            len(run.holdout_schedule(chosen[0][3])) != 64):
        raise ValueError("arm count or schedule differs")
    prior_selection_sha, prior_validated_sha = check_dispatch.prior_result()
    paths = [
        ROOT / "Cargo.lock",
        ROOT / "examples/koblitz_orbit_dlp_fast.rs",
        ROOT / "examples/n83_rho_optimization_eval.rs",
        ROOT / "research/n83_indexed5_20261013/analyze.py",
        ROOT / "research/n83_smallk_challenge_20261017/run.py",
        ROOT / "research/n83_online_finalists_20261018/run.py",
        ROOT / "research/n83_root4_highk_tier1_retry_20261019/select.py",
        ROOT / "research/n83_k3072_current_source_20261017/receipt_K3450_wide3.json",
        ROOT / "research/n83_k3072_current_source_20261017/receipt_K3500_root4.json",
    ]
    paths.extend(arms.manifest(arm)[0] for arm in chosen)
    paths.extend(path for path in HERE.iterdir()
                 if path.is_file() and path.name != "dispatch.json")
    hashes = {str(path.relative_to(ROOT)): digest(path)
              for path in sorted(set(paths))}
    return {
        "schema": "n83.online-distribution-dispatch/v1",
        "status": "SEALED_FOR_DISPATCH",
        "source_commit": check_dispatch.SOURCE_COMMIT,
        "source_sha256": run.SOURCE_SHA256,
        "screen_targets_sha256": run.SCREEN_TARGETS_SHA256,
        "holdout_targets_sha256": run.HOLDOUT_TARGETS_SHA256,
        "prior_selection_sha256": prior_selection_sha,
        "prior_validated_sha256": prior_validated_sha,
        "ordered_candidate_ids": [arm[3] for arm in chosen],
        "screen_point_count": 192, "holdout_point_count": 8,
        "screen_event_count": 262, "holdout_event_count": 64,
        "producer_cap_seconds": run.GLOBAL_PRODUCER_CAP_SECONDS,
        "sha256": hashes,
    }


def seal():
    path = HERE / "dispatch.json"
    blob = (json.dumps(build(), indent=2, sort_keys=True) + "\n").encode()
    if path.exists() and path.read_bytes() != blob:
        raise ValueError("sealed distribution dispatch differs")
    path.write_bytes(blob)
    print(json.dumps({"status": "SEALED_FOR_DISPATCH",
                      "dispatch_sha256": digest(path)}, sort_keys=True))


if __name__ == "__main__":
    seal()
