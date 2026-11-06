#!/usr/bin/env python3
"""Fail closed if the frozen N83 distribution dispatch differs."""

import hashlib
import importlib.util
import json
from pathlib import Path

from research.n83_highk_online_distribution_20261020 import arms

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
SOURCE_COMMIT = "e7634a81c22b60ad37e5d6bdb817a3b84867ae86"
PRIOR_IDS = (
    "IC1N83Ckb1a1fb547800PDP4rootRCguidedLAgaussTDdirectISO0h8d1e886f8586",
    "IC1N83Ckb1a1fb713800PDP4rootRCguidedLAgaussTDdirectISO0h394bfa10df77",
    "IC1N83Ckb1a1fb731230PDP4rootRCguidedLAgaussTDdirectISO0h2f81ad5da3e7",
)


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def prior_result():
    selection_path = HERE / "prior_highk_selection.json"
    validated_path = HERE / "prior_highk_validated.json"
    selection = json.loads(selection_path.read_text())
    validated = json.loads(validated_path.read_text())
    path = ROOT / "research/n83_root4_highk_tier1_retry_20261019/select.py"
    spec = importlib.util.spec_from_file_location("n83_prior_highk_selector", path)
    selector = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(selector)
    if (selection != selector.select(validated) or
            selection.get("schema") != "n83.highk-retry-selection/v1" or
            selection.get("admission_status") != "PASS" or
            selection.get("candidate_ids") != list(PRIOR_IDS) or
            selection.get("point_count") != 8 or
            selection.get("online", {}).get("status") != "BOUNDED_UNKNOWN" or
            selection["online"].get("selected_candidate_id") is not None or
            selection.get("cold", {}).get("status") != "SELECTED" or
            selection["cold"].get("selected_candidate_id") != PRIOR_IDS[0] or
            validated.get("evidence_status") != "PASS_INDEPENDENT_REPLAY" or
            validated.get("single_cloud_instance") is not True or
            validated.get("same_candidate_bases") is not True or
            validated.get("targets_sha256") !=
            "2824e714820524e81d7e3cb16821529827d82a18fc1c9de8a0e7315ab940deac"):
        raise ValueError("upstream admitted high-K outcome differs")
    return digest(selection_path), digest(validated_path)


def verify():
    record = json.loads((HERE / "dispatch.json").read_text())
    selection_sha, validated_sha = prior_result()
    if (record.get("schema") != "n83.online-distribution-dispatch/v1" or
            record.get("status") != "SEALED_FOR_DISPATCH" or
            record.get("source_commit") != SOURCE_COMMIT or
            record.get("source_sha256") !=
            "4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507" or
            record.get("screen_targets_sha256") !=
            digest(HERE / "screen_targets.jsonl") or
            record.get("holdout_targets_sha256") !=
            digest(HERE / "holdout_targets.jsonl") or
            record.get("ordered_candidate_ids") !=
            [arm[3] for arm in arms.ARMS] or
            record.get("prior_selection_sha256") != selection_sha or
            record.get("prior_validated_sha256") != validated_sha or
            record.get("screen_point_count") != 192 or
            record.get("holdout_point_count") != 8 or
            record.get("screen_event_count") != 262 or
            record.get("holdout_event_count") != 64 or
            record.get("producer_cap_seconds") != 24000):
        raise ValueError("distribution dispatch metadata differs")
    actual = {name: digest(ROOT / name) for name in record["sha256"]}
    if actual != record["sha256"]:
        raise ValueError("sealed distribution file differs")
    return record


if __name__ == "__main__":
    checked = verify()
    print(json.dumps({"status": "PASS_SEALED_DISPATCH",
                      "file_count": len(checked["sha256"])}, sort_keys=True))
