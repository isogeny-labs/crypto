#!/usr/bin/env python3
"""Run two high-K cancellation feasibility controls on one exposed N83 point."""

import argparse
import hashlib
import json
import shutil
from pathlib import Path

from research.n83_target_cancel_tier1_repeat_20261017 import run as prior

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "examples/koblitz_orbit_dlp_fast.rs"
TARGET = ROOT / "research/n83_cold_factor_base_20261016/targets.jsonl"
ANSWER = ROOT / "research/n83_cold_factor_base_20261016/answers.jsonl"
EXPECTED = {
    SOURCE: "8109fb33374ce2aa66c6a5a0b6e144d3b0143168ac756f9e0bd70af60a911829",
    TARGET: "2c068d2f659588edcdbaa685fc73bb146e2bfee3be123d377d537658f8021c6d",
    ANSWER: "b71c5a8bfdae603931fa6c03fc6f6faa3b261288cf787ac682bf8a3e3afe262d",
}
ARMS = (("k2816_cancel", 2816), ("k3072_cancel", 3072))
CAP_SECONDS = 900


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def preflight(binary):
    for path, expected in EXPECTED.items():
        if sha256(path) != expected:
            raise ValueError(f"frozen input changed: {path.relative_to(ROOT)}")
    if not binary.is_file():
        raise ValueError(f"missing release binary: {binary}")
    if not (ROOT / "Cargo.lock").is_file():
        raise ValueError("Cargo.lock missing")
    return {"source_sha256": EXPECTED[SOURCE],
            "target_sha256": EXPECTED[TARGET],
            "answer_sha256": EXPECTED[ANSWER],
            "binary_sha256": sha256(binary),
            "cargo_lock_sha256": sha256(ROOT / "Cargo.lock")}


def run(binary, output):
    source = preflight(binary)
    if output.exists():
        raise ValueError(f"output already exists: {output}")
    output.mkdir(parents=True)
    executed = output / "executed_bin"
    shutil.copy2(binary, executed)
    if sha256(executed) != source["binary_sha256"]:
        raise ValueError("copied binary differs")
    source.update({"schema": "n83.highk-cancel-pilot-source/v1",
                   "runner_sha256": sha256(Path(__file__)),
                   "arms": [list(arm) for arm in ARMS],
                   "cap_seconds": CAP_SECONDS})
    prior.parent.write_json(output / "source.json", source)
    group = prior.parent.cgroup_root()
    prior.parent.write_json(output / "host.json", prior.parent.host_receipt(group))
    for name, k in ARMS:
        folder = output / "runs" / name
        environment = prior.ic_environment(folder, "root11_dynamic",
                                            "parallel_cancel")
        if (environment["KIC_TARGET_THREADS"] != "2" or
                environment["KIC_TARGET_CANCEL"] != "1"):
            raise ValueError("cancellation flags differ")
        command = [str(executed), f"construct:83:1:{k}", str(TARGET), "29",
                   str(folder / "targets_result.jsonl")]
        usage = prior.parent.one_run(folder, command, environment,
                                     CAP_SECONDS, group)
        print(json.dumps({"arm": name, "status": usage["status"],
                          "wall_seconds": usage["wall_seconds"]},
                         sort_keys=True), flush=True)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--preflight", action="store_true")
    args = parser.parse_args()
    binary = args.binary.resolve()
    if args.preflight:
        print(json.dumps(preflight(binary), sort_keys=True))
    else:
        run(binary, args.output.resolve())


if __name__ == "__main__":
    main()
