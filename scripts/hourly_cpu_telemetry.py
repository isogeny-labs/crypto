"""Generic, conservative CPU telemetry for the frozen synthetic control."""
import csv
import hashlib
import json
import math
import os
from pathlib import Path
import platform
import shutil
import statistics
import subprocess
import tempfile
import time

EXTRA = ["runner_arch", "build_id", "measurement_mode", "baseline_version",
         "baseline_samples", "baseline_state", "throughput_deviation_pct",
         "wall_deviation_pct", "cycles", "instructions", "cache_misses",
         "task_clock_ms", "perf_status"]
KEY = ["host_cpu", "runner_arch", "build_id", "measurement_mode",
       "curve_n", "curve_a", "factor_base_hash", "natural_trials",
       "planted_count", "unsat_count"]
METRICS = ["relations_per_sec", "wall_seconds"]
EVENTS = {"cycles": "cycles", "instructions": "instructions",
          "cache-misses": "cache_misses", "task-clock": "task_clock_ms"}
MINIMUM = 10


def fingerprint(binary):
    # Exact executable identity, compiler version and relevant build flags.
    # Do not pool nonidentical builds merely because the CPU matches.
    digest = hashlib.sha256()
    with binary.open("rb") as f:
        for block in iter(lambda: f.read(1024 * 1024), b""):
            digest.update(block)
    return hashlib.sha256(json.dumps({
        "binary_sha256": digest.hexdigest(),
        "compiler": subprocess.check_output(["rustc", "-Vv"], text=True, timeout=10),
        "flags": {k: os.getenv(k, "") for k in (
            "RUSTFLAGS", "CARGO_INCREMENTAL", "CARGO_PROFILE_RELEASE_DEBUG",
            "CARGO_BUILD_TARGET")},
    }, sort_keys=True).encode()).hexdigest()


def cohort(row):
    values = ["" if row.get(k) is None else str(row[k]) for k in KEY]
    if not all(values):
        return None
    return hashlib.sha256(json.dumps(values).encode()).hexdigest()


def known_good(row):
    try:
        return (row.get("status") == "success" and not row.get("anomaly")
                and int(row["natural_trials"]) == 256
                and int(row["natural_relations"]) == 163
                and int(row["planted_count"]) == int(row["planted_verified"]) == 64
                and int(row["unsat_count"]) == 64 and int(row["unsat_hits"]) == 0
                and all(math.isfinite(float(row[k])) and float(row[k]) > 0
                        for k in METRICS))
    except (KeyError, TypeError, ValueError):
        return False


def stats(rows, key):
    a = [float(r[key]) for r in rows]
    median = statistics.median(a)
    return {"median": median, "mad": statistics.median(abs(x - median) for x in a)}


def compare_cpu(row, history, store):
    """Use exactly the first ten independently recorded, matching prior successes."""
    key = cohort(row)
    row.update(baseline_state="provisional", baseline_samples=0)
    if key is None:
        return []
    records = store.setdefault("cohorts", {})
    baseline = records.get(key)
    if baseline is None:
        good, seen = [], set()
        for r in history:
            run = r.get("run_url")
            if cohort(r) == key and known_good(r) and run and run not in seen and run != row.get("run_url"):
                good.append(r)
                seen.add(run)
            if len(good) == MINIMUM:
                break
        row["baseline_samples"] = len(good)
        if len(good) < MINIMUM:
            return []
        baseline = {"identity": {k: str(row[k]) for k in KEY},
                    "sample_count": MINIMUM, "runs": [r["run_url"] for r in good],
                    **{k: stats(good, k) for k in METRICS}}
        baseline["version"] = hashlib.sha256(
            json.dumps(baseline, sort_keys=True).encode()).hexdigest()
        records[key] = baseline
    row.update(baseline_state="qualified", baseline_samples=baseline["sample_count"],
               baseline_version=baseline["version"])
    issues = []
    for key, field in zip(METRICS, ["throughput_deviation_pct", "wall_deviation_pct"]):
        old = baseline[key]["median"]
        delta = (float(row[key]) / old - 1) * 100
        row[field] = round(delta, 6)
        if abs(delta) > 20 + 1e-9:
            issues.append(f"{key}: {old:.9g} -> {row[key]} ({delta:+.1f}% vs CPU cohort)")
    return issues


def read_history(path):
    if not path.exists():
        return []
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def append_row(path, header, row):
    # Append new columns once without dropping old logical CSV records.
    path.parent.mkdir(parents=True, exist_ok=True)
    if path.exists() and path.stat().st_size:
        with path.open(newline="", encoding="utf-8") as f:
            old = next(csv.reader(f))
        if old != header:
            if header[:len(old)] != old:
                raise ValueError("unexpected historical CSV schema")
            rows = read_history(path)
            temp = path.with_suffix(".migration.tmp")
            with temp.open("w", newline="", encoding="utf-8") as f:
                writer = csv.DictWriter(f, fieldnames=header)
                writer.writeheader()
                writer.writerows(rows)
            temp.replace(path)
    new = not path.exists() or not path.stat().st_size
    with path.open("a", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=header)
        if new:
            writer.writeheader()
        writer.writerow(row)


def perf_values(path):
    values = {}
    for line in path.read_text().splitlines():
        parts = line.split(";")
        if len(parts) < 3 or parts[2] not in EVENTS:
            continue
        try:
            value = float(parts[0])
            if math.isfinite(value) and value >= 0:
                values[EVENTS[parts[2]]] = value
        except ValueError:
            pass
    return values


def run_measured(binary, run_command, timeout, row):
    row.update(runner_arch=platform.machine(), build_id=fingerprint(binary),
               measurement_mode="plain", perf_status="unavailable")
    def timed(args):
        start = time.perf_counter()
        result = run_command(args, timeout)
        return result, time.perf_counter() - start

    perf = shutil.which("perf")
    if not perf:
        return timed([str(binary)])
    with tempfile.TemporaryDirectory() as directory:
        path = Path(directory) / "stat"
        prefix = [perf, "stat", "-x", ";", "--no-big-num", "-e",
                  ",".join(EVENTS), "-o", str(path), "--"]
        # Probe access without executing the benchmark or fabricating counters.
        try:
            probe = run_command(prefix + ["true"], 10)
            available = probe.returncode == 0 and len(perf_values(path)) == len(EVENTS)
        except (OSError, subprocess.TimeoutExpired):
            available = False
        if not available:
            return timed([str(binary)])
        row["measurement_mode"] = "perf-stat"
        path.unlink(missing_ok=True)
        outcome, seconds = timed(prefix + [str(binary)])
        values = perf_values(path) if path.exists() else {}
        row.update(values)
        row["perf_status"] = "available" if len(values) == len(EVENTS) else "partial"
        return outcome, seconds
