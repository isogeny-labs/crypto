"""Source-bound full-D18 native-model/XCNF equivalence supervisor."""

import argparse
import base64
import datetime
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import signal
import subprocess
import sys
import time

OBJECT = "c295da8898cfad29a053621c768ca059eea4263f89f5ca78b4f98af43c383f3f.jsonl.gz"
OBJECT_SHA = "eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b"
WRAPPER_SHA = "1b93351febd47dac4de8c5dd39dbf58564d0057a044054baf448d978f6c0d616"
XCNF_SHA = "0d7bb7ef45ae19937220945129f61baccdd72b85c58bb7f16dcb56ca3ec24d7c"
FIXTURE = "research/koblitz_n83_factor_base_sweep_20261008/pilot-01"


def sha(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def save(path, data):
    with path.open("x") as stream:
        json.dump(data, stream, sort_keys=True, indent=2)
        stream.write("\n")


def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def preflight(source, binary, base):
    if subprocess.check_output(["git", "-C", str(source), "status", "--porcelain"], text=True):
        raise ValueError("source checkout is not clean")
    if binary.name != "koblitz_n83_factor_base_export" or not binary.is_file():
        raise ValueError("release exporter missing")
    obj = base / "objects" / OBJECT
    if obj.stat().st_size != 160_747_529 or sha(obj) != OBJECT_SHA:
        raise ValueError("fresh compressed D18 object differs")
    wrapper_path = base / "s3-manifest.json"
    if sha(wrapper_path) != WRAPPER_SHA:
        raise ValueError("fresh S3 wrapper differs")
    wrapper = json.loads(wrapper_path.read_text())
    if (wrapper["compressed_sha256"] != OBJECT_SHA
            or wrapper["compressed_bytes"] != 160_747_529
            or wrapper["columns"] != 32_643
            or wrapper["points"] != 5_418_738
            or wrapper["producer_manifest_sha256"] != sha(base / "manifest.json")):
        raise ValueError("S3 wrapper-to-producer binding differs")
    for name, expected_sha, expected_length in (
        ("object-get-metadata.json", OBJECT_SHA, 160_747_529),
        ("manifest-get-metadata.json", WRAPPER_SHA, wrapper_path.stat().st_size),
    ):
        metadata = json.loads((base / name).read_text())
        if (metadata["ContentLength"] != expected_length
                or metadata["ChecksumType"] != "FULL_OBJECT"
                or metadata["ChecksumSHA256"] != base64.b64encode(bytes.fromhex(expected_sha)).decode()
                or metadata["ServerSideEncryption"] != "AES256"):
            raise ValueError("provider checksum, length or encryption differs")
    return {"schema": "n83.d18-native-xcnf-witness-identity/v1",
            "source_commit": subprocess.check_output(["git", "-C", str(source), "rev-parse", "HEAD"], text=True).strip(),
            "binary_sha256": sha(binary), "object_sha256": OBJECT_SHA,
            "wrapper_sha256": WRAPPER_SHA, "expected_xcnf_sha256": XCNF_SHA,
            "host": platform.platform(), "selected_best_total_runtime": None}


def run(args):
    source, binary, base, output = (Path(value).resolve() for value in
                                    (args.source, args.binary, args.base, args.output))
    identity = preflight(source, binary, base)
    output.mkdir(exist_ok=False)
    save(output / "identity.json", identity)
    arm = output / "witness"
    arm.mkdir(exist_ok=False)
    result_path = arm / "result.json"
    argv = ["/usr/bin/time", "-l", str(binary),
            "primary-full-d18-m4-l0-xcnf-witness", str(base),
            str(source / FIXTURE), str(result_path)]
    save(arm / "command.json", argv)
    started, monotonic = utc(), time.monotonic()
    timed_out = False
    with (arm / "stdout.txt").open("wb") as stdout, (arm / "stderr-time.txt").open("wb") as stderr:
        process = subprocess.Popen(argv, cwd=source, stdout=stdout, stderr=stderr,
                                   start_new_session=True)
        try:
            code = process.wait(timeout=310)
        except subprocess.TimeoutExpired:
            timed_out = True
            os.killpg(process.pid, signal.SIGKILL)
            code = process.wait()
    usage = (arm / "stderr-time.txt").read_text(errors="replace")
    match = re.search(r"(\d+)\s+maximum resident set size", usage)
    receipt = {"schema": "n83.d18-native-xcnf-witness-supervisor/v1",
               "source_commit": identity["source_commit"],
               "binary_sha256": identity["binary_sha256"],
               "object_sha256": OBJECT_SHA,
               "started_utc": started, "ended_utc": utc(),
               "elapsed_wall_seconds": time.monotonic() - monotonic,
               "outer_cap_seconds": 310, "outer_cap_reached": timed_out,
               "exit_code": code,
               "maximum_resident_set_bytes": int(match.group(1)) if match else None,
               "isolation": "L0_shared_host_no_hard_cgroup",
               "selected_best_total_runtime": None}
    save(arm / "supervisor.json", receipt)
    if code != 0 or timed_out:
        return 2
    result = json.loads(result_path.read_text())
    xcnf, model = arm / "result.xcnf", arm / "result.model.bin"
    if (result["status"] != "PASS_NATIVE_XCNF_WITNESS"
            or result["source_commit"] != identity["source_commit"]
            or result["binary_sha256"] != identity["binary_sha256"]
            or result["xcnf_sha256"] != XCNF_SHA
            or result["xcnf_sha256"] != sha(xcnf)
            or result["xcnf_bytes"] != xcnf.stat().st_size
            or result["model_sha256"] != sha(model)
            or result["model_bytes"] != model.stat().st_size):
        raise ValueError("native witness or exact prior XCNF identity differs")
    return 0


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    for name in ("source", "binary", "base", "output"):
        parser.add_argument("--" + name, required=True)
    sys.exit(run(parser.parse_args()))
