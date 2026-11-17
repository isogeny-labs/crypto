#!/usr/bin/env python3
"""Validate `validation/v1` records without third-party dependencies.

    python3 tools/validate_validation_json.py path/to/VALIDATION.json ...
    python3 tools/validate_validation_json.py --legacy-ok research/**

Files whose `schema` is not `validation/v1` are reported and skipped when
`--legacy-ok` is given, because records written before the vocabulary existed
are immutable. A file that claims `validation/v1` is held to every rule.

Exit 0 when every checked record is valid, 1 otherwise.
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

SCHEMA = "validation/v1"
TIERS = {"toy", "medium", "crypto"}
STATUSES = {"validated", "pre-gate", "unvalidated", "failed"}
KINDS = {"none", "replay", "proof", "solver"}
SHA = re.compile(r"^sha256:[0-9a-f]{64}$")
COMMIT = re.compile(r"^[0-9a-f]{40}$")
UID = re.compile(r"^urn:ec-(record|candidate):1:sha256:[0-9a-f]{64}$")


def check(record: dict) -> list[str]:
    errors: list[str] = []
    for key in ("schema", "claim_tier", "status", "certificate", "reference", "inputs"):
        if key not in record:
            errors.append(f"missing {key}")
    if errors:
        return errors
    if record["claim_tier"] not in TIERS:
        errors.append(f"claim_tier {record['claim_tier']!r} not in {sorted(TIERS)}")
    if record["status"] not in STATUSES:
        errors.append(f"status {record['status']!r} not in {sorted(STATUSES)}")

    cert = record["certificate"]
    if not isinstance(cert, dict) or cert.get("kind") not in KINDS:
        errors.append("certificate.kind must be one of none|replay|proof|solver")
    elif cert["kind"] == "none":
        if cert.get("digest") is not None or cert.get("verifier") is not None:
            errors.append("certificate kind=none requires digest=null and verifier=null")
    else:
        if not SHA.match(str(cert.get("digest", ""))):
            errors.append("certificate.digest must be sha256:<64 hex>")
        if not str(cert.get("verifier") or "").strip():
            errors.append("certificate.verifier is required")

    ref = record["reference"]
    if not isinstance(ref, dict) or ref.get("kind") != "matched-rho":
        errors.append("reference.kind must be 'matched-rho'")
    else:
        if not SHA.match(str(ref.get("point_sha256", ""))):
            errors.append("reference.point_sha256 must be sha256:<64 hex>")
        if sorted(map(str, ref.get("band", []))) != ["0", "0.1", "1"]:
            errors.append("reference.band must be [0, 0.1, 1]")

    inputs = record["inputs"]
    if not isinstance(inputs, dict):
        errors.append("inputs must be an object")
    else:
        if not UID.match(str(inputs.get("curve_uid", ""))):
            errors.append("inputs.curve_uid must be urn:ec-record:1:sha256:<64 hex>")
        if not COMMIT.match(str(inputs.get("code_commit", ""))):
            errors.append("inputs.code_commit must be a 40-hex commit")
        if "seed" not in inputs:
            errors.append("inputs.seed is required")
    return errors


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("paths", nargs="+", type=Path)
    parser.add_argument("--legacy-ok", action="store_true")
    args = parser.parse_args(argv)

    failures = 0
    checked = 0
    for path in args.paths:
        try:
            record = json.loads(path.read_text())
        except (OSError, json.JSONDecodeError) as exc:
            print(f"{path}: unreadable: {exc}", file=sys.stderr)
            failures += 1
            continue
        schema = record.get("schema") if isinstance(record, dict) else None
        if schema != SCHEMA:
            if args.legacy_ok:
                print(f"{path}: skipped legacy schema {schema!r}")
            else:
                print(f"{path}: schema is {schema!r}, not {SCHEMA!r}", file=sys.stderr)
                failures += 1
            continue
        checked += 1
        errors = check(record)
        if errors:
            failures += 1
            for line in errors:
                print(f"{path}: {line}", file=sys.stderr)
    if failures:
        print(f"validation/v1: {failures} problem(s)", file=sys.stderr)
        return 1
    print(f"validation/v1: OK ({checked} record(s))")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
