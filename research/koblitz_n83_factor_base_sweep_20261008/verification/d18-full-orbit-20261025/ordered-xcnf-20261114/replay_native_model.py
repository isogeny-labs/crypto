#!/usr/bin/env python3
"""Replay the archived native assignment against a fresh original D18 XCNF."""

import argparse
import hashlib
import importlib.util
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
PREVIOUS = HERE.parent / "native-cadence-replay-20261026/attempt-7/evidence/positive-2"
VERIFIER = HERE.parent / "external-cms-20261114/verify_xcnf_model.py"
MODEL_SHA256 = "e41a2df1bd8cbf940f04797318f071045bb547e7270991f92f2155979920ee3f"
TARGET = ["6594052263204820946437856", "9593508870534830450764009"]
VARS = 329808
CNF = 2807999
XOR = 3901


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def require(ok, message):
    if not ok:
        raise ValueError(message)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("run_dir", type=Path)
    args = parser.parse_args()
    run_dir = args.run_dir.resolve()
    freeze = json.loads((run_dir / "FREEZE.json").read_text())
    receipt_path = run_dir / "export-positive/result.json"
    receipt = json.loads(receipt_path.read_text())
    old = json.loads((PREVIOUS / "result.json").read_text())
    xcnf = run_dir / "export-positive/result.xcnf"
    model_path = PREVIOUS / "result.model.bin"
    require(freeze["schema"] == "n83.d18-ordered-xcnf-freeze/v1", "freeze schema differs")
    require(receipt["status"] == "PASS_XCNF_EXPORT_ONLY", "export status differs")
    require(receipt["source_commit"] == freeze["source_commit"], "source commit differs")
    require(receipt["binary_sha256"] == freeze["binary_sha256"], "binary differs")
    require(receipt["target"] == TARGET == old["target"], "target differs")
    require(receipt["solver_search_executed"] is False, "export ran a solver")
    shape = receipt["shape"]
    require(shape["variables"] == VARS and shape["clauses"] == CNF and shape["xor_rows"] == XOR,
            "formula shape differs")
    require(shape["ordered_first_pair"] is True and shape["xor_decision_cadence"] == 32,
            "ordered policy differs")
    require(shape["root_encoding"] == "shared_root_bits" and shape["choice_pin_policy"] == "none",
            "domain policy differs")
    require(shape["search_policy"] == "phase_zero_first_ordered_first_pair_xor_cadence32",
            "search policy differs")
    require(sha256(xcnf) == receipt["xcnf_sha256"], "XCNF bytes differ")
    require(xcnf.stat().st_size == receipt["xcnf_bytes"], "XCNF length differs")
    require(sha256(model_path) == MODEL_SHA256 == old["model_certificate"]["sha256"],
            "native model differs")
    packed = model_path.read_bytes()
    require(len(packed) * 8 == VARS, "native assignment length differs")
    values = bytearray(VARS + 1)
    for index in range(VARS):
        values[index + 1] = 1 if (packed[index // 8] >> (index % 8)) & 1 else 2

    spec = importlib.util.spec_from_file_location("n83_original_xcnf_verifier", VERIFIER)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    module.selftest()
    cnf_count, xor_count = module.verify_formula(xcnf, values, CNF + XOR)
    require((cnf_count, xor_count) == (CNF, XOR), "constraint counts differ")
    report = {
        "schema": "n83.d18-ordered-xcnf-native-model-replay/v1",
        "status": "PASS_ORIGINAL_XCNF_REPLAY",
        "source_commit": freeze["source_commit"],
        "binary_sha256": freeze["binary_sha256"],
        "receipt_sha256": sha256(receipt_path),
        "xcnf_sha256": receipt["xcnf_sha256"],
        "model_sha256": MODEL_SHA256,
        "checked_variables": VARS,
        "checked_cnf_clauses": cnf_count,
        "checked_xor_rows": xor_count,
        "target": TARGET,
        "native_group_certificate": old["status"],
        "selected_best_total_runtime": None,
    }
    with (run_dir / "replay.json").open("x") as output:
        json.dump(report, output, sort_keys=True, indent=2)
        output.write("\n")
    print(json.dumps({"status": report["status"], "cnf": cnf_count, "xor": xor_count}, sort_keys=True))


if __name__ == "__main__":
    main()
