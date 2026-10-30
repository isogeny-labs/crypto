#!/usr/bin/env python3
"""Freeze the metadata-only inventory of open point-driven-pencil workloads.

This tool reads the frozen ecbench archive and the two standardized pencil
indexes.  It does not import or invoke collector, matrix, nullspace, replay,
or recovery code.  ``--check`` reconstructs the canonical bytes in memory and
checks every pinned identity, status, count, size, and digest assertion.
"""

from __future__ import annotations

import argparse
import collections
import csv
import hashlib
import json
import subprocess
import sys
from pathlib import Path
from typing import Any, Iterable


REPO = Path(__file__).resolve().parents[1]
ARCHIVE = Path("research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst")
ARCHIVE_SHA256 = "550f7178e30b60db62931a752b0c179beeef7ea99a96cf94486ff8aee631d339"
SPARSE_INDEX = Path("research/pencil_sparse_20261022/results/targets.jsonl")
SPARSE_INDEX_SHA256 = "18f48596af6f3d724eebd7cb710798477dc6f927fa510eae78dc54228c5d5e23"
MARKOWITZ_INDEX = Path("research/pencil_markowitz_20261022/results/targets.jsonl")
MARKOWITZ_INDEX_SHA256 = "5a4f2e3de2b4c896af0aabd4e32244aba5a49ae78aeb84f801acf6ba5459d503"
COVERAGE_INDEX = Path("research/four_followups_review_20261022/results/pencil-coverage.csv")
COVERAGE_INDEX_SHA256 = "738083bd44d9a1e5133701b37985fd7c9f5e293065ddcbc3876a0082948169af"
DEFAULT_OUTPUT = Path(
    "research/four_followups_review_20261022/open-pencil-inventory/open-workloads.jsonl"
)

BITS = (16, 20, 24, 28, 32, 36, 40)
ARCHIVE_ARMS = {
    ("genus_choice2", "candidate"),
    ("genus_projective", "candidate"),
    ("rho_neg", "reference"),
}
EXPECTED_ARCHIVE_WORKLOADS = 1260
EXPECTED_VERIFIED = 543
EXPECTED_OPEN = 717
EXPECTED_OPEN_BY_BITS = {28: 177, 32: 180, 36: 180, 40: 180}
EXPECTED_CAP_KEY = (32, 186379, 0)
EXPECTED_CAP_WORKLOAD = "Wa0fc1e72d7a0"
EXPECTED_OUTPUT_BYTES = 434_290
EXPECTED_OUTPUT_SHA256 = "efd504b64437309408c3f199fcf3ed85e94322ef01d7d1545d87f6bf1409ae3e"

WorkloadKey = tuple[int, int, int]
WorkloadValue = tuple[str, str, str, str, int, int]


class InventoryError(RuntimeError):
    """A frozen-input or canonical-inventory assertion failed."""


def require(condition: bool, message: str) -> None:
    if not condition:
        raise InventoryError(message)


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def json_lines(path: Path) -> Iterable[dict[str, Any]]:
    with path.open(encoding="utf-8") as stream:
        for line_number, line in enumerate(stream, 1):
            if not line.strip():
                continue
            try:
                row = json.loads(line)
            except json.JSONDecodeError as error:
                raise InventoryError(f"{path}:{line_number}: invalid JSON: {error}") from error
            require(isinstance(row, dict), f"{path}:{line_number}: row must be an object")
            yield row


def archive_member_lines(archive: Path, member: str) -> Iterable[dict[str, Any]]:
    command = ["tar", "--zstd", "-xOf", str(archive), member]
    try:
        completed = subprocess.run(
            command,
            cwd=REPO,
            check=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )
    except FileNotFoundError as error:
        raise InventoryError("tar with zstd support is required") from error
    except subprocess.CalledProcessError as error:
        detail = error.stderr.strip() or f"exit {error.returncode}"
        raise InventoryError(f"cannot read {member} from {archive}: {detail}") from error
    for line_number, line in enumerate(completed.stdout.splitlines(), 1):
        if not line.strip():
            continue
        try:
            row = json.loads(line)
        except json.JSONDecodeError as error:
            raise InventoryError(f"{member}:{line_number}: invalid JSON: {error}") from error
        require(isinstance(row, dict), f"{member}:{line_number}: row must be an object")
        yield row


def load_archive() -> dict[WorkloadKey, WorkloadValue]:
    archive = REPO / ARCHIVE
    require(archive.is_file(), f"missing frozen archive: {ARCHIVE}")
    require(
        sha256_file(archive) == ARCHIVE_SHA256,
        f"frozen archive digest mismatch: {ARCHIVE}",
    )

    workloads: dict[WorkloadKey, WorkloadValue] = {}
    repetitions: collections.Counter[WorkloadKey] = collections.Counter()
    observed_arms: dict[WorkloadKey, set[tuple[str, str]]] = collections.defaultdict(set)
    workload_ids: dict[str, WorkloadKey] = {}
    workload_payloads: dict[WorkloadKey, dict[str, Any]] = {}
    algorithm_seeds: dict[WorkloadKey, str] = {}

    for bits in BITS:
        member = f"sessions_projective_chart_v2/order-genus-{bits}/records.jsonl"
        member_count = 0
        for row in archive_member_lines(archive, member):
            member_count += 1
            require(row.get("schema") == "ecbench.record/v1", f"{member}: unexpected schema")
            workload = row.get("workload")
            require(isinstance(workload, dict), f"{member}: missing workload object")
            curve_spec = workload.get("curve_spec")
            require(isinstance(curve_spec, dict), f"{member}: missing workload.curve_spec")
            outcome = row.get("outcome")
            require(isinstance(outcome, dict), f"{member}: missing outcome object")
            require(outcome.get("status") == "verified", f"{member}: unverified archive record")
            require(outcome.get("matches_target") is True, f"{member}: target verification failed")
            require(outcome.get("matches_planted") is True, f"{member}: planted-scalar check failed")

            try:
                key = (bits, int(curve_spec["p"]), int(workload["target_index"]))
                workload_id = str(workload["workload_id"])
                workload_sha256 = str(workload["workload_sha256"])
                arm_role = (str(row["arm"]), str(row["role"]))
            except (KeyError, TypeError, ValueError) as error:
                raise InventoryError(f"{member}: incomplete workload identity") from error

            require(len(workload_sha256) == 64, f"{member}: malformed workload SHA-256")
            require(
                workload_id == "W" + workload_sha256[:12],
                f"{member}: workload ID/hash mismatch for {key}",
            )
            try:
                planted = int(workload["planted"])
                target_seed = int(workload["target_seed"])
            except (KeyError, TypeError, ValueError) as error:
                raise InventoryError(f"{member}: missing planted scalar or target seed") from error
            algorithm_seed = str(row.get("algorithm_seed", ""))
            require(bool(algorithm_seed), f"{member}: missing algorithm seed for {key}")
            value = (workload_id, workload_sha256, member, algorithm_seed, planted, target_seed)
            previous = workloads.setdefault(key, value)
            require(previous == value, f"{member}: inconsistent repeated identity for {key}")
            previous_key = workload_ids.setdefault(workload_id, key)
            require(previous_key == key, f"{member}: duplicate workload ID {workload_id}")
            previous_payload = workload_payloads.setdefault(key, workload)
            require(previous_payload == workload, f"{member}: repeated workload payload drift for {key}")
            previous_seed = algorithm_seeds.setdefault(key, algorithm_seed)
            require(previous_seed == algorithm_seed, f"{member}: algorithm seed drift for {key}")
            require(arm_role in ARCHIVE_ARMS, f"{member}: unexpected arm/role {arm_role}")
            require(
                arm_role not in observed_arms[key],
                f"{member}: duplicate arm/role {arm_role} for {key}",
            )
            observed_arms[key].add(arm_role)
            repetitions[key] += 1

        require(member_count == 540, f"{member}: expected 540 records, got {member_count}")

    require(
        len(workloads) == EXPECTED_ARCHIVE_WORKLOADS,
        f"expected {EXPECTED_ARCHIVE_WORKLOADS} archive workloads, got {len(workloads)}",
    )
    require(len(workload_ids) == len(workloads), "archive workload IDs are not globally unique")
    require(set(repetitions.values()) == {3}, "each archive workload must have exactly three records")
    require(
        all(arms == ARCHIVE_ARMS for arms in observed_arms.values()),
        "each archive workload must have exactly the two IC arms and matched rho arm",
    )
    require(
        collections.Counter(key[0] for key in workloads) == collections.Counter({b: 180 for b in BITS}),
        "archive bit-shard counts differ from 180 each",
    )
    return workloads


def load_pencil_partition(
    workloads: dict[WorkloadKey, WorkloadValue],
) -> tuple[set[WorkloadKey], set[WorkloadKey], dict[WorkloadKey, set[str]]]:
    sources = (
        (SPARSE_INDEX, SPARSE_INDEX_SHA256, None, 1449, 1449),
        (MARKOWITZ_INDEX, MARKOWITZ_INDEX_SHA256, "markowitz", 121, 61),
    )
    verified: set[WorkloadKey] = set()
    capped: set[WorkloadKey] = set()
    raw_sources: dict[WorkloadKey, set[str]] = collections.defaultdict(set)

    for relative_path, expected_digest, mode_filter, expected_rows, expected_used in sources:
        path = REPO / relative_path
        require(path.is_file(), f"missing standardized pencil index: {relative_path}")
        require(sha256_file(path) == expected_digest, f"index digest mismatch: {relative_path}")
        rows = list(json_lines(path))
        require(len(rows) == expected_rows, f"{relative_path}: expected {expected_rows} rows")
        used = 0
        for row in rows:
            require(row.get("schema") == "pencil-sparse-target/v1", f"{relative_path}: bad schema")
            if mode_filter is not None and row.get("solver_mode") != mode_filter:
                continue
            used += 1
            try:
                key = (int(row["bits"]), int(row["curve_p"]), int(row["target_index"]))
                identity = (str(row["workload_id"]), str(row["workload_sha256"]))
                raw_path = str(row["raw_path"])
            except (KeyError, TypeError, ValueError) as error:
                raise InventoryError(f"{relative_path}: incomplete pencil identity") from error
            require(key in workloads, f"{relative_path}: unknown workload key {key}")
            require(workloads[key][:2] == identity, f"{relative_path}: identity drift for {key}")
            require(bool(raw_path), f"{relative_path}: empty raw_path for {key}")
            require(row.get("rho_verified") is True, f"{relative_path}: unverified rho for {key}")
            candidate_verified = row.get("candidate_verified")
            require(isinstance(candidate_verified, bool), f"{relative_path}: invalid candidate status")
            if candidate_verified:
                require(row.get("status") == "verified", f"{relative_path}: verified/status mismatch")
                require(row.get("failure") is None, f"{relative_path}: verified row has failure")
                require(row.get("recovered") == row.get("planted"), f"{relative_path}: scalar mismatch")
                verified.add(key)
            else:
                require(row.get("status") == "incomplete", f"{relative_path}: cap/status mismatch")
                require(row.get("failure") == "sparse_matrix_cap", f"{relative_path}: unexpected failure")
                require(row.get("recovered") is None, f"{relative_path}: capped row recovered a scalar")
                capped.add(key)
            raw_sources[key].add(raw_path)
        require(used == expected_used, f"{relative_path}: expected {expected_used} selected rows, got {used}")

    require(not verified & capped, "a workload is both verified and capped")
    require(len(verified) == EXPECTED_VERIFIED, f"expected {EXPECTED_VERIFIED} verified keys")
    require(capped == {EXPECTED_CAP_KEY}, f"unexpected cap-only keys: {sorted(capped)}")
    require(
        workloads[EXPECTED_CAP_KEY][0] == EXPECTED_CAP_WORKLOAD,
        "cap-only workload ID changed",
    )
    return verified, capped, raw_sources


def check_coverage_index(
    workloads: dict[WorkloadKey, WorkloadValue],
    verified: set[WorkloadKey],
    capped: set[WorkloadKey],
) -> None:
    path = REPO / COVERAGE_INDEX
    require(path.is_file(), f"missing coverage index: {COVERAGE_INDEX}")
    require(sha256_file(path) == COVERAGE_INDEX_SHA256, f"coverage digest mismatch: {COVERAGE_INDEX}")
    with path.open(newline="", encoding="utf-8") as stream:
        rows = list(csv.DictReader(stream))
    require(len(rows) == EXPECTED_ARCHIVE_WORKLOADS, "coverage index must contain 1,260 rows")
    coverage: dict[WorkloadKey, tuple[str, str]] = {}
    for row in rows:
        try:
            key = (int(row["bits"]), int(row["p"]), int(row["target_index"]))
            value = (str(row["workload_id"]), str(row["coverage_status"]))
        except (KeyError, TypeError, ValueError) as error:
            raise InventoryError(f"{COVERAGE_INDEX}: incomplete identity/status row") from error
        require(key not in coverage, f"{COVERAGE_INDEX}: duplicate key {key}")
        coverage[key] = value
    require(set(coverage) == set(workloads), "coverage/archive key sets differ")
    for key, (workload_id, status) in coverage.items():
        require(workloads[key][0] == workload_id, f"coverage workload ID drift for {key}")
        expected_status = (
            "verified_candidate_present"
            if key in verified
            else "cap_without_verified_candidate"
            if key in capped
            else "no_indexed_pencil_record"
        )
        require(status == expected_status, f"coverage status drift for {key}: {status}")




def canonical_inventory() -> tuple[bytes, dict[str, Any]]:
    workloads = load_archive()
    verified, capped, raw_sources = load_pencil_partition(workloads)
    check_coverage_index(workloads, verified, capped)
    open_keys = set(workloads) - verified
    require(len(open_keys) == EXPECTED_OPEN, f"expected {EXPECTED_OPEN} open workloads")
    open_by_bits = collections.Counter(key[0] for key in open_keys)
    require(dict(sorted(open_by_bits.items())) == EXPECTED_OPEN_BY_BITS, "open bit-shard counts changed")

    rows: list[dict[str, Any]] = []
    for bits, p, target_index in sorted(open_keys):
        key = (bits, p, target_index)
        workload_id, workload_sha256, member, algorithm_seed, planted, target_seed = workloads[key]
        rows.append(
            {
                "schema": "pencil-open-workload/v1",
                "bits": bits,
                "p": p,
                "target_index": target_index,
                "workload_id": workload_id,
                "workload_sha256": workload_sha256,
                "algorithm_seed": algorithm_seed,
                "planted": planted,
                "target_seed": target_seed,
                "coverage_status": (
                    "cap_without_verified_candidate" if key in capped else "no_indexed_pencil_record"
                ),
                "source_archive": str(ARCHIVE),
                "source_archive_sha256": ARCHIVE_SHA256,
                "source_member": member,
                "prior_source_records": sorted(raw_sources[key]),
            }
        )

    blob = "".join(
        json.dumps(row, sort_keys=True, separators=(",", ":")) + "\n" for row in rows
    ).encode("utf-8")
    digest = hashlib.sha256(blob).hexdigest()
    require(len(blob) == EXPECTED_OUTPUT_BYTES, f"canonical byte count changed: {len(blob)}")
    require(digest == EXPECTED_OUTPUT_SHA256, f"canonical digest changed: {digest}")
    require(rows[0]["bits"] == 28 and rows[-1]["bits"] == 40, "canonical sort order changed")
    summary = {
        "schema": "pencil-open-inventory-check/v1",
        "archive_workloads": len(workloads),
        "verified_workloads": len(verified),
        "open_workloads": len(open_keys),
        "open_by_bits": {str(k): v for k, v in sorted(open_by_bits.items())},
        "cap_only_workloads": len(capped),
        "output_bytes": len(blob),
        "output_sha256": digest,
    }
    return blob, summary


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--output",
        type=Path,
        default=DEFAULT_OUTPUT,
        help=f"output path relative to the repository (default: {DEFAULT_OUTPUT})",
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="reconstruct and validate in memory; if output is hydrated, verify it too",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        blob, summary = canonical_inventory()
        output = args.output if args.output.is_absolute() else REPO / args.output
        if args.check:
            if output.exists():
                require(output.read_bytes() == blob, f"hydrated output differs: {output}")
            summary["mode"] = "check"
            summary["hydrated_output_checked"] = output.exists()
        else:
            output.parent.mkdir(parents=True, exist_ok=True)
            output.write_bytes(blob)
            summary["mode"] = "write"
            summary["output"] = str(output.relative_to(REPO))
        print(json.dumps(summary, sort_keys=True))
        return 0
    except InventoryError as error:
        print(f"freeze-pencil-open-inventory: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
