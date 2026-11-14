#!/usr/bin/env python3
"""Export one source-bound D18 natural XCNF under the frozen local cap."""

import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import shutil
import signal
import subprocess
import time


SOURCE_FILES = (
    "examples/koblitz_n83_factor_base_export.rs",
    "research/koblitz_n83_factor_base_sweep_20261008/primary_adapter.rs",
    "src/cryptanalysis/binary_semaev_chain_sat.rs",
    "src/cryptanalysis/sat.rs",
    "src/cryptanalysis/koblitz_index_calculus.rs",
    "research/koblitz_n83_factor_base_sweep_20261008/size-frontier-v2.json",
)


def require(ok, message):
    if not ok:
        raise ValueError(message)


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def swapouts():
    return int(subprocess.check_output(["vm_stat"], text=True).split("Swapouts:", 1)[1].split()[0].rstrip("."))


def preflight(root, worker, lock):
    require(sha256(worker) == lock["worker_binary_sha256"], "worker differs")
    input_root = Path(lock["input_root"])
    require(sha256(input_root / "manifest.json") == lock["producer_manifest_sha256"], "manifest differs")
    object_path = input_root / "objects/c295da8898cfad29a053621c768ca059eea4263f89f5ca78b4f98af43c383f3f.jsonl.gz"
    require(sha256(object_path) == lock["compressed_object_sha256"], "D18 object differs")
    require(sha256(Path(lock["fixture_root"]) / "probe-corpus.json") == lock["public_corpus_sha256"], "corpus differs")
    positive = json.loads((root / "solver-positive/group-replay.json").read_text())
    require(positive["status"] == "PASS_ORIGINAL_GROUP_AND_ROW_REPLAY", "positive group gate not passed")
    samples = (root / "host-preflight-natural.txt").read_text().splitlines()
    require(len(samples) == 12, "three host samples required")
    loads = [float(samples[index].split()[1]) for index in (1, 5, 9)]
    swaps = [int(samples[index].split()[1].rstrip(".")) for index in (2, 6, 10)]
    require(all(load <= 24 for load in loads) and os.getloadavg()[0] <= 24, "host load gate failed")
    require(len(set(swaps)) == 1 and swapouts() == swaps[-1], "host swapouts changed")
    require(shutil.disk_usage(root).free >= 2 * 1024**3, "scratch gate failed")
    require(not (root / "export-natural").exists(), "natural export output already exists")
    return loads, swaps[-1]


def source_snapshot(root, repo, commit):
    snapshot = root / "source-snapshot-natural"
    snapshot.mkdir(exist_ok=False)
    source_hashes = {}
    for name in SOURCE_FILES:
        data = subprocess.check_output(["git", "-C", str(repo), "show", f"{commit}:{name}"])
        path = snapshot / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data)
        source_hashes[name] = hashlib.sha256(data).hexdigest()
    attestation = {
        "schema": "n83.chain-s3-source-attestation/v1",
        "source_commit": commit,
        "status_clean": True,
        "method": "exact_git_commit_object_snapshot",
        "source_sha256": source_hashes,
    }
    (snapshot / "attestation.json").write_text(json.dumps(attestation, sort_keys=True, indent=2) + "\n")
    return snapshot


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("root", type=Path)
    parser.add_argument("repo", type=Path)
    parser.add_argument("worker", type=Path)
    parser.add_argument("--preflight-only", action="store_true")
    args = parser.parse_args()
    root = args.root.resolve()
    lock = json.loads(Path(__file__).with_name("NATURAL_LOCK.json").read_text())
    require(lock["schema"] == "n83.d18-ordered-fullgauss-natural-lock/v1", "lock schema differs")
    loads, baseline_swaps = preflight(root, args.worker, lock)
    if args.preflight_only:
        print(json.dumps({"status": "PREFLIGHT_PASS", "loads": loads, "swapouts": baseline_swaps}, sort_keys=True))
        return
    snapshot = source_snapshot(root, args.repo, lock["source_commit"])
    out = root / "export-natural"
    out.mkdir(exist_ok=False)
    command = [str(args.worker), "primary-full-d18-m4-l0-ordered-xcnf", "natural",
               lock["input_root"], lock["fixture_root"], str(out / "result.json")]
    (out / "command.json").write_text(json.dumps(command, indent=2) + "\n")
    environment = dict(os.environ, ICV1_FROZEN_SOURCE_DIR=str(snapshot))
    started = datetime.now(timezone.utc).isoformat()
    start = time.monotonic()
    timed_out = False
    with (out / "stdout.txt").open("wb") as stdout, (out / "stderr.txt").open("wb") as stderr:
        process = subprocess.Popen(command, stdout=stdout, stderr=stderr, env=environment, start_new_session=True)
        try:
            code = process.wait(timeout=lock["outer_watchdog_seconds"])
        except subprocess.TimeoutExpired:
            timed_out = True
            os.killpg(process.pid, signal.SIGTERM)
            try:
                code = process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                os.killpg(process.pid, signal.SIGKILL)
                code = process.wait()
    receipt_path = out / "result.json"
    receipt = json.loads(receipt_path.read_text()) if receipt_path.exists() else None
    status = "WATCHDOG" if timed_out else "PRODUCER_FAILURE"
    if code == 0 and receipt is not None:
        shape = receipt["shape"]
        xcnf = out / "result.xcnf"
        require(receipt["status"] == "PASS_XCNF_EXPORT_ONLY", "worker export status differs")
        require(receipt["source_commit"] == lock["source_commit"] and receipt["source_attestation_mode"] == "supervised_snapshot", "source binding differs")
        require(receipt["target"] == lock["expected_target"] and receipt["mode"] == "natural", "natural target differs")
        require(shape["ordered_first_pair"] is True and shape["xor_decision_cadence"] == 32, "formula policy differs")
        require(receipt["solver_search_executed"] is False, "worker ran a solver")
        require(xcnf.stat().st_size <= lock["xcnf_cap_bytes"] and sha256(xcnf) == receipt["xcnf_sha256"], "XCNF differs")
        status = "PASS_XCNF_EXPORT_ONLY"
    record = {
        "schema": "n83.d18-ordered-natural-export-supervisor/v1",
        "status": status,
        "start_utc": started,
        "end_utc": datetime.now(timezone.utc).isoformat(),
        "elapsed_wall_seconds": time.monotonic() - start,
        "exit_code": code,
        "outer_timeout": timed_out,
        "source_snapshot_attestation_sha256": sha256(snapshot / "attestation.json"),
        "worker_binary_sha256": sha256(args.worker),
        "host_load_1m_preflight": loads,
        "swapouts_preflight": baseline_swaps,
        "swapouts_after": swapouts(),
        "stdout_sha256": sha256(out / "stdout.txt"),
        "stderr_sha256": sha256(out / "stderr.txt"),
        "worker_receipt_sha256": sha256(receipt_path) if receipt_path.exists() else None,
        "xcnf_sha256": sha256(out / "result.xcnf") if (out / "result.xcnf").exists() else None,
    }
    (out / "process.json").write_text(json.dumps(record, sort_keys=True, indent=2) + "\n")
    print(json.dumps({"status": status, "exit_code": code, "wall_seconds": record["elapsed_wall_seconds"]}, sort_keys=True))


if __name__ == "__main__":
    main()
