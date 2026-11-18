#!/usr/bin/env python3
"""Bounded repeatable Stage-21 natural relation-yield measurement.

Only natural-arm exact verified hits count as measured relations.  Planted and
proven-UNSAT arms are correctness controls, not part of throughput or prevalence.
n=23 Koblitz is a frozen public-synthetic experimental control, NOT ECC2K-130.
"""
from __future__ import annotations

import argparse
import csv
from datetime import datetime, timezone
import json
import os
from pathlib import Path
import platform
import subprocess
import sys
import time

REPO = Path(__file__).resolve().parents[1]
CSV_PATH = REPO / "results/hourly_log.csv"
BASELINE_PATH = REPO / "results/relation_baseline.json"
ALERT_PATH = REPO / "results/hourly_alert.md"
HEADER = [
    "timestamp_utc", "source_sha", "curve_n", "curve_a", "factor_base_hash",
    "natural_trials", "natural_relations", "planted_count", "planted_verified",
    "unsat_count", "unsat_hits", "wall_seconds", "relations_per_sec",
    "success_rate", "status", "anomaly", "host_cpu", "run_url", "details",
]
THRESHOLD = 0.20
EXPECTED_NATURAL = 256
EXPECTED_PLANTED = 64
EXPECTED_UNSAT = 64


def git_sha() -> str:
    return subprocess.check_output(
        ["git", "rev-parse", "HEAD"], cwd=REPO, text=True, timeout=10
    ).strip()


def cpu_name() -> str:
    try:
        for line in Path("/proc/cpuinfo").read_text().splitlines():
            if line.startswith("model name") and ":" in line:
                return line.split(":", 1)[1].strip()
    except OSError:
        pass
    return platform.processor() or platform.machine()


def verified_metrics(result: dict, wall_seconds: float) -> dict:
    if result.get("schema") != "koblitz_relation_yield_bridge.v1" or result.get("status") != "complete":
        raise ValueError("unexpected producer schema/status")
    if not result.get("production_defaults_used"):
        raise ValueError("smoke fixture is not admitted to the production log")
    policy = result["frozen_policy"]
    if (policy["n"], policy["a"], policy["m"]) != (23, 0, 2):
        raise ValueError("curve differs from the frozen Stage-21 n=23,a=0,m=2 control")
    arms = result["arms"]
    natural, planted, unsat = (arms[k] for k in ("natural", "planted_sat", "proven_unsat"))
    if (natural["count"], planted["count"], unsat["count"]) != (
        EXPECTED_NATURAL, EXPECTED_PLANTED, EXPECTED_UNSAT
    ):
        raise ValueError("production target mixture changed")
    nhits = natural["hits"]
    if not (0 <= nhits <= natural["count"]):
        raise ValueError("natural hits out of range")
    if natural["verified_point_witnesses"] != nhits:
        raise ValueError("natural witnesses not all verified")
    if planted["hits"] != planted["count"] or planted["verified_point_witnesses"] != planted["count"]:
        raise ValueError("planted-SAT correctness control failed")
    if unsat["hits"] != 0 or unsat["verified_point_witnesses"] != 0:
        raise ValueError("proven-UNSAT correctness control failed")
    if wall_seconds <= 0:
        raise ValueError("invalid wall time")
    rate = nhits / wall_seconds
    return dict(
        curve_n=23, curve_a=0,
        factor_base_hash=result["factor_base"]["factor_base_blake3"],
        natural_trials=natural["count"], natural_relations=nhits,
        planted_count=planted["count"], planted_verified=planted["verified_point_witnesses"],
        unsat_count=unsat["count"], unsat_hits=unsat["hits"],
        wall_seconds=round(wall_seconds, 6),
        relations_per_sec=round(rate, 9),
        success_rate=round(nhits / natural["count"], 9),
    )


def percent_change(value: float, baseline: float) -> float:
    if baseline == 0:
        return 0.0 if value == 0 else float("inf")
    return (value / baseline - 1) * 100.0


def compare(metrics: dict, baseline: dict) -> list[str]:
    anomalies = []
    if (
        metrics["curve_n"] != baseline.get("curve_n")
        or metrics["curve_a"] != baseline.get("curve_a")
        or metrics["factor_base_hash"] != baseline.get("factor_base_hash")
    ):
        return ["curve/factor-base identity no longer matches the baseline"]
    for key in ("relations_per_sec", "success_rate"):
        old = float(baseline[key])
        new = float(metrics[key])
        change = percent_change(new, old)
        if abs(change) > THRESHOLD * 100:
            anomalies.append(
                f"{key}: {old:.9g} -> {new:.9g} ({change:+.1f}% vs baseline)"
            )
    return anomalies


def append_log(row: dict) -> None:
    CSV_PATH.parent.mkdir(parents=True, exist_ok=True)
    needs_header = not CSV_PATH.exists() or CSV_PATH.stat().st_size == 0
    with CSV_PATH.open("a", encoding="utf-8", newline="") as output:
        writer = csv.DictWriter(output, fieldnames=HEADER, extrasaction="ignore")
        if needs_header:
            writer.writeheader()
        writer.writerow(row)


def run_command(args: list[str], timeout: int) -> subprocess.CompletedProcess:
    return subprocess.run(
        args, cwd=REPO, text=True, capture_output=True,
        timeout=timeout, check=False,
    )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--skip-build", action="store_true", help="for offline parser tests")
    parser.add_argument("--build-timeout", type=int, default=1200)
    parser.add_argument("--run-timeout", type=int, default=900)
    args = parser.parse_args()
    ALERT_PATH.unlink(missing_ok=True)
    timestamp = datetime.now(timezone.utc).isoformat()
    source_sha = git_sha()
    run_url = (
        f"https://github.com/{os.environ['GITHUB_REPOSITORY']}/actions/runs/{os.environ['GITHUB_RUN_ID']}"
        if os.getenv("GITHUB_REPOSITORY") and os.getenv("GITHUB_RUN_ID") else ""
    )
    row = dict.fromkeys(HEADER, "")
    row.update(timestamp_utc=timestamp, source_sha=source_sha, curve_n=23, curve_a=0,
               status="failed", host_cpu=cpu_name(), run_url=run_url)
    issues = []
    try:
        if not args.skip_build:
            build = run_command(
                ["cargo", "build", "--release", "--locked", "--example", "koblitz_relation_yield_bridge"],
                args.build_timeout,
            )
            if build.returncode:
                raise RuntimeError(f"Cargo build rc={build.returncode}: {build.stderr[-1800:]}")
        binary = REPO / "target/release/examples/koblitz_relation_yield_bridge"
        start = time.perf_counter()
        outcome = run_command([str(binary)], args.run_timeout)
        wall_seconds = time.perf_counter() - start
        if outcome.returncode:
            raise RuntimeError(f"relation harness rc={outcome.returncode}: {outcome.stderr[-1800:]}")
        data = json.loads(outcome.stdout)
        metrics = verified_metrics(data, wall_seconds)
        row.update(metrics)
        row["status"] = "success"
        raw = os.getenv("RUNNER_TEMP")
        if raw:
            Path(raw, "hourly_relation_result.json").write_text(
                json.dumps(data, indent=2, sort_keys=True) + "\n", encoding="utf-8"
            )
        if BASELINE_PATH.exists():
            baseline = json.loads(BASELINE_PATH.read_text(encoding="utf-8"))
            issues = compare(metrics, baseline)
        else:
            baseline = {
                "schema": "hourly_relation_baseline.v1", "established_at": timestamp,
                "source_sha": source_sha, "curve_n": 23, "curve_a": 0,
                "factor_base_hash": metrics["factor_base_hash"],
                "relations_per_sec": metrics["relations_per_sec"],
                "success_rate": metrics["success_rate"],
                "host_cpu": row["host_cpu"],
                "threshold_fraction": THRESHOLD,
                "scope": "frozen public-synthetic n=23 exact two-summand natural-arm yield; not ECC2K130",
            }
            BASELINE_PATH.parent.mkdir(parents=True, exist_ok=True)
            BASELINE_PATH.write_text(json.dumps(baseline, indent=2) + "\n", encoding="utf-8")
            row["details"] = "first successful run; baseline established; no comparison"
        if issues:
            row["status"] = "regression"
    except (OSError, ValueError, KeyError, json.JSONDecodeError, subprocess.TimeoutExpired, RuntimeError) as exc:
        issues = [f"run failed: {type(exc).__name__}: {exc}"]
        row["details"] = str(exc)[:500].replace("\n", " ")
    row["anomaly"] = " | ".join(issues)[:1000]
    append_log(row)
    if issues:
        ALERT_PATH.write_text(
            "# Hourly Koblitz relation-finding alert\n\n"
            f"- UTC timestamp: {timestamp}\n- Commit: `{source_sha}`\n"
            f"- Curve: Koblitz over GF(2^23), a=0, m=2 (public-synthetic control)\n"
            f"- Job: {run_url or '(local run)'}\n- CPU: {row['host_cpu']}\n"
            f"- Natural trials: {row['natural_trials']}\n"
            f"- Natural verified relations: {row['natural_relations']}\n"
            f"- Wall seconds: {row['wall_seconds']}\n"
            f"- Relations/second: {row['relations_per_sec']}\n"
            f"- Success rate: {row['success_rate']}\n"
            f"- Threshold: > {THRESHOLD * 100:.0f}% change from committed first-run baseline\n\n"
            + "\n".join(f"- {issue}" for issue in issues) +
            "\n\nThe CSV row is retained even on failure. Throughput excludes compilation but includes "
            "factor-base construction and exact pair enumeration. Runner-to-runner CPU variation "
            "may make wall throughput incomparable; inspect CPU models before interpreting anomalies.\n",
            encoding="utf-8",
        )
    print(json.dumps({key: row[key] for key in HEADER}, sort_keys=True))
    return 1 if issues else 0


if __name__ == "__main__":
    sys.exit(main())
