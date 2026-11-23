"""Transactional address for one frozen unsigned historical workflow source.

The actual current Git blob is held whole and restored. This changes only a
data address during original attestation replay, never the verifier/keyring,
mathematical source, claim or ledger. Historical bytes are not current-source
execution evidence. Keep every new current-source proof on an immutable copy.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import stat
import subprocess
import tempfile

LEGACY = ".github/workflows/pcc-gate.yml"


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def git(repo: Path, *args: str) -> bytes:
    return subprocess.check_output(["git", *args], cwd=repo, stderr=subprocess.PIPE)


def json_write(path: Path, value: dict) -> None:
    fd, name = tempfile.mkstemp(prefix=".source-data-receipt-", dir=path.parent)
    temporary = Path(name)
    with os.fdopen(fd, "w") as stream:
        json.dump(value, stream, indent=2)
        stream.write("\n")
        stream.flush()
        os.fsync(stream.fileno())
    temporary.replace(path)


def prepare(repo: Path, spec: dict, receipt_path: Path) -> dict:
    repo = repo.resolve()
    if spec.get("schema") != "pcc.historical-workflow-data-address/v2" or spec.get("legacy_path") != LEGACY:
        raise ValueError("only the declared historical workflow data address is allowed")
    claim_path = repo / "pcc/claims" / (spec["claim"] + ".json")
    if not claim_path.resolve().is_relative_to(repo) or digest(claim_path.read_bytes()) != spec["claim_sha256"]:
        raise ValueError("frozen claim digest/path mismatch")
    claim = json.loads(claim_path.read_text())
    if claim.get("id") != spec["claim"] or claim.get("kind") != "statement" or claim.get("claim_type") != "data":
        raise ValueError("historical source address is only for unsigned statement/data")
    proof = claim["proofs"][spec["proof_index"]]
    if proof.get("kind") != "attestation" or proof.get("subject") != {"path": LEGACY, "sha256": spec["sha256"]}:
        raise ValueError("frozen data attestation pin mismatch")
    users = []
    for path in sorted((repo / "pcc/claims").glob("*.json")):
        other = json.loads(path.read_text())
        for index, candidate in enumerate(other.get("proofs", [])):
            subject = candidate.get("subject")
            if isinstance(subject, dict) and subject.get("path") == LEGACY:
                if candidate.get("kind") != "attestation" or other.get("kind") != "statement" or other.get("claim_type") != "data" or subject.get("sha256") != spec["sha256"]:
                    raise ValueError("conflicting or non-data historical address user")
                users.append([other.get("id"), index])
    if users != [[spec["claim"], spec["proof_index"]]]:
        raise ValueError("historical address users differ from the complete declared set")
    relative = Path(spec["canonical_path"])
    canonical = repo / relative
    if relative.is_absolute() or not canonical.resolve().is_relative_to(repo) or canonical.is_symlink() or not canonical.is_file():
        raise ValueError("canonical old source must be a regular in-repository file")
    if digest(canonical.read_bytes()) != spec["sha256"]:
        raise ValueError("canonical historical bytes do not match the frozen pin")
    legacy = repo / LEGACY
    if not legacy.resolve().is_relative_to(repo) or legacy.is_symlink() or not legacy.is_file():
        raise ValueError("current source must be a regular in-repository Git file")
    current = legacy.read_bytes()
    if git(repo, "status", "--porcelain", "--untracked-files=all", "--", LEGACY) or current != git(repo, "show", "HEAD:" + LEGACY):
        raise ValueError("current workflow differs from its actual Git HEAD blob")
    if receipt_path.resolve().is_relative_to(repo) or receipt_path.parent.exists():
        raise ValueError("use a fresh external receipt/hold directory")
    head = git(repo, "rev-parse", "HEAD").decode().strip()
    receipt_path.parent.mkdir(mode=0o700, parents=True)
    held = receipt_path.parent / "entire-actual-current-workflow.yml"
    with held.open("xb") as stream:
        stream.write(current)
        stream.flush()
        os.fsync(stream.fileno())
    mode = stat.S_IMODE(legacy.stat().st_mode)
    os.chmod(held, mode)
    receipt = {"schema": "pcc.historical-workflow-data-receipt/v2", "repo": str(repo), "head": head, "legacy_path": LEGACY, "canonical": str(canonical.resolve()), "historical_sha256": spec["sha256"], "held_current": str(held), "current_sha256": digest(current), "current_bytes": len(current), "mode": mode, "data_address_users": users, "state": "current-source-held-before-data-view", "scope": "unsigned historical byte address only; not current-source execution verification"}
    json_write(receipt_path, receipt)
    temporary = legacy.with_name(legacy.name + ".historical-data-view")
    if temporary.exists() or temporary.is_symlink():
        raise ValueError("unowned temporary source address exists; held current is preserved")
    temporary.symlink_to(canonical.resolve())
    # The receipt and full current file are durable before this atomic change.
    temporary.replace(legacy)
    receipt["state"] = "historical-data-view-active"
    json_write(receipt_path, receipt)
    return receipt


def restore(receipt_path: Path) -> dict:
    receipt = json.loads(receipt_path.read_text())
    if receipt.get("schema") != "pcc.historical-workflow-data-receipt/v2" or receipt.get("legacy_path") != LEGACY:
        raise ValueError("unsupported source-data receipt")
    repo = Path(receipt["repo"]).resolve()
    legacy = repo / LEGACY
    held = Path(receipt["held_current"])
    current = held.read_bytes()
    if held.is_symlink() or digest(current) != receipt["current_sha256"] or len(current) != receipt["current_bytes"]:
        raise ValueError("held current source changed; refusing overwrite")
    already_restored = legacy.is_file() and not legacy.is_symlink() and legacy.read_bytes() == current
    current_head = git(repo, "rev-parse", "HEAD").decode().strip()
    if git(repo, "show", "HEAD:" + LEGACY) != current or (not already_restored and current_head != receipt["head"]):
        raise ValueError("actual source HEAD changed; held current remains preserved")
    if not already_restored:
        if not legacy.is_symlink() or legacy.resolve() != Path(receipt["canonical"]) or digest(legacy.read_bytes()) != receipt["historical_sha256"]:
            raise ValueError("source data address changed or unrelated; held current remains preserved")
        fd, name = tempfile.mkstemp(prefix=".pcc-current-source-restore-", dir=legacy.parent)
        with os.fdopen(fd, "wb") as stream:
            stream.write(current)
            stream.flush()
            os.fsync(stream.fileno())
        os.chmod(name, receipt["mode"])
        os.replace(name, legacy)
    if legacy.is_symlink() or legacy.read_bytes() != current or git(repo, "status", "--porcelain", "--untracked-files=all", "--", LEGACY):
        raise ValueError("current source restoration is not exact")
    receipt["state"] = "exact-current-source-restored"
    receipt["restored_source_head"] = current_head
    json_write(receipt_path, receipt)
    return receipt


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, default=Path.cwd())
    parser.add_argument("--manifest", type=Path)
    parser.add_argument("--receipt", type=Path, required=True)
    parser.add_argument("--restore", action="store_true")
    args = parser.parse_args()
    if args.restore:
        result = restore(args.receipt)
    else:
        result = prepare(args.repo, json.loads(args.manifest.read_text()), args.receipt)
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()
