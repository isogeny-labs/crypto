#!/usr/bin/env python3
"""Replay frozen native bytes with their recorded input, after a live-model check.

The original scripts, producers, auditors, assertions and controls are unchanged.
Each replay uses an isolated detached worktree with exact source copies and the
recorded registry. The current checkout and every frozen output stay untouched.
"""

import argparse
from contextlib import contextmanager
import hashlib
import json
from pathlib import Path
import signal
import shutil
import subprocess
import tempfile


PIN = "3681cea07a38418e5ef40bfda3466f1e6ec2b447"
PIN_SHA256 = "87724546ad2e971572c4f62ae0fbc4b2bfed8cd028ae30ded78bc4c4bb9287ca"
REGISTRY = "docs/curves/registry.json"
STUDY = "research/quadratic_twist_evidence_20261021"
FROZEN = {
    "mixed": "native/mixed-20261022T0718Z/results.json",
    "pairing": "native/pairing-20261024T0442Z/results.json",
    "s3-yield": "native/s3-yield-20261024/cases/index.json",
}
# These four metadata fields are not read by any of the three native producers
# or independent auditors. Every other field of each consumed model must match.
NON_INPUT_FIELDS = {"sources", "aliases", "representations", "ec1_unresolved"}


def digest(data):
    return hashlib.sha256(data).hexdigest()


def population(data):
    selected = {}
    for row in data["curves"]:
        if row.get("family") != "prime":
            continue
        try:
            key = tuple(int(row["params"][k]) for k in ("p", "a", "b"))
        except (KeyError, TypeError, ValueError):
            continue
        p, a, b = key
        finite = (p in (5, 7, 11, 13, 17, 19) and 0 <= a < p and
                  0 <= b < p and (4 * a**3 + 27 * b**2) % p != 0)
        if not (finite or key in ((43, 1, 0), (43, 4, 0))):
            continue
        if key in selected:
            raise ValueError(f"duplicate finite model: {key}")
        selected[key] = {k: v for k, v in row.items() if k not in NON_INPUT_FIELDS}
    if len(selected) != 944:
        raise ValueError(f"expected all 942 models plus both F43 fixtures, got {len(selected)}")
    return selected


def check_current(frozen, current):
    old = population(json.loads(frozen))
    new = population(json.loads(current))
    if old != new:
        raise ValueError("current finite-population identity/model binding changed")
    encoded = json.dumps(sorted(old.items()), sort_keys=True, separators=(",", ":")).encode()
    return {"model_records_compared": len(old), "model_record_sha256": digest(encoded)}


def historical_registry(root):
    git = ["git", "--no-replace-objects", "-C", str(root)]
    exists = subprocess.run(git + ["cat-file", "-e", PIN + "^{commit}"],
                            stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    if exists.returncode:
        subprocess.run(git + ["fetch", "--no-tags", "--no-write-fetch-head",
                              "--depth=1", "--filter=blob:none", "origin", PIN],
                       check=True, timeout=180)
    frozen = subprocess.check_output(git + ["show", PIN + ":" + REGISTRY], timeout=180)
    if digest(frozen) != PIN_SHA256:
        raise ValueError("historical registry bytes do not match the frozen provenance")
    return frozen


@contextmanager
def replay_root(root, frozen, temporary_parent):
    """Copy the complete dependency closure, including hydrated frozen files."""
    git = ["git", "-C", str(root)]
    with tempfile.TemporaryDirectory(prefix="twist-replay-", dir=temporary_parent) as tmp:
        isolated = Path(tmp) / "source"
        subprocess.run(git + ["worktree", "add", "--detach", "--no-checkout",
                              str(isolated), "HEAD"], check=True)
        try:
            files = [REGISTRY, "src/cryptanalysis/identity_certificate.rs",
                     "src/hash/sha256.rs", "src/binary_ecc/f2m.rs",
                     "src/binary_ecc/operation_tally.rs",
                     "docs/identity-certificates/schoenhage-10mult.json",
                     "docs/ic/calibration.json"]
            files += [f"{STUDY}/run-{phase}.sh" for phase in FROZEN]
            files += [f"{STUDY}/native/{name}" for name in
                      ("mixed.cpp", "pairing.cpp", "s3-yield.cpp", "full.cpp", "json.hpp")]
            for relative in files:
                dest = isolated / relative
                dest.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(root / relative, dest)
            directories = ["lean", "native/mixed-audit", "native/pairing-audit",
                           "native/s3-yield-audit", "native/mixed-20261022T0718Z",
                           "native/pairing-20261024T0442Z", "native/s3-yield-20261024"]
            for relative in directories:
                shutil.copytree(root / STUDY / relative, isolated / STUDY / relative,
                                symlinks=True,
                                ignore=shutil.ignore_patterns("target", ".lake", "__pycache__"))
            (isolated / REGISTRY).write_bytes(frozen)
            yield isolated
        finally:
            # Only this function's newly created temporary worktree is removed.
            # All command outputs and failure receipts are outside it.
            subprocess.run(git + ["worktree", "remove", "--force", str(isolated)], check=True)


def interrupted(signum, _frame):
    raise SystemExit(128 + signum)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("phase", choices=FROZEN)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    root = Path(subprocess.check_output(["git", "rev-parse", "--show-toplevel"], text=True).strip())
    out = args.output.resolve()
    if out.exists():
        raise ValueError("refusing to replace an earlier replay directory")
    recorded = json.loads((root / STUDY / FROZEN[args.phase]).read_bytes())
    if recorded["registry_sha256"] != PIN_SHA256:
        raise ValueError("unexpected frozen replay registry binding")
    current = (root / REGISTRY).read_bytes()
    frozen = historical_registry(root)
    binding = check_current(frozen, current)
    receipt = {"schema": "native-frozen-registry-input/v1", "standing": "unverified",
               "phase": args.phase, "historical_commit": PIN,
               "source_commit": subprocess.check_output(
                   ["git", "-C", str(root), "rev-parse", "HEAD"], text=True).strip(),
               "execution_root": "isolated detached worktree",
               "historical_registry_sha256": digest(frozen),
               "current_registry_sha256": digest(current), **binding}
    receipts = out.parent / "twist-registry-inputs"
    receipts.mkdir(parents=True, exist_ok=True)
    receipt_file = receipts / (out.name + ".json")
    if receipt_file.exists():
        raise ValueError("refusing to replace an earlier input receipt")
    code = None
    signal.signal(signal.SIGTERM, interrupted)
    signal.signal(signal.SIGINT, interrupted)
    try:
        with replay_root(root, frozen, out.parent) as isolated:
            result = subprocess.run(["bash", str(isolated / STUDY / ("run-" + args.phase + ".sh")),
                                     str(out)], cwd=isolated)
            code = result.returncode
    finally:
        receipt["command_exit"] = code
        receipt["command_completed"] = code is not None
        receipt["unchanged_current_registry_sha256"] = digest((root / REGISTRY).read_bytes())
        receipt_file.write_text(json.dumps(receipt, indent=2) + "\n")
        if (root / REGISTRY).read_bytes() != current:
            raise RuntimeError("current registry changed during isolated replay")
    raise SystemExit(code)


if __name__ == "__main__":
    main()
