#!/usr/bin/env python3
"""Run the frozen two-by-two CryptoMiniSat access screen on N83 XOR-DIMACS."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import signal
import subprocess
import time

DIMACS_SHA256 = "c62fe682879435e8f1ee30a80ac7830f2aef81dd4f600d10a004ac45a4f914b8"
CMS_SHA256 = "a3f85c3709b5e2a040bf82a4a604d1c7b9f10219bbf180a9e0f72319a2e892ac"
GAUSS = ["--maxmatrixrows", "8000", "--maxmatrixcols", "8000",
         "--maxnummatrices", "20", "--autodisablegauss", "0",
         "--gaussusefulcutoff", "0"]
ARMS = [("default", []), ("wide_gauss", GAUSS),
        ("vsids", ["--branchstr", "vsids"]),
        ("wide_gauss_vsids", ["--branchstr", "vsids", *GAUSS])]


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def run(cms: Path, dimacs: Path, output: Path, name: str, flags: list[str]) -> dict:
    command = ["/usr/bin/time", "-l", str(cms), "--threads", "1",
               "--verb", "0", "--printsol", "1", "--maxtime", "120",
               "--maxconfl", "1000000", *flags, str(dimacs)]
    stdout_path = output / f"{name}.stdout"
    stderr_path = output / f"{name}.stderr"
    started = time.monotonic()
    with stdout_path.open("xb") as stdout, stderr_path.open("xb") as stderr:
        process = subprocess.Popen(command, stdout=stdout, stderr=stderr,
                                   start_new_session=True)
        try:
            exit_code = process.wait(timeout=150)
            timed_out = False
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGKILL)
            process.wait()
            exit_code = process.returncode
            timed_out = True
    wall = time.monotonic() - started
    stdout = stdout_path.read_bytes()
    stderr = stderr_path.read_bytes()
    statuses = re.findall(rb"(?m)^s (SATISFIABLE|UNSATISFIABLE|INDETERMINATE|UNKNOWN)$",
                          stdout)
    rss = re.findall(rb"(?m)^\s*(\d+)\s+maximum resident set size$", stderr)
    return {"command": command, "flags": flags,
            "exit_code": exit_code, "timed_out": timed_out,
            "statuses": [value.decode() for value in statuses],
            "wall_seconds_L0": wall,
            "peak_rss_bytes_L0": int(rss[-1]) if rss else None,
            "stdout_sha256": hashlib.sha256(stdout).hexdigest(),
            "stderr_sha256": hashlib.sha256(stderr).hexdigest()}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--dimacs", type=Path, required=True)
    parser.add_argument("--cms", type=Path, default=Path("/opt/homebrew/bin/cryptominisat5"))
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    if (sha256(args.dimacs) != DIMACS_SHA256 or
            sha256(args.cms) != CMS_SHA256 or
            args.dimacs.stat().st_size != 11_641_543):
        raise ValueError("frozen input or solver identity differs")
    args.output.mkdir(parents=True, exist_ok=False)
    receipt = {"schema": "n83.m6-cms-config-screen-raw/v1",
               "status": "RAW_RESULTS_PENDING_INDEPENDENT_AUDIT",
               "dimacs_sha256": DIMACS_SHA256, "cms_sha256": CMS_SHA256,
               "protocol_sha256": sha256(Path(__file__).with_name("CONFIG_SCREEN_PROTOCOL.md")),
               "runner_sha256": sha256(Path(__file__)),
               "arms": {}, "conservative_envelopes_seconds": 600,
               "selected_best_total_runtime": None}
    for name, flags in ARMS:
        receipt["arms"][name] = run(args.cms, args.dimacs, args.output, name, flags)
        print(json.dumps({"arm": name, **receipt["arms"][name]}, sort_keys=True), flush=True)
    raw_rows = [[path.name, sha256(path)] for path in sorted(args.output.iterdir())
                if path.is_file()]
    receipt["raw_file_sha256"] = raw_rows
    receipt["raw_tree_sha256"] = hashlib.sha256(
        json.dumps(raw_rows, separators=(",", ":")).encode()).hexdigest()
    with (args.output / "result.json").open("x") as output:
        json.dump(receipt, output, sort_keys=True, indent=2)
        output.write("\n")
    print(json.dumps({"raw_tree_sha256": receipt["raw_tree_sha256"],
                      "status": receipt["status"]}, sort_keys=True))


if __name__ == "__main__":
    main()
