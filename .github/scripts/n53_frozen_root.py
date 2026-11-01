#!/usr/bin/env python3
"""Build an isolated N53 replay root from the frozen Cargo baseline and exact pins."""
from __future__ import annotations

import hashlib
import json
from pathlib import Path
import subprocess
import tarfile
import tempfile

from frozen_source_selector import exact_historical_bytes, n53_sources, sha

BUILD_BASE = "d18b694c756e425158ef1548dc1a39df9005e405"


def git(repo: Path, *args: str) -> bytes:
    return subprocess.check_output(["git", *args], cwd=repo)


def head_bytes(repo: Path, relative: str) -> bytes:
    """Read committed checkout bytes; never attribute dirty bytes to HEAD."""
    return git(repo, "show", f"HEAD:{relative}")


def baseline_paths(repo: Path) -> list[str]:
    tracked = git(repo, "ls-tree", "-r", "--name-only", BUILD_BASE).decode().splitlines()
    return [path for path in tracked
            if path == "Cargo.toml" or path.startswith(("src/", "examples/", "tests/",
                                                         "benches/", "docs/"))]


def archive_baseline(repo: Path, target: Path) -> dict[str, str]:
    # Capture complete build-visible source trees at the pinned historical
    # baseline, including include_str!/include_bytes! payloads and targets that
    # Cargo discovers automatically. Never fill missing paths from checkout.
    paths = baseline_paths(repo)
    data = git(repo, "archive", BUILD_BASE, "--", *paths)
    hashes = {}
    with tarfile.open(fileobj=__import__("io").BytesIO(data), mode="r:") as archive:
        members = archive.getmembers()
        for member in members:
            candidate = Path(member.name)
            if candidate.is_absolute() or ".." in candidate.parts:
                raise ValueError(f"unsafe baseline path {member.name}")
        archive.extractall(target, members=members, filter="data")
    for member in members:
        if member.isfile():
            path = target / member.name
            hashes[member.name] = sha(path.read_bytes())
    return hashes


def manifest_entries(repo: Path) -> tuple[dict, dict[str, tuple[bytes, str, str]]]:
    selected = n53_sources(repo)
    source_rows = {}
    for key, relative, expected in selected:
        data, commit = exact_historical_bytes(repo, relative, expected)
        if sha(data) != expected:
            raise AssertionError(f"{key}: selected hash mismatch")
        source_rows[relative] = (data, key, commit)
    lock_rel = next(path for key, path, _ in selected if key == "cargo_lock")
    lock_hash = dict((key, digest) for key, _, digest in selected)["cargo_lock"]
    lock_data, lock_commit = exact_historical_bytes(repo, lock_rel, lock_hash)
    source_rows["Cargo.lock"] = (lock_data, "build_cargo_lock", lock_commit)

    bundle = repo / "research/sat_factor_base_review_20260908/autolab_n53_native_cyclic_l384_20260929"
    manifest_rel = str((bundle / "FROZEN.json").relative_to(repo))
    manifest_bytes = head_bytes(repo, manifest_rel)
    manifest = json.loads(manifest_bytes)
    head = git(repo, "rev-parse", "HEAD").decode().strip()
    for relative, expected in ((str((bundle / "FROZEN.json").relative_to(repo)), None),
                               (str((bundle / "PROTOCOL.md").relative_to(repo)), manifest["protocol_sha256"])):
        data = manifest_bytes if relative == manifest_rel else head_bytes(repo, relative)
        digest = sha(data)
        if expected is not None and digest != expected:
            raise LookupError(f"frozen bundle metadata {relative} differs from its manifest hash")
        source_rows[relative] = (data, f"bundle:{Path(relative).name}", head)

    import sys
    sys.path.insert(0, str(bundle))
    import check_protocol
    for key, path in check_protocol.inputs().items():
        relative = str(path.relative_to(repo))
        expected = manifest["input_sha256"][key]
        data = head_bytes(repo, relative)
        if sha(data) != expected:
            raise LookupError(f"frozen input {relative} differs from its manifest hash")
        if relative in source_rows and sha(source_rows[relative][0]) != expected:
            raise LookupError(f"source and input pins conflict for {relative}")
        source_rows.setdefault(relative, (data, f"input:{key}", head))
    return manifest, source_rows


def prepare(repo: Path, target: Path) -> dict:
    repo, target = repo.resolve(), target.resolve()
    if target == repo or repo in target.parents or target.exists():
        raise ValueError("frozen source root must be a new path outside the checkout")
    git(repo, "cat-file", "-e", f"{BUILD_BASE}^{{commit}}")
    manifest, overlays = manifest_entries(repo)
    target.mkdir(parents=True)
    baseline_hashes = archive_baseline(repo, target)
    base_overrides = {}
    for relative, (data, key, commit) in sorted(overlays.items()):
        path = target / relative
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data)
        base_overrides[relative] = key

    # Materialize paths required by overlays that were absent from the baseline
    # archive. The root is closed: no checkout symlinks, build.rs, .cargo
    # configuration, or auto-discovered targets may enter it implicitly.
    for relative in overlays:
        path = target / relative
        if not path.exists():
            raise AssertionError(f"overlay was not materialized: {relative}")

    base_rows = [{"path": path, "sha256": digest, "source_commit": BUILD_BASE,
                  "overridden_by_experiment_pin": path in base_overrides}
                 for path, digest in sorted(baseline_hashes.items())]
    overlay_rows = []
    selected = n53_sources(repo)
    selected_by_path = {path: (key, digest) for key, path, digest in selected}
    bundle = repo / "research/sat_factor_base_review_20260908/autolab_n53_native_cyclic_l384_20260929"
    lock_rel = next(path for key, path, _ in selected if key == "cargo_lock")
    for relative, (data, key, commit) in sorted(overlays.items()):
        expected = selected_by_path[relative][1] if relative in selected_by_path else sha(data)
        if relative == "Cargo.lock":
            expected = dict((name, digest) for name, _, digest in selected)["cargo_lock"]
        overlay_rows.append({"key": key, "path": relative,
                             "source_path": lock_rel if key == "build_cargo_lock" else relative,
                             "expected_sha256": expected,
                             "checkout_sha256": sha((repo / relative).read_bytes()) if (repo / relative).is_file() else None,
                             "materialized_sha256": sha((target / relative).read_bytes()),
                             "source_commit": commit})
    result = {"schema": "n53-frozen-source-root-v2",
              "checkout_head": git(repo, "rev-parse", "HEAD").decode().strip(),
              "source_root": str(target), "build_baseline_commit": BUILD_BASE,
              "historical_replay_status": "exact_pins_over_coherent_frozen_build_baseline",
              "current_code_parity_status": "not_assessed",
              "baseline_files": base_rows, "selected_sources": overlay_rows}
    if not set(manifest["source_sha256"]).issubset({row["key"] for row in overlay_rows}):
        raise AssertionError("selected source set does not cover frozen source manifest")
    return result


def verify(repo: Path, target: Path, report: Path) -> dict:
    value = json.loads(report.read_text())
    repo, target = repo.resolve(), target.resolve()
    head = git(repo, "rev-parse", "HEAD").decode().strip()
    if value.get("checkout_head") != head or Path(value.get("source_root", "")).resolve() != target:
        raise AssertionError("frozen source report belongs to a different checkout/root")
    if value.get("build_baseline_commit") != BUILD_BASE:
        raise AssertionError("build baseline commit differs from reviewed frozen baseline")
    # Re-derive the full inventories rather than trusting a mutable report list.
    expected_base = archive_hashes(repo)
    base_rows = value.get("baseline_files", [])
    got_base = {row["path"]: row["sha256"] for row in base_rows}
    if (len(got_base) != len(base_rows) or got_base != expected_base
            or any(row.get("source_commit") != BUILD_BASE for row in base_rows)):
        raise AssertionError("baseline source inventory is incomplete or changed")
    manifest, overlays = manifest_entries(repo)
    overlay_rows = value.get("selected_sources", [])
    got_overlays = {row["path"]: (row["expected_sha256"], row["key"], row["source_commit"])
                    for row in overlay_rows}
    expected_overlays = {path: (sha(data), key, commit)
                         for path, (data, key, commit) in overlays.items()}
    if len(got_overlays) != len(overlay_rows) or got_overlays != expected_overlays:
        raise AssertionError("selected source/input inventory is incomplete or changed")
    if value.get("current_code_parity_status") != "not_assessed":
        raise AssertionError("historical source selection cannot claim current-code parity")
    for path, expected in expected_base.items():
        if path in expected_overlays:
            continue
        if sha((target / path).read_bytes()) != expected:
            raise AssertionError(f"build-baseline file changed: {path}")
    for path, (expected, _, _) in expected_overlays.items():
        if sha((target / path).read_bytes()) != expected:
            raise AssertionError(f"selected frozen source changed: {path}")
    expected_files = set(expected_base) | set(expected_overlays)
    actual_files = set()
    for path in target.rglob("*"):
        relative = path.relative_to(target).as_posix()
        if relative == "target" or relative.startswith("target/"):
            continue
        if path.is_symlink():
            raise AssertionError(f"unexpected source-root symlink: {relative}")
        if path.is_file():
            actual_files.add(relative)
    if actual_files != expected_files:
        raise AssertionError("source root has unrecorded or missing build-visible files")
    return value


def archive_hashes(repo: Path) -> dict[str, str]:
    paths = baseline_paths(repo)
    data = git(repo, "archive", BUILD_BASE, "--", *paths)
    with tarfile.open(fileobj=__import__("io").BytesIO(data), mode="r:") as archive:
        return {member.name: sha(archive.extractfile(member).read())
                for member in archive.getmembers() if member.isfile()}
