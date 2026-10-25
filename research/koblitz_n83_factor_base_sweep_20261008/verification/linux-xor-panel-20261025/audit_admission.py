#!/usr/bin/env python3
"""Replay downloaded N83 XCNF bytes and the bounded provider admission record."""

import argparse
import base64
import hashlib
import json
from pathlib import Path


HERE = Path(__file__).resolve().parent
PREVIOUS = HERE.parent / "external-xor-20261025" / "receipt.json"
EXPECTED_NAMES = {"hash_5_1", "gray_5_1", "hash_4_2", "gray_4_2"}


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def audit(downloads: Path) -> dict:
    admission = json.loads((HERE / "admission.json").read_bytes())
    previous = json.loads(PREVIOUS.read_bytes())
    require(admission["schema"] == "n83.linux-xor-panel-admission/v1", "admission schema")
    require(admission["curve"] == previous["curve_slug"], "curve identity")
    require(admission["status"] == "PROVIDER_ACCOUNT_BLOCKED"
            and admission["selected_best_total_runtime"] is None
            and admission["solver_runs"] == [], "censored admission state")
    require(admission["aws"]["dry_run_response"] == "DryRunOperation"
            and admission["aws"]["launch_response_code"] == "Blocked"
            and admission["aws"]["allocated_instance_ids"] == [],
            "provider admission classification")
    require(admission["s3"]["input_put_condition"] == "If-None-Match: *"
            and admission["s3"]["server_side_encryption"] == "AES256",
            "S3 custody policy")
    inputs = admission["s3"]["inputs"]
    require(set(inputs) == EXPECTED_NAMES
            and {arm["name"] for arm in previous["arms"]} == EXPECTED_NAMES,
            "four-arm identity")
    previous_by_name = {arm["name"]: arm for arm in previous["arms"]}
    replayed = []
    for name in sorted(inputs):
        spec = inputs[name]
        raw = (downloads / f"{name}.xcnf").read_bytes()
        digest = hashlib.sha256(raw).digest()
        require(len(raw) == spec["bytes"], f"{name}: byte length")
        require(digest.hex() == spec["sha256"] == previous_by_name[name]["xcnf_sha256"],
                f"{name}: source/XCNF digest")
        require(base64.b64encode(digest).decode("ascii") == spec["s3_checksum_sha256_base64"],
                f"{name}: S3 checksum")
        replayed.append({"arm": name, "bytes": len(raw), "sha256": digest.hex()})

    prefix = f"arn:aws:s3:::{admission['s3']['bucket']}/{admission['s3']['prefix']}"
    policy = json.loads((HERE / "s3-policy.json").read_bytes())
    require(policy["Version"] == "2012-10-17" and len(policy["Statement"]) == 2,
            "IAM policy shape")
    permissions = {(statement["Effect"], statement["Action"], statement["Resource"])
                   for statement in policy["Statement"]}
    require(permissions == {
        ("Allow", "s3:GetObject", prefix + "inputs/*"),
        ("Allow", "s3:PutObject", prefix + "outputs/*"),
    }, "IAM policy scope")
    trust = json.loads((HERE / "ec2-trust.json").read_bytes())
    require(trust == {"Version": "2012-10-17", "Statement": [{
        "Effect": "Allow", "Principal": {"Service": "ec2.amazonaws.com"},
        "Action": "sts:AssumeRole"}]}, "EC2 trust scope")
    user_data = (HERE / "ec2-user-data.sh").read_text()
    require("shutdown -h +90" in user_data
            and admission["aws"]["shutdown_minutes"] == 90
            and admission["aws"]["instance_initiated_shutdown_behavior"] == "terminate",
            "intended worker deadline")
    return {
        "schema": "n83.linux-xor-panel-admission-replay/v1",
        "status": "PASS_BOUNDED_PROVIDER_ADMISSION",
        "curve": admission["curve"],
        "downloaded_inputs": replayed,
        "provider_response": admission["aws"]["launch_response_code"],
        "solver_runs": 0,
        "selected_best_total_runtime": None,
        "limits": [
            "Provider error is transcribed from the CLI response, not independently fetched from AWS",
            "Local S3 download replay proves object bytes at observation time, not S3 object-lock immutability"
        ],
    }


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--downloads-dir", type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(audit(args.downloads_dir), indent=2, sort_keys=True))
