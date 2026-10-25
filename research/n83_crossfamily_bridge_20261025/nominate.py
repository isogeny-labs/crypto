#!/usr/bin/env python3
"""Nominate exact N83 bridge arms from independently admitted panels."""

import argparse
import hashlib
import json
import math
from pathlib import Path

from research.n83_crossfamily_current_20261025 import arms as wide_arms
from research.n83_crossfamily_current_20261025 import select as wide_select
from research.n83_nearby_grid_confirm16_20261024 import aggregate as nearby_aggregate
from research.n83_nearby_grid_confirm16_20261024 import protocol as nearby_protocol
from research.n83_nearby_grid_tier1_20261024 import protocol as nearby_catalog


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"),
                      ensure_ascii=False).encode()


def sha(value):
    return hashlib.sha256(canonical(value)).hexdigest()


def reject(reason):
    return {"schema": "n83.crossfamily-bridge-nomination/v1",
            "status": "BOUNDED_UNKNOWN", "reason": reason,
            "candidate_ids": None}


def nearby_recheck(aggregate):
    selected = aggregate.get("selected_ks")
    if (aggregate.get("schema") != "n83.nearby-grid-confirm16-aggregate/v1" or
            aggregate.get("status") != "PASS_TIER1_CONFIRM16" or
            not isinstance(selected, list) or
            selected != sorted(set(selected)) or
            not 3 <= len(selected) <= 7 or
            any(type(k) is not int or k not in nearby_catalog.CANDIDATES
                for k in selected) or
            aggregate.get("one_target_rows_verified") != 16 * len(selected) or
            aggregate.get("stratum_statuses") != ["STRATUM_ADMITTED"] * 2 or
            len(aggregate.get("host_instance_ids") or []) != 2 or
            len(set(aggregate.get("host_instance_ids") or [])) != 2 or
            len(aggregate.get("runner_names") or []) != 2 or
            len(set(aggregate.get("runner_names") or [])) != 2):
        return None
    _, points = nearby_protocol.fixture()
    rows = aggregate.get("rows")
    if not isinstance(rows, list) or len(rows) != 16 * len(selected):
        return None
    online = {k: {} for k in selected}
    cold = {k: {} for k in selected}
    observed = set()
    for row in rows:
        k, index = row.get("k"), row.get("point_index")
        if (type(k) is not int or k not in online or
                type(index) is not int or index not in range(16) or
                (index, k) in observed or
                row.get("status") != "PASS_INDEPENDENT_REPLAY" or
                row.get("candidate_id") != nearby_catalog.CANDIDATES[k][0] or
                row.get("workload_id") != points[index][3] or
                row.get("run_id") !=
                f"{nearby_catalog.CANDIDATES[k][0]}W{points[index][3]}R1" or
                row.get("stratum") != index // 8):
            return None
        observed.add((index, k))
        for key, target in (("ic_online_ms", online),
                            ("ic_cold_process_ms", cold)):
            value = row.get(key)
            if (not isinstance(value, (int, float)) or isinstance(value, bool) or
                    not math.isfinite(value) or value <= 0):
                return None
            target[k][index] = value
    if observed != {(index, k) for index in range(16) for k in selected}:
        return None
    alpha = 0.05 / (len(selected) * (len(selected) - 1) // 2)
    expected_online = nearby_aggregate.select_metric(selected, online,
                                                     "online", alpha)
    expected_cold = nearby_aggregate.select_metric(selected, cold,
                                                   "cold", alpha)
    if (aggregate.get("alpha_per_unordered_pair") != alpha or
            aggregate.get("online") != expected_online or
            aggregate.get("cold") != expected_cold):
        return None
    return expected_online, expected_cold


def nominate(nearby, wide_validated):
    if nearby_catalog.SOURCE_SHA256 != wide_arms.SOURCE_SHA256:
        return reject("SOURCE_VERSION_DIFFERS")
    nearby_choices = nearby_recheck(nearby)
    if nearby_choices is None:
        return reject("NEARBY_PANEL_NOT_ADMITTED_OR_CHANGED")
    wide_choice = wide_select.select(wide_validated)
    if (wide_choice.get("admission_status") != "PASS" or
            wide_choice.get("candidate_ids") != [arm[3] for arm in wide_arms.ARMS] or
            wide_validated.get("source_sha256") != wide_arms.SOURCE_SHA256):
        return reject("WIDE_PANEL_NOT_ADMITTED")
    if (any(choice.get("status") != "PASS_SELECTED" or
            choice.get("selected_k") is None
            for choice in nearby_choices) or
            any(wide_choice[metric].get("status") != "SELECTED" or
                wide_choice[metric].get("selected_candidate_id") is None
                for metric in ("online", "cold"))):
        return reject("WITHIN_FAMILY_SELECTION_INCONCLUSIVE")
    small_ids = [nearby_catalog.CANDIDATES[choice["selected_k"]][0]
                 for choice in nearby_choices]
    wide_ids = [wide_choice[metric]["selected_candidate_id"]
                for metric in ("online", "cold")]
    ids = sorted(set(small_ids + wide_ids))
    wide_hashes = wide_arms.choose()[1]["candidate_manifest_sha256"]
    manifest_hashes = {nearby_catalog.CANDIDATES[k][0]:
                       nearby_catalog.CANDIDATES[k][1]
                       for k in nearby.get("selected_ks", [])}
    manifest_hashes.update(wide_hashes)
    if not 2 <= len(ids) <= 4 or any(candidate not in manifest_hashes
                                    for candidate in ids):
        return reject("BRIDGE_CANDIDATE_IDENTITY_DIFFERS")
    index_policy = {candidate: "root5_dynamic_load75_no_bloom"
                    for candidate in small_ids}
    index_policy.update({
        arm[3]: ("root5_fixed30_bloom_wide3" if arm[2] == "root11_wide3"
                 else "root4_fixed31_bloom_standard")
        for arm in wide_arms.ARMS if arm[3] in wide_ids})
    return {
        "schema": "n83.crossfamily-bridge-nomination/v1",
        "status": "READY_FOR_BRIDGE_DISPATCH",
        "source_sha256": wide_arms.SOURCE_SHA256,
        "curve_id": "EC1N83Ckb1a1haacb69ee9f37",
        "nearby_aggregate_sha256": sha(nearby),
        "wide_validated_sha256": sha(wide_validated),
        "candidate_ids": ids,
        "candidate_manifest_sha256": {candidate: manifest_hashes[candidate]
                                      for candidate in ids},
        "comparison_variable": "factor_base_and_index_policy",
        "index_policy_by_candidate_id": {candidate: index_policy[candidate]
                                         for candidate in ids},
        "small_online_candidate_id": small_ids[0],
        "small_cold_candidate_id": small_ids[1],
        "wide_online_candidate_id": wide_ids[0],
        "wide_cold_candidate_id": wide_ids[1],
        "fresh_bridge_targets_sha256": hashlib.sha256(
            (Path(__file__).resolve().parent / "targets.jsonl").read_bytes()
        ).hexdigest(),
        "next_gate": "independent_raw_replay_then_seal_bridge_dispatch",
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
