"""Duplicate-safe receipts and bounded process handling for the XOR panel."""

import argparse
import hashlib
import importlib.util
import pathlib

import pytest


SOURCE = pathlib.Path(__file__).with_name("run_panel.py")
SPEC = importlib.util.spec_from_file_location("n83_xor_panel", SOURCE)
PANEL = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PANEL)


def fixture(tmp_path, program: str, watchdog: float = 5.0):
    solver = tmp_path / "solver"
    solver.write_text("#!/bin/sh\n" + program)
    solver.chmod(0o755)
    formula_dir = tmp_path / "formulas/hash_5_1"
    formula_dir.mkdir(parents=True)
    formula = formula_dir / "instance.xcnf"
    formula.write_text("p cnf 1 1\n1 0\n")
    output = tmp_path / "output"
    output.mkdir()
    args = argparse.Namespace(out=output, solver=solver,
                              formulas_dir=tmp_path / "formulas",
                              archives_dir=tmp_path / "archives")
    digest = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
    protocol = {"inputs": {"hash_5_1": {"xcnf_sha256": digest(formula),
                                           "pool_object_blake3": "pool"}},
                "base_object_blake3": "base",
                "solver_binary_sha256": digest(solver), "common_solver_flags": [],
                "process_watchdog_seconds": watchdog}
    arm = {"id": "hash_A1", "input": "hash_5_1", "branchstr": "vsids",
           "autodisablegauss": 1}
    return args, protocol, arm


def test_completed_arm_replays_and_refuses_changed_or_duplicate_output(tmp_path):
    args, protocol, arm = fixture(
        tmp_path, "printf 'c conflicts                : 1000000\\ns INDETERMINATE\\n'\nexit 15\n")
    receipt = PANEL.run_one(args, protocol, arm)
    assert receipt["status"] == "INDETERMINATE"
    assert receipt["conflicts"] == 1000000
    assert PANEL.replay_completed(args, protocol, arm) == receipt
    with pytest.raises(FileExistsError):
        PANEL.run_one(args, protocol, arm)
    stdout = args.out / arm["id"] / "solver.stdout"
    stdout.write_bytes(stdout.read_bytes().replace(b"INDETERMINATE", b"SATISFIABLE"))
    with pytest.raises(ValueError, match="raw output changed"):
        PANEL.replay_completed(args, protocol, arm)


def test_watchdog_preserves_censored_receipt(tmp_path):
    args, protocol, arm = fixture(tmp_path, "sleep 10\n", watchdog=0.1)
    receipt = PANEL.run_one(args, protocol, arm)
    assert receipt["status"] == "WATCHDOG"
    assert receipt["conflicts"] is None
    assert PANEL.replay_completed(args, protocol, arm) == receipt
