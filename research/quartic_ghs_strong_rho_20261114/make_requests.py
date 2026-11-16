#!/usr/bin/env python3
"""Fail-closed generator for the four Round-A IsoLab command requests."""

from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import math
import re
import subprocess
from pathlib import Path
from typing import Any

ACCOUNT = "104186811093"
REGION = "us-east-2"
ROUND_ID = "ghs-strong-rho-r1-duration-a"
STUDY = "quartic_ghs_strong_rho_20261114"
IMAGE_RE = re.compile(
    r"^104186811093\.dkr\.ecr\.us-east-2\.amazonaws\.com/isolab/tasks@sha256:[a-f0-9]{64}$"
)
HEX64_RE = re.compile(r"^[a-f0-9]{64}$")
REV_RE = re.compile(r"^[a-f0-9]{40}$")
MAIN_SHA256 = "b2244ffece12855622a88f9a2af4926b1fac6113b9519fdf11e2bc7449fa27c8"
STRONG_SHA256 = "73abd342613dee852add1e3459c94bc7f949801c94eb875a309cdfc4f6b0f5de"
POD_RUN_SHA256 = "a7a7795c0013485855805c87c14bb67fee2864273a934e72b85ebb3bef7e0a6e"
SPLITS = {"low": 3600, "mid": 3600, "high": 7200, "top": 10800}
DAILY_BUDGETS = {"isolab-daily-cost-100", "spot-experiments-daily-25"}
MONTHLY_BUDGET = "isolab-monthly-account-cost"
BUDGET_LIMITS = {
    "isolab-daily-cost-100": ("DAILY", 200.0),
    "spot-experiments-daily-25": ("DAILY", 200.0),
    "isolab-monthly-account-cost": ("MONTHLY", 1000.0),
}


class Refusal(ValueError):
    """The launch package is incomplete or outside the frozen contract."""


def need(condition: bool, message: str) -> None:
    if not condition:
        raise Refusal(message)


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def parse_time(value: str) -> dt.datetime:
    need(isinstance(value, str), "timestamp is not a string")
    try:
        parsed = dt.datetime.fromisoformat(value.replace("Z", "+00:00"))
    except ValueError as exc:
        raise Refusal(f"invalid timestamp: {value}") from exc
    need(parsed.tzinfo is not None, "timestamp lacks a timezone")
    return parsed.astimezone(dt.timezone.utc)


def nonnegative_number(value: Any, label: str) -> float:
    need(isinstance(value, (int, float)) and not isinstance(value, bool), f"{label} is not numeric")
    parsed = float(value)
    need(math.isfinite(parsed) and parsed >= 0.0, f"{label} is negative or non-finite")
    return parsed


def validate_preflight(data: dict[str, Any], now: dt.datetime) -> None:
    """Validate a read-only operator receipt without granting launch authority."""
    need(data.get("schema") == "ghs.strong-rho.preflight/v1", "wrong preflight schema")
    need(data.get("account_id") == ACCOUNT, "wrong AWS account")
    need(data.get("region") == REGION, "wrong AWS region")
    observed = parse_time(data.get("observed_at", ""))
    age = (now.astimezone(dt.timezone.utc) - observed).total_seconds()
    need(-60 <= age <= 1800, "preflight is stale or from the future")

    budgets = data.get("budgets")
    need(isinstance(budgets, dict), "missing budget receipt")
    need(nonnegative_number(budgets.get("daily_allowance_usd"), "daily allowance") == 200.0, "daily allowance mismatch")
    need(nonnegative_number(budgets.get("monthly_allowance_usd"), "monthly allowance") == 1000.0, "monthly allowance mismatch")
    readbacks = budgets.get("readbacks")
    need(isinstance(readbacks, list), "budget readbacks missing")
    by_name = {row.get("name"): row for row in readbacks if isinstance(row, dict)}
    need(len(by_name) == len(readbacks), "budget readbacks contain a duplicate or non-object")
    need(set(by_name) == set(BUDGET_LIMITS), "budget readbacks are missing or contain an unknown budget")
    computed_headrooms = []
    for name in (*sorted(DAILY_BUDGETS), MONTHLY_BUDGET):
        row = by_name[name]
        need(row.get("read_ok") is True, f"budget read failed: {name}")
        expected_period, expected_limit = BUDGET_LIMITS[name]
        need(row.get("period") == expected_period, f"budget period mismatch: {name}")
        limit = nonnegative_number(row.get("limit_usd"), f"{name} limit")
        actual = nonnegative_number(row.get("actual_spend_usd"), f"{name} actual spend")
        reserved = nonnegative_number(row.get("reserved_spend_usd"), f"{name} reserved spend")
        remaining = nonnegative_number(row.get("remaining_usd"), f"{name} remaining")
        need(limit == expected_limit, f"budget limit mismatch: {name}")
        computed = limit - actual - reserved
        need(computed >= 0.0, f"budget is exhausted: {name}")
        need(math.isclose(remaining, computed, rel_tol=0.0, abs_tol=0.01), f"budget arithmetic mismatch: {name}")
        computed_headrooms.append(computed)
        retrieved = parse_time(row.get("retrieved_at", ""))
        read_age = (now.astimezone(dt.timezone.utc) - retrieved).total_seconds()
        need(-60 <= read_age <= 1800, f"budget readback is stale: {name}")
    declared_headroom = nonnegative_number(budgets.get("unreserved_headroom_usd"), "unreserved headroom")
    effective_headroom = min(computed_headrooms)
    need(math.isclose(declared_headroom, effective_headroom, rel_tol=0.0, abs_tol=0.01), "unreserved headroom does not match the live readbacks")
    need(effective_headroom >= 5.0, "less than USD 5 unreserved headroom")

    pool = data.get("pool")
    need(isinstance(pool, dict), "pool receipt missing")
    need(pool.get("pool") == "cpu-spot", "wrong NodePool")
    need(pool.get("all_tenant_inventory") is True, "pool receipt is not all-tenant")
    need(pool.get("fits_next_wave") is True, "next 8-vCPU wave does not fit")
    requested_cpu = pool.get("requested_cpu")
    need(isinstance(requested_cpu, int) and not isinstance(requested_cpu, bool) and requested_cpu == 8,
         "pool receipt must price exactly 8 vCPU")
    need(pool.get("requested_memory") == "8Gi", "pool receipt must price 8Gi")
    need(HEX64_RE.fullmatch(str(pool.get("receipt_sha256", ""))) is not None, "invalid pool receipt hash")

    quota = data.get("quota")
    need(isinstance(quota, dict) and quota.get("fits_next_wave") is True, "namespace quota does not fit")
    need(HEX64_RE.fullmatch(str(quota.get("receipt_sha256", ""))) is not None, "invalid quota receipt hash")
    jobs = data.get("active_jobs")
    need(isinstance(jobs, dict) and jobs.get("recorded") is True, "active Jobs were not recorded")
    need(HEX64_RE.fullmatch(str(jobs.get("receipt_sha256", ""))) is not None, "invalid Jobs receipt hash")
    for gate_name in ("checkpoint_gate", "git_custody"):
        gate = data.get(gate_name)
        need(isinstance(gate, dict) and gate.get("ok") is True, f"{gate_name} did not pass")
        need(HEX64_RE.fullmatch(str(gate.get("receipt_sha256", ""))) is not None, f"invalid {gate_name} hash")


def validate_image(data: dict[str, Any], revision: str, root: Path) -> None:
    need(data.get("schema") == "ghs.strong-rho.image/v1", "wrong image schema")
    need(data.get("source_revision") == revision, "image/source revision mismatch")
    need(IMAGE_RE.fullmatch(str(data.get("image_uri", ""))) is not None, "image is not the pinned IsoLab repository digest")
    expected = {
        "main_sha256": MAIN_SHA256,
        "strong_rho_sha256": STRONG_SHA256,
        "pod_run_sha256": POD_RUN_SHA256,
        "protocol_sha256": sha256_file(root / "PROTOCOL.md"),
        "round_sha256": sha256_file(root / "ROUND_A.md"),
        "dockerfile_sha256": sha256_file(root / "Dockerfile"),
    }
    for key, value in expected.items():
        need(data.get(key) == value, f"image manifest {key} mismatch")
    parse_time(data.get("built_at", ""))


def build_requests(image: dict[str, Any], revision: str, protocol_hash: str, round_hash: str,
                   preflight_hash: str) -> dict[str, dict[str, Any]]:
    need(REV_RE.fullmatch(revision) is not None, "invalid source revision")
    need(all(HEX64_RE.fullmatch(v) for v in (protocol_hash, round_hash, preflight_hash)), "invalid provenance hash")
    requests: dict[str, dict[str, Any]] = {}
    for split, seconds in SPLITS.items():
        requests[split] = {
            "schema": "isolab.eks.command-request/v1",
            "job": {
                "job_name": f"ghs-r1a-{split}-{revision[:10]}",
                "image": image["image_uri"],
                "command": [
                    "/usr/bin/env",
                    "--chdir=/opt/crypto",
                    "/opt/crypto/research/quartic_ghs_strong_rho_20261114/pod_run.sh",
                    split,
                ],
                "cpu": 4,
                "memory": "4Gi",
                "seconds": seconds,
                "isolation_profile": "standard",
                "retries": 0,
                "wait_seconds": 0,
            },
            "provenance": {
                "harness": "isolab-eks-command-round-a",
                "source_revision": revision,
                "study": STUDY,
                "protocol_sha256": protocol_hash,
                "note": (
                    f"round={ROUND_ID};round_sha256={round_hash};"
                    f"preflight_sha256={preflight_hash};waves=low+mid,high+top"
                ),
            },
        }
    return requests


def git_output(root: Path, *args: str) -> str:
    return subprocess.check_output(["git", "-C", str(root), *args], text=True).strip()


def write_json(path: Path, value: Any) -> None:
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def command_generate(args: argparse.Namespace) -> None:
    study = Path(__file__).resolve().parent
    repository = study.parents[1]
    revision = args.source_revision
    need(REV_RE.fullmatch(revision) is not None, "source revision must be a full SHA-1")
    need(git_output(repository, "rev-parse", "HEAD") == revision, "checkout is not the requested revision")
    need(git_output(repository, "status", "--porcelain") == "", "checkout is dirty")
    need(sha256_file(study / "main.rs") == MAIN_SHA256, "main.rs changed after implementation freeze")
    need(sha256_file(study / "strong_rho.rs") == STRONG_SHA256, "strong_rho.rs changed after implementation freeze")
    need(sha256_file(study / "pod_run.sh") == POD_RUN_SHA256, "pod_run.sh changed after implementation freeze")

    now = parse_time(args.now) if args.now else dt.datetime.now(dt.timezone.utc)
    preflight_path = Path(args.preflight).resolve()
    image_path = Path(args.image).resolve()
    preflight = json.loads(preflight_path.read_text(encoding="utf-8"))
    image = json.loads(image_path.read_text(encoding="utf-8"))
    validate_preflight(preflight, now)
    validate_image(image, revision, study)

    destination = Path(args.output).resolve()
    need(not destination.exists(), f"output already exists: {destination}")
    destination.mkdir(parents=True)
    protocol_hash = sha256_file(study / "PROTOCOL.md")
    round_hash = sha256_file(study / "ROUND_A.md")
    preflight_hash = sha256_file(preflight_path)
    requests = build_requests(image, revision, protocol_hash, round_hash, preflight_hash)
    for split, request in requests.items():
        write_json(destination / f"request-{split}.json", request)
    manifest = {
        "schema": "ghs.strong-rho.request-set/v1",
        "round_id": ROUND_ID,
        "source_revision": revision,
        "protocol_sha256": protocol_hash,
        "round_sha256": round_hash,
        "preflight_sha256": preflight_hash,
        "image_manifest_sha256": sha256_file(image_path),
        "waves": [["low", "mid"], ["high", "top"]],
        "requests": {
            split: {
                "path": f"request-{split}.json",
                "sha256": sha256_file(destination / f"request-{split}.json"),
            }
            for split in SPLITS
        },
    }
    write_json(destination / "manifest.json", manifest)
    print(destination / "manifest.json")


def command_validate_preflight(args: argparse.Namespace) -> None:
    now = parse_time(args.now) if args.now else dt.datetime.now(dt.timezone.utc)
    data = json.loads(Path(args.preflight).read_text(encoding="utf-8"))
    validate_preflight(data, now)
    print("preflight: admissible for request generation")


def parser() -> argparse.ArgumentParser:
    out = argparse.ArgumentParser(description=__doc__)
    sub = out.add_subparsers(dest="command", required=True)
    validate = sub.add_parser("validate-preflight")
    validate.add_argument("preflight")
    validate.add_argument("--now", help="ISO-8601 clock override for deterministic replay")
    validate.set_defaults(func=command_validate_preflight)
    generate = sub.add_parser("generate")
    generate.add_argument("--preflight", required=True)
    generate.add_argument("--image", required=True)
    generate.add_argument("--source-revision", required=True)
    generate.add_argument("--output", required=True)
    generate.add_argument("--now", help="ISO-8601 clock override for deterministic replay")
    generate.set_defaults(func=command_generate)
    return out


def main() -> int:
    try:
        args = parser().parse_args()
        args.func(args)
    except (OSError, json.JSONDecodeError, subprocess.CalledProcessError, Refusal) as exc:
        print(f"make_requests: refused: {exc}", file=__import__("sys").stderr)
        return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
