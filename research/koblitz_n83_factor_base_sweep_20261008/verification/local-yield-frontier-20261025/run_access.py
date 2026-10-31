#!/usr/bin/env python3
"""One source-pinned primary-N83 XCNF access call with fail-closed model replay."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import resource
import signal
import subprocess
import time


HERE = Path(__file__).resolve().parent
STUDY = HERE.parent.parent
PREVIOUS = HERE.parent / "external-xor-20261025" / "receipt.json"


def require(ok: bool, why: str) -> None:
    if not ok:
        raise ValueError(why)


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def write_new(path: Path, value: dict) -> None:
    with path.open("x", encoding="utf-8") as stream:
        json.dump(value, stream, indent=2, sort_keys=True)
        stream.write("\n")
        stream.flush()
        os.fsync(stream.fileno())


def paths(args, protocol):
    formula_dir = args.formulas_dir / protocol["input_name"]
    base = args.archives_dir / f"s3-{protocol['base_object_blake3']}.jsonl.gz"
    pool = args.archives_dir / f"s3-{protocol['pool_object_blake3']}.jsonl.gz"
    return formula_dir, base, pool


def preflight(args, protocol):
    require(protocol["schema"] == "n83.local-xor-natural-access/v1"
            and protocol["input_name"] == "hash_4_2"
            and protocol["curve_slug"] == "icv1-f2m83-tm6151469093347-debefd74",
            "protocol identity")
    require(platform.system() == "Darwin" and platform.machine() == "arm64",
            "L0 host identity")
    require(protocol["selected_best_total_runtime"] is None
            and (protocol["base_summands"], protocol["residual_summands"],
                 protocol["trial"], protocol["identity_mask"]) == (4, 2, 0, 0),
            "first-pattern shape")
    require(protocol["conflict_cap"] == 5000000
            and protocol["solver_time_cap_seconds"] == 1200
            and protocol["process_watchdog_seconds"] == 1350,
            "search limit")
    flags = protocol["solver_flags"]
    require(flags == [
        "--threads", "1", "--random", "0",
        "--branchstr", "vmtf+vsids", "--autodisablegauss", "0",
        "--maxconfl", "5000000", "--maxtime", "1200",
        "--maxmatrixcols", "25000", "--maxnummatrices", "10",
        "--verb", "1", "--verbstat", "1", "--printsol", "1",
    ], "solver flags changed")
    require(sha256_file(args.worker) == protocol["worker_binary_sha256"],
            "worker binary digest")
    require(sha256_file(args.solver) == protocol["solver_binary_sha256"],
            "solver binary digest")
    version = subprocess.run([str(args.solver), "--version"], capture_output=True, timeout=10)
    require(version.returncode == 0
            and protocol["solver_version"] in version.stdout.decode(),
            "solver version")
    require(sha256_file(PREVIOUS) == protocol["input_export_receipt_sha256"],
            "input export receipt digest")
    previous = json.loads(PREVIOUS.read_bytes())
    require(previous["curve_slug"] == protocol["curve_slug"]
            and next(row for row in previous["arms"] if row["name"] == "hash_4_2")
            ["xcnf_sha256"] == protocol["xcnf_sha256"],
            "previous export identity")
    manifest = STUDY / "pilot-01/manifest.json"
    corpus = STUDY / "pilot-01/probe-corpus.json"
    require(sha256_file(manifest) == protocol["public_manifest_sha256"]
            and sha256_file(corpus) == protocol["public_corpus_sha256"],
            "public fixture digests")
    formula_dir, base, pool = paths(args, protocol)
    require(sha256_file(formula_dir / "instance.xcnf") == protocol["xcnf_sha256"],
            "XCNF digest")
    require(sha256_file(base) == protocol["base_object_sha256"]
            and sha256_file(pool) == protocol["pool_object_sha256"],
            "S3 archive digests")
    metadata = json.loads((formula_dir / "export.json").read_bytes())
    require(metadata["curve_slug"] == protocol["curve_slug"]
            and metadata["base_object_blake3"] == protocol["base_object_blake3"]
            and metadata["pool_object_blake3"] == protocol["pool_object_blake3"]
            and metadata["base_summands"] == 4
            and metadata["residual_summands"] == 2
            and metadata["identity_mask"] == 0
            and metadata["new_n83_solver_search_executed"] is False,
            "export metadata changed")
    return formula_dir, base, pool


def classify(stdout: bytes, stderr: bytes, exit_code: int, watchdog: bool):
    statuses = re.findall(rb"^s ([A-Z]+)\s*$", stdout, re.MULTILINE)
    counters = re.findall(rb"^c conflicts\s+:\s+(\d+)\s*$", stdout, re.MULTILINE)
    conflicts = int(counters[0]) if len(counters) == 1 else None
    if watchdog:
        return "WATCHDOG", conflicts
    if stderr or len(statuses) != 1 or conflicts is None:
        return "MALFORMED", conflicts
    status = statuses[0].decode()
    if (status, exit_code) in (("INDETERMINATE", 15), ("SATISFIABLE", 10),
                               ("UNSATISFIABLE", 20)):
        return status, conflicts
    return "MALFORMED", conflicts


def run_solver(args, protocol, formula_dir):
    command = [str(args.solver), *protocol["solver_flags"],
               str(formula_dir / "instance.xcnf")]
    start = time.monotonic()
    with (args.out / "solver.stdout").open("xb") as stdout, (args.out / "solver.stderr").open("xb") as stderr:
        process = subprocess.Popen(command, stdout=stdout, stderr=stderr,
                                   start_new_session=True)
        watchdog = False
        try:
            exit_code = process.wait(timeout=protocol["process_watchdog_seconds"])
        except subprocess.TimeoutExpired:
            watchdog = True
            os.killpg(process.pid, signal.SIGKILL)
            exit_code = process.wait()
        elapsed = time.monotonic() - start
        stdout.flush(); os.fsync(stdout.fileno())
        stderr.flush(); os.fsync(stderr.fileno())
    out = (args.out / "solver.stdout").read_bytes()
    err = (args.out / "solver.stderr").read_bytes()
    status, conflicts = classify(out, err, exit_code, watchdog)
    receipt = {
        "schema": "n83.local-xor-natural-access-process/v1",
        "command": command, "status": status, "exit_code": exit_code,
        "conflicts": conflicts, "wall_seconds_L0": elapsed,
        "peak_child_rss_upper_bound_bytes_Darwin": resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss,
        "stdout_sha256": hashlib.sha256(out).hexdigest(),
        "stderr_sha256": hashlib.sha256(err).hexdigest(),
        "stdout_bytes": len(out), "stderr_bytes": len(err),
        "xcnf_sha256": protocol["xcnf_sha256"],
        "solver_binary_sha256": protocol["solver_binary_sha256"],
        "selected_best_total_runtime": None,
    }
    write_new(args.out / "process.json", receipt)
    return receipt


def verify_sat(args, protocol, formula_dir, base, pool):
    manifest = STUDY / "pilot-01/manifest.json"
    corpus = STUDY / "pilot-01/probe-corpus.json"
    output = args.out / "verify"
    command = [str(args.worker), "verify-cross-xcnf",
               protocol["base_object_blake3"], str(manifest), str(base),
               protocol["pool_object_blake3"], str(manifest), str(pool),
               str(corpus), "4", "2", str(formula_dir),
               str(args.out / "solver.stdout"), str(output)]
    start = time.monotonic()
    try:
        result = subprocess.run(command, capture_output=True,
                                timeout=protocol["verification_watchdog_seconds"])
        status = "RETURNED"
        exit_code, stdout, stderr = result.returncode, result.stdout, result.stderr
    except subprocess.TimeoutExpired as error:
        status = "WATCHDOG"
        exit_code, stdout, stderr = None, error.stdout or b"", error.stderr or b""
    (args.out / "verify.stdout").write_bytes(stdout)
    (args.out / "verify.stderr").write_bytes(stderr)
    verdict = json.loads((output / "result.json").read_bytes()) if status == "RETURNED" and exit_code == 0 else None
    receipt = {
        "schema": "n83.local-xor-natural-access-verify/v1",
        "command": command, "status": status, "exit_code": exit_code,
        "wall_seconds_L0": time.monotonic() - start,
        "stdout_sha256": hashlib.sha256(stdout).hexdigest(),
        "stderr_sha256": hashlib.sha256(stderr).hexdigest(),
        "verdict": verdict, "selected_best_total_runtime": None,
    }
    write_new(args.out / "verify-process.json", receipt)
    return receipt


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--formulas-dir", type=Path, required=True)
    parser.add_argument("--archives-dir", type=Path, required=True)
    parser.add_argument("--worker", type=Path, required=True)
    parser.add_argument("--solver", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--preflight-only", action="store_true")
    args = parser.parse_args()
    protocol = json.loads((HERE / "protocol.json").read_bytes())
    formula_dir, base, pool = preflight(args, protocol)
    if args.preflight_only:
        print(json.dumps({"status": "PASS_PREFLIGHT", "input": protocol["input_name"],
                          "selected_best_total_runtime": None}))
        return
    args.out.mkdir()
    process = run_solver(args, protocol, formula_dir)
    verification = verify_sat(args, protocol, formula_dir, base, pool) if process["status"] == "SATISFIABLE" else None
    summary = {
        "schema": "n83.local-xor-natural-access-summary/v1",
        "status": process["status"],
        "conflicts": process["conflicts"],
        "solver_wall_seconds_L0": process["wall_seconds_L0"],
        "verification_status": verification["verdict"]["status"] if verification and verification["verdict"] else None,
        "selected_best_total_runtime": None,
    }
    write_new(args.out / "summary.json", summary)
    print(json.dumps(summary, sort_keys=True))


if __name__ == "__main__":
    main()
