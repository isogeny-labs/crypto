#!/usr/bin/env python3
"""Reduce existing IsoLab limits; default is server dry-run, never provision nodes."""
import argparse
from decimal import Decimal
import json
from pathlib import Path
import re
import subprocess

HERE = Path(__file__).resolve().parent


def quantity(value):
    match = re.fullmatch(r"(\d+(?:\.\d+)?)([KMGTPE]i|[kKMGTPE]|m|[eE][+-]?\d+)?", str(value))
    if not match:
        raise ValueError(f"Unsupported quantity: {value!r}; review without changing limits")
    number, unit = match.groups()
    if not unit:
        factor = Decimal(1)
    elif unit == "m":
        factor = Decimal("0.001")
    elif len(unit) > 1 and unit[0] in "eE":
        factor = Decimal(10) ** int(unit[1:])
    elif unit.endswith("i"):
        factor = Decimal(1024) ** ("KMGTPE".index(unit[0]) + 1)
    else:
        factor = Decimal(1000) ** ("KMGTPE".index(unit.upper()) + 1)
    return Decimal(number) * factor


def reduction_patch(obj, field, desired):
    current = obj["spec"].get(field, {})
    updated = dict(current)
    for key, value in desired.items():
        # A smaller live cap always wins, including zero.
        updated[key] = current[key] if key in current and quantity(current[key]) <= quantity(value) else value
    return [
        {"op": "test", "path": "/metadata/resourceVersion", "value": obj["metadata"]["resourceVersion"]},
        {"op": "add", "path": f"/spec/{field}", "value": updated},
    ]


def plan(pools, quota, policy):
    names = {p["metadata"]["name"] for p in pools["items"]}
    unknown = names - policy["nodepools"].keys()
    missing = set(policy["required_nodepools"]) - names
    if unknown or missing:
        raise ValueError(f"Review pool inventory first: unknown={sorted(unknown)}, missing={sorted(missing)}")
    changes = []
    for obj in pools["items"]:
        if "replicas" in obj["spec"]:
            raise ValueError("Static NodePool needs separate review; no changes applied")
        name = obj["metadata"]["name"]
        changes.append((["nodepool", name], reduction_patch(obj, "limits", policy["nodepools"][name])))
    changes.append((["resourcequota", "jobs", "-n", "isolab"], reduction_patch(quota, "hard", policy["quota"])))
    return changes


def kubectl(*args):
    return json.loads(subprocess.check_output(["kubectl", "--request-timeout=20s", *args, "-o", "json"], text=True))


def save(directory, name, obj):
    with (directory / name).open("x") as stream:
        json.dump(obj, stream, indent=2)
        stream.write("\n")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--apply", action="store_true")
    parser.add_argument("--output", type=Path, required=True, help="new receipt directory")
    args = parser.parse_args()
    # Bind the selected kubeconfig to the real cluster endpoint, not a context nickname.
    endpoint = subprocess.check_output([
        "aws", "eks", "describe-cluster", "--name", "isolab", "--region", "us-east-2",
        "--query", "cluster.endpoint", "--output", "text"], text=True).strip()
    account = subprocess.check_output(["aws", "sts", "get-caller-identity", "--query", "Account", "--output", "text"], text=True).strip()
    config = kubectl("config", "view", "--minify")
    if account != "104186811093" or config["clusters"][0]["cluster"]["server"] != endpoint:
        raise ValueError("Wrong AWS account or Kubernetes endpoint")
    args.output.mkdir(parents=True, exist_ok=False)
    policy = json.loads((HERE / "cost-controls.json").read_text())
    pools = kubectl("get", "nodepools.karpenter.sh")
    quota = kubectl("get", "resourcequota", "jobs", "-n", "isolab")
    save(args.output, "nodepools-before.json", pools)
    save(args.output, "quota-before.json", quota)
    changes = plan(pools, quota, policy)
    save(args.output, "plan.json", changes)
    # Authorization/admission errors must occur before the first mutation.
    for target, patch in changes:
        kubectl("patch", *target, "--type=json", "-p", json.dumps(patch), "--dry-run=server")
    if not args.apply:
        print("All reductions passed server dry-run; no live configuration changed.")
        return
    for target, patch in changes:
        # No blind retry: resourceVersion changes require a fresh inventory.
        result = kubectl("patch", *target, "--type=json", "-p", json.dumps(patch))
        save(args.output, f"{target[0]}-{target[1]}-applied.json", result)
        result = kubectl("get", *target)
        field = "limits" if target[0] == "nodepool" else "hard"
        for key, value in patch[-1]["value"].items():
            if quantity(result["spec"][field][key]) > quantity(value):
                raise ValueError(f"Readback exceeds requested limit: {target} {key}")
        save(args.output, f"{target[0]}-{target[1]}-readback.json", result)
    print("Verified live limit reductions. Existing nodes are not evicted; inspect actual capacity separately.")


if __name__ == "__main__":
    main()
