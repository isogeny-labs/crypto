#!/usr/bin/env python3
"""Fail-closed validator for the synthetic fixed-population contract."""

from __future__ import annotations

import argparse
import hashlib
import json
import pathlib
import sys
from typing import Any


class ContractError(ValueError):
    pass


TOP_KEYS = {
    "schema_version", "spec_digest", "identity", "registered_targets",
    "targets", "relations", "rank_trace",
}
IDENTITY_KEYS = {
    "curve_id", "subgroup_order", "generator", "factor_base_id",
    "factor_base_columns", "symmetry_quotient", "target_generator", "arity",
    "solver_version", "checker_version", "scalar_field", "budgets",
    "registered_scales", "row_normalization",
}
TARGET_KEYS = {
    "target_id", "scale", "outcome", "decomposition_count", "attempts",
    "witnesses", "terminal",
}
WITNESS_KEYS = {"witness_id", "summands", "chart", "multiplicity", "verified"}
RELATION_KEYS = {
    "relation_id", "source_target_id", "coefficients", "accepted",
    "rejection_reason",
}
TRACE_KEYS = {
    "prefix", "rank", "live_columns", "dead_columns", "support_components",
    "duplicate_rows",
}
OUTCOMES = {"verified", "zero", "solver_failure", "timeout", "rejected"}


def fail(path: str, message: str) -> None:
    raise ContractError(f"{path}: {message}")


def exact_keys(value: Any, expected: set[str], path: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        fail(path, "must be an object")
    keys = set(value)
    missing = sorted(expected - keys)
    extra = sorted(keys - expected)
    if missing or extra:
        fail(path, f"keys differ (missing={missing}, extra={extra})")
    return value


def integer(value: Any, path: str, minimum: int = 0) -> int:
    if isinstance(value, bool) or not isinstance(value, int) or value < minimum:
        fail(path, f"must be an integer >= {minimum}")
    return value


def nonempty_string(value: Any, path: str) -> str:
    if not isinstance(value, str) or not value:
        fail(path, "must be a nonempty string")
    return value


def unique_strings(value: Any, path: str) -> list[str]:
    if not isinstance(value, list) or not value:
        fail(path, "must be a nonempty array")
    result = [nonempty_string(item, f"{path}[{i}]") for i, item in enumerate(value)]
    if len(set(result)) != len(result):
        fail(path, "contains duplicates")
    return result


def specification_digest(document: dict[str, Any]) -> str:
    frozen = {
        "identity": document["identity"],
        "registered_targets": document["registered_targets"],
    }
    payload = json.dumps(frozen, sort_keys=True, separators=(",", ":")).encode()
    return hashlib.sha256(payload).hexdigest()


def validate(document: Any) -> None:
    doc = exact_keys(document, TOP_KEYS, "$")
    if doc["schema_version"] != "ic.fixed-population/v1":
        fail("$.schema_version", "must equal ic.fixed-population/v1")

    identity = exact_keys(doc["identity"], IDENTITY_KEYS, "$.identity")
    for key in (
        "curve_id", "generator", "factor_base_id", "symmetry_quotient",
        "target_generator", "solver_version", "checker_version", "scalar_field",
        "row_normalization",
    ):
        nonempty_string(identity[key], f"$.identity.{key}")
    order_text = nonempty_string(identity["subgroup_order"], "$.identity.subgroup_order")
    if not order_text.isdecimal() or order_text.startswith("0"):
        fail("$.identity.subgroup_order", "must be a canonical positive decimal")
    order = int(order_text)
    columns = integer(identity["factor_base_columns"], "$.identity.factor_base_columns", 1)
    integer(identity["arity"], "$.identity.arity", 1)

    budgets = exact_keys(identity["budgets"], {"max_attempts", "max_seconds"}, "$.identity.budgets")
    max_attempts = integer(budgets["max_attempts"], "$.identity.budgets.max_attempts", 1)
    integer(budgets["max_seconds"], "$.identity.budgets.max_seconds", 1)
    scales_raw = identity["registered_scales"]
    if not isinstance(scales_raw, list) or not scales_raw:
        fail("$.identity.registered_scales", "must be a nonempty array")
    scales = [integer(x, f"$.identity.registered_scales[{i}]", 1) for i, x in enumerate(scales_raw)]
    if len(set(scales)) != len(scales):
        fail("$.identity.registered_scales", "contains duplicates")

    registered = unique_strings(doc["registered_targets"], "$.registered_targets")
    expected_digest = specification_digest(doc)
    if doc["spec_digest"] != expected_digest:
        fail("$.spec_digest", f"expected {expected_digest}")

    targets_raw = doc["targets"]
    if not isinstance(targets_raw, list):
        fail("$.targets", "must be an array")
    targets: dict[str, dict[str, Any]] = {}
    witness_ids: set[str] = set()
    for i, raw in enumerate(targets_raw):
        path = f"$.targets[{i}]"
        target = exact_keys(raw, TARGET_KEYS, path)
        target_id = nonempty_string(target["target_id"], f"{path}.target_id")
        if target_id in targets:
            fail(f"{path}.target_id", "duplicate target")
        targets[target_id] = target
        if target["terminal"] is not True:
            fail(f"{path}.terminal", "must be true")
        if target["scale"] not in scales:
            fail(f"{path}.scale", "is not registered")
        outcome = target["outcome"]
        if outcome not in OUTCOMES:
            fail(f"{path}.outcome", f"must be one of {sorted(OUTCOMES)}")
        attempts = integer(target["attempts"], f"{path}.attempts")
        if attempts > max_attempts:
            fail(f"{path}.attempts", "exceeds frozen budget")
        witnesses = target["witnesses"]
        if not isinstance(witnesses, list):
            fail(f"{path}.witnesses", "must be an array")
        for j, raw_witness in enumerate(witnesses):
            wpath = f"{path}.witnesses[{j}]"
            witness = exact_keys(raw_witness, WITNESS_KEYS, wpath)
            witness_id = nonempty_string(witness["witness_id"], f"{wpath}.witness_id")
            if witness_id in witness_ids:
                fail(f"{wpath}.witness_id", "duplicate witness")
            witness_ids.add(witness_id)
            summands = witness["summands"]
            if not isinstance(summands, list) or len(summands) != identity["arity"]:
                fail(f"{wpath}.summands", "length must equal frozen arity")
            for k, summand in enumerate(summands):
                nonempty_string(summand, f"{wpath}.summands[{k}]")
            nonempty_string(witness["chart"], f"{wpath}.chart")
            integer(witness["multiplicity"], f"{wpath}.multiplicity", 1)
            if witness["verified"] is not True:
                fail(f"{wpath}.verified", "must be true")
        count = target["decomposition_count"]
        if outcome == "verified":
            integer(count, f"{path}.decomposition_count", 1)
            if not witnesses:
                fail(f"{path}.witnesses", "verified outcome requires a witness")
        elif outcome == "zero":
            if count != 0 or witnesses:
                fail(path, "zero outcome requires count 0 and no witnesses")
        elif count is not None or witnesses:
            fail(path, "failure/rejection outcome requires null count and no witnesses")

    if set(targets) != set(registered):
        fail("$.targets", f"must exactly cover manifest (missing={sorted(set(registered)-set(targets))}, extra={sorted(set(targets)-set(registered))})")

    relations_raw = doc["relations"]
    if not isinstance(relations_raw, list):
        fail("$.relations", "must be an array")
    relation_ids: set[str] = set()
    accepted_count = 0
    for i, raw in enumerate(relations_raw):
        path = f"$.relations[{i}]"
        relation = exact_keys(raw, RELATION_KEYS, path)
        relation_id = nonempty_string(relation["relation_id"], f"{path}.relation_id")
        if relation_id in relation_ids:
            fail(f"{path}.relation_id", "duplicate relation")
        relation_ids.add(relation_id)
        if relation["source_target_id"] not in targets:
            fail(f"{path}.source_target_id", "is not a registered target")
        coefficients = relation["coefficients"]
        if not isinstance(coefficients, list) or len(coefficients) != columns:
            fail(f"{path}.coefficients", "length must equal factor_base_columns")
        for j, coefficient in enumerate(coefficients):
            text = nonempty_string(coefficient, f"{path}.coefficients[{j}]")
            if not text.isdecimal() or (text != "0" and text.startswith("0")) or int(text) >= order:
                fail(f"{path}.coefficients[{j}]", "must be a canonical scalar in [0, subgroup_order)")
        if not isinstance(relation["accepted"], bool):
            fail(f"{path}.accepted", "must be boolean")
        if relation["accepted"]:
            accepted_count += 1
            if relation["rejection_reason"] is not None:
                fail(f"{path}.rejection_reason", "accepted row must use null")
        else:
            nonempty_string(relation["rejection_reason"], f"{path}.rejection_reason")

    trace_raw = doc["rank_trace"]
    if not isinstance(trace_raw, list) or len(trace_raw) != accepted_count + 1:
        fail("$.rank_trace", "must contain one entry for every accepted prefix, including prefix 0")
    previous_rank = -1
    for i, raw in enumerate(trace_raw):
        path = f"$.rank_trace[{i}]"
        step = exact_keys(raw, TRACE_KEYS, path)
        prefix = integer(step["prefix"], f"{path}.prefix")
        rank = integer(step["rank"], f"{path}.rank")
        live = integer(step["live_columns"], f"{path}.live_columns")
        dead = integer(step["dead_columns"], f"{path}.dead_columns")
        components = integer(step["support_components"], f"{path}.support_components")
        duplicates = integer(step["duplicate_rows"], f"{path}.duplicate_rows")
        if prefix != i:
            fail(f"{path}.prefix", f"expected {i}")
        if rank < previous_rank or rank > min(prefix, columns):
            fail(f"{path}.rank", "must be monotone and at most min(prefix, columns)")
        if live + dead != columns:
            fail(path, "live_columns + dead_columns must equal factor_base_columns")
        if components > live or duplicates > prefix:
            fail(path, "support_components/duplicate_rows exceed their population")
        previous_rank = rank


def load(path: pathlib.Path) -> Any:
    try:
        return json.loads(path.read_text())
    except (OSError, json.JSONDecodeError) as exc:
        raise ContractError(f"{path}: {exc}") from exc


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("paths", nargs="+", type=pathlib.Path)
    parser.add_argument("--print-digest", action="store_true")
    args = parser.parse_args()
    failed = False
    for path in args.paths:
        try:
            document = load(path)
            if args.print_digest:
                print(f"{path}: {specification_digest(document)}")
            else:
                validate(document)
                print(f"PASS {path}")
        except ContractError as exc:
            failed = True
            print(f"FAIL {exc}", file=sys.stderr)
    return 1 if failed else 0


if __name__ == "__main__":
    raise SystemExit(main())
