#!/usr/bin/env python3
"""Independently replay the bounded N83 CryptoMiniSat configuration screen."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import re

DIMACS_SHA256 = "c62fe682879435e8f1ee30a80ac7830f2aef81dd4f600d10a004ac45a4f914b8"
CMS_SHA256 = "a3f85c3709b5e2a040bf82a4a604d1c7b9f10219bbf180a9e0f72319a2e892ac"
GAUSS = ["--maxmatrixrows", "8000", "--maxmatrixcols", "8000",
         "--maxnummatrices", "20", "--autodisablegauss", "0",
         "--gaussusefulcutoff", "0"]
FLAGS = {"default": [], "wide_gauss": GAUSS,
         "vsids": ["--branchstr", "vsids"],
         "wide_gauss_vsids": ["--branchstr", "vsids", *GAUSS]}


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def require(ok: bool, message: str) -> None:
    if not ok:
        raise ValueError(message)


def audit(dimacs: Path, cms: Path, raw: Path, script: Path) -> dict:
    require(sha256(dimacs.read_bytes()) == DIMACS_SHA256, "formula differs")
    require(sha256(cms.read_bytes()) == CMS_SHA256, "solver binary differs")
    receipt = json.loads((raw / "result.json").read_text())
    protocol = script.with_name("CONFIG_SCREEN_PROTOCOL.md")
    runner = script.with_name("run_config_screen.py")
    require(receipt["schema"] == "n83.m6-cms-config-screen-raw/v1"
            and receipt["status"] == "RAW_RESULTS_PENDING_INDEPENDENT_AUDIT"
            and receipt["dimacs_sha256"] == DIMACS_SHA256
            and receipt["cms_sha256"] == CMS_SHA256
            and receipt["protocol_sha256"] == sha256(protocol.read_bytes())
            and receipt["runner_sha256"] == sha256(runner.read_bytes())
            and receipt["conservative_envelopes_seconds"] == 600
            and receipt["selected_best_total_runtime"] is None
            and set(receipt["arms"]) == set(FLAGS), "frozen screen contract differs")
    clean = {}
    for name, flags in FLAGS.items():
        call = receipt["arms"][name]
        stdout = (raw / f"{name}.stdout").read_bytes()
        stderr = (raw / f"{name}.stderr").read_bytes()
        statuses = re.findall(rb"(?m)^s (SATISFIABLE|UNSATISFIABLE|INDETERMINATE|UNKNOWN)$",
                              stdout)
        rss = re.findall(rb"(?m)^\s*(\d+)\s+maximum resident set size$", stderr)
        expected_command = ["/usr/bin/time", "-l", str(cms), "--threads", "1",
                            "--verb", "0", "--printsol", "1", "--maxtime", "120",
                            "--maxconfl", "1000000", *flags, str(dimacs)]
        require(call["command"] == expected_command and call["flags"] == flags
                and call["stdout_sha256"] == sha256(stdout)
                and call["stderr_sha256"] == sha256(stderr)
                and 0 < call["wall_seconds_L0"] < 151,
                f"{name} command, hash or wall differs")
        if name.startswith("wide_gauss"):
            require(call["timed_out"] is True and call["exit_code"] == -9
                    and call["statuses"] == [] and statuses == []
                    and call["peak_rss_bytes_L0"] is None
                    and rss == [] and call["wall_seconds_L0"] >= 150,
                    f"{name} timeout receipt differs")
            disposition = "TIMEOUT_OUTER_CENSORED"
        else:
            require(call["timed_out"] is False and call["exit_code"] == 15
                    and call["statuses"] == ["INDETERMINATE"]
                    and statuses == [b"INDETERMINATE"]
                    and len(rss) == 1
                    and call["peak_rss_bytes_L0"] == int(rss[0]),
                    f"{name} censored solver receipt differs")
            disposition = "UNKNOWN_SOLVER_LIMIT_CENSORED"
        clean[name] = {"disposition": disposition,
                       "wall_seconds_L0": call["wall_seconds_L0"],
                       "peak_rss_bytes_L0": call["peak_rss_bytes_L0"],
                       "stdout_sha256": sha256(stdout),
                       "stderr_sha256": sha256(stderr)}
    raw_rows = [[path.name, sha256(path.read_bytes())] for path in sorted(raw.iterdir())
                if path.is_file() and path.name != "result.json"]
    require(raw_rows == receipt["raw_file_sha256"] and
            sha256(json.dumps(raw_rows, separators=(",", ":")).encode()) ==
            receipt["raw_tree_sha256"], "raw tree differs")
    return {"schema": "n83.m6-cms-config-screen-audit/v1",
            "status": "PASS_FOUR_CENSORED_ARMS",
            "dimacs_sha256": DIMACS_SHA256, "cms_sha256": CMS_SHA256,
            "protocol_sha256": receipt["protocol_sha256"],
            "runner_sha256": receipt["runner_sha256"],
            "raw_tree_sha256": receipt["raw_tree_sha256"],
            "conservative_envelopes_seconds": 600,
            "arms": clean, "selected_best_total_runtime": None}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--dimacs", type=Path, required=True)
    parser.add_argument("--cms", type=Path,
                        default=Path("/opt/homebrew/bin/cryptominisat5"))
    parser.add_argument("--raw", type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(audit(args.dimacs, args.cms, args.raw, Path(__file__)),
                     sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
