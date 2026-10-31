#!/usr/bin/env python3
"""Seal a one-target point disjoint from the fixed31 screen and holdout."""

import hashlib
import json
from pathlib import Path

from research.n83_highk_beyond4405_20261021 import audit_fresh
from research.n83_indexed5_20261013 import analyze as group

HERE = Path(__file__).resolve().parent
DOMAIN = b"n83-nearby-k-github-tier1-independent-20261021-v1"
CURVE_ID = "EC1N83Ckb1a1haacb69ee9f37"
SCALAR_RULE = "1+little_endian(SHA256(domain||u32_le(index))) mod (r-1)"
FIXED31 = {
    "screen": "e62e8ea5b77dbcbd80217b8f994ebb571fa03647355bfb5593077c58aedfdf11",
    "holdout": "a0e1f76ddec6ae901d5e766a942ad0b35299e10f175243fe7abd0fc679040a96",
}


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"),
                      ensure_ascii=False).encode()


def sha(blob):
    return hashlib.sha256(blob).hexdigest()


def write_once(path, blob):
    if path.exists() and path.read_bytes() != blob:
        raise ValueError(f"sealed fixture differs: {path}")
    path.write_bytes(blob)


def exclusion(point):
    prior_blob = audit_fresh.PRIOR_AUDIT.read_bytes()
    if sha(prior_blob) != audit_fresh.PRIOR_AUDIT_SHA256:
        raise ValueError("prior N83 exclusion inventory differs")
    old = json.loads(prior_blob)
    inventory = [("HEAD", row["path"], row["sha256"])
                 for row in old["prior_files"]] + list(audit_fresh.EXTRA)
    seen = set()
    checked = []
    for revision, path, expected in inventory:
        points = audit_fresh.verified_git_points(revision, path, expected)
        seen.update(points)
        checked.append({"revision": revision, "path": path,
                        "sha256": expected, "point_rows": len(points)})
    for kind, expected in FIXED31.items():
        path = audit_fresh.HERE / f"{kind}_targets.jsonl"
        blob = path.read_bytes()
        if sha(blob) != expected:
            raise ValueError(f"fixed31 {kind} fixture differs")
        seen.update(audit_fresh.point_rows(blob))
        checked.append({"revision": "HEAD", "path": str(path.relative_to(
            audit_fresh.ROOT)), "sha256": expected,
            "point_rows": len(audit_fresh.point_rows(blob))})
    report = {
        "schema": "n83.nearby-k-tier1-exclusion/v1",
        "prior_audit_sha256": sha(prior_blob),
        "prior_files": checked,
        "prior_unique_point_count": len(seen),
        "new_point": [str(x) for x in point],
        "overlap_count": int(point in seen),
        "status": "PASS_ZERO_OVERLAP" if point not in seen else "OVERLAP_FOUND",
    }
    if report["status"] != "PASS_ZERO_OVERLAP":
        raise ValueError("independent nearby-K point overlaps prior inventory")
    return report


def generate():
    seed = hashlib.sha256(DOMAIN + (0).to_bytes(4, "little")).digest()
    scalar = 1 + int.from_bytes(seed, "little") % (group.R - 1)
    point = group.multiply(group.G, scalar)
    if (point is None or not group.on_curve(point) or
            group.multiply(point, group.R) is not None):
        raise ValueError("new point fails subgroup replay")
    audit = exclusion(point)
    target = json.dumps([str(x) for x in point], separators=(",", ":")) + "\n"
    answer = json.dumps({"fixture_index": 0, "scalar": str(scalar),
                         "seed_sha256": seed.hex()},
                        separators=(",", ":"), sort_keys=True) + "\n"
    workload = {
        "schema": "ic-workload/v1", "curve_id": CURVE_ID,
        "subgroup_order": group.R, "target_input_law": SCALAR_RULE,
        "target_input_form": "public_encoded_point",
        "target_answer_sidecar_charged": False,
        "kind": "primary_single_target", "point_index": 0,
        "target_count": 1, "target_point": [str(x) for x in point],
        "target_stream_sha256": sha(target.encode()),
        "target_generation_seed_sha256": seed.hex(),
        "target_domain": DOMAIN.decode(),
        "cache_policy": "reusable_factor_base_index_logs_ready_before_online_clock",
    }
    workload["workload_id"] = sha(canonical(workload))[:12]
    audit_blob = json.dumps(audit, indent=2, sort_keys=True).encode() + b"\n"
    workload_blob = json.dumps(workload, indent=2, sort_keys=True).encode() + b"\n"
    manifest = {
        "schema": "n83.nearby-k-tier1-fixture/v1",
        "status": "SEALED_UNMEASURED", "domain": DOMAIN.decode(),
        "curve_id": CURVE_ID, "subgroup_order": group.R,
        "scalar_rule": SCALAR_RULE,
        "generator_sha256": sha(Path(__file__).read_bytes()),
        "target_sha256": sha(target.encode()),
        "answer_sha256": sha(answer.encode()),
        "exclusion_audit_sha256": sha(audit_blob),
        "workload_sha256": sha(workload_blob),
        "workload_id": workload["workload_id"],
    }
    write_once(HERE / "target.jsonl", target.encode())
    write_once(HERE / "answer.jsonl", answer.encode())
    write_once(HERE / "exclusion_audit.json", audit_blob)
    write_once(HERE / "workload.json", workload_blob)
    write_once(HERE / "fixture_manifest.json",
               json.dumps(manifest, indent=2, sort_keys=True).encode() + b"\n")
    return {"status": manifest["status"], "workload_id": workload["workload_id"],
            "target_sha256": manifest["target_sha256"],
            "exclusion_status": audit["status"],
            "prior_unique_point_count": audit["prior_unique_point_count"]}


if __name__ == "__main__":
    print(json.dumps(generate(), sort_keys=True))
