#!/usr/bin/env python3
"""Descriptive paired analysis of the frozen PKM cold-panel receipts.

This deliberately cannot promote a map from a partial panel or an unpriced
operation ledger. It consumes an independent replay receipt, not raw reports
alone, and leaves missing and censored arms in every coverage denominator.
"""

import argparse
import hashlib
import json
import math
import statistics
from collections import Counter, defaultdict
from pathlib import Path

ARMS = ("interval_a", "candidate_a", "candidate_b", "interval_b", "mitm", "kummer")
PAIRS = (("candidate_a", "interval_a"), ("candidate_b", "interval_b"),
         ("mitm", "interval_a"), ("kummer", "interval_a"))


def require(condition, message):
    if not condition:
        raise ValueError(message)


def read(path):
    return json.loads(path.read_bytes())


def ratio_summary(values):
    if not values:
        return None
    return {"n": len(values), "median": statistics.median(values),
            "geometric_mean": math.exp(statistics.mean(math.log(x) for x in values)),
            "min": min(values), "max": max(values)}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--store-dir", type=Path, required=True)
    parser.add_argument("--replay", type=Path, required=True)
    args = parser.parse_args()
    store = args.store_dir / "receipts.jsonl"
    data = store.read_bytes()
    replay = read(args.replay)
    require(replay["schema"] == "pkm-z2xz8-cold-independent-replay/v1",
            "wrong replay schema")
    require(replay["store_sha256"] == hashlib.sha256(data).hexdigest()
            and Path(replay["store"]).resolve() == store.resolve(),
            "replay does not cover this exact store snapshot")
    require(replay["cases_expected"] == 576, "wrong panel size")

    records = [json.loads(line) for line in data.splitlines()]
    require(records and records[0]["id"] == "header", "missing header")
    require(records[0]["schema"] == "pkm-z2xz8-heldout-cold/v4"
            and records[0]["arms"] == list(ARMS)
            and records[0]["cases"] == 576, "wrong frozen panel")
    results = {}
    for record in records[1:]:
        if not record["id"].startswith("result:"):
            continue
        case = record["id"].removeprefix("result:")
        require(case not in results, f"duplicate result: {case}")
        results[case] = record
    require(len(results) == replay["cases_recorded"], "replay/result count differs")

    coverage = defaultdict(Counter)
    pairs = defaultdict(lambda: defaultdict(list))
    pair_rows = []
    complete_cost = True
    for bits in (11, 14):
        for curve in range(3):
            for target in range(16):
                group = f"b{bits}_c{curve}_q{target}"
                rows = {arm: results.get(f"{group}:{arm}") for arm in ARMS}
                for arm, row in rows.items():
                    outcome = "unrecorded" if row is None else row["outcome"]
                    coverage[f"b{bits}_c{curve}"][f"{arm}:{outcome}"] += 1
                    if row is not None and row["outcome"] == "parsed":
                        complete_cost &= row["summary"].get("complete_cold_gae") is not None
                for candidate, control in PAIRS:
                    left, right = rows[candidate], rows[control]
                    if left is None or right is None:
                        status = "unrecorded"
                    elif left["outcome"] != "parsed" or right["outcome"] != "parsed":
                        status = "censored_or_failed"
                    elif (left["summary"].get("status") != "complete"
                          or right["summary"].get("status") != "complete"):
                        status = "incomplete_rank"
                    else:
                        require(left["summary"].get("all_verified") is True
                                and right["summary"].get("all_verified") is True,
                                f"unverified parsed pair: {group}")
                        if candidate != "kummer":
                            require(left.get("equal_size_match") is True,
                                    f"unequal factor base: {group}:{candidate}")
                        status = "paired_complete"
                        for metric in ("wall_s", "process_cpu_s", "peak_rss_kib"):
                            a, b = left[metric], right[metric]
                            require(a > 0 and b > 0, f"nonpositive {metric}: {group}")
                            pairs[(bits, curve, candidate, control)][metric].append(a / b)
                    pair_rows.append({"target": group, "candidate": candidate,
                                      "control": control, "status": status})

    require(len(pair_rows) == 384, "wrong pair count")
    paired = []
    for bits in (11, 14):
        for curve in range(3):
            for candidate, control in PAIRS:
                key = (bits, curve, candidate, control)
                paired.append({"bits": bits, "curve_index": curve,
                               "candidate": candidate, "control": control,
                               "denominator_targets": 16,
                               "paired_targets": len(pairs[key]["wall_s"]),
                               "wall_ratio": ratio_summary(pairs[key]["wall_s"]),
                               "cpu_ratio": ratio_summary(pairs[key]["process_cpu_s"]),
                               "rss_ratio": ratio_summary(pairs[key]["peak_rss_kib"])})
    full = len(results) == 576 and replay.get("claim_qualified") is True
    output = {"schema": "pkm-z2xz8-cold-paired-analysis/v1",
              "store_sha256": hashlib.sha256(data).hexdigest(),
              "replay_sha256": hashlib.sha256(args.replay.read_bytes()).hexdigest(),
              "cases_expected": 576, "cases_recorded": len(results),
              "full_independent_replay": full,
              "complete_native_cost": complete_cost and full,
              "promotion_assessment_ready": full and complete_cost,
              "coverage": dict(coverage), "paired": paired,
              "pair_status": dict(Counter(row["status"] for row in pair_rows)),
              "pair_rows": pair_rows}
    print(json.dumps(output, sort_keys=True))


if __name__ == "__main__":
    main()
