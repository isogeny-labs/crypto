#!/usr/bin/env python3
"""Frozen paired analysis for the p31 production rootfind experiment.

Input is the complete native ``ecbench.record/v1`` JSONL stream.  The script
rejects missing, duplicate, failed, warmup-deficient, or non-paired rows; it
never drops a slow row.  Bootstrap intervals are descriptive only and do not
participate in the decision rule.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import random
from pathlib import Path
from statistics import median
from typing import Any, NoReturn


ARMS = (
    "legacy-mitm",
    "subtract-scalar",
    "floor-scalar",
    "floor-batch-v1",
    "legacy-mitm-aa",
)
MEASURED_REPLICATES = 6
WARMUPS = 1
BOOTSTRAP_SEED = 20_261_126
BOOTSTRAP_RESAMPLES = 100_000


class AnalysisFailure(ValueError):
    def __init__(self, message: str, verdict: str = "invalid") -> None:
        super().__init__(message)
        self.verdict = verdict


def fail(message: str, verdict: str = "invalid") -> NoReturn:
    raise AnalysisFailure(message, verdict)


def read_records(path: Path) -> tuple[bytes, list[dict[str, Any]]]:
    raw = path.read_bytes()
    records: list[dict[str, Any]] = []
    for line_number, line in enumerate(raw.splitlines(), 1):
        if not line.strip():
            continue
        try:
            record = json.loads(line)
        except json.JSONDecodeError as exc:
            fail(f"line {line_number}: invalid JSON: {exc}")
        if not isinstance(record, dict):
            fail(f"line {line_number}: record is not an object")
        if record.get("schema") != "ecbench.record/v1":
            fail(f"line {line_number}: unexpected schema {record.get('schema')!r}")
        records.append(record)
    return raw, records


def nested(record: dict[str, Any], *path: str) -> Any:
    value: Any = record
    for key in path:
        if not isinstance(value, dict) or key not in value:
            fail(f"record {record.get('record_id')}: missing {'.'.join(path)}")
        value = value[key]
    return value


def relation_native(record: dict[str, Any]) -> dict[str, Any]:
    phases = nested(record, "phases")
    if not isinstance(phases, list):
        fail(f"record {record.get('record_id')}: phases is not an array")
    matches = [phase for phase in phases if phase.get("name") == "relations"]
    if len(matches) != 1 or not isinstance(matches[0].get("native"), dict):
        fail(f"record {record.get('record_id')}: needs exactly one relations phase")
    return matches[0]["native"]


def row_key(record: dict[str, Any]) -> tuple[str, int, str]:
    workload_id = nested(record, "workload", "workload_id")
    round_index = nested(record, "round")
    algorithm_seed = nested(record, "algorithm_seed")
    if not isinstance(workload_id, str) or not isinstance(round_index, int):
        fail(f"record {record.get('record_id')}: invalid pairing fields")
    return workload_id, round_index, str(algorithm_seed)


def validate(records: list[dict[str, Any]]) -> dict[str, dict[tuple[str, int, str], int]]:
    unknown = sorted({str(row.get("arm")) for row in records} - set(ARMS))
    if unknown:
        fail(f"unexpected arms: {unknown}")

    measured: dict[str, dict[tuple[str, int, str], int]] = {arm: {} for arm in ARMS}
    warmup_counts = {arm: 0 for arm in ARMS}
    for record in records:
        arm = record.get("arm")
        if arm not in measured:
            fail(f"missing or invalid arm on record {record.get('record_id')}")
        is_warmup = record.get("warmup")
        if not isinstance(is_warmup, bool):
            fail(f"record {record.get('record_id')}: warmup must be boolean")
        status = nested(record, "outcome", "status")
        if status != "verified":
            verdict = "censored" if status in {"timeout", "censored"} else "invalid"
            fail(f"record {record.get('record_id')}: outcome is {status!r}", verdict)
        if nested(record, "outcome", "matches_target") is not True:
            fail(f"record {record.get('record_id')}: target verification failed")
        wall_ns = nested(record, "time", "method_call_wall_ns")
        if not isinstance(wall_ns, int) or isinstance(wall_ns, bool) or wall_ns <= 0:
            fail(f"record {record.get('record_id')}: invalid method_call_wall_ns")

        native = relation_native(record)
        batch_keys = [
            "oracle_batch_preferred_width",
            "oracle_batch_max_targets",
            "oracle_batch_calls",
            "oracle_batch_targets",
            "oracle_batch_unused_results",
            "oracle_batch_unused_guarded_trials",
        ]
        if arm == "floor-batch-v1":
            missing = [key for key in batch_keys if key not in native]
            if missing:
                fail(f"record {record.get('record_id')}: missing batch counters {missing}")
            malformed = [
                key
                for key in batch_keys
                if not isinstance(native[key], int) or isinstance(native[key], bool)
            ]
            if malformed:
                fail(
                    f"record {record.get('record_id')}: "
                    f"non-integer batch counters {malformed}"
                )
            preferred = native["oracle_batch_preferred_width"]
            maximum = native["oracle_batch_max_targets"]
            calls = native["oracle_batch_calls"]
            targets = native["oracle_batch_targets"]
            unused_results = native["oracle_batch_unused_results"]
            unused_guards = native["oracle_batch_unused_guarded_trials"]
            if preferred != 8:
                fail(f"record {record.get('record_id')}: preferred width is not eight")
            if not 1 <= maximum <= preferred:
                fail(f"record {record.get('record_id')}: invalid actual batch width")
            if calls <= 0:
                fail(f"record {record.get('record_id')}: no batch calls")
            if not calls <= targets <= preferred * calls or maximum > targets:
                fail(f"record {record.get('record_id')}: inconsistent batch totals")
            if unused_results < 0 or unused_guards < 0:
                fail(f"record {record.get('record_id')}: negative unused batch work")
            if unused_results + unused_guards >= maximum:
                fail(f"record {record.get('record_id')}: invalid terminal batch overrun")
        elif any(key in native for key in batch_keys):
            fail(f"record {record.get('record_id')}: scalar identity emitted batch counters")

        if is_warmup:
            warmup_counts[arm] += 1
            continue
        key = row_key(record)
        if key in measured[arm]:
            fail(f"duplicate measured pair key for {arm}: {key}")
        measured[arm][key] = wall_ns

    for arm in ARMS:
        if warmup_counts[arm] != WARMUPS:
            fail(f"{arm}: expected {WARMUPS} warmup, got {warmup_counts[arm]}")
        if len(measured[arm]) != MEASURED_REPLICATES:
            fail(
                f"{arm}: expected {MEASURED_REPLICATES} measured rows, "
                f"got {len(measured[arm])}"
            )
    keys = set(measured[ARMS[0]])
    for arm in ARMS[1:]:
        if set(measured[arm]) != keys:
            fail(f"{arm}: measured rows are not exactly paired")
    if len({key[0] for key in keys}) != 1:
        fail("expected exactly one frozen workload")
    return measured


def percentile(sorted_values: list[float], probability: float) -> float:
    position = (len(sorted_values) - 1) * probability
    lower = math.floor(position)
    upper = math.ceil(position)
    if lower == upper:
        return sorted_values[lower]
    weight = position - lower
    return sorted_values[lower] * (1.0 - weight) + sorted_values[upper] * weight


def paired_summary(
    measured: dict[str, dict[tuple[str, int, str], int]],
    numerator: str,
    denominator: str,
) -> dict[str, Any]:
    keys = sorted(measured[numerator])
    ratios = [measured[numerator][key] / measured[denominator][key] for key in keys]
    rng = random.Random(BOOTSTRAP_SEED)
    boot = []
    for _ in range(BOOTSTRAP_RESAMPLES):
        boot.append(median(ratios[rng.randrange(len(ratios))] for _ in ratios))
    boot.sort()
    return {
        "numerator": numerator,
        "denominator": denominator,
        "pair_keys": [
            {"workload_id": key[0], "round": key[1], "algorithm_seed": key[2]}
            for key in keys
        ],
        "paired_ratios": ratios,
        "median": median(ratios),
        "range": [min(ratios), max(ratios)],
        "descriptive_bootstrap_ci95": [percentile(boot, 0.025), percentile(boot, 0.975)],
    }


def analyze(path: Path) -> dict[str, Any]:
    raw, records = read_records(path)
    measured = validate(records)
    legacy = paired_summary(measured, "legacy-mitm", "floor-batch-v1")
    subtract = paired_summary(measured, "subtract-scalar", "floor-batch-v1")
    aa = paired_summary(measured, "legacy-mitm", "legacy-mitm-aa")
    aa_symmetric = [max(ratio, 1.0 / ratio) for ratio in aa["paired_ratios"]]
    aa_upper = max(aa_symmetric)
    legacy_pass = legacy["median"] >= 50.0 and legacy["median"] > aa_upper
    subtract_pass = subtract["median"] > aa_upper
    thresholds_met = legacy_pass and subtract_pass
    return {
        "schema": "rootfind-production-analysis/v1",
        "input": {
            "path": str(path),
            "sha256": hashlib.sha256(raw).hexdigest(),
            "records": len(records),
            "measured_records": len(ARMS) * MEASURED_REPLICATES,
            "warmup_records": len(ARMS) * WARMUPS,
        },
        "bootstrap": {
            "role": "descriptive_only",
            "seed": BOOTSTRAP_SEED,
            "resamples": BOOTSTRAP_RESAMPLES,
            "percentile_method": "linear interpolation at (N-1)*p",
        },
        "comparisons": {
            "legacy_over_batch": legacy,
            "subtract_over_batch": subtract,
            "legacy_aa": aa,
        },
        "aa_symmetric_upper_bound": aa_upper,
        "decision": {
            "legacy_threshold": 50.0,
            "legacy_pass": legacy_pass,
            "subtract_pass": subtract_pass,
            "timing_thresholds_met": thresholds_met,
            "timing_verdict": "thresholds_met" if thresholds_met else "goal_not_met",
            "rule": (
                "median(legacy/batch) >= 50 and strictly above max_i max(AA_i,1/AA_i); "
                "median(subtract/batch) strictly above the same symmetric A/A upper bound"
            ),
            "significance_test": None,
            # Timing alone cannot decide the experiment: even an apparently
            # unfavorable run may be inadmissible because its provenance,
            # L2/replay, A/A, or S3 receipts fail.  Keep the final verdict
            # pending for every complete timing data set.
            "final_verdict": "requires_external_admission",
            "admission_status": "not_evaluated",
            "required_external_admission": [
                "exact source, binary, image and spec identity",
                "all measured records admitted at L2",
                "native replay and audit pass",
                "native A/A admission pass",
                "S3 payload hashes verified",
            ],
        },
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("records", type=Path, help="complete ecbench record JSONL")
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    invalid = False
    try:
        analysis = analyze(args.records)
    except (OSError, ValueError) as exc:
        invalid = True
        verdict = exc.verdict if isinstance(exc, AnalysisFailure) else "invalid"
        raw = args.records.read_bytes() if args.records.is_file() else b""
        analysis = {
            "schema": "rootfind-production-analysis/v1",
            "input": {
                "path": str(args.records),
                "sha256": hashlib.sha256(raw).hexdigest() if raw else None,
            },
            "decision": {
                "timing_thresholds_met": False,
                "timing_verdict": verdict,
                "final_verdict": verdict,
            },
            "error": str(exc),
        }
    result = json.dumps(analysis, sort_keys=True, indent=2) + "\n"
    if args.output is None:
        print(result, end="")
    else:
        args.output.write_text(result)
    if invalid:
        raise SystemExit(2)


if __name__ == "__main__":
    main()
