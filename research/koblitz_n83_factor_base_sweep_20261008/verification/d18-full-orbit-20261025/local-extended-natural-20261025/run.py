"""Serial source-bound local D18 control and extended natural screen."""

import argparse
import datetime
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import time

OBJECT_NAME = "c295da8898cfad29a053621c768ca059eea4263f89f5ca78b4f98af43c383f3f.jsonl.gz"
OBJECT_SHA = "eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b"
MANIFEST_SHA = "1b93351febd47dac4de8c5dd39dbf58564d0057a044054baf448d978f6c0d616"
EXAMPLE = "koblitz_n83_factor_base_export"


def sha(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def save(path, data):
    with path.open("x") as stream:
        json.dump(data, stream, sort_keys=True, indent=2)
        stream.write("\n")


def run_arm(name, mode, outer_cap, source, binary, base, output):
    arm = output / name
    arm.mkdir()
    result = arm / "result.json"
    command = ["/usr/bin/time", "-l", str(binary), mode, str(base),
               str(source / "research/koblitz_n83_factor_base_sweep_20261008/pilot-01"),
               str(result)]
    save(arm / "command.json", command)
    started, monotonic = utc(), time.monotonic()
    with (arm / "stdout.json").open("wb") as stdout, (arm / "stderr-time.txt").open("wb") as stderr:
        process = subprocess.Popen(command, cwd=source, stdout=stdout, stderr=stderr)
        try:
            exit_code = process.wait(timeout=outer_cap)
            outer_reached = False
        except subprocess.TimeoutExpired:
            process.kill()
            exit_code = process.wait()
            outer_reached = True
    ended = utc()
    receipt = {"schema": "n83.d18-local-extended-supervisor/v1", "arm": name,
               "source_commit": subprocess.check_output(["git", "-C", str(source), "rev-parse", "HEAD"], text=True).strip(),
               "binary_sha256": sha(binary), "object_sha256": OBJECT_SHA,
               "started_utc": started, "ended_utc": ended,
               "elapsed_wall_seconds": time.monotonic() - monotonic,
               "outer_cap_seconds": outer_cap, "outer_cap_reached": outer_reached,
               "exit_code": exit_code, "isolation": "L0_shared_host_no_hard_cgroup",
               "selected_best_total_runtime": None}
    save(arm / "supervisor.json", receipt)
    return receipt


def main(args):
    source, binary, base, output = (Path(value).resolve() for value in
                                     (args.source, args.binary, args.base, args.output))
    if output.exists() != args.natural_only:
        raise ValueError("initial output must be new and resumed output must exist")
    if subprocess.check_output(["git", "-C", str(source), "status", "--porcelain"], text=True).strip():
        raise ValueError("source checkout is not clean")
    if not binary.is_file() or not binary.name == EXAMPLE:
        raise ValueError("wrong release example binary")
    obj = base / "objects" / OBJECT_NAME
    if obj.stat().st_size != 160_747_529 or sha(obj) != OBJECT_SHA or sha(base / "s3-manifest.json") != MANIFEST_SHA:
        raise ValueError("full D18 S3 input identity differs")
    if not (base / "object-get-metadata.json").is_file() or not (base / "manifest-get-metadata.json").is_file():
        raise ValueError("fresh S3 GET metadata missing")
    source_commit = subprocess.check_output(["git", "-C", str(source), "rev-parse", "HEAD"], text=True).strip()
    if args.natural_only:
        identity = json.loads((output / "identity.json").read_text())
        positive_audit = json.loads((output / "positive-audit.json").read_text())
        if (identity["source_commit"] != source_commit
                or identity["binary_sha256"] != sha(binary)
                or positive_audit["status"] != "PASS_CONSTRUCTED_CONTROL"
                or positive_audit["source_commit"] != source_commit
                or positive_audit["binary_sha256"] != sha(binary)):
            raise ValueError("independent positive gate or identity differs")
        natural = run_arm("natural", "primary-full-d18-m4-l0-shared-phase-first-natural-1200",
                          1210, source, binary, base, output)
        save(output / "natural-supervisor.json", natural)
    else:
        output.mkdir()
        save(output / "identity.json", {"schema": "n83.d18-local-extended-identity/v1",
             "source_commit": source_commit, "binary_sha256": sha(binary),
             "object_sha256": OBJECT_SHA, "manifest_sha256": MANIFEST_SHA,
             "started_utc": utc(), "selected_best_total_runtime": None})
        positive = run_arm("positive", "primary-full-d18-m4-l0-shared-phase-first",
                           310, source, binary, base, output)
        save(output / "gate.json", {"schema": "n83.d18-local-extended-gate/v1",
             "positive": positive, "natural_pending": True,
             "selected_best_total_runtime": None})
    return 0


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    for arg in ("source", "binary", "base", "output"):
        parser.add_argument("--" + arg, required=True)
    parser.add_argument("--natural-only", action="store_true")
    sys.exit(main(parser.parse_args()))
