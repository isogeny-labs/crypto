#!/usr/bin/env python3
"""Bind one exact previously audited N83 XCNF to its provider ZIP and S3 lineage."""

import argparse
import hashlib
import json
from pathlib import Path
import zipfile


HERE = Path(__file__).resolve().parent
PRIOR = HERE.parent / "natural-yield-panel-20261025"


def require(ok: bool, reason: str) -> None:
    if not ok:
        raise ValueError(reason)


def sha(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def fetch(protocol_path: Path, stratum_id: str, archive: Path,
          metadata_path: Path, out: Path) -> dict:
    protocol = json.loads(protocol_path.read_bytes())
    matches = [row for row in protocol["strata"] if row["id"] == stratum_id]
    require(len(matches) == 1, "one frozen stratum")
    row = matches[0]
    require(protocol["schema"] == "n83.solver-setting-screen-protocol/v1"
            and protocol["selected_best_total_runtime"] is None,
            "frozen solver-setting protocol")
    require(sha((PRIOR / "protocol.json").read_bytes())
            == protocol["source_protocol_sha256"], "prior frozen protocol")
    audit_path = PRIOR / "run-40185426122-attempt-1" / f"audit-t{row['trial']}_m0.json"
    require(sha(audit_path.read_bytes()) == row["source_audit_json_sha256"],
            "committed source artifact audit")
    audit = json.loads(audit_path.read_bytes())
    require(audit["status"] == "PASS_HOSTED_STRATUM_REPLAY"
            and audit["artifact_id"] == row["source_artifact_id"]
            and audit["artifact_zip_sha256"] == row["source_artifact_sha256"]
            and audit["run_id"] == protocol["source_run_id"]
            and audit["attempt"] == protocol["source_run_attempt"],
            "source provider replay")
    metadata_raw = metadata_path.read_bytes()
    metadata = json.loads(metadata_raw)
    raw = archive.read_bytes()
    require(metadata["id"] == row["source_artifact_id"]
            and metadata["name"] == f"n83-natural-pattern-t{row['trial']}_m0-40185426122-1"
            and metadata["workflow_run"]["id"] == protocol["source_run_id"]
            and metadata["workflow_run"]["head_sha"] == protocol["source_head_sha"]
            and metadata["size_in_bytes"] == len(raw)
            and not metadata["expired"]
            and sha(raw) == row["source_artifact_sha256"],
            "exact provider ZIP and identity")
    name = row["formula"]
    formula = protocol["formulas"][name]
    member = f"n83-pattern-results/exports/{name}/instance.xcnf"
    export_member = f"n83-pattern-results/exports/{name}/export.json"
    with zipfile.ZipFile(archive) as bundle:
        names = bundle.namelist()
        require(len(names) == len(set(names))
                and all(not n.startswith("/") and ".." not in Path(n).parts
                        for n in names), "safe unique ZIP members")
        xcnf = bundle.read(member)
        export = json.loads(bundle.read(export_member))
    require(len(xcnf) == formula["bytes"] and sha(xcnf) == formula["sha256"]
            and export["schema"] == "n83.cross-stream-pattern-xcnf/v2"
            and export["trial"] == row["trial"]
            and export["identity_mask"] == 0
            and export["base_summands"] == row["base_summands"]
            and export["residual_summands"] == row["residual_summands"]
            and export["xcnf_bytes"] == len(xcnf),
            "exact exported formula and shape")
    out.mkdir()
    (out / f"{name}.xcnf").write_bytes(xcnf)
    receipt = {"schema": "n83.solver-setting-input/v1",
               "status": "PASS_EXACT_PRIOR_S3_BACKED_XCNF",
               "stratum": stratum_id, "formula": name,
               "source_run_id": protocol["source_run_id"],
               "source_artifact_id": metadata["id"],
               "source_artifact_zip_sha256": sha(raw),
               "source_artifact_metadata_sha256": sha(metadata_raw),
               "source_audit_json_sha256": row["source_audit_json_sha256"],
               "xcnf_bytes": len(xcnf), "xcnf_sha256": sha(xcnf),
               "selected_best_total_runtime": None}
    (out / "input-provenance.json").write_text(
        json.dumps(receipt, indent=2, sort_keys=True) + "\n")
    return receipt


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--protocol", type=Path, default=HERE / "protocol.json")
    parser.add_argument("--stratum", required=True)
    parser.add_argument("--archive", type=Path, required=True)
    parser.add_argument("--metadata", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(fetch(args.protocol, args.stratum, args.archive,
                           args.metadata, args.out), sort_keys=True))
