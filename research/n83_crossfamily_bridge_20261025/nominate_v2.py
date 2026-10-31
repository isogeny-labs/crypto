#!/usr/bin/env python3
"""Carry admitted but unresolved N83 family frontiers to unused bridge points."""

import argparse
import hashlib
import json
import math
from pathlib import Path

from research.n83_crossfamily_bridge_20261025 import nominate as original
from research.n83_crossfamily_current_20261025 import arms as wide_arms
from research.n83_crossfamily_current_20261025 import select as wide_select
from research.n83_nearby_grid_tier1_20261024 import protocol as nearby_catalog


SCHEMA = "n83.crossfamily-bridge-nomination/v2"


def reject(reason):
    return {"schema": SCHEMA, "status": "BOUNDED_UNKNOWN",
            "reason": reason, "candidate_ids": None}


def top_ids(scores, ids, count):
    if (not isinstance(scores, dict) or set(scores) != set(ids) or
            any(not isinstance(value, (int, float)) or
                isinstance(value, bool) or not math.isfinite(value) or
                value <= 0 for value in scores.values())):
        return None
    return sorted(ids, key=lambda candidate: (scores[candidate], candidate))[:count]


def small_frontier(nearby, choices):
    selected_ks = nearby["selected_ks"]
    if 1107 not in selected_ks:
        return None
    ranked = []
    for choice in choices:
        if choice.get("status") not in ("PASS_SELECTED", "BOUNDED_UNKNOWN"):
            return None
        scores = choice.get("score_by_k")
        keys = [str(k) for k in selected_ks]
        top = top_ids(scores, keys, min(3, len(keys)))
        if top is None:
            return None
        ranked.append([int(k) for k in top])
    return ranked[0], ranked[1], sorted(set(ranked[0] + ranked[1] + [1107]))


def wide_frontier(choice):
    ids = [arm[3] for arm in wide_arms.ARMS]
    ranked = []
    for metric in ("online", "cold"):
        item = choice.get(metric)
        if not isinstance(item, dict):
            return None
        if item.get("status") == "SELECTED":
            candidate = item.get("selected_candidate_id")
            if candidate not in ids:
                return None
            ranked.append([candidate])
        elif item.get("status") == "BOUNDED_UNKNOWN":
            top = top_ids(item.get("mean_by_candidate_ms"), ids, 2)
            if top is None or item.get("selected_candidate_id") is not None:
                return None
            ranked.append(top)
        else:
            return None
    return ranked[0], ranked[1], sorted(set(ranked[0] + ranked[1]))


def nominate(nearby, wide_validated):
    if nearby_catalog.SOURCE_SHA256 != wide_arms.SOURCE_SHA256:
        return reject("SOURCE_VERSION_DIFFERS")
    if not original.middle_identity() or not original.frontier_identity():
        return reject("FIXED_CONTROL_IDENTITY_DIFFERS")
    nearby_choices = original.nearby_recheck(nearby)
    if nearby_choices is None:
        return reject("NEARBY_PANEL_NOT_ADMITTED_OR_CHANGED")
    small = small_frontier(nearby, nearby_choices)
    if small is None:
        return reject("SMALL_FRONTIER_INCOMPLETE")
    wide_choice = wide_select.select(wide_validated)
    wide_hashes = wide_arms.choose()[1]
    if (wide_choice.get("admission_status") != "PASS" or
            wide_choice.get("candidate_ids") !=
            [arm[3] for arm in wide_arms.ARMS] or
            wide_validated.get("choice") != wide_hashes or
            wide_validated.get("source_sha256") != wide_arms.SOURCE_SHA256):
        return reject("WIDE_PANEL_NOT_ADMITTED")
    wide = wide_frontier(wide_choice)
    if wide is None:
        return reject("WIDE_FRONTIER_INCOMPLETE")
    small_online, small_cold, small_ks = small
    wide_online, wide_cold, wide_ids = wide
    small_ids = [nearby_catalog.CANDIDATES[k][0] for k in small_ks]
    fixed_ids = [original.MIDDLE_ID] + [item[0]
                                        for item in original.FRONTIER.values()]
    ids = sorted(set(small_ids + wide_ids + fixed_ids))
    manifest_hashes = {nearby_catalog.CANDIDATES[k][0]:
                       nearby_catalog.CANDIDATES[k][1] for k in small_ks}
    manifest_hashes.update(wide_hashes["candidate_manifest_sha256"])
    manifest_hashes[original.MIDDLE_ID] = original.MIDDLE_MANIFEST_SHA256
    manifest_hashes.update({item[0]: item[1]
                            for item in original.FRONTIER.values()})
    if not 6 <= len(ids) <= 14 or any(item not in manifest_hashes for item in ids):
        return reject("BRIDGE_CANDIDATE_IDENTITY_DIFFERS")
    policies = {item: "root5_dynamic_load75_no_bloom" for item in small_ids}
    policies.update({
        arm[3]: ("root5_fixed30_bloom_wide3" if arm[2] == "root11_wide3"
                 else "root4_fixed31_bloom_standard")
        for arm in wide_arms.ARMS if arm[3] in wide_ids})
    policies[original.MIDDLE_ID] = "root5_dynamic_load75_no_bloom"
    policies.update({item[0]: "root5_fixed30_bloom_wide3"
                     for item in original.FRONTIER.values()})
    return {
        "schema": SCHEMA, "status": "READY_FOR_BRIDGE_DISPATCH",
        "source_sha256": wide_arms.SOURCE_SHA256,
        "curve_id": "EC1N83Ckb1a1haacb69ee9f37",
        "nearby_aggregate_sha256": original.sha(nearby),
        "wide_validated_sha256": original.sha(wide_validated),
        "candidate_ids": ids,
        "candidate_manifest_sha256": {item: manifest_hashes[item] for item in ids},
        "index_policy_by_candidate_id": {item: policies[item] for item in ids},
        "comparison_variable": "factor_base_and_index_policy",
        "nearby_online_selection_status": nearby_choices[0]["status"],
        "nearby_cold_selection_status": nearby_choices[1]["status"],
        "wide_online_selection_status": wide_choice["online"]["status"],
        "wide_cold_selection_status": wide_choice["cold"]["status"],
        "small_top_online_ks": small_online,
        "small_top_cold_ks": small_cold,
        "small_shortlist_ks": small_ks,
        "wide_top_online_candidate_ids": wide_online,
        "wide_top_cold_candidate_ids": wide_cold,
        "wide_shortlist_candidate_ids": wide_ids,
        "fixed_control_candidate_ids": fixed_ids,
        "fresh_bridge_targets_sha256": hashlib.sha256(
            (Path(__file__).resolve().parent / "targets.jsonl").read_bytes()
        ).hexdigest(),
        "next_gate": "independent_raw_replay_then_seal_capacity_and_bridge_dispatch",
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--nearby-aggregate", type=Path, required=True)
    parser.add_argument("--wide-validated", type=Path, required=True)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    result = nominate(json.loads(args.nearby_aggregate.read_text()),
                      json.loads(args.wide_validated.read_text()))
    blob = json.dumps(result, sort_keys=True, indent=2) + "\n"
    if args.output:
        args.output.write_text(blob)
    else:
        print(blob, end="")
    return 0 if result["status"] == "READY_FOR_BRIDGE_DISPATCH" else 2


if __name__ == "__main__":
    raise SystemExit(main())
