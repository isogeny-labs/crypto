#!/usr/bin/env python3
"""One-point N83 K1107 root-layout process comparison on a local host."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import subprocess
import time

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
EXPECTED_HEAD = "8bf07aefb3fb2a65248ce379da7284cb58a5581d"
TARGET_SHA256 = "5378b8e1b3b5e01f4590ddd89610ab39b3fca28ecaaa050013f2031dc3c1bb1b"
RHO_CONFIG_SHA256 = "cf76ebb5372acfc1355b88374f875980be66cdffba1a82cb6e4c2f28bcfa723b"
SOURCE_SHA256 = "4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507"


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def flags(folder, layout):
    root22 = layout == "root22"
    return {
        "PATH": os.environ.get("PATH", "/usr/bin:/bin"),
        "KIC_PACKED_ROOT_TABLE": "1", "KIC_PACKED_ROOT_TABLE_DENSE": "0",
        "KIC_PACKED_ROOT_TABLE_LOAD75": "0", "KIC_STATE28": "0", "KIC_STATE26": "0",
        "KIC_ROOT_ONLY22": "1" if root22 else "0", "KIC_ROOT_ONLY21": "0",
        "KIC_ROOT_ONLY11": "0" if root22 else "1", "KIC_INDEXED_ROOT5": "1",
        "KIC_INDEXED_ROOT4": "0", "KIC_ROOT_BLOOM": "0",
        "KIC_SYMMETRIC_S3": "1", "KIC_INDEXED5_LOAD75": "1",
        "KIC_INDEXED5_FIXED30": "0", "KIC_INDEXED4_FIXED31": "0",
        "KIC_SPARSE_CANONICAL": "1", "KIC_RANK_THREADS": "4",
        "KIC_TARGET_THREADS": "2", "KIC_INDEXED5_ID30": "0",
        "KIC_TARGET_CANCEL": "1", "KIC_INDEX_ROOT_BATCH": "1",
        "KIC_DUMP_BASE": str(folder / "base_dump.jsonl"),
    }


def one_child(folder, argv, env, cap):
    folder.mkdir(parents=True, exist_ok=False)
    start = time.monotonic()
    try:
        completed = subprocess.run(argv, env=env, capture_output=True,
                                   timeout=cap, check=False)
        status = "COMPLETE" if completed.returncode == 0 else "EXIT_NONZERO"
        code = completed.returncode
        stdout, stderr = completed.stdout, completed.stderr
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
               "files": files}
    (folder / "receipt.json").write_text(json.dumps(receipt, sort_keys=True, indent=2) + "\n")
    return receipt


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--ic-binary", type=Path, required=True)
    parser.add_argument("--rho-binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    ic, rho, output = args.ic_binary.resolve(), args.rho_binary.resolve(), args.output.resolve()
    head = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip()
    target = HERE / "target.jsonl"
    rho_config = ROOT / "research/n83_dense_root_20261013/rho_config.json"
    source = ROOT / "examples/koblitz_orbit_dlp_fast.rs"
    if (head != EXPECTED_HEAD or digest(target) != TARGET_SHA256 or
            digest(rho_config) != RHO_CONFIG_SHA256 or digest(source) != SOURCE_SHA256):
        raise ValueError("frozen source or inputs differ")
    output.mkdir(parents=True, exist_ok=False)
    host = {"platform": platform.platform(), "machine": platform.machine(),
            "cpu_count": os.cpu_count(), "source_commit": head,
            "input_sha256": {"target": digest(target), "rho_config": digest(rho_config),
                             "ic_binary": digest(ic), "rho_binary": digest(rho)}}
    (output / "host.json").write_text(json.dumps(host, sort_keys=True, indent=2) + "\n")
    receipts = []
    for name, layout in (("root11_a", "root11"), ("root22_a", "root22"),
                         ("root22_b", "root22"), ("root11_b", "root11")):
        folder = output / name
        command = [str(ic), "construct:83:1:1107", str(target), "29",
                   str(folder / "targets_result.jsonl")]
        receipt = one_child(folder, command, flags(folder, layout), 300)
        receipts.append({"name": name, **receipt})
        print(f"{name}: {receipt['status']} {receipt['wall_seconds']:.3f}s", flush=True)
    folder = output / "rho"
    command = [str(rho), str(target), "2026102401", str(rho_config),
               str(folder / "rho_result.json")]
    receipt = one_child(folder, command, {"PATH": os.environ.get("PATH", "/usr/bin:/bin")}, 120)
    receipts.append({"name": "rho", **receipt})
    print(f"rho: {receipt['status']} {receipt['wall_seconds']:.3f}s", flush=True)
    (output / "summary.json").write_text(json.dumps(receipts, sort_keys=True, indent=2) + "\n")


if __name__ == "__main__":
    main()
