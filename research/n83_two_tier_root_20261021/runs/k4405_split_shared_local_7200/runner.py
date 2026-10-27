"""Bounded local correctness run for the N83 K4405 split root index."""

import argparse
import hashlib
import json
import os
import shutil
import signal
import subprocess
import time
from pathlib import Path


FLAGS = {
    "KIC_INDEXED5_ID30": "1",
    "KIC_INDEXED5_LOAD80": "1",
    "KIC_INDEXED5_SPLIT_AT": "4300",
    "KIC_INDEXED_ROOT5": "1",
    "KIC_INDEX_ROOT_BATCH": "1",
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


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def terminate_group(pid, mode):
    try:
        os.killpg(pid, mode)
    except ProcessLookupError:
        pass


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--target", type=Path, required=True)
    parser.add_argument("--expected-scalar", type=int, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--variant", choices=("dual", "shared"), required=True)
    parser.add_argument("--cap-seconds", type=int, default=7200)
    parser.add_argument("--rss-cap-gib", type=int, default=26)
    args = parser.parse_args()
    if not 60 <= args.cap_seconds <= 7200 or not 1 <= args.rss_cap_gib <= 48:
        parser.error("cap must be 60..7200 seconds and RSS cap 1..48 GiB")
    binary = args.binary.resolve(strict=True)
    target_source = args.target.resolve(strict=True)
    root = args.output.resolve()
    root.mkdir(parents=True, exist_ok=False)
    shutil.copyfile(Path(__file__).resolve(), root / "runner.py")
    target = root / "target.jsonl"
    shutil.copyfile(target_source, target)
    repo = Path(__file__).resolve().parents[2]
    source = repo / "examples/koblitz_orbit_dlp_fast.rs"
    head = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=repo, text=True).strip()
    committed_source = subprocess.check_output(
        ["git", "show", "HEAD:examples/koblitz_orbit_dlp_fast.rs"], cwd=repo)
    if hashlib.sha256(committed_source).hexdigest() != sha256(source):
        raise ValueError("example source differs from committed HEAD")
    env = {key: value for key, value in os.environ.items() if not key.startswith("KIC_")}
    env.update(FLAGS)
    env["KIC_INDEXED5_SPLIT_SHARED_BLOOM"] = "1" if args.variant == "shared" else "0"
    env["KIC_DUMP_BASE"] = str(root / "base_dump.jsonl")
    scratch = Path("/private/tmp/n83_two_tier_run_tmp")
    scratch.mkdir(parents=True, exist_ok=True)
    env["TMPDIR"] = str(scratch)
    command = [str(binary), "construct:83:1:4405", str(target), "29",
               str(root / "targets_result.jsonl")]
    receipt = {
        "schema": "n83.split-full-local-control/v1",
        "variant": args.variant,
        "source_head": head,
        "source_sha256": sha256(source),
        "binary_sha256": sha256(binary),
        "runner_sha256": sha256(root / "runner.py"),
        "target_sha256": sha256(target),
        "expected_scalar": args.expected_scalar,
        "cap_seconds": args.cap_seconds,
        "rss_cap_bytes": args.rss_cap_gib * 1024**3,
        "tmpdir": str(scratch),
        "command": command,
        "environment_flags": {key: env[key] for key in sorted(env) if key.startswith("KIC_")},
    }
    (root / "input.json").write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n")
    start = time.monotonic()
    stop_reason = None
    peak_kib = 0
    with (root / "stdout.jsonl").open("wb") as stdout, \
         (root / "stderr.log").open("wb") as stderr:
        child = subprocess.Popen(command, cwd=repo, env=env, stdout=stdout,
                                 stderr=stderr, start_new_session=True)
        while child.poll() is None:
            if time.monotonic() - start > args.cap_seconds:
                stop_reason = f"TIMEOUT_{args.cap_seconds}S"
                terminate_group(child.pid, signal.SIGTERM)
                break
            sample = subprocess.run(["ps", "-o", "rss=", "-p", str(child.pid)],
                                    capture_output=True, text=True)
            if sample.returncode == 0 and sample.stdout.strip().isdigit():
                peak_kib = max(peak_kib, int(sample.stdout.strip()))
                if peak_kib * 1024 > receipt["rss_cap_bytes"]:
                    stop_reason = "RSS_CAP"
                    terminate_group(child.pid, signal.SIGTERM)
                    break
            time.sleep(2)
        try:
            code = child.wait(timeout=20)
        except subprocess.TimeoutExpired:
            terminate_group(child.pid, signal.SIGKILL)
            code = child.wait()
    result = {
        "schema": "n83.split-full-local-control-result/v1",
        "status": stop_reason or ("PASS_PROCESS" if code == 0 else "FAILED_PROCESS"),
        "exit_code": code,
        "wall_seconds": time.monotonic() - start,
        "sampled_peak_rss_bytes": peak_kib * 1024,
    }
    (root / "process_result.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps(result, sort_keys=True), flush=True)


if __name__ == "__main__":
    main()
