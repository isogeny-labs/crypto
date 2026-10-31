#!/usr/bin/env python3
"""Run a bounded paired 2x2 CryptoMiniSat screen on one exact N83 formula."""

import argparse
import json
from pathlib import Path
import platform
import subprocess
import sys


HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent / "linux-xor-natural-panel-20261025"))
from run_panel import file_sha, host, require, run_arm, write_new  # noqa: E402


ORDER = ["base", "vsids", "matrix48", "vsids_matrix48", "base_repeat"]


def frozen(protocol: dict) -> None:
    expected = {f"{pool}_{a}_{b}_t{trial}_m0"
                for trial in (1, 2) for pool in ("hash", "gray")
                for a, b in ((5, 1), (4, 2))}
    require(protocol["schema"] == "n83.solver-setting-screen-protocol/v1"
            and protocol["selected_best_total_runtime"] is None
            and len(protocol["strata"]) == 8
            and {row["id"] for row in protocol["strata"]} == expected
            and set(protocol["formulas"]) == expected,
            "eight exact frozen mask-zero formula strata")
    configs = protocol["configurations"]
    require(set(configs) == set(ORDER)
            and configs["base"] == configs["base_repeat"]
            == {"branchstr": "vmtf+vsids", "maxnummatrices": 10}
            and configs["vsids"]
            == {"branchstr": "vsids", "maxnummatrices": 10}
            and configs["matrix48"]
            == {"branchstr": "vmtf+vsids", "maxnummatrices": 48}
            and configs["vsids_matrix48"]
            == {"branchstr": "vsids", "maxnummatrices": 48},
            "paired branch-and-matrix 2x2 plus baseline repeat")
    require(protocol["solver_time_cap_seconds"] == 600
            and protocol["conflict_cap"] == 2_000_000
            and protocol["process_watchdog_seconds"] == 630
            and protocol["memory_address_space_limit_bytes"] == 8 * 1024**3,
            "frozen solver limits")
    for row in protocol["strata"]:
        require(row["id"] == row["formula"]
                and row["config_order"] == ORDER
                and row["trial"] in (1, 2)
                and row["pool"] in ("hash", "gray")
                and (row["base_summands"], row["residual_summands"])
                in ((5, 1), (4, 2))
                and protocol["formulas"][row["formula"]]["sha256"]
                and len(row["source_artifact_sha256"]) == 64,
                f"{row['id']}: frozen source address")


def flags(protocol: dict, setting: dict) -> list[str]:
    return ["--threads", "1", "--random", "0",
            "--branchstr", setting["branchstr"],
            "--autodisablegauss", "0", "--maxmatrixcols", "25000",
            "--maxnummatrices", str(setting["maxnummatrices"]),
            "--verb", "1", "--verbstat", "1", "--printsol", "1",
            "--maxconfl", str(protocol["conflict_cap"]),
            "--maxtime", str(protocol["solver_time_cap_seconds"])]


def run(protocol: dict, row: dict, inputs: Path, solver: Path,
        out: Path) -> dict:
    frozen(protocol)
    require(platform.system() == "Linux"
            and platform.machine() in ("aarch64", "arm64"),
            "qualified Linux ARM host")
    name = row["formula"]
    spec = protocol["formulas"][name]
    xcnf = inputs / f"{name}.xcnf"
    provenance = json.loads((inputs / "input-provenance.json").read_bytes())
    require(xcnf.stat().st_size == spec["bytes"]
            and file_sha(xcnf) == spec["sha256"]
            and provenance["status"] == "PASS_EXACT_PRIOR_S3_BACKED_XCNF"
            and provenance["stratum"] == row["id"]
            and provenance["source_artifact_id"] == row["source_artifact_id"]
            and provenance["source_artifact_zip_sha256"]
            == row["source_artifact_sha256"]
            and provenance["xcnf_sha256"] == spec["sha256"]
            and file_sha(solver) == protocol["solver_sha256"],
            "exact source-backed formula and solver")
    version = subprocess.run([str(solver), "--version"], capture_output=True,
                             timeout=30, check=True)
    require(b"CryptoMiniSat version 5.11.15" in version.stdout,
            "source-qualified solver version")
    package = subprocess.run(
        ["dpkg-query", "-W", "-f=${Package}=${Version}\n", "cryptominisat"],
        capture_output=True, text=True, timeout=30, check=True)
    require(package.stdout.strip() == protocol["solver_package"],
            "exact solver package")
    out.mkdir()
    write_new(out / "host.json", dict(host(), schema="n83.solver-setting-host/v1"))
    write_new(out / "preflight.json", {
        "schema": "n83.solver-setting-preflight/v1",
        "status": "PASS_EXACT_INPUT_AND_SOLVER", "stratum": row["id"],
        "protocol_sha256": file_sha(HERE / "protocol.json"),
        "xcnf_sha256": spec["sha256"],
        "solver_sha256": protocol["solver_sha256"],
        "source_artifact_id": row["source_artifact_id"],
        "selected_best_total_runtime": None})
    completed = []
    stop_reason = None
    candidates = []
    for config_id in ORDER:
        setting = protocol["configurations"][config_id]
        arm = {"id": config_id, "input": name,
               "configuration": setting, "stratum": row["id"]}
        solver_protocol = {
            "solver_flags": flags(protocol, setting),
            "memory_address_space_limit_bytes":
            protocol["memory_address_space_limit_bytes"],
            "process_watchdog_seconds": protocol["process_watchdog_seconds"],
            "solver_sha256": protocol["solver_sha256"],
            "inputs": {name: spec["sha256"]},
        }
        require(file_sha(xcnf) == spec["sha256"],
                "formula SHA-256 immediately before search")
        receipt = run_arm(solver_protocol, arm, inputs, solver, out)
        completed.append(receipt)
        print(json.dumps({"configuration": config_id,
                          "status": receipt["status"],
                          "conflicts": receipt["conflicts"],
                          "wall_seconds_L0": receipt["wall_seconds_L0"]},
                         sort_keys=True), flush=True)
        if receipt["status"] in ("SATISFIABLE", "UNSATISFIABLE"):
            candidates.append({"configuration": config_id,
                               "solver_status": receipt["status"]})
        elif receipt["status"] != "INDETERMINATE":
            stop_reason = receipt["status"]
        if stop_reason:
            break
    summary = {"schema": "n83.solver-setting-panel/v1",
               "status": "COMPLETE_BOUNDED" if stop_reason is None else "STOPPED",
               "stratum": row["id"], "stop_reason": stop_reason,
               "completed_arms": [receipt["arm"]["id"] for receipt in completed],
               "summed_solver_wall_seconds_L0":
               sum(receipt["wall_seconds_L0"] for receipt in completed),
               "candidate_statuses": candidates,
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
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    protocol = json.loads(args.protocol.read_bytes())
    frozen(protocol)
    matches = [row for row in protocol["strata"] if row["id"] == args.stratum]
    require(len(matches) == 1, "one named stratum")
    run(protocol, matches[0], args.inputs, args.solver, args.out)
