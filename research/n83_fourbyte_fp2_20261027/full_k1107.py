"""Capped full-process K1107 correctness control for the FP2 ID31 index."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import resource
import shutil
import signal
import subprocess
import time


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
TARGET_SHA256 = "2c068d2f659588edcdbaa685fc73bb146e2bfee3be123d377d537658f8021c6d"
FLAGS = {
    "KIC_INDEXED_ROOT4": "1",
    "KIC_INDEXED4_ID31": "1",
    "KIC_INDEXED4_FP2": "1",
    "KIC_INDEXED5_SPLIT_AT": "553",
    "KIC_INDEX_ROOT_BATCH": "1",
    "KIC_INDEX_ROOT_THREADS": "4",
    "KIC_PACKED_ROOT_TABLE": "1",
    "KIC_RANK_THREADS": "4",
    "KIC_ROOT_BLOOM": "1",
    "KIC_ROOT_BLOOM_PROFILE": "wide3",
    "KIC_ROOT_ONLY11": "1",
    "KIC_SPARSE_CANONICAL": "1",
    "KIC_SYMMETRIC_S3": "1",
    "KIC_TARGET_CANCEL": "1",
    "KIC_TARGET_THREADS": "2",
}
CAP_SECONDS = 900


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    source_head = subprocess.check_output(
        ["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip()
    committed_source = subprocess.check_output(
        ["git", "show", f"{source_head}:examples/koblitz_orbit_dlp_fast.rs"],
        cwd=ROOT)
    source_digest = sha256(ROOT / "examples/koblitz_orbit_dlp_fast.rs")
    if hashlib.sha256(committed_source).hexdigest() != source_digest:
        raise ValueError("producer source differs from committed source")
    target = HERE / "target_k1107.jsonl"
    if sha256(target) != TARGET_SHA256:
        raise ValueError("public target differs from frozen input")
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    executed_binary = output / "executed_binary"
    shutil.copy2(binary, executed_binary)
    if sha256(executed_binary) != sha256(binary):
        raise ValueError("retained executable differs from checked binary")
    environment = {key: value for key, value in os.environ.items()
                   if not key.startswith("KIC_")}
    environment.update(FLAGS)
    environment["KIC_DUMP_BASE"] = str(output / "base_dump.jsonl")
    command = [str(executed_binary), "construct:83:1:1107", str(target), "29",
               str(output / "targets_result.jsonl")]
    launch = {"schema": "n83.fourbyte-fp2-full-k1107/v1",
              "command": command, "flags": FLAGS,
              "source_sha256": source_digest, "source_git_head": source_head,
              "binary_sha256": sha256(executed_binary), "target_sha256": TARGET_SHA256,
              "cap_seconds": CAP_SECONDS}
    (output / "launch.json").write_text(json.dumps(launch, sort_keys=True,
                                                  indent=2) + "\n")
    started = time.monotonic()
    with (output / "stdout.jsonl").open("wb") as stdout, \
         (output / "stderr.log").open("wb") as stderr:
        process = subprocess.Popen(command, env=environment, stdout=stdout,
                                   stderr=stderr, start_new_session=True)
        try:
            exit_code = process.wait(timeout=CAP_SECONDS)
            status = "PROCESS_EXITED"
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGKILL)
            exit_code = process.wait()
            status = "TIMEOUT"
    receipt = {"schema": launch["schema"], "status": status,
               "exit_code": exit_code,
               "process_wall_seconds": time.monotonic() - started,
               "max_child_rss_bytes": resource.getrusage(
                   resource.RUSAGE_CHILDREN).ru_maxrss,
               "files_sha256": {name: sha256(output / name) if
                                (output / name).exists() else None
                                for name in ("executed_binary", "stdout.jsonl", "stderr.log",
                                             "targets_result.jsonl", "base_dump.jsonl")}}
    (output / "receipt.json").write_text(json.dumps(receipt, sort_keys=True,
                                                    indent=2) + "\n")
    print(json.dumps({"status": status, "exit_code": exit_code,
                      "output": str(output)}, sort_keys=True))
    return 0 if status == "PROCESS_EXITED" and exit_code == 0 else 1


if __name__ == "__main__":
    raise SystemExit(main())
