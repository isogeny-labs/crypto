"""Bounded child execution and fail-closed result classification."""

import argparse
import importlib.util
from pathlib import Path

import pytest


SOURCE = Path(__file__).with_name("run_access.py")
SPEC = importlib.util.spec_from_file_location("n83_local_access", SOURCE)
RUN = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(RUN)


def fixture(tmp_path, script, watchdog):
    solver = tmp_path / "solver"
    solver.write_text("#!/bin/sh\n" + script)
    solver.chmod(0o755)
    formula_dir = tmp_path / "formulas/hash_4_2"
    formula_dir.mkdir(parents=True)
    (formula_dir / "instance.xcnf").write_text("p cnf 1 1\n1 0\n")
    out = tmp_path / "out"
    out.mkdir()
    args = argparse.Namespace(solver=solver, out=out)
    return args, {"solver_flags": [], "process_watchdog_seconds": watchdog,
                  "xcnf_sha256": "fixture", "solver_binary_sha256": "fixture"}, formula_dir


def test_capped_unknown_keeps_raw_receipt_and_cannot_overwrite(tmp_path):
    args, protocol, formula_dir = fixture(
        tmp_path, "printf 'c conflicts                : 20001\ns INDETERMINATE\n'\nexit 15\n", 5)
    result = RUN.run_solver(args, protocol, formula_dir)
    assert (result["status"], result["conflicts"], result["exit_code"]) == (
        "INDETERMINATE", 20001, 15)
    assert (args.out / "solver.stdout").read_bytes()
    assert (args.out / "process.json").is_file()
    with pytest.raises(FileExistsError):
        RUN.run_solver(args, protocol, formula_dir)


def test_watchdog_is_censored_and_preserves_partial_stream(tmp_path):
    args, protocol, formula_dir = fixture(
        tmp_path, "printf 'c search began\n'\nsleep 10\n", 0.75)
    result = RUN.run_solver(args, protocol, formula_dir)
    assert result["status"] == "WATCHDOG" and result["conflicts"] is None
    assert result["exit_code"] != 0
    assert b"search began" in (args.out / "solver.stdout").read_bytes()
