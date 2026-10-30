#!/usr/bin/env python3
"""Validate fabricated pencil intake bundles without executing research code.

This executable deliberately has one surface: validating the repository's
synthetic fixture contract.  It has no production mode and no command runner.
"""

from __future__ import annotations

import hashlib
import json
import sys
from pathlib import Path
from typing import Any


RECEIPT_SCHEMA = "pencil-intake-validation/v1"
CONTRACT_SCHEMA = "pencil-intake-contract/v1"
BUNDLE_SCHEMA = "pencil-intake-bundle/v1"
RECORD_SCHEMA = "pencil-intake-record/v1"
SETUP_SCHEMA = "pencil-intake-setup/v1"
PROCESS_RECEIPT_SCHEMA = "pencil-intake-process-receipt/v1"
SYNTHETIC_PREFIX = "SYNTH-PENCIL-"
CONTRACT_PATH = Path(__file__).with_name("intake") / "contract-v1.json"
ROLES = ("primary", "replay")
TERMINAL_STATUSES = {"solved", "failed", "capped"}
RECORD_FIELDS = {
    "schema",
    "workload_id",
    "bits",
    "p",
    "target_index",
    "workload_sha256",
    "role",
    "process_receipt_id",
    "seed",
    "setup_id",
    "setup_sha256",
    "protocol_digest",
    "algorithm_digest",
    "parameter_digest",
    "binary_sha256",
    "solver_mode",
    "source_revision",
    "archive_sha256",
    "plan_set_sha256",
    "resource_guards",
    "status",
    "failure_reason",
    "candidate",
    "rho",
    "phase_counts",
    "operation_counts",
    "coefficient_visits",
    "row_updates",
    "peak_memory_bytes",
    "peak_stored_coefficients",
    "wall_ns",
}
RESOURCE_GUARD_FIELDS = {
    "matrix_coefficient_cap",
    "memory_limit_bytes",
    "wall_limit_seconds_per_target",
}
PROCESS_RECEIPT_FIELDS = {
    "schema",
    "process_receipt_id",
    "workload_id",
    "role",
    "binary_sha256",
    "source_revision",
    "exit_status",
    "wall_ns",
    "peak_memory_bytes",
    "peak_stored_coefficients",
    "guard_outcome",
}
GUARD_OUTCOME_FIELDS = {
    "wall_limit_exceeded",
    "memory_limit_exceeded",
    "matrix_coefficient_cap_reached",
}
MANIFEST_FILE_NAMES = {"setup", "primary", "replay", "process_receipts", "checksums"}
MANIFEST_PATHS = {
    "setup": "setup.json",
    "primary": "primary.jsonl",
    "replay": "replay.jsonl",
    "process_receipts": "process-receipts.jsonl",
    "checksums": "files.sha256",
}
BUNDLE_FILE_NAMES = {
    "manifest.json",
    "setup.json",
    "primary.jsonl",
    "replay.jsonl",
    "process-receipts.jsonl",
    "files.sha256",
}


class IntakeError(Exception):
    """An input violates the fail-closed intake contract."""

    def __init__(
        self,
        code: str,
        detail: str | None = None,
        *,
        path: str | None = None,
        workload_id: str | None = None,
        field: str | None = None,
    ) -> None:
        if detail is None:
            detail = code
            code = "validation_error"
        super().__init__(detail)
        self.code = code
        self.detail = detail
        self.path = path
        self.workload_id = workload_id
        self.field = field

    def receipt_object(self) -> dict[str, Any]:
        return {
            "code": self.code,
            "path": self.path,
            "workload_id": self.workload_id,
            "field": self.field,
            "detail": self.detail,
        }


def _no_duplicate_keys(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise IntakeError("duplicate_json_key", f"duplicate JSON key: {key}", field=key)
        result[key] = value
    return result


def _load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"), object_pairs_hook=_no_duplicate_keys)
    except (OSError, UnicodeError, json.JSONDecodeError) as exc:
        raise IntakeError("json_read_error", f"cannot read JSON {path.name}: {exc}", path=path.name) from exc


def _load_jsonl(path: Path) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    try:
        lines = path.read_text(encoding="utf-8").splitlines()
    except (OSError, UnicodeError) as exc:
        raise IntakeError("jsonl_read_error", f"cannot read JSONL {path.name}: {exc}", path=path.name) from exc
    for line_number, line in enumerate(lines, 1):
        if not line.strip():
            raise IntakeError("jsonl_row_error", f"blank JSONL row in {path.name}:{line_number}", path=path.name)
        try:
            row = json.loads(line, object_pairs_hook=_no_duplicate_keys)
        except json.JSONDecodeError as exc:
            raise IntakeError("jsonl_row_error", f"invalid JSONL row in {path.name}:{line_number}: {exc}", path=path.name) from exc
        if not isinstance(row, dict):
            raise IntakeError("jsonl_row_error", f"non-object JSONL row in {path.name}:{line_number}", path=path.name)
        rows.append(row)
    return rows


def _canonical_sha256(value: Any) -> str:
    encoded = json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True).encode()
    return hashlib.sha256(encoded).hexdigest()


def _file_sha256(path: Path) -> str:
    digest = hashlib.sha256()
    try:
        with path.open("rb") as stream:
            for chunk in iter(lambda: stream.read(1024 * 1024), b""):
                digest.update(chunk)
    except OSError as exc:
        raise IntakeError("file_hash_error", f"cannot hash {path.name}: {exc}", path=path.name) from exc
    return digest.hexdigest()


def _require_exact_fields(value: Any, fields: set[str], label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise IntakeError("schema_error", f"{label} must be an object", path=label)
    actual = set(value)
    if actual != fields:
        missing = sorted(fields - actual)
        extra = sorted(actual - fields)
        raise IntakeError("schema_error", f"{label} fields differ: missing={missing}, extra={extra}", path=label)
    return value


def _require_sha256(value: Any, label: str) -> str:
    if not isinstance(value, str) or len(value) != 64 or any(c not in "0123456789abcdef" for c in value):
        raise IntakeError("digest_format_error", f"{label} must be a lowercase SHA-256 hex digest", field=label)
    return value


def _require_nonnegative_int(value: Any, label: str) -> int:
    if isinstance(value, bool) or not isinstance(value, int) or value < 0:
        raise IntakeError("value_error", f"{label} must be a nonnegative integer", field=label)
    return value


def _safe_relative_path(
    base: Path, raw: Any, label: str, *, must_exist: bool, require_file: bool = True
) -> Path:
    if not isinstance(raw, str) or not raw or "\\" in raw:
        raise IntakeError("unsafe_path", f"{label} must be a nonempty POSIX relative path", path=str(raw) if raw is not None else None)
    candidate = Path(raw)
    if candidate.is_absolute() or ".." in candidate.parts or "." in candidate.parts:
        raise IntakeError("unsafe_path", f"{label} must not be absolute or escape its root", path=raw)
    joined = base / candidate
    try:
        resolved_base = base.resolve(strict=True)
        resolved = joined.resolve(strict=must_exist)
    except OSError as exc:
        raise IntakeError("missing_path", f"{label} cannot be resolved: {exc}", path=raw) from exc
    if resolved != resolved_base and resolved_base not in resolved.parents:
        raise IntakeError("unsafe_path", f"{label} escapes its root", path=raw)
    current = resolved_base
    for part in candidate.parts:
        current = current / part
        if current.exists() and current.is_symlink():
            raise IntakeError("unsafe_path", f"{label} traverses a symbolic link", path=raw)
    if must_exist and require_file and not resolved.is_file():
        raise IntakeError("missing_path", f"{label} is missing or is not a regular file", path=raw)
    if must_exist and not require_file and not resolved.is_dir():
        raise IntakeError("missing_path", f"{label} is missing or is not a directory", path=raw)
    return resolved


def _parse_cli(argv: list[str]) -> tuple[str, str | None]:
    bundle: str | None = None
    receipt: str | None = None
    index = 0
    while index < len(argv):
        token = argv[index]
        if token not in {"--bundle", "--receipt"} or index + 1 >= len(argv):
            raise IntakeError("usage_error", "usage: intake_gate.py --bundle RELATIVE_DIR [--receipt RELATIVE_JSON]")
        value = argv[index + 1]
        if token == "--bundle":
            if bundle is not None:
                raise IntakeError("usage_error", "--bundle may appear only once", field="--bundle")
            bundle = value
        else:
            if receipt is not None:
                raise IntakeError("usage_error", "--receipt may appear only once", field="--receipt")
            receipt = value
        index += 2
    if bundle is None:
        raise IntakeError("usage_error", "usage: intake_gate.py --bundle RELATIVE_DIR [--receipt RELATIVE_JSON]")
    return bundle, receipt


def _load_contract() -> dict[str, Any]:
    contract = _require_exact_fields(
        _load_json(CONTRACT_PATH),
        {
            "schema",
            "synthetic_only",
            "fixture_namespace",
            "setup_id",
            "protocol_digest",
            "algorithm_digest",
            "parameter_digest",
            "binary_sha256",
            "solver_mode",
            "source_revision",
            "archive_sha256",
            "setup_sha256",
            "resource_guards",
            "price_cells",
            "expected_workloads",
            "plan_set_sha256",
        },
        "contract",
    )
    if contract["schema"] != CONTRACT_SCHEMA or contract["synthetic_only"] is not True:
        raise IntakeError("contract is not the synthetic-only v1 contract")
    if contract["fixture_namespace"] != SYNTHETIC_PREFIX:
        raise IntakeError("contract fixture namespace differs")
    for field in (
        "protocol_digest",
        "algorithm_digest",
        "parameter_digest",
        "binary_sha256",
        "archive_sha256",
        "setup_sha256",
        "plan_set_sha256",
    ):
        _require_sha256(contract[field], f"contract.{field}")
    if not isinstance(contract["solver_mode"], str) or not contract["solver_mode"]:
        raise IntakeError("contract_error", "contract.solver_mode must be a nonempty string", field="solver_mode")
    if not isinstance(contract["source_revision"], str) or not contract["source_revision"]:
        raise IntakeError("contract_error", "contract.source_revision must be a nonempty string", field="source_revision")
    guards = _require_exact_fields(contract["resource_guards"], RESOURCE_GUARD_FIELDS, "contract.resource_guards")
    for field, value in guards.items():
        _require_nonnegative_int(value, f"contract.resource_guards.{field}")
    workloads = contract["expected_workloads"]
    if not isinstance(workloads, list) or not workloads:
        raise IntakeError("contract.expected_workloads must be a nonempty array")
    ids: set[str] = set()
    for index, workload in enumerate(workloads):
        row = _require_exact_fields(
            workload,
            {
                "workload_id",
                "bits",
                "p",
                "target_index",
                "seed",
                "workload_sha256",
                "known_answer_scalar",
            },
            f"contract workload {index}",
        )
        workload_id = row["workload_id"]
        if not isinstance(workload_id, str) or not workload_id.startswith(SYNTHETIC_PREFIX):
            raise IntakeError("contract contains a non-synthetic workload ID")
        if workload_id in ids:
            raise IntakeError(f"contract repeats workload {workload_id}")
        ids.add(workload_id)
        for field in ("bits", "p", "target_index", "seed", "known_answer_scalar"):
            _require_nonnegative_int(row[field], f"{workload_id}.{field}")
        _require_sha256(row["workload_sha256"], f"{workload_id}.workload_sha256")
    if _canonical_sha256(workloads) != contract["plan_set_sha256"]:
        raise IntakeError("contract plan-set digest does not match its workload array")
    cells = contract["price_cells"]
    if not isinstance(cells, list) or not cells or len(set(cells)) != len(cells) or not all(isinstance(c, str) and c for c in cells):
        raise IntakeError("contract.price_cells must be unique nonempty strings")
    return contract


def _validate_prices(value: Any, cells: list[str], label: str) -> None:
    prices = _require_exact_fields(value, set(cells), label)
    for cell, price in prices.items():
        if isinstance(price, bool) or not isinstance(price, (int, float)) or price < 0:
            raise IntakeError(f"{label}.{cell} must be a nonnegative number")


def _validate_record(
    row: dict[str, Any],
    role: str,
    expected: dict[str, dict[str, Any]],
    contract: dict[str, Any],
    setup_sha256: str,
) -> None:
    workload_id = row.get("workload_id", "<missing>")
    _require_exact_fields(row, RECORD_FIELDS, f"{role} record {workload_id}")
    if row["schema"] != RECORD_SCHEMA or row["role"] != role:
        raise IntakeError("record_schema_error", f"{workload_id} has wrong record schema or role", workload_id=str(workload_id), field="schema_or_role")
    if not isinstance(workload_id, str) or not workload_id.startswith(SYNTHETIC_PREFIX):
        raise IntakeError("non_synthetic_workload", f"real or non-synthetic workload ID rejected: {workload_id}", workload_id=str(workload_id), field="workload_id")
    if workload_id not in expected:
        raise IntakeError("unknown_workload", f"unknown workload ID: {workload_id}", workload_id=workload_id, field="workload_id")
    spec = expected[workload_id]
    for field in ("bits", "p", "target_index", "workload_sha256", "seed"):
        if row[field] != spec[field]:
            raise IntakeError("record_identity_drift", f"{field} drift for {workload_id}", workload_id=workload_id, field=field)
    for field in (
        "protocol_digest",
        "algorithm_digest",
        "parameter_digest",
        "binary_sha256",
        "solver_mode",
        "source_revision",
        "archive_sha256",
        "plan_set_sha256",
    ):
        if row[field] != contract[field]:
            raise IntakeError("record_identity_drift", f"{field} drift for {workload_id}", workload_id=workload_id, field=field)
    if row["resource_guards"] != contract["resource_guards"]:
        raise IntakeError("resource_guard_mismatch", f"resource guard drift for {workload_id}", workload_id=workload_id, field="resource_guards")
    if row["setup_id"] != contract["setup_id"] or row["setup_sha256"] != setup_sha256:
        raise IntakeError("setup_mismatch", f"setup drift for {workload_id}", workload_id=workload_id, field="setup")
    process_id = row["process_receipt_id"]
    if not isinstance(process_id, str) or not process_id.startswith("SYNTH-PROCESS-"):
        raise IntakeError("process_receipt_invalid", f"invalid {role} process receipt for {workload_id}", workload_id=workload_id, field="process_receipt_id")
    if row["status"] not in TERMINAL_STATUSES:
        raise IntakeError("status_invalid", f"invalid terminal status for {workload_id}", workload_id=workload_id, field="status")
    failure_reason = row["failure_reason"]
    if row["status"] == "solved":
        if failure_reason is not None:
            raise IntakeError("candidate_status_invalid", f"solved record has a failure reason for {workload_id}", workload_id=workload_id, field="failure_reason")
        candidate = _require_exact_fields(row["candidate"], {"verified", "scalar", "prices"}, f"candidate for {workload_id}")
        if candidate["verified"] is not True:
            raise IntakeError("candidate_unverified", f"candidate is unverified for {workload_id}", workload_id=workload_id, field="candidate.verified")
        _require_nonnegative_int(candidate["scalar"], f"candidate scalar for {workload_id}")
        if candidate["scalar"] != spec["known_answer_scalar"]:
            raise IntakeError(
                "known_answer_mismatch",
                f"candidate scalar disagrees with the known answer for {workload_id}",
                workload_id=workload_id,
                field="candidate.scalar",
            )
        _validate_prices(candidate["prices"], contract["price_cells"], f"candidate prices for {workload_id}")
    else:
        if not isinstance(failure_reason, str) or not failure_reason:
            raise IntakeError("failure_reason_missing", f"{row['status']} record lacks a failure reason for {workload_id}", workload_id=workload_id, field="failure_reason")
        if row["candidate"] is not None:
            raise IntakeError("failure_pricing_nonnull", f"{row['status']} record has a non-null candidate price for {workload_id}", workload_id=workload_id, field="candidate")
    rho = _require_exact_fields(row["rho"], {"verified", "scalar", "prices"}, f"rho for {workload_id}")
    if rho["verified"] is not True:
        raise IntakeError("rho_unverified", f"rho is unverified for {workload_id}", workload_id=workload_id, field="rho.verified")
    _require_nonnegative_int(rho["scalar"], f"rho scalar for {workload_id}")
    if rho["scalar"] != spec["known_answer_scalar"]:
        raise IntakeError(
            "known_answer_mismatch",
            f"rho scalar disagrees with the known answer for {workload_id}",
            workload_id=workload_id,
            field="rho.scalar",
        )
    _validate_prices(rho["prices"], contract["price_cells"], f"rho prices for {workload_id}")
    for field in ("phase_counts", "operation_counts"):
        counts = row[field]
        if not isinstance(counts, dict) or not counts:
            raise IntakeError(f"{field} must be a nonempty object for {workload_id}")
        for key, count in counts.items():
            if not isinstance(key, str) or not key:
                raise IntakeError(f"{field} has an invalid key for {workload_id}")
            _require_nonnegative_int(count, f"{field}.{key} for {workload_id}")
    for field in (
        "coefficient_visits",
        "row_updates",
        "peak_memory_bytes",
        "peak_stored_coefficients",
        "wall_ns",
    ):
        _require_nonnegative_int(row[field], f"{field} for {workload_id}")


def _non_wall_record(row: dict[str, Any]) -> dict[str, Any]:
    return {key: value for key, value in row.items() if key not in {"role", "process_receipt_id", "wall_ns"}}


def _validate_process_receipts(
    rows: list[dict[str, Any]],
    by_role: dict[str, dict[str, dict[str, Any]]],
    contract: dict[str, Any],
) -> None:
    expected_ids = {
        record["process_receipt_id"]
        for role_records in by_role.values()
        for record in role_records.values()
    }
    seen: set[str] = set()
    for row in rows:
        process_id = row.get("process_receipt_id", "<missing>")
        _require_exact_fields(row, PROCESS_RECEIPT_FIELDS, f"process receipt {process_id}")
        if row["schema"] != PROCESS_RECEIPT_SCHEMA:
            raise IntakeError("process_receipt_schema_error", f"wrong process receipt schema for {process_id}", field="schema")
        if process_id in seen:
            raise IntakeError("duplicate_process_receipt", f"duplicate process receipt {process_id}", field="process_receipt_id")
        seen.add(process_id)
        role = row["role"]
        workload_id = row["workload_id"]
        if role not in ROLES or workload_id not in by_role.get(role, {}):
            raise IntakeError("unknown_process_receipt", f"process receipt {process_id} has an unknown role/workload", workload_id=str(workload_id), field="role_or_workload_id")
        record = by_role[role][workload_id]
        if process_id != record["process_receipt_id"]:
            raise IntakeError("process_identity_drift", f"process receipt ID differs for {workload_id}", workload_id=workload_id, field="process_receipt_id")
        for field in ("binary_sha256", "source_revision"):
            if row[field] != contract[field]:
                raise IntakeError("process_identity_drift", f"process {field} differs for {workload_id}", workload_id=workload_id, field=field)
        for field in ("exit_status", "wall_ns", "peak_memory_bytes", "peak_stored_coefficients"):
            _require_nonnegative_int(row[field], f"process receipt {process_id}.{field}")
        for field in ("wall_ns", "peak_memory_bytes", "peak_stored_coefficients"):
            if row[field] != record[field]:
                raise IntakeError("process_measurement_mismatch", f"process {field} differs from its target record for {workload_id}", workload_id=workload_id, field=field)
        outcome = _require_exact_fields(row["guard_outcome"], GUARD_OUTCOME_FIELDS, f"process receipt {process_id}.guard_outcome")
        if not all(isinstance(value, bool) for value in outcome.values()):
            raise IntakeError("guard_outcome_invalid", f"guard outcomes must be booleans for {workload_id}", workload_id=workload_id, field="guard_outcome")
        guards = contract["resource_guards"]
        derived = {
            "wall_limit_exceeded": row["wall_ns"] > guards["wall_limit_seconds_per_target"] * 1_000_000_000,
            "memory_limit_exceeded": row["peak_memory_bytes"] > guards["memory_limit_bytes"],
            "matrix_coefficient_cap_reached": row["peak_stored_coefficients"] >= guards["matrix_coefficient_cap"],
        }
        if outcome != derived:
            raise IntakeError("guard_outcome_mismatch", f"guard outcome disagrees with measurements for {workload_id}", workload_id=workload_id, field="guard_outcome")
        guard_stopped = any(derived.values())
        status = record["status"]
        if status == "solved" and (guard_stopped or row["exit_status"] != 0):
            raise IntakeError("guard_status_inconsistent", f"solved record exceeded a guard or has nonzero exit for {workload_id}", workload_id=workload_id, field="status")
        if status == "capped" and (not guard_stopped or row["exit_status"] == 0):
            raise IntakeError("guard_status_inconsistent", f"capped record lacks a measured guard stop or nonzero exit for {workload_id}", workload_id=workload_id, field="status")
        if status == "failed" and (guard_stopped or row["exit_status"] == 0):
            raise IntakeError("guard_status_inconsistent", f"failed record has a guard stop or zero exit for {workload_id}", workload_id=workload_id, field="status")
    if seen != expected_ids:
        raise IntakeError(
            "process_receipt_set_mismatch",
            f"process receipt set mismatch: missing={sorted(expected_ids - seen)}, extra={sorted(seen - expected_ids)}",
            field="process_receipt_id",
        )


def validate_bundle(bundle_raw: str, receipt: dict[str, Any]) -> None:
    cwd = Path.cwd()
    bundle_dir = _safe_relative_path(
        cwd, bundle_raw, "bundle path", must_exist=True, require_file=False
    )
    actual_bundle_names = {entry.name for entry in bundle_dir.iterdir()}
    if actual_bundle_names != BUNDLE_FILE_NAMES or any(
        not entry.is_file() or entry.is_symlink() for entry in bundle_dir.iterdir()
    ):
        raise IntakeError(
            "bundle_file_set_mismatch",
            f"bundle file set mismatch: missing={sorted(BUNDLE_FILE_NAMES - actual_bundle_names)}, unlisted={sorted(actual_bundle_names - BUNDLE_FILE_NAMES)}",
            path=bundle_raw,
            field="bundle_files",
        )
    receipt["bundle"] = bundle_raw
    contract = _load_contract()
    receipt["contract_version"] = contract["schema"]
    receipt["plan_set_sha256"] = contract["plan_set_sha256"]
    receipt["checks"].append("contract")

    manifest_path = _safe_relative_path(bundle_dir, "manifest.json", "manifest path", must_exist=True)
    manifest = _require_exact_fields(
        _load_json(manifest_path),
        {
            "schema",
            "synthetic_only",
            "contract_version",
            "plan_set_sha256",
            "protocol_digest",
            "algorithm_digest",
            "parameter_digest",
            "binary_sha256",
            "solver_mode",
            "source_revision",
            "archive_sha256",
            "setup_sha256",
            "resource_guards",
            "setup_id",
            "files",
        },
        "manifest",
    )
    if manifest["schema"] != BUNDLE_SCHEMA or manifest["synthetic_only"] is not True:
        raise IntakeError("manifest is not a synthetic-only v1 bundle")
    for field in (
        "contract_version",
        "plan_set_sha256",
        "protocol_digest",
        "algorithm_digest",
        "parameter_digest",
        "binary_sha256",
        "solver_mode",
        "source_revision",
        "archive_sha256",
        "setup_sha256",
        "resource_guards",
        "setup_id",
    ):
        expected_value = contract["schema"] if field == "contract_version" else contract[field]
        if manifest[field] != expected_value:
            raise IntakeError("manifest_identity_drift", f"manifest {field} drift", field=field)
    if not isinstance(manifest["files"], dict) or set(manifest["files"]) != MANIFEST_FILE_NAMES:
        actual_manifest_files = set(manifest["files"]) if isinstance(manifest["files"], dict) else set()
        raise IntakeError(
            "manifest_file_set_mismatch",
            f"manifest file set mismatch: missing={sorted(MANIFEST_FILE_NAMES - actual_manifest_files)}, extra={sorted(actual_manifest_files - MANIFEST_FILE_NAMES)}",
            field="manifest.files",
        )
    files = manifest["files"]
    resolved_files: dict[str, Path] = {}
    receipt["input_sha256"] = {}
    for name in ("setup", "primary", "replay", "process_receipts", "checksums"):
        entry = _require_exact_fields(files[name], {"path", "sha256", "bytes"}, f"manifest.files.{name}")
        if entry["path"] != MANIFEST_PATHS[name]:
            raise IntakeError(
                "manifest_path_mismatch",
                f"manifest path for {name} must be {MANIFEST_PATHS[name]}",
                path=str(entry["path"]),
                field=f"manifest.files.{name}.path",
            )
        expected_hash = _require_sha256(entry["sha256"], f"manifest.files.{name}.sha256")
        expected_bytes = _require_nonnegative_int(entry["bytes"], f"manifest.files.{name}.bytes")
        path = _safe_relative_path(bundle_dir, entry["path"], f"manifest.files.{name}.path", must_exist=True)
        actual_hash = _file_sha256(path)
        if actual_hash != expected_hash:
            raise IntakeError("file_hash_mismatch", f"file hash drift for {name}", path=str(entry["path"]), field="sha256")
        if path.stat().st_size != expected_bytes:
            raise IntakeError("file_length_mismatch", f"file byte length drift for {name}", path=str(entry["path"]), field="bytes")
        resolved_files[name] = path
        receipt["input_sha256"][name] = actual_hash
    receipt["checks"].extend(["bundle_file_set", "manifest_file_set", "paths", "file_hashes", "file_lengths"])

    checksum_names = ("primary", "process_receipts", "replay", "setup")
    expected_checksum_text = "".join(
        f"{files[name]['sha256']}  {files[name]['path']}\n" for name in checksum_names
    )
    try:
        checksum_text = resolved_files["checksums"].read_text(encoding="ascii")
    except (OSError, UnicodeError) as exc:
        raise IntakeError("checksum_index_error", f"cannot read files.sha256: {exc}", path="files.sha256") from exc
    if checksum_text != expected_checksum_text:
        raise IntakeError("checksum_index_mismatch", "files.sha256 does not exactly enumerate the payload digests", path="files.sha256", field="contents")
    receipt["checks"].append("checksum_index")

    setup = _require_exact_fields(
        _load_json(resolved_files["setup"]),
        {"schema", "setup_id", "public_fixture", "field_modulus", "curve_model", "factor_base_size", "note"},
        "setup",
    )
    if setup["schema"] != SETUP_SCHEMA or setup["public_fixture"] is not True or setup["setup_id"] != contract["setup_id"]:
        raise IntakeError("missing or mismatched public synthetic setup")
    _require_nonnegative_int(setup["field_modulus"], "setup.field_modulus")
    _require_nonnegative_int(setup["factor_base_size"], "setup.factor_base_size")
    if not isinstance(setup["curve_model"], str) or not isinstance(setup["note"], str):
        raise IntakeError("setup text fields must be strings")
    setup_sha256 = receipt["input_sha256"]["setup"]
    if setup_sha256 != contract["setup_sha256"]:
        raise IntakeError("setup_substitution", "setup bytes differ from the contract-pinned digest", path=files["setup"]["path"], field="setup_sha256")
    receipt["checks"].append("setup")

    expected = {row["workload_id"]: row for row in contract["expected_workloads"]}
    by_role: dict[str, dict[str, dict[str, Any]]] = {}
    process_ids: dict[str, set[str]] = {}
    for role in ROLES:
        rows = _load_jsonl(resolved_files[role])
        mapped: dict[str, dict[str, Any]] = {}
        ids: set[str] = set()
        for row in rows:
            _validate_record(row, role, expected, contract, setup_sha256)
            workload_id = row["workload_id"]
            if workload_id in mapped:
                raise IntakeError("duplicate_workload", f"duplicate {role} workload ID: {workload_id}", workload_id=workload_id, field="workload_id")
            process_id = row["process_receipt_id"]
            if process_id in ids:
                raise IntakeError(
                    "process_receipt_alias",
                    f"multiple {role} target records share process receipt {process_id}",
                    workload_id=workload_id,
                    field="process_receipt_id",
                )
            mapped[workload_id] = row
            ids.add(process_id)
        actual_set = set(mapped)
        expected_set = set(expected)
        if actual_set != expected_set:
            raise IntakeError(
                "workload_set_mismatch",
                f"{role} workload set mismatch: missing={sorted(expected_set - actual_set)}, unknown={sorted(actual_set - expected_set)}",
                field=f"{role}.workload_ids",
            )
        by_role[role] = mapped
        process_ids[role] = ids
        receipt["counts"][role] = len(mapped)
    receipt["checks"].extend(["workload_sets", "seeds_and_digests", "rho_and_failure_pricing"])

    overlap = process_ids["primary"] & process_ids["replay"]
    if overlap:
        raise IntakeError("process_overlap", f"primary and replay process receipts overlap: {sorted(overlap)}", field="process_receipt_id")
    process_rows = _load_jsonl(resolved_files["process_receipts"])
    _validate_process_receipts(process_rows, by_role, contract)
    receipt["counts"]["process_receipts"] = len(process_rows)
    receipt["checks"].extend(["binary_identity", "process_receipts", "resource_guard_outcomes"])
    for workload_id in sorted(expected):
        if _non_wall_record(by_role["primary"][workload_id]) != _non_wall_record(by_role["replay"][workload_id]):
            raise IntakeError("replay_mismatch", f"non-wall replay mismatch for {workload_id}", workload_id=workload_id, field="non_wall_record")
    receipt["checks"].extend(["process_separation", "non_wall_replay_equality"])


def main(argv: list[str]) -> int:
    receipt: dict[str, Any] = {
        "schema": RECEIPT_SCHEMA,
        "synthetic_only": True,
        "status": "rejected",
        "accepted": False,
        "contract_version": None,
        "bundle": None,
        "plan_set_sha256": None,
        "counts": {"primary": 0, "replay": 0, "process_receipts": 0},
        "input_sha256": {},
        "checks": [],
        "errors": [],
    }
    receipt_raw: str | None = None
    try:
        bundle_raw, receipt_raw = _parse_cli(argv)
        validate_bundle(bundle_raw, receipt)
        receipt["accepted"] = True
        receipt["status"] = "synthetic_fixture_accepted"
    except IntakeError as exc:
        receipt["errors"].append(exc.receipt_object())
    except (OSError, ValueError, TypeError) as exc:
        receipt["errors"].append(
            IntakeError("internal_validation_error", str(exc)).receipt_object()
        )

    if receipt_raw is not None:
        try:
            output_path = _safe_relative_path(Path.cwd(), receipt_raw, "receipt path", must_exist=False)
            output_path.parent.mkdir(parents=False, exist_ok=True)
            output_path.write_text(json.dumps(receipt, indent=2) + "\n", encoding="utf-8")
        except IntakeError as exc:
            receipt["accepted"] = False
            receipt["status"] = "rejected"
            receipt["errors"].append(exc.receipt_object())
        except OSError as exc:
            receipt["accepted"] = False
            receipt["status"] = "rejected"
            receipt["errors"].append(
                IntakeError("receipt_write_error", str(exc), path=receipt_raw).receipt_object()
            )
    sys.stdout.write(json.dumps(receipt, separators=(",", ":")) + "\n")
    return 0 if receipt["accepted"] else 1


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
