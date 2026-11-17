#!/usr/bin/env python3
"""Bounded CLI export checks against a pinned, previously generated fixture."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import resource
import signal
import subprocess
import time

PINNED_RECEIPT = "e3f0d56284a7c32c90ead0f0ad9049801c809b2362f8c8caf07e2baf064e9d3f"


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write(path, value):
    with path.open("x") as stream:
        json.dump(value, stream, sort_keys=True, indent=2)
        stream.write("\n")


def source_inventory(root):
    assert digest(root / "receipt.json") == PINNED_RECEIPT
    receipt = json.loads((root / "receipt.json").read_text())
    expected = {row["path"] for row in receipt["inventory"]} | {"receipt.json"}
    actual = set()
    for path in root.rglob("*"):
        assert not path.is_symlink(), path
        if path.is_file():
            actual.add(str(path.relative_to(root)))
    assert actual == expected, (actual - expected, expected - actual)
    for row in receipt["inventory"]:
        path = root / row["path"]
        assert path.resolve().is_relative_to(root)
        assert path.stat().st_size == row["bytes"] and digest(path) == row["sha256"], row["path"]
    return len(receipt["inventory"])


def scratch_bytes(root):
    total = 0
    def walk_error(error):
        if not isinstance(error, FileNotFoundError):
            raise error
    for directory, _, files in os.walk(root, onerror=walk_error):
        for name in files:
            try:
                total += (Path(directory) / name).stat().st_size
            except FileNotFoundError:
                # Export atomically removes its staging directory on completion.
                continue
    return total


def command(binary, source, destination, scratch, number, expected_exit, records):
    argv = [str(binary), "dataset", "--color", "never", "export", "--out", str(source), "--destination", str(destination)]
    env = {"PATH": "/usr/bin:/bin", "LANG": "C", "LC_ALL": "C", "RAYON_NUM_THREADS": "1"}
    started = time.monotonic()
    def limits():
        signal.pthread_sigmask(signal.SIG_SETMASK, old_mask)
        resource.setrlimit(resource.RLIMIT_AS, (512 * 1024**2, 512 * 1024**2))
        resource.setrlimit(resource.RLIMIT_FSIZE, (16 * 1024**2, 16 * 1024**2))
    log = scratch / f"command-{number}.log"
    child = None
    write(scratch / f"command-{number}.started.json", {"argv": argv, "expected_exit": expected_exit})
    try:
        with log.open("xb") as output:
            old_mask = signal.pthread_sigmask(signal.SIG_BLOCK, {signal.SIGINT, signal.SIGTERM})
            try:
                child = subprocess.Popen(argv, env=env, stdout=output, stderr=subprocess.STDOUT, start_new_session=True, preexec_fn=limits)
            finally:
                signal.pthread_sigmask(signal.SIG_SETMASK, old_mask)
            while child.poll() is None:
                if time.monotonic() - started > 60:
                    raise TimeoutError("export command exceeded60seconds")
                output_bytes = scratch_bytes(scratch)
                if output_bytes > 64 * 1024**2:
                    raise RuntimeError("polled fixture output exceeded64MiB")
                time.sleep(0.05)
            exit_code = child.wait()
            if scratch_bytes(scratch) > 64 * 1024**2:
                raise RuntimeError("fixture output exceeded64MiB at command exit")
    finally:
        if child is not None and child.poll() is None:
            os.killpg(child.pid, signal.SIGKILL)
            child.wait()
    row = {"argv": argv, "exit_code": exit_code, "expected_exit": expected_exit,
           "log": log.name, "log_sha256": digest(log), "elapsed_seconds_diagnostic": time.monotonic() - started}
    records.append(row)
    write(scratch / f"command-{number}.receipt.json", row)
    assert exit_code == expected_exit, row


def check_graph(source, destination, twist):
    manifest = json.loads((destination / "manifest.json").read_text())
    assert manifest["feature_policy"]["version"] == "strict-structural-allowlist-v4"
    assert manifest["prime_chain_graph_occurrences"] == 1
    for artifact in manifest["artifacts"]:
        path = destination / artifact["path"]
        assert path.resolve().is_relative_to(destination) and not path.is_symlink()
        assert path.stat().st_size == artifact["bytes"] and digest(path) == artifact["sha256"]
    rows = [json.loads(line) for line in (destination / "features.jsonl").read_text().splitlines()]
    assert len(rows) == 1
    row = rows[0]
    assert row["inputs"]["twist"] is twist
    inputs = row["inputs"]
    canonical = json.dumps(inputs, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()
    assert hashlib.sha256(canonical).hexdigest() == row["feature_id"]
    graph = inputs["prime_chain_graph"]
    assert graph["schema"] == "prime-chain-graph/v1"
    assert graph["encoding"] == "odd-prime-affine-s3-chain/v1"
    cell = next((source / "cells").iterdir())
    original = json.loads((cell / "prime-f4-input.json").read_text())
    p = int(original["modulus"])
    def field(value):
        assert isinstance(value, str) and value.isascii() and value.isdecimal()
        assert str(int(value)) == value and 0 <= int(value) < p
        return int(value)
    assert graph["field"]["modulus"] == original["modulus"]
    for value in [*graph["curve"].values(), *graph["target"].values()]:
        field(value)
    finite = [field(v) for v in graph["domains"][0]["values"]]
    assert graph["domains"][1] == {"id": 1, "kind": "all_Fp"}
    assert finite == original["domains"][0]
    assert all(domain == finite for domain in original["domains"][:original["m"]])
    assert all(domain is None for domain in original["domains"][original["m"]:])
    monomials = [node["exponents"] for node in graph["monomials"]]
    recovered = [dict() for _ in graph["equations"]]
    for edge in graph["equation_monomial_edges"]:
        exponent = tuple(monomials[edge["monomial"]])
        assert exponent not in recovered[edge["equation"]]
        recovered[edge["equation"]][exponent] = field(edge["coefficient"])
    expected = [{tuple(term["exponents"]): int(term["coefficient"]) for term in polynomial} for polynomial in original["polynomials"]]
    assert recovered == expected
    assert len(graph["points"]) == len(original["signed_factor_base"])
    for node, point in zip(graph["points"], original["signed_factor_base"]):
        assert {"x": str(field(node["x"])), "y": str(field(node["y"]))} == point
    public_bytes = (destination / "features.jsonl").read_text()
    for forbidden in ["known_answer_scalar", "private_witness", "solver_completed", "native_backend_invoked", "source_commit", "curve_seed"]:
        assert forbidden not in public_bytes
    assert (destination / "restricted/targets.jsonl").is_file()
    assert (destination / "restricted/index.jsonl").is_file()
    return {"twist": twist, "feature_id": row["feature_id"], "equations": len(recovered), "manifest_sha256": digest(destination / "manifest.json")}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--binary-sha256", required=True)
    parser.add_argument("--source-commit", required=True)
    parser.add_argument("--source-fixture", type=Path, required=True)
    parser.add_argument("--scratch", type=Path, required=True)
    args = parser.parse_args()
    binary, source = args.binary.resolve(strict=True), args.source_fixture.resolve(strict=True)
    assert len(args.source_commit) == 40 and all(c in "0123456789abcdef" for c in args.source_commit)
    assert digest(binary) == args.binary_sha256
    fixture = Path(__file__).resolve().parent
    frozen = json.loads((fixture / "fixture-manifest.json").read_text())
    for row in frozen["files"]:
        assert digest(fixture / row["path"]) == row["sha256"]
    assert source_inventory(source) == 158
    scratch = args.scratch.resolve()
    assert not scratch.is_relative_to(source) and not source.is_relative_to(scratch)
    scratch.mkdir(parents=False, exist_ok=False)
    records, graphs = [], []
    receipt = {"schema": "ml-dataset.graph-export-cli-receipt/v1", "binary_sha256": args.binary_sha256,
               "supplied_build_source_commit": args.source_commit, "build_source_requires_separate_attestation": True,
               "source_fixture_receipt_sha256": PINNED_RECEIPT, "commands": records, "graphs": graphs,
               "corpus_rows_added": 0, "verified_cloud_dataset_cells": 0, "clock_valid_for_comparison": False}
    try:
        for shard, twist in [(62, False), (63, True)]:
            destination = scratch / f"export-{shard}"
            command(binary, source / f"shard-{shard}", destination, scratch, len(records) + 1, 0, records)
            result = check_graph(source / f"shard-{shard}", destination, twist)
            command(binary, source / f"shard-{shard}", destination, scratch, len(records) + 1, 0, records)
            assert digest(destination / "manifest.json") == result["manifest_sha256"]
            graphs.append(result)
        rejected = scratch / "rejected"
        command(binary, source / "tampered-copy", rejected, scratch, 5, 1, records)
        assert not (rejected / "manifest.json").exists()
        assert source_inventory(source) == 158
        assert digest(binary) == args.binary_sha256
        receipt["verdict"] = "passed_software_export_checks"
    except BaseException as error:
        receipt["verdict"] = "failed"
        receipt["error"] = str(error)
        raise
    finally:
        write(scratch / "receipt.json", receipt)


if __name__ == "__main__":
    def interrupted(signum, frame):
        raise KeyboardInterrupt(f"signal{signum}")
    signal.signal(signal.SIGINT, interrupted)
    signal.signal(signal.SIGTERM, interrupted)
    main()
