#!/usr/bin/env python3
"""Verify custody and internal parity of the ECC2K-130 four-point sweep.

The progress log contains one JSON object after each completed support. This
verifier requires those objects, the standalone per-support records, and the
summary to agree value-for-value after removing the standalone-only
``relations`` list. It deliberately does not rerun the 37.5 million-pair
experiment.
"""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
DEFAULT_RESULTS = HERE / "results"
EXPECTED_NAMES = (
    "four_weight_two",
    "four_weight_two_frobenius_stable",
    "four_random_matched",
)
EXPECTED_SUBGROUP_ORDER = "680564733841876926932320129493409985129"
EXPECTED_SIGNED_PAIRS = 37_507_774


class EvidenceError(ValueError):
    """The committed evidence sources disagree or are incomplete."""


def require(condition: bool, message: str) -> None:
    if not condition:
        raise EvidenceError(message)


def load_json(path: Path) -> dict:
    try:
        value = json.loads(path.read_text())
    except (OSError, json.JSONDecodeError) as exc:
        raise EvidenceError(f"cannot read valid JSON from {path}: {exc}") from exc
    require(isinstance(value, dict), f"{path} is not a JSON object")
    return value


def log_records(path: Path) -> list[dict]:
    """Extract the top-level JSON records embedded in a progress log."""
    try:
        text = path.read_text()
    except OSError as exc:
        raise EvidenceError(f"cannot read {path}: {exc}") from exc
    decoder = json.JSONDecoder()
    records = []
    cursor = 0
    while True:
        start = text.find("{", cursor)
        if start < 0:
            break
        try:
            value, end = decoder.raw_decode(text, start)
        except json.JSONDecodeError:
            cursor = start + 1
            continue
        cursor = end
        if isinstance(value, dict) and value.get("name") in EXPECTED_NAMES:
            records.append(value)
    return records


def without_relations(record: dict) -> dict:
    return {key: value for key, value in record.items() if key != "relations"}


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def verify(results: Path = DEFAULT_RESULTS) -> dict:
    summary_path = results / "four_point_summary.json"
    log_path = results / "four_point.log"
    summary = load_json(summary_path)

    require(summary.get("instance") == "ECC2K-130", "summary instance mismatch")
    require(
        summary.get("subgroup_order_r") == EXPECTED_SUBGROUP_ORDER,
        "summary subgroup order mismatch",
    )
    require(summary.get("digest_bits") == 38, "summary digest width mismatch")

    experiments = summary.get("experiments")
    require(isinstance(experiments, list), "summary experiments is not a list")
    summary_by_name = {row.get("name"): row for row in experiments}
    require(
        tuple(summary_by_name) == EXPECTED_NAMES,
        "summary experiment names or order mismatch",
    )

    embedded = log_records(log_path)
    log_by_name = {row.get("name"): row for row in embedded}
    require(len(embedded) == len(EXPECTED_NAMES), "progress log JSON record count mismatch")
    require(
        tuple(log_by_name) == EXPECTED_NAMES,
        "progress log experiment names or order mismatch",
    )

    files = [summary_path, log_path]
    total_pairs = 0
    total_candidates = 0
    total_relations = 0
    rows = []
    for name in EXPECTED_NAMES:
        path = results / f"{name}.json"
        files.append(path)
        standalone = load_json(path)
        stripped = without_relations(standalone)
        require(summary_by_name[name] == stripped, f"summary differs from {path.name}")
        require(log_by_name[name] == stripped, f"progress log differs from {path.name}")
        require(standalone.get("complete") is True, f"{name} is not complete")
        require(
            standalone.get("signed_pairs_consumed") == standalone.get("signed_pairs_total"),
            f"{name} did not consume its declared pair space",
        )
        require(
            standalone.get("keys_stored") == standalone.get("signed_pairs_consumed"),
            f"{name} stored-key count mismatch",
        )
        require(
            standalone.get("candidates_checked")
            == standalone.get("degenerate_candidates", 0)
            + standalone.get("false_positive_candidates", 0),
            f"{name} candidate classification mismatch",
        )
        relations = standalone.get("relations")
        require(isinstance(relations, list), f"{name} relations is not a list")
        require(
            standalone.get("four_point_relations") == len(relations),
            f"{name} relation count mismatch",
        )
        total_pairs += standalone["signed_pairs_consumed"]
        total_candidates += standalone["candidates_checked"]
        total_relations += standalone["four_point_relations"]
        rows.append(
            {
                "name": name,
                "signed_pairs": standalone["signed_pairs_consumed"],
                "candidates_checked": standalone["candidates_checked"],
                "relations": standalone["four_point_relations"],
            }
        )

    require(total_pairs == EXPECTED_SIGNED_PAIRS, "aggregate signed-pair count mismatch")
    require(total_relations == 0, "the frozen null-result relation count changed")

    return {
        "operation": "verify-ecc2k130-four-point-evidence",
        "status": "pass",
        "experiments": rows,
        "totals": {
            "signed_pairs": total_pairs,
            "candidates_checked": total_candidates,
            "relations": total_relations,
        },
        "sha256": {path.name: sha256(path) for path in files},
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--results", type=Path, default=DEFAULT_RESULTS)
    args = parser.parse_args()
    try:
        receipt = verify(args.results)
    except EvidenceError as exc:
        parser.error(str(exc))
    print(json.dumps(receipt, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
