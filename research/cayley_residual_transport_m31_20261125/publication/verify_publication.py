#!/usr/bin/env python3
"""Verify publication metadata and byte custody; never run research code.

This utility does not recompute curve arithmetic, experimental statistics,
mathematical certificates, or performance claims. The historical checker is
archive-only and is explicitly NOT repository-verified by this command.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
from pathlib import Path, PurePosixPath


SCHEMA = "isogeny-labs/cayley-residual-publication/v1"
REPORT_SCHEMA = "isogeny-labs/cayley-residual-publication-verification/v1"
PACKAGING = "publication/packaging.json"
VALIDATION = "evidence/validation-attempt-1"
CHECKER = f"{VALIDATION}/checker"
RAW_NAMES = ("cases.jsonl", "intersections.jsonl", "prefixes.jsonl", "seeds.jsonl")
REQUIRED_PUBLICATION_FILES = {
    "README.md", "HANDOFF.json", "publication/verify_publication.py",
    "publication/replay_published.sh",
}
SHA256 = re.compile(r"[0-9a-f]{64}\Z")


class VerificationError(Exception):
    """A publication consistency or custody failure."""


def require(condition: bool, message: str) -> None:
    if not condition:
        raise VerificationError(message)


def relative_name(value: object) -> str:
    require(isinstance(value, str) and bool(value), "File path must be a nonempty string")
    name = str(value)
    p = PurePosixPath(name)
    require(
        not p.is_absolute() and ".." not in p.parts and str(p) == name
        and name != "." and "\\" not in name and "\x00" not in name,
        f"Unsafe or noncanonical package-relative path: {name!r}",
    )
    return name


def local_file(root: Path, name: object) -> Path:
    rel = relative_name(name)
    path = root / rel
    require(path.resolve().is_relative_to(root), f"File escapes package root: {rel}")
    require(
        path.is_file(),
        f"Missing publication file: {rel}. Hydrate historical evidence with the "
        "repository artifact tool before verification; do not regenerate archival records.",
    )
    return path


def digest(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            h.update(block)
    return h.hexdigest()


def checked_digest(value: object, label: str) -> str:
    require(isinstance(value, str) and SHA256.fullmatch(value) is not None,
            f"Invalid SHA-256 for {label}")
    return str(value)


def checked_size(value: object, label: str) -> int:
    require(type(value) is int and value >= 0, f"Invalid byte size for {label}")
    return int(value)


def verify_bytes(path: Path, expected_hash: str, expected_size: int | None, label: str) -> None:
    if expected_size is not None:
        require(path.stat().st_size == expected_size, f"Byte-size mismatch: {label}")
    require(digest(path) == expected_hash, f"SHA-256 mismatch: {label}")


def read_json(path: Path) -> object:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (ValueError, UnicodeError) as exc:
        raise VerificationError(f"Invalid JSON in {path.name}: {exc}") from exc


def read_manifest(path: Path, *, historical_checker: bool = False) -> dict[str, str]:
    result: dict[str, str] = {}
    for number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        match = re.fullmatch(r"([0-9a-f]{64}) [ *](.+)", line)
        require(match is not None, f"Malformed checksum line {number} in {path.name}")
        assert match is not None
        value, name = match.groups()
        if not (historical_checker and name == "../checker"):
            relative_name(name)
        require(name not in result, f"Duplicate checksum path in {path.name}: {name}")
        result[name] = value
    require(bool(result), f"Empty checksum manifest: {path.name}")
    return result


def verify(root: Path, replay_dir: Path | None) -> dict[str, object]:
    package = read_json(local_file(root, PACKAGING))
    require(isinstance(package, dict), "packaging.json must contain an object")
    require(package.get("schema") == SCHEMA, "Unsupported packaging.json schema")
    original = package.get("original_manifest")
    require(isinstance(original, dict), "original_manifest must be an object")
    original_path = local_file(root, original.get("path"))
    original_hash = checked_digest(original.get("sha256"), "original manifest")
    verify_bytes(original_path, original_hash, None, "original bundle manifest")
    manifest = read_manifest(original_path)
    require(type(original.get("entry_count")) is int
            and original["entry_count"] == len(manifest),
            "Original manifest entry_count does not match its actual entries")

    mappings = package.get("files")
    require(isinstance(mappings, list), "files must be a mapping-entry array")
    mapping: dict[str, dict] = {}
    for entry in mappings:
        require(isinstance(entry, dict), "Every files item must be an object")
        name = relative_name(entry.get("original_path"))
        require(name not in mapping, f"Duplicate original file mapping: {name}")
        require(name in manifest, f"Mapping is absent from original manifest: {name}")
        value = checked_digest(entry.get("sha256"), name)
        require(value == manifest[name], f"Mapping changes original digest: {name}")
        checked_size(entry.get("size_bytes"), name)
        mapping[name] = entry
    require(set(mapping) == set(manifest), "Mapping must cover every original manifest entry exactly once")

    present: dict[str, Path] = {}
    for name, entry in mapping.items():
        status = entry.get("status")
        if status == "archive_only":
            require(name == CHECKER, f"Only the historical checker may be archive_only: {name}")
            require(isinstance(entry.get("archive_reference"), str)
                    and bool(entry["archive_reference"].strip()),
                    "archive_only checker requires a durable archive_reference")
            require("published_path" not in entry,
                    "archive_only checker must not imply a locally verified published_path")
            continue
        require(status == "present", f"Unknown custody status for {name}: {status!r}")
        path = local_file(root, entry.get("published_path"))
        verify_bytes(path, manifest[name], entry["size_bytes"], name)
        present[name] = path
    require(CHECKER in mapping and mapping[CHECKER].get("status") == "archive_only",
            "Historical checker must be explicitly identified as archive_only, not repository-verified")
    require(len(present) == len(manifest) - 1, "Unexpected archive-only file count")

    publication = package.get("publication_files")
    require(isinstance(publication, list), "publication_files must be an array")
    published_names: set[str] = set()
    for entry in publication:
        require(isinstance(entry, dict), "Every publication_files item must be an object")
        name = relative_name(entry.get("path"))
        require(name != PACKAGING, "packaging.json cannot include its own circular content hash")
        require(name not in published_names, f"Duplicate publication file: {name}")
        verify_bytes(local_file(root, name), checked_digest(entry.get("sha256"), name),
                     checked_size(entry.get("size_bytes"), name), name)
        published_names.add(name)
    require(REQUIRED_PUBLICATION_FILES <= published_names,
            "publication_files must bind README, HANDOFF and both publication support scripts")

    registration_name = relative_name(package.get("source_registration_path"))
    require(registration_name in present, "Source registration is not verified historical evidence")
    registration = read_json(present[registration_name])
    require(isinstance(registration, dict) and isinstance(registration.get("hashes"), dict),
            "Malformed final source registration")
    registered = registration["hashes"]
    require("replay.sh" in registered and "PROTOCOL.md" in registered and "SEEDS.json" in registered,
            "Source registration omits the frozen replay/protocol/seed boundary")
    for name, value in registered.items():
        relative_name(name)
        checked_digest(value, name)
        require(name in mapping and manifest[name] == value,
                f"Registered source disagrees with original manifest: {name}")
        require(mapping[name].get("status") == "present"
                and mapping[name].get("published_path") == name,
                f"Registered executable inputs must remain at their unchanged canonical paths: {name}")
        verify_bytes(local_file(root, name), value, mapping[name]["size_bytes"], name)

    prebuild_name = f"{VALIDATION}/prebuild.sha256"
    require(prebuild_name in present, "Missing verified historical prebuild manifest")
    prebuild = read_manifest(present[prebuild_name])
    require(prebuild == {name: value for name, value in registered.items() if name != "replay.sh"},
            "Executed-source prebuild manifest disagrees with final registered inputs")
    for name, value in prebuild.items():
        snapshot = f"{VALIDATION}/source/{name}"
        require(snapshot in manifest and manifest[snapshot] == value,
                f"Historical immutable source snapshot disagrees: {name}")
    expected_call = {**prebuild, "../checker": manifest[CHECKER]}
    for phase in ("precall", "postcall"):
        for run in (1, 2):
            name = f"{VALIDATION}/{phase}-{run}.sha256"
            require(name in present, f"Missing verified historical call manifest: {name}")
            require(read_manifest(present[name], historical_checker=True) == expected_call,
                    f"Historical call manifest disagrees with source/binary identities: {name}")

    receipt_name = relative_name(package.get("historical_receipt_path"))
    require(receipt_name in present, "Receipt is not verified historical evidence")
    receipt = read_json(present[receipt_name])
    require(isinstance(receipt, dict), "Historical receipt must contain an object")
    require(receipt.get("source_registration") == registration_name,
            "Receipt names a different final source registration")
    require(receipt.get("binary_sha256") == manifest[CHECKER],
            "Receipt and original manifest disagree on archive-only checker identity")
    require(receipt.get("protocol_sha256") == registered["PROTOCOL.md"]
            and receipt.get("seed_manifest_sha256") == registered["SEEDS.json"],
            "Receipt disagrees with frozen protocol/seed identities")
    raw_hashes = receipt.get("run1_sha256")
    require(isinstance(raw_hashes, dict) and set(raw_hashes) == set(RAW_NAMES),
            "Receipt must bind all four raw output streams")
    summary_hash = checked_digest(receipt.get("summary_sha256"), "historical summary")
    replay_records = 0
    for run in (1, 2):
        for basename in RAW_NAMES:
            name = f"{VALIDATION}/run{run}/{basename}"
            require(name in present and manifest[name] == raw_hashes[basename],
                    f"Historical raw data disagree with receipt: {name}")
            if replay_dir is not None:
                generated = local_file(replay_dir, f"run{run}/{basename}")
                verify_bytes(generated, manifest[name], mapping[name]["size_bytes"],
                             f"new run {run}: {basename}")
                replay_records += 1
        name = f"{VALIDATION}/summary-{run}.json"
        require(name in present and manifest[name] == summary_hash,
                f"Historical summary disagrees with receipt: {name}")
        require(read_json(present[name]) == receipt.get("result"),
                f"Historical summary object disagrees with receipt.result: {name}")
        if replay_dir is not None:
            generated = local_file(replay_dir, f"summary-{run}.json")
            verify_bytes(generated, summary_hash, mapping[name]["size_bytes"], f"new summary {run}")
            replay_records += 1

    binary = mapping[CHECKER]
    return {
        "schema": REPORT_SCHEMA,
        "ok": True,
        "scope": "publication_metadata_and_byte_integrity_only",
        "scientific_or_signed_validation": False,
        "original_manifest_entries": len(manifest),
        "mapped_original_files_verified": len(present),
        "registered_source_files_verified": len(registered),
        "historical_source_snapshot_entries": len(prebuild),
        "historical_call_manifests_verified": 4,
        "historical_raw_files_verified": 8,
        "historical_summary_files_verified": 2,
        "publication_files_verified": len(publication),
        "historical_checker": {
            "status": "archive_only_not_repository_verified",
            "original_path": CHECKER,
            "sha256_from_original_manifest": manifest[CHECKER],
            "size_bytes_declared": binary["size_bytes"],
            "archive_reference": binary["archive_reference"],
            "locally_verified": False,
        },
        "replay_comparison": {
            "requested": replay_dir is not None,
            "new_output_files_compared": replay_records,
            "historical_binary_identity_required": False,
            "new_execution_performed_by_this_verifier": False,
            "new_binary_or_runtime_independently_validated": False,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--package-root", type=Path,
                        default=Path(__file__).resolve().parent.parent,
                        help="study directory containing publication/packaging.json")
    parser.add_argument("--replay-dir", type=Path,
                        help="compare existing new output bytes with hydrated historical evidence; does not execute")
    parser.add_argument("--json", action="store_true", help="emit one machine-readable report")
    args = parser.parse_args()
    try:
        root = args.package_root.resolve()
        replay = args.replay_dir.resolve() if args.replay_dir is not None else None
        if replay is not None:
            require(replay.is_dir(), "--replay-dir must name an existing execution-output directory")
        report = verify(root, replay)
    except (VerificationError, OSError, UnicodeError, TypeError, KeyError) as exc:
        report = {"schema": REPORT_SCHEMA, "ok": False,
                  "scope": "publication_metadata_and_byte_integrity_only", "error": str(exc)}
        if args.json:
            print(json.dumps(report, indent=2, sort_keys=True))
        else:
            print(f"FAIL: {exc}", file=sys.stderr)
        return 1
    if args.json:
        print(json.dumps(report, indent=2, sort_keys=True))
    else:
        print(f"PASS: {report['mapped_original_files_verified']} mapped original files, "
              f"{report['registered_source_files_verified']} registered sources, and "
              f"{report['publication_files_verified']} publication files verified.")
        print("NOTE: historical checker is archive-only and is NOT repository-verified.")
        if replay is not None:
            print("PASS: 10 existing replay JSONL/summary files match historical bytes; "
                  "no binary-identity or independent runtime claim.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
