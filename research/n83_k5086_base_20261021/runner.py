"""Bounded local enumeration of an N83 orbit factor base without a root index."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import signal
import subprocess
import time


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
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--k", type=int, choices=(128, 4405, 5086, 5500), required=True)
    parser.add_argument("--cap-seconds", type=int, default=1800)
    parser.add_argument("--rss-cap-gib", type=int, default=8)
    args = parser.parse_args()
    if not 60 <= args.cap_seconds <= 3600 or not 1 <= args.rss_cap_gib <= 16:
        parser.error("wall cap must be 60..3600 seconds and RSS cap 1..16 GiB")
    repo = Path('/Volumes/SSD990-2/tmp/n83_two_tier_20261021')
    source = repo / "examples/koblitz_orbit_dlp_fast.rs"
    binary = args.binary.resolve(strict=True)
    target = args.target.resolve(strict=True)
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    shutil.copyfile(Path(__file__).resolve(), output / "runner.py")
    shutil.copyfile(source, output / "source_snapshot.rs")
    shutil.copy2(binary, output / "executable_snapshot")
    shutil.copyfile(target, output / "target.jsonl")
    env = {key: value for key, value in os.environ.items() if not key.startswith("KIC_")}
    env["KIC_BASE_BUILD_ONLY"] = "1"
    env["KIC_DUMP_BASE"] = str(output / "base_dump.jsonl")
    command = [str(output / "executable_snapshot"), f"construct:83:1:{args.k}",
               str(output / "target.jsonl"), "29", str(output / "targets_result.jsonl")]
    receipt = {"schema": "n83.base-only-input/v1", "k": args.k,
               "source_git_head": subprocess.check_output(
                   ["git", "rev-parse", "HEAD"], cwd=repo, text=True).strip(),
               "source_sha256": sha256(output / "source_snapshot.rs"),
               "binary_sha256": sha256(output / "executable_snapshot"),
               "runner_sha256": sha256(output / "runner.py"),
               "target_sha256": sha256(output / "target.jsonl"),
               "cap_seconds": args.cap_seconds,
               "rss_cap_bytes": args.rss_cap_gib * 1024**3,
               "command": command,
               "environment_flags": {key: env[key] for key in sorted(env)
                                     if key.startswith("KIC_")}}
    (output / "input.json").write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n")
    started = time.monotonic()
    peak_kib = 0
    stop_reason = None
    with (output / "stdout.jsonl").open("wb") as stdout, \
         (output / "stderr.log").open("wb") as stderr:
        child = subprocess.Popen(command, cwd=repo, env=env, stdout=stdout,
                                 stderr=stderr, start_new_session=True)
        while child.poll() is None:
            if time.monotonic() - started > args.cap_seconds:
                stop_reason = "TIMEOUT"
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
            time.sleep(1)
        try:
            code = child.wait(timeout=20)
        except subprocess.TimeoutExpired:
            terminate_group(child.pid, signal.SIGKILL)
            code = child.wait()
    result = {"schema": "n83.base-only-process/v1", "k": args.k,
              "status": stop_reason or ("PASS_PROCESS" if code == 0 else "FAILED_PROCESS"),
              "exit_code": code, "wall_seconds": time.monotonic() - started,
              "sampled_peak_rss_bytes": peak_kib * 1024,
              "child_pid": child.pid}
    (output / "process_result.json").write_text(
        json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps(result, sort_keys=True), flush=True)
    return 0 if result["status"] == "PASS_PROCESS" else 1


if __name__ == "__main__":
    raise SystemExit(main())
