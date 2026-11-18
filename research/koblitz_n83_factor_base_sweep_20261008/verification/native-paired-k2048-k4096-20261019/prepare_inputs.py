#!/usr/bin/env python3
"""Stage and hash-check the exact two retained N83 orbit bases and fixtures."""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import shutil
import subprocess

HERE = Path(__file__).resolve().parent
STUDY = HERE.parents[1]
REPO = HERE.parents[3]
SOURCES = (
    "examples/koblitz_n83_factor_base_export.rs",
    "research/koblitz_n83_factor_base_sweep_20261008/primary_adapter.rs",
    "research/koblitz_n83_factor_base_sweep_20261008/size-frontier-v2.json",
    "src/cryptanalysis/binary_semaev_chain_sat.rs",
    "src/cryptanalysis/koblitz_index_calculus.rs",
)
PANEL_SOURCES = {
    2048: STUDY / "verification/k2048-natural-highcap-20261018/evidence/frozen-input/panel",
    4096: STUDY / "verification/k4096-orbit-model-20261018/evidence/panel",
}
FIXTURES = STUDY / "verification/k4096-orbit-model-20261018/evidence/fixtures"


def require(ok: bool, reason: str) -> None:
    if not ok:
        raise ValueError(reason)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def checked_copy(source: Path, target: Path, digest: str) -> None:
    require(sha256(source) == digest, f"frozen source SHA-256 differs: {source}")
    target.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(source, target)
    require(sha256(target) == digest, f"staged SHA-256 differs: {target}")


def stage(object_paths: dict[int, Path], out: Path, commit: str,
          clean_source: bool) -> dict:
    frozen = json.loads((STUDY / "verification/cloud-matched-k2048-k4096-20261018/input/fresh-inputs.json").read_text())
    rows = {row["columns"]: row for row in frozen["bases"]}
    require(set(rows) == {2048, 4096}, "frozen input manifest size set differs")
    if clean_source:
        head = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=REPO,
                                       text=True).strip()
        status = subprocess.check_output(["git", "status", "--porcelain"],
                                         cwd=REPO, text=True)
        require(head == commit and not status,
                "worker source checkout is not the declared clean commit")
    staged = []
    for k in (2048, 4096):
        row = rows[k]
        panel = out / f"panel-k{k}"
        panel.mkdir(parents=True, exist_ok=False)
        for name, digest in row["panel_sha256"].items():
            source = PANEL_SOURCES[k] / name
            if name == "probe-corpus.json" and not source.is_file():
                source = FIXTURES / name
            checked_copy(source, panel / name, digest)
        object_name = row["blake3"] + ".jsonl.gz"
        checked_copy(object_paths[k], panel / "objects" / object_name, row["sha256"])
        require((panel / "objects" / object_name).stat().st_size == row["bytes"],
                f"K{k} object byte length differs")
        staged.append({"columns": k, "object": object_name,
                       "sha256": row["sha256"], "bytes": row["bytes"]})
    fixture_out = out / "fixtures"
    fixture_out.mkdir(parents=True, exist_ok=False)
    for name, digest in frozen["fixture_sha256"].items():
        checked_copy(FIXTURES / name, fixture_out / name, digest)
    snapshot = out / "source-snapshot"
    snapshot.mkdir(parents=True, exist_ok=False)
    source_sha = {}
    for name in SOURCES:
        source = REPO / name
        content = source.read_bytes()
        if clean_source:
            reference = subprocess.check_output(["git", "show", f"{commit}:{name}"],
                                                cwd=REPO)
            require(content == reference, f"pushed source differs: {name}")
        target = snapshot / name
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(content)
        source_sha[name] = hashlib.sha256(content).hexdigest()
    attestation = {
        "schema": "n83.chain-s3-source-attestation/v1",
        "status_clean": clean_source,
        "source_commit": commit,
    }
    (snapshot / "attestation.json").write_text(
        json.dumps(attestation, sort_keys=True) + "\n")
    return {"schema": "n83.native-paired-preflight/v1",
            "status": "PASS_FROZEN_INPUTS", "source_commit": commit,
            "source_clean": clean_source, "bases": staged,
            "source_sha256": source_sha,
            "selected_best_total_runtime": None}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--object-k2048", type=Path, required=True)
    parser.add_argument("--object-k4096", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--commit", required=True)
    parser.add_argument("--clean-source", action="store_true")
    args = parser.parse_args()
    require(len(args.commit) == 40 and
            all(ch in "0123456789abcdef" for ch in args.commit),
            "source commit is not a full SHA-1")
    result = stage({2048: args.object_k2048, 4096: args.object_k4096},
                   args.out, args.commit, args.clean_source)
    with (args.out / "preflight.json").open("x") as stream:
        json.dump(result, stream, sort_keys=True, indent=2)
        stream.write("\n")
    print(json.dumps(result, sort_keys=True))


if __name__ == "__main__":
    main()
