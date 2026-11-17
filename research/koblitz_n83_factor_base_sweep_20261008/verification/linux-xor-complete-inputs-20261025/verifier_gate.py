#!/usr/bin/env python3
"""Rebuild one exact XCNF from fetched inputs and reject an incomplete model."""

import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile

from audit import HERE, audit, digest, require


STUDY = HERE.parent.parent
BASE = "3888e2597fd3519807c9d722f49f83329c93ce5cb0929501cf7cb7cbc3cab1a0"
HASH_POOL = "b4dda45e6c3edfca673db6f3f82c16cce8c860f5e62c092155258df399a0363e"
SYNTHETIC_STDOUT = b"s SATISFIABLE\nv 1 0\n"


def gate(inputs: Path, worker: Path, expected_worker_sha256: str) -> dict:
    audit(inputs)
    worker_sha256 = digest(worker, "sha256")
    require(worker_sha256 == expected_worker_sha256, "worker binary SHA-256")
    manifest = STUDY / "pilot-01/manifest.json"
    corpus = STUDY / "pilot-01/probe-corpus.json"
    with tempfile.TemporaryDirectory(prefix="n83-verifier-input-") as directory:
        scratch = Path(directory)
        export = scratch / "export"
        export.mkdir()
        (export / "instance.xcnf").symlink_to(inputs / "hash_4_2.xcnf")
        (export / "export.json").symlink_to(
            inputs / "exports/hash_4_2/export.json")
        stdout = scratch / "solver.stdout"
        stdout.write_bytes(SYNTHETIC_STDOUT)
        command = [
            str(worker), "verify-cross-xcnf",
            BASE, str(manifest), str(inputs / "archives" / f"s3-{BASE}.jsonl.gz"),
            HASH_POOL, str(manifest),
            str(inputs / "archives" / f"s3-{HASH_POOL}.jsonl.gz"),
            str(corpus), "4", "2", str(export), str(stdout),
            str(scratch / "verdict"),
        ]
        result = subprocess.run(command, capture_output=True, text=True,
                                timeout=120)
        require(result.returncode != 0
                and "external SAT model is incomplete" in result.stderr
                and not (scratch / "verdict").exists(),
                "verifier must rebuild exact inputs then reject incomplete model")
    return {
        "schema": "n83.linux-xor-verifier-input-gate/v1",
        "status": "PASS_REBUILT_XCNF_REJECTED_INCOMPLETE_MODEL",
        "curve": "icv1-f2m83-tm6151469093347-debefd74",
        "arm": "hash_4_2",
        "input_manifest_sha256": digest(HERE / "manifest.json", "sha256"),
        "xcnf_sha256": digest(inputs / "hash_4_2.xcnf", "sha256"),
        "worker_sha256": worker_sha256,
        "synthetic_stdout_sha256": hashlib.sha256(SYNTHETIC_STDOUT).hexdigest(),
        "expected_parser_rejection": "external SAT model is incomplete",
        "selected_best_total_runtime": None,
    }


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--inputs-dir", type=Path, required=True)
    parser.add_argument("--worker", type=Path, required=True)
    parser.add_argument("--worker-sha256", required=True)
    args = parser.parse_args()
    print(json.dumps(gate(args.inputs_dir, args.worker,
                          args.worker_sha256), indent=2, sort_keys=True))
