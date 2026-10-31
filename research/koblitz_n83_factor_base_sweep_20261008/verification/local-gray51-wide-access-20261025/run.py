#!/usr/bin/env python3
"""One frozen primary-N83 Gray 5+1 natural-query wide-XOR access call."""

import argparse
import hashlib
import importlib.util
import json
import platform
from pathlib import Path
import subprocess
import time


HERE = Path(__file__).resolve().parent
STUDY = HERE.parent.parent
BUNDLE = HERE.parent / "linux-xor-complete-inputs-20261025"
CORE = HERE.parent / "local-yield-frontier-20261025/run_access.py"
SOURCE = HERE.parent / "external-xor-20261025/receipt.json"
BASE = "3888e2597fd3519807c9d722f49f83329c93ce5cb0929501cf7cb7cbc3cab1a0"
POOL = "966a0493c96674f0af6d184615ed05309d08fba5c7342957b4186cd50366dc47"


def module(name: str, path: Path):
    spec = importlib.util.spec_from_file_location(name, path)
    loaded = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(loaded)
    return loaded


core = module("n83_gray51_core", CORE)
bundle_audit = module("n83_gray51_bundle_audit", BUNDLE / "audit.py")


def require(condition: bool, reason: str) -> None:
    if not condition:
        raise ValueError(reason)


def sha256(path: Path) -> str:
    return core.sha256_file(path)


def preflight(args, protocol: dict) -> None:
    require(all(path.is_absolute() for path in
                (args.inputs_dir, args.worker, args.solver, args.out)),
            "absolute input, executable and output paths")
    require(protocol["schema"] == "n83.local-gray51-wide-natural-access/v1"
            and protocol["curve_slug"] == "icv1-f2m83-tm6151469093347-debefd74"
            and protocol["input_name"] == "gray_5_1"
            and protocol["trial"] == protocol["identity_mask"] == 0
            and [protocol["base_summands"], protocol["residual_summands"]] == [5, 1]
            and protocol["selected_best_total_runtime"] is None,
            "frozen primary first-pattern identity")
    require(platform.system() == "Darwin" and platform.machine() == "arm64",
            "local host identity")
    require(sha256(Path(__file__)) == protocol["runner_sha256"]
            and sha256(CORE) == protocol["core_runner_sha256"]
            and sha256(SOURCE) == protocol["source_receipt_sha256"]
            and sha256(BUNDLE / "manifest.json") == protocol["input_manifest_sha256"]
            and sha256(BUNDLE / "receipt.json") == protocol["input_receipt_sha256"],
            "pinned source and S3 custody bytes")
    require(sha256(args.worker) == protocol["worker_binary_sha256"]
            and sha256(args.solver) == protocol["solver_binary_sha256"],
            "pinned executable bytes")
    version = subprocess.run([str(args.solver), "--version"],
                             capture_output=True, timeout=10)
    require(version.returncode == 0
            and protocol["solver_version"].encode() in version.stdout,
            "solver version")
    require(sha256(STUDY / "pilot-01/manifest.json")
            == protocol["public_manifest_sha256"]
            and sha256(STUDY / "pilot-01/probe-corpus.json")
            == protocol["public_corpus_sha256"],
            "public fixture bytes")
    require(bundle_audit.audit(args.inputs_dir)["status"]
            == "PASS_COMPLETE_INPUT_REPLAY", "S3 inputs")
    require(sha256(args.inputs_dir / "gray_5_1.xcnf")
            == protocol["xcnf_sha256"], "exact Gray 5+1 XCNF")
    require(protocol["solver_flags"] == [
        "--threads", "1", "--random", "0",
        "--branchstr", "vmtf+vsids", "--autodisablegauss", "0",
        "--maxconfl", "5000000", "--maxtime", "1500",
        "--maxmatrixcols", "25000", "--maxnummatrices", "10",
        "--verb", "1", "--verbstat", "1", "--printsol", "1",
    ] and protocol["conflict_cap"] == 5000000
        and protocol["solver_time_cap_seconds"] == 1500
        and protocol["process_watchdog_seconds"] == 1650
        and protocol["verification_watchdog_seconds"] == 300,
        "frozen solver settings and caps")


def verify(args, protocol, formula: Path) -> dict:
    manifest = STUDY / "pilot-01/manifest.json"
    corpus = STUDY / "pilot-01/probe-corpus.json"
    archives = args.inputs_dir / "archives"
    command = [
        str(args.worker), "verify-cross-xcnf",
        BASE, str(manifest), str(archives / f"s3-{BASE}.jsonl.gz"),
        POOL, str(manifest), str(archives / f"s3-{POOL}.jsonl.gz"),
        str(corpus), "5", "1", str(formula),
        str(args.out / "solver.stdout"), str(args.out / "verify"),
    ]
    started = time.monotonic()
    try:
        result = subprocess.run(command, capture_output=True,
                                timeout=protocol["verification_watchdog_seconds"])
        status, exit_code = "RETURNED", result.returncode
        stdout, stderr = result.stdout, result.stderr
    except subprocess.TimeoutExpired as error:
        status, exit_code = "WATCHDOG", None
        stdout, stderr = error.stdout or b"", error.stderr or b""
    (args.out / "verify.stdout").write_bytes(stdout)
    (args.out / "verify.stderr").write_bytes(stderr)
    verdict_path = args.out / "verify/result.json"
    verdict = json.loads(verdict_path.read_bytes()) if status == "RETURNED" and exit_code == 0 and verdict_path.exists() else None
    receipt = {
        "schema": "n83.local-gray51-wide-verify/v1",
        "status": status, "exit_code": exit_code,
        "wall_seconds_L0": time.monotonic() - started,
        "stdout_sha256": hashlib.sha256(stdout).hexdigest(),
        "stderr_sha256": hashlib.sha256(stderr).hexdigest(),
        "verdict": verdict, "selected_best_total_runtime": None,
    }
    core.write_new(args.out / "verify-process.json", receipt)
    return receipt


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--inputs-dir", type=Path, required=True)
    parser.add_argument("--worker", type=Path, required=True)
    parser.add_argument("--solver", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--preflight-only", action="store_true")
    args = parser.parse_args()
    protocol_path = HERE / "protocol.json"
    protocol = json.loads(protocol_path.read_bytes())
    preflight(args, protocol)
    if args.preflight_only:
        print(json.dumps({"status": "PASS_PREFLIGHT", "input": "gray_5_1",
                          "selected_best_total_runtime": None}))
        return
    args.out.mkdir()
    formula = args.out / "formula"
    formula.mkdir()
    (formula / "instance.xcnf").symlink_to(args.inputs_dir / "gray_5_1.xcnf")
    (formula / "export.json").symlink_to(
        args.inputs_dir / "exports/gray_5_1/export.json")
    (args.out / "host-before.txt").write_bytes(
        subprocess.check_output(["uptime"], timeout=10))
    process = core.run_solver(args, protocol, formula)
    verification = verify(args, protocol, formula) if process["status"] == "SATISFIABLE" else None
    (args.out / "host-after.txt").write_bytes(
        subprocess.check_output(["uptime"], timeout=10))
    verdict = verification["verdict"] if verification else None
    summary = {
        "schema": "n83.local-gray51-wide-summary/v1",
        "protocol_sha256": sha256(protocol_path),
        "status": process["status"], "conflicts": process["conflicts"],
        "solver_wall_seconds_L0": process["wall_seconds_L0"],
        "verification_status": verdict.get("status") if verdict else None,
        "selected_best_total_runtime": None,
    }
    core.write_new(args.out / "summary.json", summary)
    print(json.dumps(summary, sort_keys=True))


if __name__ == "__main__":
    main()
