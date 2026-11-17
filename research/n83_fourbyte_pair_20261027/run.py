"""Sequential same-binary K1107 plain/FP2 full-process control."""

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

from research.n83_fourbyte_fp2_20261027.full_k1107 import FLAGS

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
TARGET = ROOT / "research/n83_fourbyte_fp2_20261027/target_k1107.jsonl"
TARGET_SHA256 = "2c068d2f659588edcdbaa685fc73bb146e2bfee3be123d377d537658f8021c6d"
SOURCE_SHA256 = "a99c5a554af48949fe253c28a440ce03095ecdc14eadc8e6087dab8df509e049"
BINARY_SHA256 = "4de2b556aa4963f125ac0fb86b58467d71cb7bcdaf2d74b2e8d99880ea0b6449"
CAP_SECONDS = 900


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def write_json(path, value):
    path.write_text(json.dumps(value, sort_keys=True, indent=2) + "\n")


def run_arm(binary, output, layout, source_head, sequence):
    output.mkdir(parents=True, exist_ok=False)
    executed = output / "executed_binary"
    shutil.copy2(binary, executed)
    flags = dict(FLAGS)
    if layout == "plain":
        flags.pop("KIC_INDEXED4_FP2")
    environment = {key: value for key, value in os.environ.items()
                   if not key.startswith("KIC_")}
    environment.update(flags)
    environment["KIC_DUMP_BASE"] = str(output / "base_dump.jsonl")
    command = [str(executed), "construct:83:1:1107", str(TARGET), "29",
               str(output / "targets_result.jsonl")]
    launch = {"schema": "n83.fourbyte-paired-k1107/v1", "sequence": sequence,
              "layout": layout, "source_git_head": source_head,
              "source_sha256": SOURCE_SHA256, "binary_sha256": BINARY_SHA256,
              "target_sha256": TARGET_SHA256, "flags": flags,
              "cap_seconds": CAP_SECONDS, "command": command}
    write_json(output / "launch.json", launch)
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
    receipt = {"schema": launch["schema"], "layout": layout,
               "sequence": sequence, "status": status, "exit_code": exit_code,
               "process_wall_seconds": time.monotonic() - started,
               "max_child_rss_bytes": resource.getrusage(
                   resource.RUSAGE_CHILDREN).ru_maxrss,
               "files_sha256": {name: sha256(output / name) if
                                (output / name).exists() else None
                                for name in ("executed_binary", "stdout.jsonl",
                                             "stderr.log", "targets_result.jsonl",
                                             "base_dump.jsonl")}}
    write_json(output / "receipt.json", receipt)
    print(json.dumps({"layout": layout, "sequence": sequence,
                      "status": status, "exit_code": exit_code,
                      "wall_seconds": receipt["process_wall_seconds"]}), flush=True)
    return status == "PROCESS_EXITED" and exit_code == 0


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--order", choices=("plain-fp2", "fp2-plain"),
                        default="plain-fp2")
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    source_head = subprocess.check_output(
        ["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip()
    committed = subprocess.check_output(
        ["git", "show", f"{source_head}:examples/koblitz_orbit_dlp_fast.rs"],
        cwd=ROOT)
    if (sha256(ROOT / "examples/koblitz_orbit_dlp_fast.rs") != SOURCE_SHA256 or
            hashlib.sha256(committed).hexdigest() != SOURCE_SHA256 or
            sha256(binary) != BINARY_SHA256 or sha256(TARGET) != TARGET_SHA256):
        raise ValueError("source, binary or target differs from sealed control")
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    write_json(output / "schedule.json", {
        "schema": "n83.fourbyte-paired-k1107-schedule/v1",
        "source_git_head": source_head, "source_sha256": SOURCE_SHA256,
        "binary_sha256": BINARY_SHA256, "target_sha256": TARGET_SHA256,
        "order": args.order, "cap_seconds_per_arm": CAP_SECONDS,
        "host_tier": "shared_local_diagnostic"})
    for sequence, layout in enumerate(args.order.split("-")):
        if not run_arm(binary, output / f"{sequence}_{layout}", layout,
                       source_head, sequence):
            return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
