#!/usr/bin/env python3
"""Replay S3 inputs, export pinned N83 patterns, and run bounded XOR searches."""

import argparse
import hashlib
import json
from pathlib import Path
import platform
import subprocess
import sys
import time


HERE = Path(__file__).resolve().parent
INPUT_MANIFEST = HERE.parent / "linux-xor-complete-inputs-20261025/manifest.json"
sys.path.insert(0, str(HERE.parent / "linux-xor-natural-panel-20261025"))
from run_panel import (file_sha, host, require, run_arm, sha,
                       write_new)  # noqa: E402


def frozen_protocol(protocol: dict) -> None:
    require(protocol["schema"] == "n83.natural-pattern-panel-protocol/v1"
            and protocol["selected_best_total_runtime"] is None,
            "frozen pattern protocol")
    require(len(protocol["formulas"]) == 16 and len(protocol["strata"]) == 4,
            "sixteen formulas and four strata")
    expected = {f"{pool}_{small}_{residual}_t{trial}_m{mask}"
                for trial in (1, 2) for mask in (0, 1)
                for pool in ("hash", "gray")
                for small, residual in ((5, 1), (4, 2))}
    require(set(protocol["formulas"]) == expected, "finite formula grid")
    require({row["id"] for row in protocol["strata"]}
            == {"t1_m0", "t1_m1", "t2_m0", "t2_m1"},
            "four distinct frozen strata")
    for name, formula in protocol["formulas"].items():
        require(name == f"{formula['pool']}_{formula['base_summands']}_"
                f"{formula['residual_summands']}_t{formula['trial']}_"
                f"m{formula['identity_mask']}"
                and len(formula["sha256"]) == 64
                and formula["base_summands"] + formula["residual_summands"] == 6,
                f"{name}: formula address")
    for stratum in protocol["strata"]:
        trial, mask = stratum["trial"], stratum["identity_mask"]
        require(stratum["id"] == f"t{trial}_m{mask}"
                and [row["id"] for row in stratum["arms"]]
                == ["hash51_A1", "hash51_A2", "gray51", "hash42",
                    "gray42", "hash51_A3"]
                and [row["formula"] for row in stratum["arms"]]
                == [f"hash_5_1_t{trial}_m{mask}",
                    f"hash_5_1_t{trial}_m{mask}",
                    f"gray_5_1_t{trial}_m{mask}",
                    f"hash_4_2_t{trial}_m{mask}",
                    f"gray_4_2_t{trial}_m{mask}",
                    f"hash_5_1_t{trial}_m{mask}"],
                f"{stratum['id']}: frozen six-arm order")
    require(protocol["solver_flags"] == [
        "--threads", "1", "--random", "0", "--branchstr", "vmtf+vsids",
        "--autodisablegauss", "0", "--maxmatrixcols", "25000",
        "--maxnummatrices", "10", "--verb", "1", "--verbstat", "1",
        "--printsol", "1", "--maxconfl", "1000000", "--maxtime", "300",
    ] and protocol["conflict_cap"] == 1_000_000
            and protocol["solver_time_cap_seconds"] == 300
            and protocol["process_watchdog_seconds"] == 330
            and protocol["memory_address_space_limit_bytes"] == 8 * 1024**3,
            "frozen solver bounds")


def admission(protocol: dict, stratum: dict, inputs: Path, solver: Path,
              worker: Path, source: Path, build_receipt: Path) -> dict:
    frozen_protocol(protocol)
    require(platform.system() == "Linux"
            and platform.machine() in ("aarch64", "arm64"), "Linux ARM host")
    require(file_sha(INPUT_MANIFEST) == protocol["input_manifest_sha256"],
            "eleven-object manifest digest")
    manifest = json.loads(INPUT_MANIFEST.read_bytes())
    require(manifest["curve"] == protocol["curve"]
            and len(manifest["entries"]) == 11, "primary N83 input identity")
    for row in manifest["entries"]:
        relative = row["key"].split("/inputs/", 1)[1]
        path = inputs / relative
        require(path.is_file() and path.stat().st_size == row["bytes"]
                and file_sha(path) == row["sha256"],
                f"{relative}: fresh S3 input identity")
    require(stratum in protocol["strata"], "selected frozen stratum")
    source_commit = subprocess.run(
        ["git", "-C", str(source), "rev-parse", "HEAD"],
        capture_output=True, text=True, timeout=30, check=True).stdout.strip()
    build = json.loads(build_receipt.read_bytes())
    require(source_commit == protocol["worker_source_commit"]
            and build["source_commit"] == source_commit
            and build["worker_sha256"] == file_sha(worker),
            "source-pinned worker binary")
    require(file_sha(solver) == protocol["solver_sha256"], "pinned solver bytes")
    version = subprocess.run([str(solver), "--version"], capture_output=True,
                             timeout=30, check=True)
    require(b"CryptoMiniSat version 5.11.15" in version.stdout,
            "pinned solver version")
    package = subprocess.run(
        ["dpkg-query", "-W", "-f=${Package}=${Version}\n", "cryptominisat"],
        capture_output=True, text=True, timeout=30, check=True)
    require(package.stdout.strip() == protocol["solver_package"],
            "pinned package version")
    return {"schema": "n83.natural-pattern-admission/v1",
            "status": "PASS_EXACT_INPUTS_BINARIES_AND_STRATUM",
            "stratum": stratum["id"], "source_commit": source_commit,
            "worker_sha256": build["worker_sha256"],
            "solver_sha256": protocol["solver_sha256"],
            "input_manifest_sha256": protocol["input_manifest_sha256"],
            "selected_best_total_runtime": None}


def check_export_metadata(metadata: dict, spec: dict, protocol: dict,
                          xcnf: Path) -> None:
    require(metadata["schema"] == "n83.cross-stream-pattern-xcnf/v2"
            and metadata["study"] == "koblitz_n83_factor_base_sweep_20261008"
            and metadata["curve_slug"] == protocol["curve"]
            and metadata["base_object_blake3"] == protocol["base_object_blake3"]
            and metadata["pool_object_blake3"]
            == protocol["pool_objects_blake3"][spec["pool"]]
            and metadata["trial"] == spec["trial"]
            and metadata["source_id"] == spec["trial"] + 1
            and metadata["identity_mask"] == spec["identity_mask"]
            and metadata["order_first_pair"] is True
            and metadata["base_summands"] == spec["base_summands"]
            and metadata["residual_summands"] == spec["residual_summands"]
            and metadata["coef_a"] == spec["coef_a"]
            and metadata["coef_b"] == spec["coef_b"]
            and metadata["variables"] == spec["variables"]
            and metadata["original_clauses"] == spec["original_clauses"]
            and metadata["xor_rows"] == spec["xor_rows"]
            and metadata["xcnf_bytes"] == spec["bytes"] == xcnf.stat().st_size
            and file_sha(xcnf) == spec["sha256"]
            and metadata["selected_best_total_runtime"] is None,
            "exact fresh-trial XCNF and export metadata")


def worker_args(protocol: dict, spec: dict, inputs: Path, source: Path) -> list[str]:
    study = source / "research/koblitz_n83_factor_base_sweep_20261008"
    base = protocol["base_object_blake3"]
    pool = protocol["pool_objects_blake3"][spec["pool"]]
    manifest = str(study / "pilot-01/manifest.json")
    return [base, manifest, str(inputs / "archives" / f"s3-{base}.jsonl.gz"),
            pool, manifest, str(inputs / "archives" / f"s3-{pool}.jsonl.gz"),
            str(study / "pilot-01/probe-corpus.json"),
            str(spec["base_summands"]), str(spec["residual_summands"]),
            str(spec["trial"]), str(spec["identity_mask"])]


def export_formula(protocol: dict, name: str, inputs: Path, worker: Path,
                   source: Path, out: Path) -> dict:
    spec = protocol["formulas"][name]
    export = out / "exports" / name
    command = [str(worker), "export-cross-xcnf-pattern",
               *worker_args(protocol, spec, inputs, source), str(export)]
    begin = time.monotonic()
    result = subprocess.run(command, capture_output=True, timeout=180)
    export.mkdir(exist_ok=True, parents=True)
    (export / "export.stdout").write_bytes(result.stdout)
    (export / "export.stderr").write_bytes(result.stderr)
    require(result.returncode == 0, f"{name}: worker export failed")
    metadata = json.loads((export / "export.json").read_bytes())
    xcnf = export / "instance.xcnf"
    check_export_metadata(metadata, spec, protocol, xcnf)
    receipt = {"schema": "n83.natural-pattern-export/v1", "name": name,
               "command": command, "worker_sha256": file_sha(worker),
               "wall_seconds_L0": time.monotonic() - begin,
               "stdout_sha256": sha(result.stdout),
               "stderr_sha256": sha(result.stderr),
               "xcnf_sha256": spec["sha256"],
               "metadata_sha256": file_sha(export / "export.json"),
               "selected_best_total_runtime": None}
    write_new(export / "export-process.json", receipt)
    return receipt


def verify_sat(protocol: dict, spec: dict, inputs: Path, worker: Path,
               source: Path, directory: Path, export: Path) -> dict:
    command = [str(worker), "verify-cross-xcnf-pattern",
               *worker_args(protocol, spec, inputs, source), str(export),
               str(directory / "solver.stdout"), str(directory / "verdict")]
    begin = time.monotonic()
    result = subprocess.run(command, capture_output=True, timeout=180)
    (directory / "verify.stdout").write_bytes(result.stdout)
    (directory / "verify.stderr").write_bytes(result.stderr)
    verdict = directory / "verdict/result.json"
    data = json.loads(verdict.read_bytes()) if result.returncode == 0 and verdict.exists() else None
    status = data["status"] if data else "REJECTED_OR_INCOMPLETE_MODEL"
    require(status in ("PASS_ONE_EXTERNAL_PARTIAL", "GROUP_REJECTED_EXTERNAL_MODEL",
                       "REJECTED_OR_INCOMPLETE_MODEL"), "verifier status")
    if data:
        require(data["schema"] == "n83.cross-stream-pattern-external-model/v2"
                and data["trial"] == spec["trial"]
                and data["identity_mask"] == spec["identity_mask"]
                and data["selected_best_total_runtime"] is None,
                "exact verified model identity")
        if status == "PASS_ONE_EXTERNAL_PARTIAL":
            candidate = data["candidate"]
            require(candidate["source_id"] == spec["trial"] + 1
                    and candidate["a"] == spec["coef_a"]
                    and candidate["b"] == spec["coef_b"],
                    "exact candidate trial and coefficients")
    receipt = {"schema": "n83.natural-pattern-model-verification/v1",
               "status": status, "exit_code": result.returncode,
               "wall_seconds_L0": time.monotonic() - begin,
               "stdout_sha256": sha(result.stdout),
               "stderr_sha256": sha(result.stderr),
               "result": data, "selected_best_total_runtime": None}
    write_new(directory / "verify-process.json", receipt)
    return receipt


def run(protocol: dict, stratum: dict, inputs: Path, solver: Path,
        worker: Path, source: Path, build_receipt: Path, out: Path) -> dict:
    out.mkdir()
    write_new(out / "host.json", dict(host(), schema="n83.natural-pattern-host/v1"))
    checked = admission(protocol, stratum, inputs, solver, worker,
                        source, build_receipt)
    write_new(out / "preflight.json", checked)
    (out / "exports").mkdir()
    formulas = out / "formulas"
    formulas.mkdir()
    exported = []
    for name in dict.fromkeys(arm["formula"] for arm in stratum["arms"]):
        exported.append(export_formula(protocol, name, inputs, worker, source, out))
        (formulas / f"{name}.xcnf").symlink_to(
            (out / "exports" / name / "instance.xcnf").resolve())
    solver_protocol = {"solver_flags": protocol["solver_flags"],
                       "memory_address_space_limit_bytes":
                       protocol["memory_address_space_limit_bytes"],
                       "process_watchdog_seconds": protocol["process_watchdog_seconds"],
                       "solver_sha256": protocol["solver_sha256"],
                       "inputs": {name: protocol["formulas"][name]["sha256"]
                                  for name in protocol["formulas"]}}
    completed = []
    model_hashes = set()
    stop_reason = None
    for arm in stratum["arms"]:
        require(file_sha(formulas / f"{arm['formula']}.xcnf")
                == protocol["formulas"][arm["formula"]]["sha256"],
                "formula identity immediately before search")
        exact_arm = dict(arm, input=arm["formula"], trial=stratum["trial"],
                         identity_mask=stratum["identity_mask"])
        receipt = run_arm(solver_protocol, exact_arm, formulas, solver, out)
        completed.append(receipt)
        progress = {"arm": arm["id"], "status": receipt["status"],
                    "conflicts": receipt["conflicts"],
                    "wall_seconds_L0": receipt["wall_seconds_L0"]}
        if receipt["status"] == "SATISFIABLE":
            verified = verify_sat(protocol, protocol["formulas"][arm["formula"]],
                                  inputs, worker, source, out / arm["id"],
                                  out / "exports" / arm["formula"])
            progress["verification"] = verified["status"]
            if verified["status"] == "PASS_ONE_EXTERNAL_PARTIAL":
                candidate = verified["result"]["candidate"]
                model_hashes.add(hashlib.sha256(json.dumps(
                    candidate, sort_keys=True, separators=(",", ":")
                ).encode()).hexdigest())
            else:
                stop_reason = verified["status"]
        elif receipt["status"] not in ("INDETERMINATE", "UNSATISFIABLE"):
            stop_reason = receipt["status"]
        print(json.dumps(progress, sort_keys=True), flush=True)
        if stop_reason:
            break
    summary = {"schema": "n83.natural-pattern-panel/v1",
               "status": "COMPLETE_BOUNDED" if stop_reason is None else "STOPPED",
               "stratum": stratum["id"], "stop_reason": stop_reason,
               "completed_arms": [row["arm"]["id"] for row in completed],
               "unique_group_checked_candidate_hashes": sorted(model_hashes),
               "summed_export_wall_seconds_L0": sum(row["wall_seconds_L0"] for row in exported),
               "summed_solver_wall_seconds_L0": sum(row["wall_seconds_L0"] for row in completed),
               "selected_best_total_runtime": None}
    write_new(out / "summary.json", summary)
    print(json.dumps(summary, sort_keys=True), flush=True)
    return summary


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--protocol", type=Path, default=HERE / "protocol.json")
    parser.add_argument("--stratum", required=True)
    parser.add_argument("--inputs", type=Path, required=True)
    parser.add_argument("--solver", type=Path, required=True)
    parser.add_argument("--worker", type=Path, required=True)
    parser.add_argument("--source", type=Path, required=True)
    parser.add_argument("--build-receipt", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    protocol = json.loads(args.protocol.read_bytes())
    frozen_protocol(protocol)
    matches = [row for row in protocol["strata"] if row["id"] == args.stratum]
    require(len(matches) == 1, "one frozen stratum")
    run(protocol, matches[0], args.inputs, args.solver, args.worker,
        args.source, args.build_receipt, args.out)
