#!/usr/bin/env python3
"""Build the three binaries from the v2 source freeze plus the new generator."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import traceback
import io
import tarfile

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
sys.path.insert(
    0, str(ROOT / "research/notes/ecc2k130/compact_frozen_source_replay_20260929")
)
from materialize import load_manifest, snapshot_bytes  # noqa: E402

SOURCE_FREEZE = ROOT / "research/notes/ecc2k130/compact_s3_prefilter_20260930/FROZEN.json"
SOURCE_FREEZE_SHA256 = "3e9f67cc2cd6de5a8458badb3525983d561d9c3449c05118819fb424c5093b2b"
FROZEN_LOCK = ROOT / "research/notes/ecc2k130/compact_shared_log_20260925/Cargo.lock"
FROZEN_LOCK_SHA256 = "7d671f48c2da93f133d98802e80f858d1d9ea3b86996f7037f758990e1566627"
FAST_ARITH_SHA256 = "2a5c54bdedb6a0b1ffcf95dd7badd32c41d1482b53055e31fb9f28d514ac8c9e"
HISTORICAL_BASE = "2c207b41a3be3552c1754ee78540f13be258a105"
EXAMPLES = (
    "koblitz_base_window",
    "koblitz_orbit_dlp_s3_batch",
    "koblitz_rho_batch_ks_v3",
)


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write_json(path: Path, value: dict) -> None:
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")


def materialize_historical(root: Path, dest: Path, freeze: dict) -> dict:
    tracked = subprocess.check_output(
        ["git", "ls-tree", "-r", "--name-only", HISTORICAL_BASE], cwd=root, text=True
    ).splitlines()
    paths = [path for path in tracked
             if path == "Cargo.toml" or path.startswith(("src/", "examples/", "tests/",
                                                          "benches/", "docs/"))]
    archive = subprocess.check_output(
        ["git", "archive", HISTORICAL_BASE, "--", *paths], cwd=root
    )
    baseline = {}
    with tarfile.open(fileobj=io.BytesIO(archive), mode="r:") as packed:
        members = packed.getmembers()
        for member in members:
            name = Path(member.name)
            assert not name.is_absolute() and ".." not in name.parts
        packed.extractall(dest, members=members)
    for path in paths:
        item = dest / path
        if item.is_file():
            baseline[path] = sha(item)

    snapshots = load_manifest()
    replaced = {}
    live = []
    for relative, digest in sorted(freeze["source_sha256"].items()):
        item = dest / relative
        current = sha(item) if item.is_file() else None
        if current == digest:
            live.append(relative)
            continue
        data = snapshot_bytes(relative, digest, snapshots)
        item.parent.mkdir(parents=True, exist_ok=True)
        item.write_bytes(data)
        replaced[relative] = {"frozen_sha256": digest, "baseline_sha256": current}
    for relative, digest in freeze["source_sha256"].items():
        assert sha(dest / relative) == digest, relative
    return {"schema": "ecc2k130-historical-build-source-v1",
            "baseline_commit": HISTORICAL_BASE,
            "baseline_sha256": baseline,
            "pinned_files": len(freeze["source_sha256"]),
            "live": live,
            "replayed_from_snapshot": replaced}


def build(out: Path, profile: str, offline: bool,
          historical_source_root: Path | None = None,
          source_selection_report: Path | None = None) -> dict:
    assert profile in ("dev", "release")
    assert not out.exists(), "never overwrite a frozen build"
    assert not out.is_relative_to(ROOT), "build directory must be outside the checkout"
    assert sha(SOURCE_FREEZE) == SOURCE_FREEZE_SHA256
    if historical_source_root is None and source_selection_report is None:
        assert sha(FROZEN_LOCK) == FROZEN_LOCK_SHA256
    cfg = json.loads((HERE / "CONFIG.json").read_text())
    source_freeze = json.loads(SOURCE_FREEZE.read_text())
    assert source_freeze["source_sha256"]["examples/koblitz_orbit_dlp_s3_batch.rs"] == (
        cfg["compact_source_sha256"]
    )
    assert source_freeze["source_sha256"]["examples/koblitz_rho_batch_ks_v3.rs"] == (
        cfg["rho_source_sha256"]
    )
    out.mkdir(parents=True)
    source = out / "source"
    source_freeze = json.loads(SOURCE_FREEZE.read_text())
    snapshot = materialize_historical(ROOT, source, source_freeze)
    write_json(out / "materialization.json", snapshot)
    shutil.copy2(ROOT / "examples/koblitz_base_window.rs",
                 source / "examples/koblitz_base_window.rs")
    selection_provenance = None
    if historical_source_root is not None or source_selection_report is not None:
        assert historical_source_root is not None and source_selection_report is not None
        selection = json.loads(source_selection_report.read_text())
        assert selection["target_root"] == str(historical_source_root.resolve())
        pins = {
            "src/cryptanalysis/koblitz_fast_arith.rs": FAST_ARITH_SHA256,
            "research/notes/ecc2k130/compact_shared_log_20260925/Cargo.lock": FROZEN_LOCK_SHA256,
        }
        rows = {row["path"]: row for row in selection["selections"]}
        assert set(rows) == set(pins)
        for relative, expected in pins.items():
            row = rows[relative]
            selected = historical_source_root.resolve() / relative
            assert row["expected_sha256"] == expected
            assert row["materialized_sha256"] == expected
            assert len(row["source_commit"]) == 40
            committed = subprocess.check_output(
                ["git", "show", f"{row['source_commit']}:{relative}"], cwd=ROOT
            )
            assert hashlib.sha256(committed).hexdigest() == expected
            assert sha(selected) == expected
        shutil.copy2(historical_source_root / "src/cryptanalysis/koblitz_fast_arith.rs",
                     source / "src/cryptanalysis/koblitz_fast_arith.rs")
        shutil.copy2(
            historical_source_root / "research/notes/ecc2k130/compact_shared_log_20260925/Cargo.lock",
            source / "Cargo.lock",
        )
        selection_provenance = {
            "selection_report_sha256": sha(source_selection_report),
            "source_root": str(historical_source_root.resolve()),
            "selected_files": [
                {"path": relative, "sha256": row["materialized_sha256"],
                 "source_commit": row["source_commit"]}
                for relative, row in sorted(rows.items())
            ],
        }
    else:
        shutil.copy2(FROZEN_LOCK, source / "Cargo.lock")
    for name, expected in source_freeze["source_sha256"].items():
        assert sha(source / name) == expected, name
    assert sha(source / "src/cryptanalysis/koblitz_fast_arith.rs") == FAST_ARITH_SHA256
    assert sha(source / "Cargo.lock") == FROZEN_LOCK_SHA256
    assert sha(source / "examples/koblitz_base_window.rs") == sha(
        ROOT / "examples/koblitz_base_window.rs"
    )
    command = ["cargo", "build", "--locked"]
    if offline:
        command.append("--offline")
    if profile == "release":
        command.append("--release")
    for name in EXAMPLES:
        command.extend(("--example", name))
    environment = dict(os.environ)
    environment["CARGO_TARGET_DIR"] = str(out / "target")
    receipt = {
        "schema": "ecc2k130-base-window-frozen-build-v1",
        "status": "RUNNING",
        "builder_source_sha256": sha(Path(__file__).resolve()),
        "profile": profile,
        "offline": offline,
        "command": command,
        "source_freeze_sha256": SOURCE_FREEZE_SHA256,
        "materialization_sha256": sha(out / "materialization.json"),
        "pinned_files": snapshot["pinned_files"],
        "generator_source_sha256": sha(source / "examples/koblitz_base_window.rs"),
        "compact_source_sha256": cfg["compact_source_sha256"],
        "rho_source_sha256": cfg["rho_source_sha256"],
        "fast_arith_sha256": FAST_ARITH_SHA256,
        "historical_source_selection": selection_provenance,
        "historical_replay_status": "selected_historical_sources_build_only" if selection_provenance else "legacy_snapshot_build",
        "current_code_parity_status": "not_assessed",
        "cargo_lock_sha256": FROZEN_LOCK_SHA256,
        "rustc_version_verbose": subprocess.check_output(
            ["rustc", "--version", "--verbose"], text=True
        ).strip(),
        "cargo_version": subprocess.check_output(["cargo", "--version"], text=True).strip(),
    }
    write_json(out / "BUILD_RECEIPT.json", receipt)
    stdout = out / "build.stdout.txt"
    stderr = out / "build.stderr.txt"
    try:
        with stdout.open("xb") as output, stderr.open("xb") as errors:
            code = subprocess.call(
                command, cwd=source, env=environment, stdout=output, stderr=errors
            )
        receipt["exit_code"] = code
        receipt["build_stdout_sha256"] = sha(stdout)
        receipt["build_stderr_sha256"] = sha(stderr)
        if code != 0:
            receipt["status"] = "FAIL"
        else:
            directory = out / "target" / ("release" if profile == "release" else "debug") / "examples"
            receipt["binaries"] = {
                name: {
                    "path": str(directory / name),
                    "sha256": sha(directory / name),
                    "bytes": (directory / name).stat().st_size,
                }
                for name in EXAMPLES
            }
            receipt["status"] = "PASS"
    except BaseException as error:
        receipt["status"] = "FAIL"
        receipt["error_type"] = type(error).__name__
        receipt["error"] = str(error)
        receipt["traceback"] = traceback.format_exc()
        write_json(out / "BUILD_RECEIPT.json", receipt)
        raise
    write_json(out / "BUILD_RECEIPT.json", receipt)
    return receipt


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--profile", choices=("dev", "release"), default="release")
    parser.add_argument("--offline", action="store_true")
    parser.add_argument("--historical-source-root", type=Path)
    parser.add_argument("--source-selection-report", type=Path)
    args = parser.parse_args()
    result = build(args.out.resolve(), args.profile, args.offline,
                   args.historical_source_root, args.source_selection_report)
    print(json.dumps({
        "status": result["status"],
        "profile": result["profile"],
        "binaries": result.get("binaries"),
    }, sort_keys=True))
    if result["status"] != "PASS":
        raise SystemExit(1)


if __name__ == "__main__":
    main()
