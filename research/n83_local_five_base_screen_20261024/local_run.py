#!/usr/bin/env python3
"""Run the frozen five N83 factor bases on one new local public point."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import subprocess
import time

from research.n83_fresh_cold_finalists_20261019 import arms as frozen_arms
from research.n83_fresh_cold_finalists_20261019 import run as frozen

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
SOURCE_COMMIT = "8bf07aefb3fb2a65248ce379da7284cb58a5581d"
SOURCE_SHA256 = "4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507"
TARGET_SHA256 = "b1a9b1bc398b1c73867b8b0c19ef8a7438225917989e08b1c15aef2c7640c0bf"
RHO_CONFIG_SHA256 = "cf76ebb5372acfc1355b88374f875980be66cdffba1a82cb6e4c2f28bcfa723b"
EXPECTED_NAMES = ("k992_root11", "k1024_root11", "k3300_wide3",
                  "k2304_root11", "k1107_root11")


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def sealed_arms():
    dispatch_path = ROOT / "research/n83_fresh_cold_finalists_20261019/dispatch.json"
    dispatch = json.loads(dispatch_path.read_text())
    if (dispatch["status"] != "SEALED_FOR_DISPATCH" or
            dispatch["source_sha256"] != SOURCE_SHA256 or
            dispatch["ordered_candidate_ids"] != dispatch["choice"]["ordered_candidate_ids"]):
        raise ValueError("sealed dispatch differs")
    inventory = {arm[3]: arm for arm in
                 (*frozen_arms.SMALL, *frozen_arms.MID, *frozen_arms.six_run.ARMS)}
    arms = tuple(inventory[item] for item in dispatch["ordered_candidate_ids"])
    if tuple(arm[0] for arm in arms) != EXPECTED_NAMES:
        raise ValueError("five-arm order differs")
    for _, _, _, candidate_id in arms:
        path, _ = frozen_arms.manifest(candidate_id)
        if digest(path) != dispatch["choice"]["candidate_manifest_sha256"][candidate_id]:
            raise ValueError(f"manifest differs: {candidate_id}")
    return arms, digest(dispatch_path)


def one_child(folder, argv, env, cap, meta):
    folder.mkdir(parents=True, exist_ok=False)
    start = time.monotonic()
    try:
        completed = subprocess.run(argv, env=env, capture_output=True,
                                   timeout=cap, check=False)
        status = "COMPLETE" if completed.returncode == 0 else "EXIT_NONZERO"
        code, stdout, stderr = completed.returncode, completed.stdout, completed.stderr
    except subprocess.TimeoutExpired as exc:
        status, code = "TIMEOUT", None
        stdout, stderr = exc.stdout or b"", exc.stderr or b""
    wall = time.monotonic() - start
    (folder / "stdout.jsonl").write_bytes(stdout)
    (folder / "stderr.txt").write_bytes(stderr)
    files = {}
    for path in sorted(folder.rglob("*")):
        if path.is_file():
            files[str(path.relative_to(folder))] = {
                "sha256": digest(path), "bytes": path.stat().st_size}
    receipt = {"argv": argv, "environment": env, "cap_seconds": cap,
               "status": status, "exit_code": code, "wall_seconds": wall,
               "files": files, **meta}
    (folder / "receipt.json").write_text(json.dumps(receipt, sort_keys=True, indent=2) + "\n")
    return receipt


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--ic-binary", required=True, type=Path)
    parser.add_argument("--rho-binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    ic, rho, output = args.ic_binary.resolve(), args.rho_binary.resolve(), args.output.resolve()
    head = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip()
    target = HERE / "target.jsonl"
    rho_config = ROOT / "research/n83_dense_root_20261013/rho_config.json"
    source = ROOT / "examples/koblitz_orbit_dlp_fast.rs"
    arms, dispatch_sha256 = sealed_arms()
    if (head != SOURCE_COMMIT or digest(source) != SOURCE_SHA256 or
            digest(target) != TARGET_SHA256 or digest(rho_config) != RHO_CONFIG_SHA256):
        raise ValueError("source or public input differs")
    output.mkdir(parents=True, exist_ok=False)
    host = {"schema": "n83.local-five-base-host/v1",
            "source_commit": head, "source_sha256": SOURCE_SHA256,
            "dispatch_sha256": dispatch_sha256, "platform": platform.platform(),
            "machine": platform.machine(), "cpu_count": os.cpu_count(),
            "input_sha256": {"target": digest(target), "rho_config": digest(rho_config),
                             "ic_binary": digest(ic), "rho_binary": digest(rho)}}
    (output / "host.json").write_text(json.dumps(host, sort_keys=True, indent=2) + "\n")
    receipts = []

    def rho_child(name, seed):
        folder = output / name
        command = [str(rho), str(target), seed, str(rho_config),
                   str(folder / "rho_result.json")]
        receipt = one_child(folder, command,
                            {"PATH": os.environ.get("PATH", "/usr/bin:/bin")},
                            frozen.RHO_CAP_SECONDS, {"name": name, "kind": "rho", "seed": seed})
        receipts.append(receipt)
        print(f"{name}: {receipt['status']} {receipt['wall_seconds']:.3f}s", flush=True)

    rho_child("rho_before", "2026102402")
    schedule = [("a", arm) for arm in arms] + [("b", arm) for arm in reversed(arms)]
    for repetition, (name, k, layout, candidate_id) in schedule:
        folder = output / f"{repetition}_{name}"
        command = [str(ic), f"construct:83:1:{k}", str(target), "29",
                   str(folder / "targets_result.jsonl")]
        receipt = one_child(folder, command, frozen.ic_environment(folder, layout, name),
                            frozen.IC_CAP_SECONDS[name],
                            {"name": folder.name, "kind": "ic", "k": k,
                             "layout": layout, "candidate_id": candidate_id,
                             "repetition": repetition})
        receipts.append(receipt)
        print(f"{folder.name}: {receipt['status']} {receipt['wall_seconds']:.3f}s", flush=True)
    rho_child("rho_after", "2026102403")
    (output / "summary.json").write_text(json.dumps(receipts, sort_keys=True, indent=2) + "\n")


if __name__ == "__main__":
    main()
