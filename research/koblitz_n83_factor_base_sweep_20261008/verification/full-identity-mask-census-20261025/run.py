#!/usr/bin/env python3
"""Reproduce the complete finite N83 identity-mask formula census."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import subprocess
import time


HERE = Path(__file__).resolve().parent
STUDY = HERE.parents[1]
PARENT = HERE.parent / "natural-yield-panel-20261025/protocol.json"
MANIFEST = HERE.parent / "linux-xor-complete-inputs-20261025/manifest.json"


def require(ok: bool, reason: str) -> None:
    if not ok:
        raise ValueError(reason)


def sha(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def file_sha(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def specs(protocol: dict):
    require(protocol["trials"] == [0, 1, 2]
            and protocol["pools"] == ["hash", "gray"]
            and protocol["splits"] == [[5, 1], [4, 2]]
            and protocol["identity_masks"] == list(range(16))
            and protocol["expected_formula_count"] == 192,
            "frozen full identity-mask grid")
    for trial in protocol["trials"]:
        for pool in protocol["pools"]:
            for small, residual in protocol["splits"]:
                for mask in protocol["identity_masks"]:
                    yield {"name": f"{pool}_{small}_{residual}_t{trial}_m{mask}",
                           "trial": trial, "pool": pool, "base_summands": small,
                           "residual_summands": residual, "identity_mask": mask}


def preflight(protocol: dict, parent: dict, inputs: Path, worker: Path,
              source: Path) -> dict:
    require(protocol["schema"] == "n83.full-identity-mask-census-protocol/v1"
            and protocol["selected_best_total_runtime"] is None
            and protocol["per_export_watchdog_seconds"] == 180,
            "frozen census protocol")
    require(platform.system() == "Darwin" and platform.machine() == "arm64",
            "pinned local host class")
    require(file_sha(PARENT) == protocol["parent_panel_protocol_sha256"]
            and file_sha(MANIFEST) == protocol["input_manifest_sha256"],
            "frozen parent panel and S3 manifest bytes")
    require(file_sha(STUDY / "pilot-01/manifest.json")
            == protocol["public_manifest_sha256"]
            and file_sha(STUDY / "pilot-01/probe-corpus.json")
            == protocol["public_corpus_sha256"],
            "exact public source fixtures")
    manifest = json.loads(MANIFEST.read_bytes())
    require(manifest["curve"] == protocol["curve"] == parent["curve"]
            and len(manifest["entries"]) == 11
            and len(parent["formulas"]) == 16, "primary curve and prior panel")
    archive_rows = [row for row in manifest["entries"] if row["kind"] == "archive"]
    require(len(archive_rows) == 3, "three exact S3-backed source archives")
    for row in archive_rows:
        path = inputs / "archives" / Path(row["key"]).name
        require(path.is_file() and path.stat().st_size == row["bytes"]
                and file_sha(path) == row["sha256"],
                f"{path.name}: exact cached S3 archive")
    source_commit = protocol["worker_source_commit"]
    require(parent["worker_source_commit"] == source_commit
            and subprocess.run(["git", "-C", str(source), "cat-file", "-e",
                                f"{source_commit}^{{commit}}"],
                               capture_output=True).returncode == 0,
            "pinned mathematical worker commit")
    changed = subprocess.run(
        ["git", "-C", str(source), "diff", "--name-only", source_commit,
         "HEAD", "--", "examples/koblitz_n83_mixed_orbit_worker.rs",
         "src/cryptanalysis/koblitz_large_prime.rs",
         "src/cryptanalysis/binary_semaev_chain_sat.rs",
         "src/cryptanalysis/sat.rs"], capture_output=True, check=True)
    require(not changed.stdout.strip(), "mathematical source changed since pin")
    require(file_sha(worker) == protocol["worker_binary_sha256_darwin_arm64"],
            "exact source-built local worker binary")
    rows = list(specs(protocol))
    require(len(rows) == 192 and len({row["name"] for row in rows}) == 192,
            "complete unique 192-formula Cartesian product")
    return {"schema": "n83.full-identity-mask-census-preflight/v1",
            "status": "PASS_EXACT_SOURCE_ARCHIVES_AND_GRID",
            "worker_sha256": file_sha(worker),
            "input_manifest_sha256": file_sha(MANIFEST),
            "parent_panel_protocol_sha256": file_sha(PARENT),
            "source_commit": source_commit,
            "selected_best_total_runtime": None}


def run(protocol: dict, parent: dict, inputs: Path, worker: Path, source: Path,
        out: Path, expected: dict | None = None) -> dict:
    admission = preflight(protocol, parent, inputs, worker, source)
    out.mkdir()
    (out / "preflight.json").write_text(json.dumps(admission, indent=2, sort_keys=True) + "\n")
    manifest = STUDY / "pilot-01/manifest.json"
    corpus = STUDY / "pilot-01/probe-corpus.json"
    require(manifest.is_file() and corpus.is_file(), "public source fixtures")
    prior = parent["formulas"]
    rows = []
    started = time.monotonic()
    with (out / "rows.jsonl").open("x") as progress:
        for spec in specs(protocol):
            name = spec["name"]
            pool_hash = parent["pool_objects_blake3"][spec["pool"]]
            base_hash = parent["base_object_blake3"]
            directory = out / name
            command = [str(worker), "export-cross-xcnf-pattern",
                       base_hash, str(manifest),
                       str(inputs / "archives" / f"s3-{base_hash}.jsonl.gz"),
                       pool_hash, str(manifest),
                       str(inputs / "archives" / f"s3-{pool_hash}.jsonl.gz"),
                       str(corpus), str(spec["base_summands"]),
                       str(spec["residual_summands"]), str(spec["trial"]),
                       str(spec["identity_mask"]), str(directory)]
            begin = time.monotonic()
            result = subprocess.run(command, capture_output=True,
                                    timeout=protocol["per_export_watchdog_seconds"])
            elapsed = time.monotonic() - begin
            require(result.returncode == 0, f"{name}: worker export failed: {result.stderr[:300]!r}")
            xcnf = directory / "instance.xcnf"
            metadata_path = directory / "export.json"
            metadata = json.loads(metadata_path.read_bytes())
            digest = file_sha(xcnf)
            require(metadata["schema"] == "n83.cross-stream-pattern-xcnf/v2"
                    and metadata["curve_slug"] == protocol["curve"]
                    and metadata["base_object_blake3"] == base_hash
                    and metadata["pool_object_blake3"] == pool_hash
                    and metadata["trial"] == spec["trial"]
                    and metadata["source_id"] == spec["trial"] + 1
                    and metadata["identity_mask"] == spec["identity_mask"]
                    and metadata["base_summands"] == spec["base_summands"]
                    and metadata["residual_summands"] == spec["residual_summands"]
                    and metadata["order_first_pair"] is True
                    and metadata["xcnf_bytes"] == xcnf.stat().st_size
                    and metadata["selected_best_total_runtime"] is None,
                    f"{name}: exact export metadata")
            row = dict(spec, sha256=digest, bytes=xcnf.stat().st_size,
                       variables=metadata["variables"],
                       original_clauses=metadata["original_clauses"],
                       xor_rows=metadata["xor_rows"],
                       coef_a=metadata["coef_a"], coef_b=metadata["coef_b"],
                       group_target=metadata["group_target"],
                       metadata_sha256=file_sha(metadata_path),
                       stdout_sha256=sha(result.stdout),
                       stderr_sha256=sha(result.stderr),
                       export_wall_seconds_L0=elapsed)
            if name in prior:
                require(all(row[key] == prior[name][key]
                            for key in ("sha256", "bytes", "variables",
                                        "original_clauses", "xor_rows", "coef_a",
                                        "coef_b")),
                        f"{name}: earlier frozen panel digest and shape")
            if expected is not None:
                old = expected["rows"][len(rows)]
                require(all(row[key] == old[key] for key in row
                            if key != "export_wall_seconds_L0"),
                        f"{name}: repeat full-census row replay")
            progress.write(json.dumps(row, sort_keys=True) + "\n")
            progress.flush()
            os.fsync(progress.fileno())
            rows.append(row)
            xcnf.unlink()
    require(len(rows) == protocol["expected_formula_count"], "complete census")
    result = {"schema": "n83.full-identity-mask-census/v1",
              "protocol_sha256": file_sha(HERE / "protocol.json"),
              "worker_sha256": admission["worker_sha256"],
              "input_manifest_sha256": admission["input_manifest_sha256"],
              "formula_count": len(rows), "rows": rows,
              "summed_export_wall_seconds_L0": sum(row["export_wall_seconds_L0"] for row in rows),
              "elapsed_panel_wall_seconds_L0": time.monotonic() - started,
              "solver_processes": 0,
              "selected_best_total_runtime": None}
    (out / "census.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    return result


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--inputs", type=Path, required=True)
    parser.add_argument("--worker", type=Path, required=True)
    parser.add_argument("--source", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--expected", type=Path)
    parser.add_argument("--preflight-only", action="store_true")
    args = parser.parse_args()
    protocol = json.loads((HERE / "protocol.json").read_bytes())
    parent = json.loads(PARENT.read_bytes())
    admission = preflight(protocol, parent, args.inputs, args.worker, args.source)
    if args.preflight_only:
        print(json.dumps(admission, sort_keys=True))
    else:
        expected = json.loads(args.expected.read_bytes()) if args.expected else None
        summary = run(protocol, parent, args.inputs, args.worker, args.source,
                      args.out, expected)
        print(json.dumps({key: summary[key] for key in
                          ("schema", "formula_count", "summed_export_wall_seconds_L0",
                           "elapsed_panel_wall_seconds_L0", "solver_processes",
                           "selected_best_total_runtime")}, sort_keys=True))
