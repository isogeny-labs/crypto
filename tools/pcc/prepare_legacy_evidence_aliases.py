"""Restore immutable unsigned data addresses, retaining original PCC checks.

No verifier, claim, ledger, keyring or canonical receipt is modified. A legacy
address becomes a symlink to its byte-identical canonical in-repository file;
the original verifier still resolves containment and checks the original digest.
Existing unrelated paths are never overwritten.
"""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path


def digest(path: Path) -> str:
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def prepare(repo: Path, spec: dict) -> dict:
    repo = repo.resolve()
    if spec.get("schema") != "pcc.legacy-evidence-addresses/v1":
        raise ValueError("unsupported address manifest")
    results = []
    for item in spec["aliases"]:
        claim_path = repo / "pcc/claims" / (item["claim"] + ".json")
        if not claim_path.resolve().is_relative_to(repo) or digest(claim_path) != item["claim_sha256"]:
            raise ValueError("frozen claim digest/path mismatch")
        claim = json.loads(claim_path.read_text())
        if claim.get("kind") != "statement" or claim.get("claim_type") != "data":
            raise ValueError("only immutable unsigned data statements are addressable")
        proof = claim["proofs"][item["proof_index"]]
        if proof.get("kind") != "attestation":
            raise ValueError("only attestation data addresses are supported")
        subject = proof["subject"]
        legacy = Path(item["legacy_path"])
        if not legacy.is_absolute() or str(legacy) != subject["path"] or subject["sha256"] != item["sha256"]:
            raise ValueError("legacy address does not match the frozen proof")
        relative = Path(item["canonical_path"])
        canonical = (repo / relative).resolve()
        if relative.is_absolute() or not canonical.is_relative_to(repo) or not canonical.is_file():
            raise ValueError("canonical receipt must be an existing in-repository file")
        if digest(canonical) != subject["sha256"]:
            raise ValueError("canonical receipt differs from the original frozen digest")
        if legacy.is_symlink():
            if legacy.resolve() != canonical:
                raise ValueError("existing legacy symlink points to a different file")
            created = False
        elif legacy.exists():
            raise ValueError("existing non-symlink legacy path is preserved; refusing overwrite")
        else:
            legacy.parent.mkdir(parents=True, exist_ok=True)
            legacy.symlink_to(canonical)
            created = True
        if legacy.resolve() != canonical or not legacy.resolve().is_relative_to(repo) or digest(legacy) != subject["sha256"]:
            raise ValueError("post-creation address binding failed")
        results.append({"claim": item["claim"], "legacy_path": str(legacy), "canonical_path": str(canonical), "sha256": subject["sha256"], "created": created})
    return {"schema": "pcc.legacy-evidence-address-receipt/v1", "addresses": results, "scope": "original containment/digest/signature checks remain unchanged; data remain unsigned, not execution verified"}


def cleanup(receipt: dict) -> None:
    for item in receipt["addresses"]:
        if item["created"]:
            path = Path(item["legacy_path"])
            if not path.is_symlink() or path.resolve() != Path(item["canonical_path"]) or digest(path) != item["sha256"]:
                raise ValueError("refusing cleanup of a changed or unrelated address")
            path.unlink()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, default=Path.cwd())
    parser.add_argument("--manifest", type=Path)
    parser.add_argument("--receipt", type=Path, required=True)
    parser.add_argument("--cleanup", action="store_true")
    args = parser.parse_args()
    if args.cleanup:
        cleanup(json.loads(args.receipt.read_text()))
    else:
        if args.manifest is None or args.receipt.exists():
            raise ValueError("new receipt and explicit manifest required")
        result = prepare(args.repo, json.loads(args.manifest.read_text()))
        args.receipt.write_text(json.dumps(result, indent=2) + "\n")


if __name__ == "__main__":
    main()
