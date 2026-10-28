#!/usr/bin/env python3
"""Reproduce the dated R0 report with its reviewed producer, without resealing."""
from pathlib import Path
import hashlib
import json
import re
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
ARCHIVE = "docs/backend-leaderboard/revisions/20261027-pre-method-registry"
LOCK = ARCHIVE + "/reproduction.lock.json"
SNAPSHOT_LOCK = "docs/backend-leaderboard/reproduction.lock.json"
SNAPSHOT = "91424ecf8cc7db04e7c4a2805c9001a02a6106ca"
LOCK_SHA256 = "5385e70d724341e07922412b007b42e0ce685fde2c3c68f462bc7dd6f8721278"
# These are the producer/build inputs declared by the sealed native producer.
# Scientific session inputs and generated report bytes never receive an exemption.
PRODUCER_INPUTS = {
    "tools/backend-leaderboard/" + name for name in (
        "Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "build.rs",
        "src/main.rs", "src/evidence.rs", "src/leaderboard.rs",
        "src/leaderboard.html", "src/reproduction.rs")
} | {
    "src/cryptanalysis/ecbench/" + name for name in
    ("band.rs", "canonical.rs", "stats.rs", "methods.rs")
} | {"src/cryptanalysis/curve_aliases.json", "docs/ic/calibration.json",
     "docs/ecbench/BACKEND_REPRODUCTION.md"}


def sha256(data):
    return hashlib.sha256(data).hexdigest()


def checked_path(root, name):
    relative = Path(name)
    if relative.is_absolute() or ".." in relative.parts:
        raise ValueError(f"unsafe frozen report path: {name}")
    path = root / relative
    if path.is_symlink() or not path.resolve().is_relative_to(root.resolve()):
        raise ValueError(f"frozen report input escapes checkout: {name}")
    return path


def verify_live_evidence(root, lock):
    if not PRODUCER_INPUTS <= set(lock["files"]):
        raise ValueError("frozen producer inventory incomplete")
    for name, expected in lock["files"].items():
        if name not in PRODUCER_INPUTS:
            if sha256(checked_path(root, name).read_bytes()) != expected:
                raise ValueError(f"frozen scientific input changed: {name}")
    for name, expected in lock["outputs"].items():
        path = checked_path(root, ARCHIVE + "/" + name)
        if sha256(path.read_bytes()) != expected:
            raise ValueError(f"frozen report output changed: {name}")


def run(args, cwd):
    subprocess.run(args, cwd=cwd, check=True)


def main():
    lock_bytes = (ROOT / LOCK).read_bytes()
    snapshot_lock = subprocess.check_output(["git", "show", SNAPSHOT + ":" + SNAPSHOT_LOCK], cwd=ROOT)
    if sha256(lock_bytes) != LOCK_SHA256 or snapshot_lock != lock_bytes:
        raise ValueError("reviewed frozen report lock changed; do not reseal")
    lock = json.loads(lock_bytes)
    verify_live_evidence(ROOT, lock)
    with tempfile.TemporaryDirectory(prefix="backend-r0-") as temporary:
        checkout = Path(temporary) / "snapshot"
        run(["git", "worktree", "add", "--detach", "--no-checkout",
             str(checkout), SNAPSHOT], ROOT)
        try:
            paths = set(lock["files"]) | {SNAPSHOT_LOCK, ".gitignore"}
            paths |= {"docs/backend-leaderboard/" + p for p in lock["outputs"]}
            patterns = ["/" + re.sub(r"([\\*?\[\]!# ])", r"\\\1", p)
                        for p in sorted(paths)]
            subprocess.run(["git", "sparse-checkout", "set", "--no-cone", "--stdin"],
                           input="\n".join(patterns) + "\n", text=True,
                           cwd=checkout, check=True)
            run(["git", "read-tree", "-mu", "HEAD"], checkout)
            for name in lock["outputs"]:
                relative = "docs/backend-leaderboard/" + name
                path = checkout / relative
                if not path.exists():
                    # The current trusted artifact pull already hash-checked the
                    # ignored report; the frozen native checker checks it again.
                    path.parent.mkdir(parents=True, exist_ok=True)
                    shutil.copyfile(checked_path(ROOT, ARCHIVE + "/" + name), path)
            print(f"R0 snapshot {SNAPSHOT}; lock SHA-256 {LOCK_SHA256}", flush=True)
            cargo = ["cargo", "+1.99.1", "run", "--locked", "--manifest-path",
                     "tools/backend-leaderboard/Cargo.toml", "--"]
            run(cargo + ["--check"], checkout)
            run(cargo + ["reproduce", "--expected-lock-sha256", LOCK_SHA256], checkout)
            verify_live_evidence(ROOT, lock)
        finally:
            run(["git", "worktree", "remove", "--force", str(checkout)], ROOT)


if __name__ == "__main__":
    main()
