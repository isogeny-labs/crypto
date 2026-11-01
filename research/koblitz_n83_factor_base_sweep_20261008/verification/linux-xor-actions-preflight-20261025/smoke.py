#!/usr/bin/env python3
"""Qualify Linux CryptoMiniSat flags without making an N83 relation claim."""

import argparse
import hashlib
import json
from pathlib import Path
import re
import shutil
import subprocess
import tempfile


FLAGS = [
    "--threads", "1", "--random", "0", "--branchstr", "vmtf+vsids",
    "--autodisablegauss", "0", "--maxconfl", "20", "--maxtime", "30",
    "--maxmatrixcols", "25000", "--maxnummatrices", "10", "--verb", "1",
    "--verbstat", "1", "--printsol", "1",
]


def smoke(out: Path) -> dict:
    solver_name = shutil.which("cryptominisat5") or shutil.which("cryptominisat")
    if solver_name is None:
        raise RuntimeError("CryptoMiniSat executable missing")
    solver = Path(solver_name).resolve()
    out.mkdir(parents=True, exist_ok=True)
    package = subprocess.run(
        ["dpkg-query", "-W", "-f=${Package}=${Version}\n", "cryptominisat"],
        capture_output=True, text=True, check=True, timeout=30,
    )
    (out / "package.txt").write_text(package.stdout)
    version = subprocess.run([str(solver), "--version"], capture_output=True,
                             timeout=30)
    (out / "solver-version.stdout").write_bytes(version.stdout)
    (out / "solver-version.stderr").write_bytes(version.stderr)
    with tempfile.TemporaryDirectory(prefix="n83-linux-xor-smoke-") as directory:
        formula = Path(directory) / "smoke.xcnf"
        formula.write_bytes(b"p cnf 2 1\nx1 2 0\n")
        result = subprocess.run([str(solver), *FLAGS, str(formula)],
                                capture_output=True, timeout=45)
    (out / "smoke.stdout").write_bytes(result.stdout)
    (out / "smoke.stderr").write_bytes(result.stderr)
    receipt = {
        "schema": "n83.linux-xor-actions-preflight/v1",
        "solver_path": str(solver),
        "solver_sha256": hashlib.sha256(solver.read_bytes()).hexdigest(),
        "package": package.stdout.strip(),
        "version_exit_code": version.returncode,
        "version_stdout_sha256": hashlib.sha256(version.stdout).hexdigest(),
        "version_stderr_sha256": hashlib.sha256(version.stderr).hexdigest(),
        "flags": FLAGS,
        "smoke_formula_sha256": hashlib.sha256(b"p cnf 2 1\nx1 2 0\n").hexdigest(),
        "smoke_exit_code": result.returncode,
        "smoke_stdout_sha256": hashlib.sha256(result.stdout).hexdigest(),
        "smoke_stderr_sha256": hashlib.sha256(result.stderr).hexdigest(),
        "smoke_sat_line": bool(re.search(rb"(?m)^s SATISFIABLE\s*$", result.stdout)),
        "selected_best_total_runtime": None,
    }
    (out / "smoke.json").write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n")
    if version.returncode != 0 or result.returncode != 10 or not receipt["smoke_sat_line"]:
        raise RuntimeError("Linux XOR solver or flag smoke failed; retained raw output")
    return receipt


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(smoke(args.out), indent=2, sort_keys=True))
