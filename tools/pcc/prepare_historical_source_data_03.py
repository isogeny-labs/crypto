"""Two frozen unsigned data addresses in a separate, detached Git worktree.

The actual source checkout is never changed. PCC's original verifier reads the
historical bytes in the data worktree. Current-source tests and ledger
publication belong to the actual source checkout after restoration. The
existing combined verdict verification/signing operation stays in the data
worktree under the original main-only key restrictions. No verifier, frozen claim,
ledger, keyring, proof kind, or exit-code allowance is changed here.
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

SCHEMA = "pcc.isolated-source-data-addresses/v1"
RECEIPT_SCHEMA = "pcc.isolated-source-data-custody/v1"
EXPECTED = {
    "src/cryptanalysis/ic_framework/binary_subtract_tests.rs": {
        "claim": "PCC-20261115-9134a9", "proof_index": 1,
        "claim_sha256": "5409d3beb01429564aac5bf687bf1845fb141744cf4be1ec3b6c155d9e1c0467",
        "sha256": "507e82201f3a76ed4650e1bfc72dddc2c8eb06dc9998ce9fc79bde1eeee674c6",
    },
    "problems/chained-transfer-current-store-and-startup.md": {
        "claim": "PCC-20261116-582327", "proof_index": 3,
        "claim_sha256": "a34bfd5788c7c146f683ea371e55bcbf750c3fa5efdf9934700711ef4459d1d3",
        "sha256": "cb689c6a358adfe531a6536759604fed10f82fe0c5fee5cde08676c0dcbdcce1",
    },
}


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def git(repo: Path, *args: str) -> bytes:
    return subprocess.check_output(["git", *args], cwd=repo,
                                   env=dict(os.environ, GIT_NO_LAZY_FETCH="1"),
                                   stderr=subprocess.PIPE)


def git_path(repo: Path, option: str) -> Path:
    return (repo / git(repo, "rev-parse", option).decode().strip()).resolve()


def identities(source: Path, data: Path) -> str:
    if source == data or source.is_relative_to(data) or data.is_relative_to(source):
        raise ValueError("use non-nested separate Git worktrees")
    if git_path(source, "--git-dir") == git_path(data, "--git-dir"):
        raise ValueError("data must be a separate Git worktree")
    for repo in (source, data):
        if Path(git(repo, "rev-parse", "--show-toplevel").decode().strip()).resolve() != repo:
            raise ValueError("use exact worktree roots")
    if git_path(source, "--git-common-dir") != git_path(data, "--git-common-dir"):
        raise ValueError("source and data worktrees must share Git objects and history")
    attached = subprocess.run(["git", "symbolic-ref", "-q", "HEAD"], cwd=data,
                              env=dict(os.environ, GIT_NO_LAZY_FETCH="1"), capture_output=True)
    if attached.returncode != 1:
        raise ValueError("data worktree must have a detached HEAD")
    head = git(source, "rev-parse", "HEAD").decode().strip()
    if git(data, "rev-parse", "HEAD").decode().strip() != head:
        raise ValueError("source and data HEAD differ")
    if git(source, "status", "--porcelain", "--untracked-files=no"):
        raise ValueError("actual source checkout must remain clean")
    return head


def address(repo: Path, relative: str) -> Path:
    name = Path(relative)
    if name.is_absolute() or ".." in name.parts:
        raise ValueError("address must be relative and contained")
    path = repo / name
    for parent in path.parents:
        if parent == repo:
            break
        if parent.is_symlink():
            raise ValueError("source address parent became a symlink")
    return path


def regular(repo: Path, relative: str) -> Path:
    path = address(repo, relative)
    if Path(relative).is_absolute() or not path.resolve().is_relative_to(repo) \
            or path.is_symlink() or not path.is_file():
        raise ValueError("expected a regular contained file: " + relative)
    return path


def json_write(path: Path, value: dict) -> None:
    fd, name = tempfile.mkstemp(prefix=".data-custody-", dir=path.parent)
    with os.fdopen(fd, "w") as stream:
        json.dump(value, stream, indent=2)
        stream.write("\n")
        stream.flush()
        os.fsync(stream.fileno())
    os.replace(name, path)


def mentions(value, target: Path, repo: Path) -> bool:
    if isinstance(value, dict):
        return any(mentions(item, target, repo) for item in value.values())
    if isinstance(value, list):
        return any(mentions(item, target, repo) for item in value)
    if isinstance(value, str) and "/" in value and len(value) <= 4096:
        try:
            return (repo / value).resolve() == target
        except (OSError, RuntimeError):
            return False
    return False


def prepare(source: Path, data: Path, spec: dict, receipt_path: Path) -> dict:
    source, data = source.resolve(), data.resolve()
    head = identities(source, data)
    if spec.get("schema") != SCHEMA or not isinstance(spec.get("records"), list):
        raise ValueError("unsupported data-address manifest")
    records = spec["records"]
    if len(records) != len(EXPECTED) or any(not isinstance(r, dict) for r in records) \
            or {r.get("legacy_path") for r in records} != set(EXPECTED):
        raise ValueError("only the complete two declared addresses are allowed")
    claims = [json.loads(path.read_text()) for path in sorted((data / "pcc/claims").glob("*.json"))]
    rows = []
    for record in records:
        relative = record["legacy_path"]
        expected = EXPECTED[relative]
        canonical_name = "pcc/historical-source-data-03/" + expected["sha256"] + ".txt"
        if any(record.get(k) != expected[k] for k in ("claim", "proof_index", "sha256")) \
                or record.get("canonical_path") != canonical_name:
            raise ValueError("manifest differs from the exact frozen address")
        raw = regular(data, "pcc/claims/" + expected["claim"] + ".json").read_bytes()
        if digest(raw) != expected["claim_sha256"] or raw != regular(
                source, "pcc/claims/" + expected["claim"] + ".json").read_bytes():
            raise ValueError("frozen claim bytes differ")
        claim_name = "pcc/claims/" + expected["claim"] + ".json"
        try:
            committed_claim = git(source, "show", "HEAD:" + claim_name)
        except subprocess.CalledProcessError:
            raise ValueError("frozen claim is not committed at current HEAD") from None
        if raw != committed_claim:
            raise ValueError("frozen claim differs from its committed bytes")
        claim = json.loads(raw)
        if claim.get("kind") != "statement" or claim.get("claim_type") != "data" \
                or any(p.get("kind") != "attestation" or "signature" in p for p in claim["proofs"]):
            raise ValueError("only unsigned statement/data attestations are allowed")
        proof = claim["proofs"][expected["proof_index"]]
        if proof != {"kind": "attestation", "subject": {"path": relative, "sha256": expected["sha256"]}}:
            raise ValueError("frozen attestation differs")
        canonical = regular(data, canonical_name)
        if digest(canonical.read_bytes()) != expected["sha256"]:
            raise ValueError("historical object does not match the frozen pin")
        try:
            committed_data = git(data, "show", "HEAD:" + canonical_name)
        except subprocess.CalledProcessError:
            raise ValueError("canonical data is not committed at current HEAD") from None
        if canonical.read_bytes() != committed_data:
            raise ValueError("canonical data differs from its committed bytes")
        actual = regular(source, relative)
        legacy = regular(data, relative)
        current = actual.read_bytes()
        if current != git(source, "show", "HEAD:" + relative) or legacy.read_bytes() != current \
                or git(data, "status", "--porcelain", "--untracked-files=all", "--", relative):
            raise ValueError("current data/source bytes differ from their Git HEAD")
        users = [[c.get("id"), i] for c in claims for i, p in enumerate(c.get("proofs", []))
                 if mentions(p, legacy.resolve(), data)]
        if users != [[expected["claim"], expected["proof_index"]]]:
            raise ValueError("historical address has a conflicting or additional user")
        rows.append({"legacy_path": relative, "canonical": str(canonical.resolve()),
                     "claim": expected["claim"], "users": users,
                     "historical_sha256": expected["sha256"], "current_sha256": digest(current),
                     "current_bytes": len(current), "mode": stat.S_IMODE(legacy.stat().st_mode),
                     "actual_source_mode": stat.S_IMODE(actual.stat().st_mode)})
    if receipt_path.resolve().is_relative_to(source) or receipt_path.resolve().is_relative_to(data) \
            or receipt_path.parent.exists():
        raise ValueError("use a fresh external custody directory")
    receipt_path.parent.mkdir(parents=True, mode=0o700)
    for index, row in enumerate(rows):
        held = receipt_path.parent / (str(index) + ".entire-current-source")
        with held.open("xb") as stream:
            stream.write((data / row["legacy_path"]).read_bytes())
            stream.flush()
            os.fsync(stream.fileno())
        os.chmod(held, row["mode"])
        row["held_current"] = str(held)
    receipt = {"schema": RECEIPT_SCHEMA, "source_repo": str(source), "data_repo": str(data),
               "head": head, "records": rows, "state": "all-current-data-sources-held",
               "scope": "unsigned historical data only; actual source checkout remains unchanged"}
    json_write(receipt_path, receipt)
    for row in rows:
        legacy = address(data, row["legacy_path"])
        temporary = legacy.with_name(legacy.name + ".historical-data-address-03")
        if temporary.exists() or temporary.is_symlink():
            raise ValueError("unowned temporary address exists; held current is preserved")
        temporary.symlink_to(row["canonical"])
        temporary.replace(legacy)
    identities(source, data)
    receipt["state"] = "isolated-historical-data-view-active"
    json_write(receipt_path, receipt)
    return receipt


def restore(receipt_path: Path) -> dict:
    receipt = json.loads(receipt_path.read_text())
    if receipt.get("schema") != RECEIPT_SCHEMA:
        raise ValueError("unsupported data custody receipt")
    source, data = Path(receipt["source_repo"]).resolve(), Path(receipt["data_repo"]).resolve()
    if identities(source, data) != receipt["head"]:
        raise ValueError("HEAD changed; held current remains preserved")
    rows = receipt["records"]
    if len(rows) != len(EXPECTED) or {r.get("legacy_path") for r in rows} != set(EXPECTED):
        raise ValueError("custody address set changed")
    # Validate every address before restoring any of them, including partial preparation.
    for row in rows:
        expected = EXPECTED[row["legacy_path"]]
        held = Path(row["held_current"])
        actual = regular(source, row["legacy_path"])
        current = held.read_bytes()
        if held.is_symlink() or not held.resolve().is_relative_to(receipt_path.parent.resolve()) \
                or digest(current) != row["current_sha256"] or len(current) != row["current_bytes"] \
                or stat.S_IMODE(held.stat().st_mode) != row["mode"]:
            raise ValueError("held whole current source changed")
        if actual.read_bytes() != current or stat.S_IMODE(actual.stat().st_mode) != row["actual_source_mode"] \
                or git(data, "show", "HEAD:" + row["legacy_path"]) != current:
            raise ValueError("actual source changed; refusing overwrite")
        legacy = address(data, row["legacy_path"])
        restored = legacy.is_file() and not legacy.is_symlink() and legacy.read_bytes() == current
        canonical = regular(data, "pcc/historical-source-data-03/" + expected["sha256"] + ".txt")
        if not restored and (not legacy.is_symlink() or legacy.resolve() != canonical.resolve()
                             or digest(legacy.read_bytes()) != expected["sha256"]):
            raise ValueError("data address changed or unrelated; held current is preserved")
    for row in rows:
        legacy = address(data, row["legacy_path"])
        fd, name = tempfile.mkstemp(prefix=".pcc-data-source-restore-", dir=legacy.parent)
        with os.fdopen(fd, "wb") as stream:
            stream.write(Path(row["held_current"]).read_bytes())
            stream.flush()
            os.fsync(stream.fileno())
        os.chmod(name, row["mode"])
        os.replace(name, legacy)
        if git(data, "status", "--porcelain", "--untracked-files=all", "--", row["legacy_path"]):
            raise ValueError("data source restoration is not exact")
    identities(source, data)
    receipt["state"] = "exact-data-sources-restored-actual-source-unchanged"
    json_write(receipt_path, receipt)
    return receipt


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-repo", type=Path)
    parser.add_argument("--data-repo", type=Path)
    parser.add_argument("--manifest", type=Path)
    parser.add_argument("--receipt", type=Path, required=True)
    parser.add_argument("--restore", action="store_true")
    args = parser.parse_args()
    if args.restore:
        result = restore(args.receipt)
    else:
        result = prepare(args.source_repo, args.data_repo,
                         json.loads(args.manifest.read_text()), args.receipt)
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()
