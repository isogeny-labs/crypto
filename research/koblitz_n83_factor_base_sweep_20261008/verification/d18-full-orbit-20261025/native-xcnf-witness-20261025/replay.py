"""Host-independent replay of a saved full-D18 native XCNF witness."""

import argparse
import base64
import gzip
import hashlib
import json
from pathlib import Path

import audit
import run


def read(path):
    return json.loads(path.read_text())


def replay(base, receipts, xcnf, model):
    base, receipts, xcnf, model = map(Path, (base, receipts, xcnf, model))
    obj = base / "objects" / run.OBJECT
    if obj.stat().st_size != 160_747_529 or run.sha(obj) != run.OBJECT_SHA:
        raise ValueError("factor-base object identity differs")
    wrapper_path = base / "s3-manifest.json"
    if run.sha(wrapper_path) != run.WRAPPER_SHA:
        raise ValueError("S3 wrapper identity differs")
    wrapper = read(wrapper_path)
    if (wrapper["compressed_sha256"] != run.OBJECT_SHA
            or wrapper["compressed_bytes"] != obj.stat().st_size
            or wrapper["producer_manifest_sha256"] != run.sha(base / "manifest.json")
            or wrapper["columns"] != 32_643
            or wrapper["points"] != 5_418_738):
        raise ValueError("S3 wrapper-to-producer binding differs")
    for name, digest, size in (
        ("object-get-metadata.json", run.OBJECT_SHA, obj.stat().st_size),
        ("manifest-get-metadata.json", run.WRAPPER_SHA, wrapper_path.stat().st_size),
    ):
        metadata = read(base / name)
        if (metadata["ContentLength"] != size
                or metadata["ChecksumType"] != "FULL_OBJECT"
                or metadata["ChecksumSHA256"] != base64.b64encode(bytes.fromhex(digest)).decode()
                or metadata["ServerSideEncryption"] != "AES256"):
            raise ValueError("S3 provider metadata differs")
    identity, result = read(receipts / "identity.json"), read(receipts / "result.json")
    supervisor = read(receipts / "supervisor.json")
    original_audit = read(receipts / "independent-audit.json")
    if (identity["source_commit"] != "d3a09099cb1f16c39981dce95786b2a857be38e4"
            or identity["object_sha256"] != run.OBJECT_SHA
            or identity["wrapper_sha256"] != run.WRAPPER_SHA
            or result["source_commit"] != identity["source_commit"]
            or result["binary_sha256"] != identity["binary_sha256"]
            or result["status"] != "PASS_NATIVE_XCNF_WITNESS"
            or result["solver_answer"] != "Sat"
            or result["shape"]["variables"] != audit.VARIABLES
            or result["shape"]["clauses"] != audit.prior.CLAUSES["positive"]
            or result["shape"]["xor_rows"] != audit.prior.XORS
            or result["model_bits"] != audit.VARIABLES
            or result["model_bytes"] != audit.MODEL_BYTES
            or result["selected_best_total_runtime"] is not None
            or result["xcnf_sha256"] != run.XCNF_SHA
            or result["xcnf_sha256"] != run.sha(xcnf)
            or result["xcnf_bytes"] != xcnf.stat().st_size
            or result["model_sha256"] != run.sha(model)
            or supervisor["source_commit"] != identity["source_commit"]
            or supervisor["binary_sha256"] != identity["binary_sha256"]
            or supervisor["exit_code"] != 0
            or supervisor["outer_cap_reached"]):
        raise ValueError("saved native witness identity or terminal receipt differs")
    values = audit.unpack(model)
    formula = audit.prior.check_formula(xcnf, values, "positive")
    xs = audit.prior.summand_x(values)
    target = tuple(map(int, result["target"]))
    group = audit.prior.replay_group(obj, xs, target, "positive")
    if (group["point_indices"] != result["relation_indices"]
            or group["rank_after_insert"] != 1
            or not group["altered_target_rejected"]):
        raise ValueError("saved witness original-field replay differs")
    flipped = values[:]
    flipped[1] ^= 1
    try:
        audit.prior.check_formula(xcnf, flipped, "positive")
    except ValueError:
        pass
    else:
        raise ValueError("flipped model passed")
    with gzip.open(obj, "rt") as stream:
        generator = tuple(map(int, json.loads(next(stream))["generator"]))
    try:
        audit.prior.replay_group(obj, xs, audit.prior.add(target, generator), "positive")
    except ValueError:
        pass
    else:
        raise ValueError("altered target passed")
    if hashlib.sha256(b"!" + xcnf.read_bytes()[1:]).hexdigest() == run.XCNF_SHA:
        raise ValueError("tampered XCNF hash matched")
    if (original_audit["status"] != "PASS_NATIVE_MODEL_SATISFIES_FROZEN_XCNF"
            or original_audit["model_sha256"] != result["model_sha256"]
            or original_audit["xcnf_sha256"] != result["xcnf_sha256"]
            or original_audit["point_indices"] != group["point_indices"]
            or original_audit["clauses_checked"] != formula["clauses_checked"]
            or original_audit["xor_rows_checked"] != formula["xor_rows_checked"]):
        raise ValueError("saved independent audit differs from replay")
    return {"schema": "n83.d18-native-xcnf-portable-replay/v1",
            "status": "PASS_SAVED_MODEL_AND_ORIGINAL_FIELD",
            "source_commit": identity["source_commit"],
            "object_sha256": run.OBJECT_SHA,
            "xcnf_sha256": run.XCNF_SHA,
            "model_sha256": result["model_sha256"],
            **formula, **group,
            "flipped_assignment_rejected": True,
            "altered_xcnf_digest_rejected": True,
            "selected_best_total_runtime": None}


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    for name in ("base", "receipts", "xcnf", "model"):
        parser.add_argument("--" + name, required=True)
    args = parser.parse_args()
    print(json.dumps(replay(args.base, args.receipts, args.xcnf, args.model),
                     sort_keys=True, indent=2))
