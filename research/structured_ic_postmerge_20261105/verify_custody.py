#!/usr/bin/env python3
"""Read-only comparisons of pinned Git objects; no execution attestation."""
from pathlib import Path
import hashlib
import json
import os
import subprocess

HERE = Path(__file__).resolve().parent
ROOTS = {
    "crypto": Path("/workspace/crypto-integration"),
    "cryptanalysis": Path("/workspace/cryptanalysis"),
    "crypto-autoresearcher": Path("/workspace/crypto-autoresearcher"),
}
REVS = json.loads((HERE / "audit.json").read_text())["revisions"]


def git(repo, *args):
    return subprocess.check_output(["git", "-C", str(ROOTS[repo]), *args])


def blob(repo, ref, path):
    return git(repo, "show", ref + ":" + path)


def digest(data):
    return hashlib.sha256(data).hexdigest()


shared = ["src/structured.rs", "src/structured_runner.rs",
          "src/bin/tensor-structured.rs", "tests/structured.rs",
          "tests/structured_cli.rs", "examples/structured-prime.json"]
parity = []
for path in shared:
    a = blob("crypto", REVS["crypto"], "crates/hashed-ecdl-tensor/" + path)
    b = blob("cryptanalysis", REVS["cryptanalysis"], "bindings/rust/hashed-ecdl-tensor/" + path)
    original = blob("crypto", "fff0182d77747b826fcc84443bde0a1c9a529509",
                    "crates/hashed-ecdl-tensor/" + path)
    assert a == b == original, path
    parity.append({"path": path, "sha256": digest(a), "original_and_current_match": True})

archive = "b7955c3d13834c076b7b345c44b4912bfdb03d4b"
receipt = json.loads(blob("crypto-autoresearcher", REVS["crypto-autoresearcher"],
                          "docs/reports/structured-prime-ic-20261104/archive-receipt.json"))
assert receipt["commit_sha"] == archive
git("crypto-autoresearcher", "merge-base", "--is-ancestor", archive, REVS["crypto-autoresearcher"])
archived = []
for path, expected in receipt["path_sha256"].items():
    pinned = blob("crypto-autoresearcher", archive, path)
    assert digest(pinned) == expected, path
    live = blob("crypto-autoresearcher", REVS["crypto-autoresearcher"], path)
    same = pinned == live
    # The historical journal remains pinned at archive; its live copy is append-only.
    if path != "docs/session-logs/2026-11-04.md":
        assert same, path
    archived.append({"path": path, "archive_sha256": digest(pinned),
                     "matches_receipt": True, "current_path_matches": same})

originals = [
    ("crypto", "fff0182d77747b826fcc84443bde0a1c9a529509", "research/structured_ic_integration_20261104"),
    ("cryptanalysis", "1c87f6887fe1047ce81a84bbf33cc97ed91e1cd7", "docs/ic/structured-prime-ic-20261104"),
]
reports = []
for repo, ref, directory in originals:
    paths = git(repo, "ls-tree", "-r", "--name-only", ref, "--", directory).decode().splitlines()
    for path in paths:
        assert blob(repo, ref, path) == blob(repo, REVS[repo], path), path
    reports.append({"repo": repo, "source_commit": ref, "paths_unchanged": len(paths)})

claims = []
for repo, old, claim in [
    ("crypto", "fff0182d77747b826fcc84443bde0a1c9a529509", "PCC-20261104-458dda"),
    ("cryptanalysis", "1c87f6887fe1047ce81a84bbf33cc97ed91e1cd7", "PCC-20261104-f011a3"),
]:
    path = "pcc/claims/" + claim + ".json"
    original = blob(repo, old, path)
    assert original == blob(repo, REVS[repo], path), claim
    claims.append({"id": claim, "body_sha256": digest(original), "unchanged": True})

suite = "crates/hashed-ecdl-tensor/src/lib.rs"
suite_now = blob("cryptanalysis", REVS["cryptanalysis"], suite)
suite_integrated = blob("cryptanalysis", "1c87f6887fe1047ce81a84bbf33cc97ed91e1cd7", suite)
assert suite_now == suite_integrated
suite_before = blob("cryptanalysis", "1c87f6887fe1047ce81a84bbf33cc97ed91e1cd7^", suite)
addition = b"/// Structured-predicate backend with its own explicitly named curve/types.\npub use structured_tensor as structured_backend;\n/// Versioned JSON interface shared with the bindings structured backend.\npub use structured_tensor::structured_runner;\n\n"
assert suite_now.replace(addition, b"", 1) == suite_before

merges = {
    "crypto": "eab6ed334715504767bee9b7c60f1ba72b789182",
    "cryptanalysis": "e3d07ed87a8bed34c2a1f9bac0bfa21081ba81d5",
    "crypto-autoresearcher": "4c8d17df817608ba83af77d1e1d0d4ee95088fb1",
}
for repo, commit in merges.items():
    git(repo, "merge-base", "--is-ancestor", commit, REVS[repo])

changes = {r: set(git(r, "diff", "--name-only", commit + "^1", commit).decode().splitlines())
           for r, commit in merges.items()}
name_inputs = ["docs/curves/registry.json", "src/cryptanalysis/ecbench/registry_index.txt",
               "scripts/check_curve_names.py", "scripts/build_curve_registry.py"]
unrelated_current = ["src/ml_dataset/custody.rs",
                     "research/pencil_large_prime_20261029/src/native_solver.rs",
                     "taskq/deploy/ephemeral/cli/src/main.rs"]
record_inputs = ["docs/benchmarks/records/20261104T1602Z-prime-field-source-control.json",
                 "research/prime_field_a5_execution_20261104/reports/a5-source-control-c0c8a9/report.pdf"]
assert not changes["crypto"].intersection(name_inputs + unrelated_current)
assert not changes["crypto-autoresearcher"].intersection(record_inputs)
attribution = {
    "crypto_names_inputs_unchanged_by_integration": name_inputs,
    "current_crypto_clippy_inputs_unchanged_by_integration": unrelated_current,
    "autoresearch_records_inputs_unchanged_by_integration": record_inputs,
    "release_failure": "remote HTTP 500 during artifact/Git transfer; no native build diagnostic in that failed step",
}

result = {"standing": "producer_assertion", "revisions": REVS, "merged_commits_reachable": merges,
          "six_file_parity": parity, "archive_commit": archive, "archived_paths": archived,
          "original_reports": reports, "original_claims": claims,
          "suite_original_exports_retained": True, "ci_attribution": attribution,
          "independent_execution_attestation": False}
output = Path(os.environ.get("STRUCTURED_IC_AUDIT_OUTPUT", str(HERE)))
output.mkdir(parents=True, exist_ok=True)
(output / "custody.json").write_text(json.dumps(result, indent=2) + "\n")
print(json.dumps({"six_file_parity": len(parity), "archived_paths_matching_receipt": len(archived),
                  "original_reports": reports, "original_claims_unchanged": len(claims),
                  "suite_original_exports_retained": True, "standing": "producer_assertion"}))
