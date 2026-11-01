#!/usr/bin/env python3
"""Fail-closed preflight for the full ECDLP dataset campaign protocol.

This validates the campaign-specific invariants in addition to (not in place of)
the shared experiment.protocol/v1 checker. ``admit`` deliberately requires
immutable execution identities and a current per-wave authorization.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import re
import sys
from pathlib import Path

SHA256 = re.compile(r"^[0-9a-f]{64}$")
COMMIT = re.compile(r"^[0-9a-f]{40}$")
IMAGE = re.compile(r"^.+@sha256:[0-9a-f]{64}$")
FAMILIES = {"koblitz", "binary", "prime"}
SOLVERS = {
    "enumerate", "bsgs", "sat", "weil", "f4", "f5", "f6", "root",
    "pkm", "double-large-prime",
}
N = set(range(23, 32)) | set(range(47, 62))
M = set(range(2, 16))


def _obj(value: object, name: str, errors: list[str]) -> dict:
    if not isinstance(value, dict):
        errors.append(f"{name}: expected object")
        return {}
    return value


def _items(value: object, name: str, errors: list[str]) -> list:
    if not isinstance(value, list):
        errors.append(f"{name}: expected array")
        return []
    return value


def _eq(actual: object, expected: object, name: str, errors: list[str]) -> None:
    if actual != expected:
        errors.append(f"{name}: expected {expected!r}, got {actual!r}")


def _sha(value: object, name: str, errors: list[str]) -> None:
    if not isinstance(value, str) or SHA256.fullmatch(value) is None:
        errors.append(f"{name}: expected lowercase SHA256")


def validate(protocol: object, *, admit: bool = False, now: dt.datetime | None = None) -> list[str]:
    """Return every structural/admission error; never mutate the record."""
    errors: list[str] = []
    p = _obj(protocol, "protocol", errors)
    _eq(p.get("schema"), "experiment.protocol/v1", "schema", errors)
    _eq(p.get("repository"), "crypto", "repository", errors)
    arms = _items(p.get("arms"), "arms", errors)
    by_id = {}
    for i, arm in enumerate(arms):
        arm = _obj(arm, f"arms[{i}]", errors)
        key = arm.get("id")
        if not isinstance(key, str) or key in by_id:
            errors.append(f"arms[{i}].id: missing or duplicate")
        else:
            by_id[key] = arm
    _eq(set(by_id), {"dataset", "reference-bsgs"}, "arm ids", errors)
    params = _obj(by_id.get("dataset", {}).get("parameters"), "dataset parameters", errors)
    reference = _obj(by_id.get("reference-bsgs", {}).get("parameters"), "reference parameters", errors)
    axes = {"families": FAMILIES, "n": N, "m": M, "solvers": SOLVERS}
    for key, expected in axes.items():
        rows = _items(params.get(key), f"dataset.{key}", errors)
        if len(rows) != len(expected) or any(type(x) not in (str, int) for x in rows) or set(rows) != expected:
            errors.append(f"dataset.{key}: incomplete or duplicated full-plan axis")
    for key, expected in {"curve_seeds": 16, "generators": 8, "keys": 64}.items():
        _eq(params.get(key), expected, f"dataset.{key}", errors)
    _eq(params.get("twists"), True, "dataset.twists", errors)
    _eq(params.get("population"), "natural", "dataset.population", errors)
    for key in ("families", "n", "m", "twists", "curve_seeds", "generators", "keys", "population"):
        _eq(reference.get(key), params.get(key), f"reference.{key}", errors)
    _eq(reference.get("solvers"), ["bsgs"], "reference.solvers", errors)
    denominator = len(FAMILIES) * len(N) * 2 * 16 * 8 * 64 * len(M) * len(SOLVERS)
    _eq(denominator, 165150720, "computed denominator", errors)
    sample = _obj(p.get("sample_size"), "sample_size", errors)
    _eq(sample.get("instances"), denominator // len(SOLVERS), "sample_size.instances", errors)
    _eq(sample.get("replicates"), 1, "sample_size.replicates", errors)
    claims = _items(p.get("claims"), "claims", errors)
    claim_by_id = {x.get("id"): x for x in claims if isinstance(x, dict)}
    for name in ("C-integrity", "C-coverage"):
        _eq(claim_by_id.get(name, {}).get("class"), "exact_computation", name, errors)
    for name in ("C-falls", "C-model", "C-speed"):
        _eq(claim_by_id.get(name, {}).get("class"), "out_of_scope", name, errors)
    controls = _obj(p.get("controls"), "controls", errors)
    for name in ("aa", "positive", "negative"):
        _eq(_obj(controls.get(name), f"controls.{name}", errors).get("present"), True,
            f"controls.{name}.present", errors)
    metric = _obj(p.get("metrics"), "metrics", errors)
    primary = _items(metric.get("primary"), "metrics.primary", errors)
    _eq({x.get("id") for x in primary if isinstance(x, dict)},
        {"verified_claims", "coverage"}, "primary metric ids", errors)
    for i, row in enumerate(primary):
        if isinstance(row, dict):
            _eq(row.get("kind"), "exact", f"metrics.primary[{i}].kind", errors)
    measurement = _obj(p.get("measurement"), "measurement", errors)
    _eq(measurement.get("clock"), "counted", "measurement.clock", errors)
    execution = _obj(p.get("execution"), "execution", errors)
    _eq(execution.get("measurement_class"), "throughput", "execution.measurement_class", errors)
    checkpoint = _obj(execution.get("checkpoint"), "execution.checkpoint", errors)
    after = _obj(checkpoint.get("after_restore"), "execution.checkpoint.after_restore", errors)
    _eq(after.get("wall_time"), "invalid", "after_restore.wall_time", errors)
    _eq(after.get("counted_units"), "valid", "after_restore.counted_units", errors)
    randomization = _obj(p.get("randomization"), "randomization", errors)
    sampling = _obj(randomization.get("instance_sampling"), "instance_sampling", errors)
    _eq(sampling.get("independent_of_arm"), True, "instance_sampling.independent_of_arm", errors)
    freeze = _obj(p.get("freeze"), "freeze", errors)
    if admit:
        _eq(freeze.get("status"), "frozen", "freeze.status", errors)
        _eq(freeze.get("filed_before_affected_data"), True,
            "freeze.filed_before_affected_data", errors)
        _sha(freeze.get("protocol_sha256"), "freeze.protocol_sha256", errors)
        if not isinstance(freeze.get("frozen_at"), str) or not freeze["frozen_at"]:
            errors.append("freeze.frozen_at: required")
        if not isinstance(freeze.get("commit"), str) or not COMMIT.fullmatch(freeze["commit"]):
            errors.append("freeze.commit: expected 40-character commit")
        instruments = {}
        for name, arm in by_id.items():
            instrument = _obj(arm.get("instrument"), f"{name}.instrument", errors)
            instruments[name] = instrument
            if not isinstance(instrument.get("commit"), str) or not COMMIT.fullmatch(instrument["commit"]):
                errors.append(f"{name}.instrument.commit: expected 40-character commit")
            _sha(instrument.get("binary_sha256"), f"{name}.instrument.binary_sha256", errors)
        if all(name in instruments for name in ("dataset", "reference-bsgs")):
            a = instruments["dataset"]
            b = instruments["reference-bsgs"]
            for field in ("commit", "binary_sha256", "threads", "build"):
                _eq(a.get(field), b.get(field), f"same instrument.{field}", errors)
        for name in ("positive", "negative"):
            control = _obj(controls.get(name), f"controls.{name}", errors)
            ids = _items(control.get("fixed_case_ids"), f"controls.{name}.fixed_case_ids", errors)
            if not ids or any(not isinstance(x, str) or not x for x in ids) or len(ids) != len(set(ids)):
                errors.append(f"controls.{name}.fixed_case_ids: distinct preregistered cases required")
        preflight = _obj(sample.get("preflight_runtime"), "sample_size.preflight_runtime", errors)
        if not isinstance(preflight.get("measured_on"), str) or "pending" in preflight["measured_on"].lower() or "not measured" in preflight["measured_on"].lower():
            errors.append("sample_size.preflight_runtime.measured_on: measured source-bound timing required")
        analysis = _obj(p.get("analysis_plan"), "analysis_plan", errors)
        script = _obj(analysis.get("script"), "analysis_plan.script", errors)
        if not isinstance(script.get("path"), str) or not script["path"]:
            errors.append("analysis_plan.script.path: required")
        _sha(script.get("sha256"), "analysis_plan.script.sha256", errors)
        _eq(analysis.get("frozen_before_data"), True, "analysis_plan.frozen_before_data", errors)
        admission = _obj(execution.get("admission"), "execution.admission", errors)
        _sha(admission.get("plan_sha256"), "execution.admission.plan_sha256", errors)
        if not isinstance(admission.get("image"), str) or not IMAGE.fullmatch(admission["image"]):
            errors.append("execution.admission.image: expected immutable image digest")
        if not isinstance(admission.get("artifact_prefix"), str) or not admission["artifact_prefix"].startswith("s3://"):
            errors.append("execution.admission.artifact_prefix: S3 prefix required")
        for name in ("worker_grant", "budget"):
            item = _obj(admission.get(name), f"execution.admission.{name}", errors)
            if not isinstance(item.get("uri"), str) or not item["uri"]:
                errors.append(f"execution.admission.{name}.uri: required")
            _sha(item.get("sha256"), f"execution.admission.{name}.sha256", errors)
            expiry = item.get("expires_at")
            try:
                when = dt.datetime.fromisoformat(expiry.replace("Z", "+00:00"))
                if when.tzinfo is None or when <= (now or dt.datetime.now(dt.timezone.utc)):
                    errors.append(f"execution.admission.{name}.expires_at: not current")
            except (AttributeError, TypeError, ValueError):
                errors.append(f"execution.admission.{name}.expires_at: expected ISO 8601 UTC")
        budget = _obj(admission.get("budget"), "execution.admission.budget", errors)
        if type(budget.get("cap_usd")) not in (int, float) or budget["cap_usd"] <= 0:
            errors.append("execution.admission.budget.cap_usd: positive cap required")
        if admission.get("region") != budget.get("region") or not admission.get("region"):
            errors.append("execution.admission.region: must match budget region")
        if not isinstance(admission.get("wave_id"), str) or not admission["wave_id"]:
            errors.append("execution.admission.wave_id: required")
    else:
        _eq(freeze.get("status"), "draft", "freeze.status", errors)
    return errors


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("mode", choices=("draft", "admit"))
    parser.add_argument("protocol", type=Path)
    args = parser.parse_args(argv)
    try:
        p = json.loads(args.protocol.read_text())
    except (OSError, ValueError) as exc:
        print(f"protocol read error: {exc}", file=sys.stderr)
        return 2
    errors = validate(p, admit=args.mode == "admit")
    for error in errors:
        print(f"ERROR: {error}", file=sys.stderr)
    if errors:
        return 1
    print(f"{args.mode} validation passed: 165150720 method cells; 16515072 public workloads")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
