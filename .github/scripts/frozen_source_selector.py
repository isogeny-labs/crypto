#!/usr/bin/env python3
"""Materialize exact hash-pinned historical source bytes for replay checks.

This helper never updates a frozen hash. It searches reachable history for bytes
whose SHA-256 equals the existing pin, materializes only those bytes, and emits
a provenance report that distinguishes the historical source from the current
checkout. Current-code validation must be performed as a separate check.
"""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import subprocess


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def git(repo: Path, *args: str) -> bytes:
    return subprocess.check_output(["git", *args], cwd=repo)


def exact_historical_bytes(repo: Path, relative_path: str, expected_sha256: str):
    """Return (bytes, commit) only when the exact expected bytes exist in history."""
    revisions = git(repo, "log", "--all", "--format=%H", "--", relative_path)
    for revision in revisions.decode().splitlines():
        try:
            data = git(repo, "show", f"{revision}:{relative_path}")
        except subprocess.CalledProcessError:
            continue
        if sha(data) == expected_sha256:
            return data, revision
    raise LookupError(
        f"no reachable historical bytes match sha256={expected_sha256} for {relative_path}"
    )


def materialize(repo: Path, target_root: Path, relative_path: str, expected_sha256: str) -> dict:
    repo = repo.resolve()
    target_root = target_root.resolve()
    destination = (target_root / relative_path).resolve()
    if not destination.is_relative_to(target_root):
        raise ValueError(f"path escapes target root: {relative_path}")
    current = repo / relative_path
    current_sha = sha(current.read_bytes()) if current.is_file() else None
    data, commit = exact_historical_bytes(repo, relative_path, expected_sha256)
    if sha(data) != expected_sha256:
        raise AssertionError("selector returned bytes with a nonmatching digest")
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_bytes(data)
    return {
        "path": relative_path,
        "expected_sha256": expected_sha256,
        "current_checkout_sha256": current_sha,
        "materialized_sha256": sha(data),
        "source_commit": commit,
        "current_checkout_was_exact": current_sha == expected_sha256,
        "status": "exact_pinned_bytes_materialized_outside_checkout",
    }


def n53_sources(repo: Path) -> list[tuple[str, str, str]]:
    bundle = repo / "research/sat_factor_base_review_20260908/autolab_n53_native_cyclic_l384_20260929"
    manifest = json.loads((bundle / "FROZEN.json").read_text())
    import sys
    sys.path.insert(0, str(bundle))
    import check_protocol
    sources = check_protocol.sources()
    pinned = manifest["source_sha256"]
    if set(sources) != set(pinned):
        raise AssertionError("source map does not match frozen source-hash keys")
    return [(name, str(path.relative_to(repo)), pinned[name]) for name, path in sources.items()]


def ecc2k130_base_window_sources() -> list[tuple[str, str, str]]:
    return [
        ("fast_arith", "src/cryptanalysis/koblitz_fast_arith.rs",
         "2a5c54bdedb6a0b1ffcf95dd7badd32c41d1482b53055e31fb9f28d514ac8c9e"),
        ("cargo_lock", "research/notes/ecc2k130/compact_shared_log_20260925/Cargo.lock",
         "7d671f48c2da93f133d98802e80f858d1d9ea3b86996f7037f758990e1566627"),
    ]


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, required=True)
    parser.add_argument("--profile", choices=("n53", "ecc2k130-base-window", "single"), required=True)
    parser.add_argument("--path", help="path for --profile single")
    parser.add_argument("--sha256", help="expected digest for --profile single")
    parser.add_argument("--target-root", type=Path, required=True,
                        help="isolated destination; must not be the checked-out repository")
    parser.add_argument("--report", type=Path, required=True)
    args = parser.parse_args()
    repo = args.repo.resolve()
    if args.profile == "n53":
        targets = [(path, digest) for _, path, digest in n53_sources(repo)]
    elif args.profile == "ecc2k130-base-window":
        if args.path or args.sha256:
            parser.error("--path/--sha256 are only valid with --profile single")
        targets = [(path, digest) for _, path, digest in ecc2k130_base_window_sources()]
    else:
        if not args.path or not args.sha256:
            parser.error("--profile single requires --path and --sha256")
        targets = [(args.path, args.sha256)]
    target_root = args.target_root.resolve()
    if target_root == repo or repo in target_root.parents:
        parser.error("--target-root must be isolated outside the checked-out repository")
    results = [materialize(repo, target_root, path, expected) for path, expected in targets]
    report = {
        "schema": "frozen-source-selection-v1",
        "current_checkout_head": git(repo, "rev-parse", "HEAD").decode().strip(),
        "target_root": str(target_root),
        "historical_replay_status": "passed_source_selection",
        "current_code_parity_status": "not_assessed_by_historical_source_selection",
        "selections": results,
    }
    args.report.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"status": report["historical_replay_status"], "selected": len(results),
                      "current_code_parity_status": report["current_code_parity_status"],
                      "report": str(args.report)}, sort_keys=True))


if __name__ == "__main__":
    main()
