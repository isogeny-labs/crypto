"""Serial, source-bound full-D18 XCNF export and external-solver supervisor."""

import argparse
import base64
import datetime
import hashlib
import json
import os
from pathlib import Path
import platform
import signal
import subprocess
import sys
import time

OBJECT = "c295da8898cfad29a053621c768ca059eea4263f89f5ca78b4f98af43c383f3f.jsonl.gz"
OBJECT_SHA = "eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b"
MANIFEST_SHA = "1b93351febd47dac4de8c5dd39dbf58564d0057a044054baf448d978f6c0d616"
CMS_SHA = "a3f85c3709b5e2a040bf82a4a604d1c7b9f10219bbf180a9e0f72319a2e892ac"
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


def command(args):
    return subprocess.check_output(args, text=True).strip()


def supervise(name, argv, limit, source, output, identity):
    arm = output / name
    arm.mkdir(exist_ok=False)
    save(arm / "command.json", argv)
    started, monotonic = utc(), time.monotonic()
    timed_out = False
    with (arm / "stdout.txt").open("wb") as stdout, (arm / "stderr-time.txt").open("wb") as stderr:
        process = subprocess.Popen(argv, cwd=source, stdout=stdout, stderr=stderr,
                                   start_new_session=True)
        try:
            exit_code = process.wait(timeout=limit)
        except subprocess.TimeoutExpired:
            timed_out = True
            os.killpg(process.pid, signal.SIGKILL)
            exit_code = process.wait()
    receipt = {"schema": "n83.d18-xcnf-supervisor/v1", "arm": name,
               "source_commit": identity["source_commit"],
               "exporter_sha256": identity["exporter_sha256"],
               "cms_sha256": identity["cms_sha256"],
               "object_sha256": OBJECT_SHA, "started_utc": started,
               "ended_utc": utc(), "elapsed_wall_seconds": time.monotonic() - monotonic,
               "outer_cap_seconds": limit, "outer_cap_reached": timed_out,
               "exit_code": exit_code, "isolation": "L0_shared_host_no_hard_cgroup",
               "selected_best_total_runtime": None}
    save(arm / "supervisor.json", receipt)
    return receipt


def preflight(source, binary, cms, base):
    if command(["git", "-C", str(source), "status", "--porcelain"]):
        raise ValueError("source checkout is not clean")
    if binary.name != "koblitz_n83_factor_base_export" or not binary.is_file():
        raise ValueError("release exporter differs")
    if not cms.is_file() or sha(cms) != CMS_SHA:
        raise ValueError("CryptoMiniSat binary differs")
    obj = base / "objects" / OBJECT
    if obj.stat().st_size != 160_747_529 or sha(obj) != OBJECT_SHA:
        raise ValueError("compressed D18 object differs")
    if sha(base / "s3-manifest.json") != MANIFEST_SHA:
        raise ValueError("S3 wrapper differs")
    for name in ("object-get-metadata.json", "manifest-get-metadata.json", "manifest.json"):
        if not (base / name).is_file():
            raise ValueError("S3 GET or producer manifest missing")
    wrapper = json.loads((base / "s3-manifest.json").read_text())
    if (wrapper["compressed_sha256"] != OBJECT_SHA
            or wrapper["compressed_bytes"] != 160_747_529
            or wrapper["columns"] != 32_643
            or wrapper["points"] != 5_418_738
            or wrapper["producer_manifest_sha256"] != sha(base / "manifest.json")):
        raise ValueError("S3 wrapper-to-producer binding differs")
    for name, expected_sha, expected_length in (
        ("object-get-metadata.json", OBJECT_SHA, 160_747_529),
        ("manifest-get-metadata.json", MANIFEST_SHA, (base / "s3-manifest.json").stat().st_size),
    ):
        metadata = json.loads((base / name).read_text())
        if (metadata["ContentLength"] != expected_length
                or metadata["ChecksumType"] != "FULL_OBJECT"
                or metadata["ChecksumSHA256"] != base64.b64encode(bytes.fromhex(expected_sha)).decode()
                or metadata["ServerSideEncryption"] != "AES256"):
            raise ValueError("provider checksum, length or encryption metadata differs")
    return {"schema": "n83.d18-xcnf-identity/v1",
            "source_commit": command(["git", "-C", str(source), "rev-parse", "HEAD"]),
            "exporter_sha256": sha(binary), "cms_sha256": CMS_SHA,
            "object_sha256": OBJECT_SHA, "manifest_sha256": MANIFEST_SHA,
            "host": platform.platform(), "selected_best_total_runtime": None}


def require_export(path, mode, identity):
    result = json.loads((path / "export.json").read_text())
    xcnf = path / "export.xcnf"
    if (result["status"] != "PASS_XCNF_EXPORT_ONLY"
            or result["mode"] != mode
            or result["source_commit"] != identity["source_commit"]
            or result["binary_sha256"] != identity["exporter_sha256"]
            or result["xcnf_bytes"] != xcnf.stat().st_size
            or result["xcnf_sha256"] != sha(xcnf)):
        raise ValueError("XCNF export identity differs")
    return xcnf


def run(args):
    source, binary, cms, base, output = (Path(value).resolve() for value in
                                       (args.source, args.binary, args.cms, args.base, args.output))
    identity = preflight(source, binary, cms, base)
    if args.stage == "export":
        output.mkdir(exist_ok=False)
        save(output / "identity.json", identity)
        for mode in ("positive", "natural"):
            result = output / ("export-" + mode) / "export.json"
            argv = ["/usr/bin/time", "-l", str(binary), "primary-full-d18-m4-l0-xcnf",
                    mode, str(base), str(source / FIXTURE), str(result)]
            receipt = supervise("export-" + mode, argv, 310, source, output, identity)
            if receipt["exit_code"] != 0 or receipt["outer_cap_reached"]:
                return 2
            require_export(result.parent, mode, identity)
    else:
        frozen = json.loads((output / "identity.json").read_text())
        if frozen != identity:
            raise ValueError("source, input, solver or host identity changed")
        mode = "positive" if args.stage == "positive" else "natural"
        xcnf = require_export(output / ("export-" + mode), mode, identity)
        if mode == "natural":
            audit = json.loads((output / "positive-audit.json").read_text())
            if (audit["status"] != "PASS_CONSTRUCTED_XCNF_SAT"
                    or audit["source_commit"] != identity["source_commit"]
                    or audit["cms_sha256"] != CMS_SHA):
                raise ValueError("independent positive XCNF gate missing")
        argv = ["/usr/bin/time", "-l", str(cms), "--threads", "1",
                "--maxtime", "1200", "--maxconfl", "100000000",
                "--verb", "1", "--verbstat", "1", "--printsol", "1", str(xcnf)]
        supervise("solve-" + mode, argv, 1210, source, output, identity)
    return 0


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--stage", choices=("export", "positive", "natural"), required=True)
    for name in ("source", "binary", "cms", "base", "output"):
        parser.add_argument("--" + name, required=True)
    sys.exit(run(parser.parse_args()))
