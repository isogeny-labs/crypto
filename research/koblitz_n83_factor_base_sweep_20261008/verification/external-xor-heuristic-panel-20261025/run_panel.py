#!/usr/bin/env python3
"""Run a pinned, resumable N83 CryptoMiniSat heuristic panel."""

import argparse
import hashlib
import json
import os
import pathlib
import platform
import re
import resource
import signal
import subprocess
import sys
import time


HERE = pathlib.Path(__file__).resolve().parent
STUDY = HERE.parent.parent


def require(ok: bool, message: str) -> None:
    if not ok:
        raise ValueError(message)


def sha256_file(path: pathlib.Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def write_json_new(path: pathlib.Path, value: dict) -> None:
    with path.open("x", encoding="utf-8") as stream:
        json.dump(value, stream, indent=2, sort_keys=True)
        stream.write("\n")
        stream.flush()
        os.fsync(stream.fileno())


def input_paths(args: argparse.Namespace, protocol: dict, input_name: str) -> tuple:
    spec = protocol["inputs"][input_name]
    formula = args.formulas_dir / input_name / "instance.xcnf"
    export = args.formulas_dir / input_name / "export.json"
    base_hash = protocol["base_object_blake3"]
    base = args.archives_dir / f"s3-{base_hash}.jsonl.gz"
    pool = args.archives_dir / f"s3-{spec['pool_object_blake3']}.jsonl.gz"
    return spec, formula, export, base, pool


def preflight(args: argparse.Namespace, protocol: dict) -> None:
    require(protocol["schema"] == "n83.external-xor-heuristic-panel-protocol/v1", "protocol schema")
    require(platform.system() == "Darwin" and platform.machine() == "arm64", "local host identity")
    require(sha256_file(args.worker) == protocol["worker_binary_sha256"], "worker binary hash")
    require(sha256_file(args.solver) == protocol["solver_binary_sha256"], "solver binary hash")
    version = subprocess.run([str(args.solver), "--version"], capture_output=True, timeout=10)
    require(version.returncode == 0 and protocol["solver_version"] in version.stdout.decode(), "solver version")
    require(len(protocol["arms"]) == 10, "frozen arm count")
    require(len({arm["id"] for arm in protocol["arms"]}) == 10, "duplicate arm id")
    require(protocol["common_solver_flags"] == [
        "--verb", "1", "--verbstat", "1", "--threads", "1", "--random", "0",
        "--maxtime", "120", "--maxconfl", "1000000", "--printsol", "1",
    ], "common solver flags changed")
    for name, spec in protocol["inputs"].items():
        _, formula, export, base, pool = input_paths(args, protocol, name)
        require(sha256_file(formula) == spec["xcnf_sha256"], f"{name}: XCNF hash")
        require(sha256_file(base) == protocol["base_object_sha256"], "base archive hash")
        require(sha256_file(pool) == spec["pool_object_sha256"], f"{name}: pool archive hash")
        metadata = json.loads(export.read_bytes())
        require(metadata["xcnf_blake3"] and metadata["base_summands"] == 5
                and metadata["residual_summands"] == 1, f"{name}: export shape")
        require(metadata["base_object_blake3"] == protocol["base_object_blake3"]
                and metadata["pool_object_blake3"] == spec["pool_object_blake3"],
                f"{name}: export object identities")
    for arm in protocol["arms"]:
        require(arm["input"] in protocol["inputs"]
                and arm["branchstr"] in ("vmtf+vsids", "vsids")
                and arm["autodisablegauss"] in (0, 1), f"{arm['id']}: option domain")


def solver_command(args: argparse.Namespace, protocol: dict, arm: dict) -> list[str]:
    _, formula, _, _, _ = input_paths(args, protocol, arm["input"])
    return [str(args.solver), *protocol["common_solver_flags"],
            "--branchstr", arm["branchstr"],
            "--autodisablegauss", str(arm["autodisablegauss"]), str(formula)]


def classify(stdout: bytes, stderr: bytes, exit_code: int, watchdog: bool) -> tuple[str, int | None]:
    statuses = re.findall(rb"^s ([A-Z]+)\s*$", stdout, re.MULTILINE)
    conflicts = re.findall(rb"^c conflicts\s+:\s+(\d+)\s*$", stdout, re.MULTILINE)
    count = int(conflicts[0]) if len(conflicts) == 1 else None
    if watchdog:
        return "WATCHDOG", count
    if stderr or len(statuses) != 1 or count is None:
        return "MALFORMED", count
    status = statuses[0].decode()
    if (status, exit_code) in (("INDETERMINATE", 15), ("SATISFIABLE", 10),
                               ("UNSATISFIABLE", 20)):
        return status, count
    return "MALFORMED", count


def run_one(args: argparse.Namespace, protocol: dict, arm: dict) -> dict:
    directory = args.out / arm["id"]
    directory.mkdir()
    command = solver_command(args, protocol, arm)
    start = time.monotonic()
    with (directory / "solver.stdout").open("xb") as stdout, (directory / "solver.stderr").open("xb") as stderr:
        process = subprocess.Popen(command, stdout=stdout, stderr=stderr, start_new_session=True)
        timed_out = False
        try:
            exit_code = process.wait(timeout=protocol["process_watchdog_seconds"])
        except subprocess.TimeoutExpired:
            timed_out = True
            os.killpg(process.pid, signal.SIGKILL)
            exit_code = process.wait()
        elapsed = time.monotonic() - start
        stdout.flush(); os.fsync(stdout.fileno())
        stderr.flush(); os.fsync(stderr.fileno())
    stdout_bytes = (directory / "solver.stdout").read_bytes()
    stderr_bytes = (directory / "solver.stderr").read_bytes()
    status, conflicts = classify(stdout_bytes, stderr_bytes, exit_code, timed_out)
    receipt = {
        "schema": "n83.external-xor-heuristic-arm/v1",
        "arm": arm, "command": command, "status": status,
        "exit_code": exit_code, "wall_seconds_L0": elapsed,
        "peak_child_rss_upper_bound_bytes_Darwin": resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss,
        "conflicts": conflicts,
        "stdout_sha256": hashlib.sha256(stdout_bytes).hexdigest(),
        "stderr_sha256": hashlib.sha256(stderr_bytes).hexdigest(),
        "stdout_bytes": len(stdout_bytes), "stderr_bytes": len(stderr_bytes),
        "xcnf_sha256": protocol["inputs"][arm["input"]]["xcnf_sha256"],
        "solver_binary_sha256": protocol["solver_binary_sha256"],
        "selected_best_total_runtime": None,
    }
    write_json_new(directory / "process.json", receipt)
    return receipt


def replay_completed(args: argparse.Namespace, protocol: dict, arm: dict) -> dict:
    directory = args.out / arm["id"]
    receipt = json.loads((directory / "process.json").read_bytes())
    require(receipt["schema"] == "n83.external-xor-heuristic-arm/v1"
            and receipt["arm"] == arm and receipt["command"] == solver_command(args, protocol, arm),
            f"{arm['id']}: completed command differs")
    require(receipt["xcnf_sha256"] == protocol["inputs"][arm["input"]]["xcnf_sha256"]
            and receipt["solver_binary_sha256"] == protocol["solver_binary_sha256"],
            f"{arm['id']}: input or solver changed")
    stdout = (directory / "solver.stdout").read_bytes()
    stderr = (directory / "solver.stderr").read_bytes()
    require(receipt["stdout_sha256"] == hashlib.sha256(stdout).hexdigest()
            and receipt["stderr_sha256"] == hashlib.sha256(stderr).hexdigest(),
            f"{arm['id']}: raw output changed")
    require((receipt["status"], receipt["conflicts"])
            == classify(stdout, stderr, receipt["exit_code"], receipt["status"] == "WATCHDOG"),
            f"{arm['id']}: output interpretation changed")
    return receipt


def verify_sat(args: argparse.Namespace, protocol: dict, arm: dict) -> dict:
    spec, _, export, base, pool = input_paths(args, protocol, arm["input"])
    directory = args.out / arm["id"]
    out = directory / "verify"
    manifest = STUDY / "pilot-01/manifest.json"
    corpus = STUDY / "pilot-01/probe-corpus.json"
    command = [str(args.worker), "verify-cross-xcnf", protocol["base_object_blake3"],
               str(manifest), str(base), spec["pool_object_blake3"], str(manifest),
               str(pool), str(corpus), "5", "1", str(export.parent),
               str(directory / "solver.stdout"), str(out)]
    start = time.monotonic()
    process = subprocess.run(command, capture_output=True, timeout=120)
    (directory / "verify.stdout").write_bytes(process.stdout)
    (directory / "verify.stderr").write_bytes(process.stderr)
    result = {"command": command, "exit_code": process.returncode,
              "wall_seconds_L0": time.monotonic() - start,
              "stdout_sha256": hashlib.sha256(process.stdout).hexdigest(),
              "stderr_sha256": hashlib.sha256(process.stderr).hexdigest(),
              "result": json.loads((out / "result.json").read_bytes()) if process.returncode == 0 else None}
    write_json_new(directory / "verify-process.json", result)
    return result


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--protocol", type=pathlib.Path, default=HERE / "protocol.json")
    parser.add_argument("--formulas-dir", type=pathlib.Path, required=True)
    parser.add_argument("--archives-dir", type=pathlib.Path, required=True)
    parser.add_argument("--worker", type=pathlib.Path, required=True)
    parser.add_argument("--solver", type=pathlib.Path, required=True)
    parser.add_argument("--out", type=pathlib.Path, required=True)
    parser.add_argument("--resume", action="store_true")
    parser.add_argument("--preflight-only", action="store_true")
    parser.add_argument("--one", help=argparse.SUPPRESS)
    args = parser.parse_args()
    protocol = json.loads(args.protocol.read_bytes())
    preflight(args, protocol)
    if args.preflight_only:
        print(json.dumps({"status": "PASS_PREFLIGHT", "arms": len(protocol["arms"])}))
        return
    if args.one:
        arm = next(arm for arm in protocol["arms"] if arm["id"] == args.one)
        print(json.dumps(run_one(args, protocol, arm), sort_keys=True))
        return
    if args.resume:
        require(args.out.is_dir(), "resume output directory missing")
    else:
        args.out.mkdir()
    if (args.out / "summary.json").exists():
        require(args.resume, "completed output directory already exists")
        summary = json.loads((args.out / "summary.json").read_bytes())
        require(summary["schema"] == "n83.external-xor-heuristic-panel/v1", "summary schema")
        require(summary["completed_arms"]
                == [arm["id"] for arm in protocol["arms"][:len(summary["completed_arms"])]],
                "summary completed-arm prefix")
        for arm in protocol["arms"]:
            if arm["id"] in summary["completed_arms"]:
                replay_completed(args, protocol, arm)
        print(json.dumps(summary, sort_keys=True))
        return
    completed = []
    stop = None
    for arm in protocol["arms"]:
        directory = args.out / arm["id"]
        if directory.exists():
            require(args.resume and (directory / "process.json").is_file(),
                    f"{arm['id']}: partial or unexpected existing arm")
            receipt = replay_completed(args, protocol, arm)
        else:
            command = [sys.executable, str(pathlib.Path(__file__).resolve()),
                       "--protocol", str(args.protocol), "--formulas-dir", str(args.formulas_dir),
                       "--archives-dir", str(args.archives_dir), "--worker", str(args.worker),
                       "--solver", str(args.solver), "--out", str(args.out), "--one", arm["id"]]
            child = subprocess.run(command, capture_output=True)
            require(child.returncode == 0, f"{arm['id']}: runner failed: {child.stderr[-500:]!r}")
            receipt = replay_completed(args, protocol, arm)
        completed.append(receipt)
        print(json.dumps({"arm": arm["id"], "status": receipt["status"],
                          "conflicts": receipt["conflicts"],
                          "wall_seconds_L0": receipt["wall_seconds_L0"]}), flush=True)
        if receipt["status"] == "SATISFIABLE":
            if not (directory / "verify-process.json").exists():
                verify_sat(args, protocol, arm)
            stop = "SAT_MODEL_REQUIRES_NEXT_GATE"
            break
        if receipt["status"] not in ("INDETERMINATE", "UNSATISFIABLE"):
            stop = receipt["status"]
            break
    result = {"schema": "n83.external-xor-heuristic-panel/v1",
              "status": "COMPLETE_BOUNDED" if stop is None else "STOPPED",
              "stop_reason": stop, "completed_arms": [row["arm"]["id"] for row in completed],
              "summed_solver_wall_seconds_L0": sum(row["wall_seconds_L0"] for row in completed),
              "selected_best_total_runtime": None}
    write_json_new(args.out / "summary.json", result)
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()
