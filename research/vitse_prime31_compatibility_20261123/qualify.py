#!/usr/bin/env python3
"""Metadata-only source preparation, build/call provenance and execution.
All field, curve, selector and compatibility computations run in Rust.
"""
from pathlib import Path
import argparse, hashlib, json, os, platform, shutil, subprocess, sys
from datetime import datetime, timezone

BASE = "7d61e3b45de6e93d235345a0061dae08db1f8b44"
PKM_BLOB = "9a801f27ae779cd35a6ee21a80252922324a9089"
ANCHOR = b"// ------------------------------- Method 2 with class number > 1 (Hilbert)"
HERE = Path(__file__).resolve().parent

def sha(data):
    return hashlib.sha256(data).hexdigest()

def pin(path):
    data = path.read_bytes()
    return {"path": str(path), "bytes": len(data), "sha256": sha(data)}

def utc():
    return datetime.now(timezone.utc).isoformat()

def write_new(path, obj):
    with path.open("x", encoding="utf-8") as f:
        json.dump(obj, f, indent=2, sort_keys=True)
        f.write("\n")

def call(command, cwd, stdout, stderr, timeout, env):
    with stdout.open("xb") as out, stderr.open("xb") as err:
        try:
            result = subprocess.run(command, cwd=cwd, stdout=out, stderr=err,
                                    timeout=timeout, env=env)
            return {"exit_code": result.returncode, "timed_out": False}
        except subprocess.TimeoutExpired:
            return {"exit_code": 124, "timed_out": True}

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--source", type=Path, required=True,
                    help="Exact source baseline pkm_smooth_tower.rs")
    ap.add_argument("--rustc", type=Path, required=True)
    ap.add_argument("--attempt", required=True)
    ap.add_argument("--evidence", type=Path, default=HERE / "evidence")
    a = ap.parse_args()
    if not a.attempt or Path(a.attempt).name != a.attempt:
        ap.error("attempt must be a simple directory name")
    source = a.source.resolve()
    rustc = a.rustc.resolve()
    attempt = a.evidence.resolve() / a.attempt
    attempt.mkdir(parents=True, exist_ok=False)
    raw = source.read_bytes()
    blob = hashlib.sha1(b"blob " + str(len(raw)).encode() + b"\0" + raw).hexdigest()
    if blob != PKM_BLOB:
        raise SystemExit(f"Wrong baseline source blob: {blob} != {PKM_BLOB}")
    if raw.count(ANCHOR) != 1:
        raise SystemExit("Prefix anchor is missing or ambiguous")
    end = raw.index(ANCHOR)
    generated = HERE / "generated"
    generated.mkdir(exist_ok=True)
    selected = raw[:end]
    dest = generated / "pkm_prefix.rs"
    if dest.exists() and dest.read_bytes() != selected:
        raise SystemExit("Refusing to replace a different generated source")
    if not dest.exists():
        dest.write_bytes(selected)
    if dest.read_bytes() != selected:
        raise SystemExit("Source copy mismatch")
    source_files = [HERE / name for name in
                    ("PROTOCOL.md", "compatibility.rs", "compatibility_tests.rs",
                     "driver.rs", "qualify.py")]
    source_files += [source, dest]
    version = subprocess.check_output([str(rustc), "-Vv"], text=True)
    common = {
        "schema": "vitse-prime31.source-binding/v1",
        "source_base": BASE,
        "source_blob": PKM_BLOB,
        "selected_start": 0,
        "selected_end_exclusive": end,
        "selected_sha256": sha(selected),
        "selection": "unchanged_complete_item_prefix",
        "rewritten": False,
        "whole_library": False,
        "compiler": version,
        "compiler_file": pin(rustc),
        "platform": platform.platform(),
        "machine": platform.machine(),
        "cpu_count": os.cpu_count(),
        "isolation": "shared development container; correctness only",
        "performance_measurement": False,
        "sources": [pin(p) for p in source_files],
    }
    write_new(attempt / "source-build.json", common)
    frozen_sources = {str(p): sha(p.read_bytes()) for p in source_files}
    env = os.environ.copy()
    env["LC_ALL"] = "C"
    for name, source_name, test in [
        ("contract-tests", "compatibility_tests.rs", True),
        ("prime31-audit", "driver.rs", False),
    ]:
        stage = attempt / name
        stage.mkdir()
        built = stage / "build-output"
        frozen = stage / "frozen-executable"
        command = [str(rustc), "--edition=2021", "-C", "opt-level=1",
                   "-C", "debuginfo=0"]
        if test:
            command.append("--test")
        command += [str(HERE / source_name), "-o", str(built)]
        write_new(stage / "build-request.json",
                  {"at": utc(), "command": command, "cwd": str(HERE),
                   "timeout_seconds": 180, "source_manifest": pin(attempt / "source-build.json")})
        result = call(command, HERE, stage / "build.stdout", stage / "build.stderr", 180, env)
        write_new(stage / "build-result.json", {"at": utc(), **result})
        if result["exit_code"] != 0:
            return result["exit_code"] or 1
        shutil.copyfile(built, frozen)
        frozen.chmod(0o555)
        for p in source_files:
            if sha(p.read_bytes()) != frozen_sources[str(p)]:
                raise SystemExit("Source changed during build: " + str(p))
        command = [str(frozen)] + (["--test-threads=1"] if test else [])
        precall = {"at": utc(), "command": command, "cwd": str(HERE),
                   "timeout_seconds": 60, "executable": pin(frozen),
                   "source_manifest": pin(attempt / "source-build.json"),
                   "sources_before_call": [pin(p) for p in source_files]}
        write_new(stage / "precall.json", precall)
        result = call(command, HERE, stage / "stdout.txt", stage / "stderr.txt", 60, env)
        after = {"at": utc(), **result, "executable": pin(frozen),
                 "stdout": pin(stage / "stdout.txt"), "stderr": pin(stage / "stderr.txt"),
                 "sources_after_call": [pin(p) for p in source_files],
                 "precall": pin(stage / "precall.json")}
        after["source_unchanged"] = all(sha(p.read_bytes()) == frozen_sources[str(p)]
                                        for p in source_files)
        after["executable_unchanged"] = after["executable"]["sha256"] == precall["executable"]["sha256"]
        write_new(stage / "postcall.json", after)
        if result["exit_code"] != 0 or not after["source_unchanged"] or not after["executable_unchanged"]:
            return result["exit_code"] or 1
    print(str(attempt))
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
