"""Independent all-clause/XOR and original-field replay of a native model."""

import argparse
import gzip
import hashlib
import importlib.util
import json
from pathlib import Path
import sys

import run

HERE = Path(__file__).resolve().parent
PRIOR = HERE.parent / "local-xcnf-20261025"
sys.path.insert(0, str(PRIOR))
SPEC = importlib.util.spec_from_file_location("prior_xcnf_audit", PRIOR / "audit.py")
prior = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(prior)
sys.path.pop(0)
VARIABLES = 329_726
MODEL_BYTES = (VARIABLES + 7) // 8


def read(path):
    return json.loads(path.read_text())


def unpack(path):
    packed = path.read_bytes()
    if len(packed) != MODEL_BYTES or packed[-1] & 0b1100_0000:
        raise ValueError("packed native model length or pad bits differ")
    values = bytearray(VARIABLES + 1)
    for index in range(VARIABLES):
        values[index + 1] = (packed[index // 8] >> (index % 8)) & 1
    return values


def audit(args):
    source, binary, base, output = (Path(value).resolve() for value in
                                    (args.source, args.binary, args.base, args.output))
    identity = run.preflight(source, binary, base)
    if read(output / "identity.json") != identity:
        raise ValueError("source, binary, S3 or host identity differs")
    arm = output / "witness"
    result, receipt = read(arm / "result.json"), read(arm / "supervisor.json")
    xcnf, model = arm / "result.xcnf", arm / "result.model.bin"
    if (result["schema"] != "n83.full-d18-m4-l0-xcnf-native-witness/v1"
            or result["status"] != "PASS_NATIVE_XCNF_WITNESS"
            or result["source_commit"] != identity["source_commit"]
            or result["source_attestation_mode"] != "clean_checkout"
            or result["binary_sha256"] != identity["binary_sha256"]
            or result["xcnf_sha256"] != run.XCNF_SHA
            or result["xcnf_sha256"] != run.sha(xcnf)
            or result["model_sha256"] != run.sha(model)
            or result["model_bits"] != VARIABLES
            or result["model_bytes"] != MODEL_BYTES
            or result["shape"]["variables"] != VARIABLES
            or result["shape"]["clauses"] != prior.CLAUSES["positive"]
            or result["shape"]["xor_rows"] != prior.XORS
            or result["solver_answer"] != "Sat"
            or result["selected_best_total_runtime"] is not None):
        raise ValueError("native result identity or shape differs")
    if (receipt["source_commit"] != identity["source_commit"]
            or receipt["binary_sha256"] != identity["binary_sha256"]
            or receipt["outer_cap_seconds"] != 310
            or receipt["outer_cap_reached"]
            or receipt["exit_code"] != 0):
        raise ValueError("supervisor source or terminal status differs")
    phases = [json.loads(line)["phase"] for line in
              (arm / "result.json.phases.jsonl").read_text().splitlines()]
    if phases != ["started", "import_checked", "model_constructed",
                  "xcnf_exported", "solver_returned", "native_model_verified"]:
        raise ValueError("native worker phase sequence differs")
    values = unpack(model)
    formula = prior.check_formula(xcnf, values, "positive")
    xs = prior.summand_x(values)
    target = tuple(map(int, result["target"]))
    group = prior.replay_group(base / "objects" / run.OBJECT, xs, target, "positive")
    if (group["point_indices"] != result["relation_indices"]
            or group["rank_after_insert"] != result["rank_after_insert"]
            or group["rank_after_insert"] != 1
            or not result["altered_target_rejected"]):
        raise ValueError("native row, rank or group replay differs")
    flipped = values[:]
    flipped[1] ^= 1
    try:
        prior.check_formula(xcnf, flipped, "positive")
    except ValueError:
        flip_rejected = True
    else:
        raise ValueError("flipped native model passed the original XCNF")
    with gzip.open(base / "objects" / run.OBJECT, "rt") as stream:
        header = json.loads(next(stream))
    generator = tuple(map(int, header["generator"]))
    try:
        prior.replay_group(base / "objects" / run.OBJECT, xs,
                           prior.add(target, generator), "positive")
    except ValueError:
        altered_target_rejected = True
    else:
        raise ValueError("altered target passed original-field replay")
    tampered_digest = hashlib.sha256(b"!" + xcnf.read_bytes()[1:]).hexdigest()
    if tampered_digest == run.XCNF_SHA:
        raise ValueError("altered XCNF digest unexpectedly matched")
    record = {"schema": "n83.d18-native-xcnf-independent-audit/v1",
              "status": "PASS_NATIVE_MODEL_SATISFIES_FROZEN_XCNF",
              "source_commit": identity["source_commit"],
              "binary_sha256": identity["binary_sha256"],
              "object_sha256": run.OBJECT_SHA,
              "wrapper_sha256": run.WRAPPER_SHA,
              "xcnf_sha256": run.XCNF_SHA,
              "model_sha256": result["model_sha256"],
              "model_bits": VARIABLES, **formula,
              "summand_x": list(map(str, xs)), **group,
              "flipped_assignment_rejected": flip_rejected,
              "altered_target_rejected": altered_target_rejected,
              "altered_xcnf_digest_rejected": True,
              "supervisor_wall_seconds": receipt["elapsed_wall_seconds"],
              "maximum_resident_set_bytes": receipt["maximum_resident_set_bytes"],
              "natural_search_executed": False,
              "selected_best_total_runtime": None}
    with (output / "independent-audit.json").open("x") as stream:
        json.dump(record, stream, sort_keys=True, indent=2)
        stream.write("\n")
    return record


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    for name in ("source", "binary", "base", "output"):
        parser.add_argument("--" + name, required=True)
    print(json.dumps(audit(parser.parse_args()), sort_keys=True, indent=2))
