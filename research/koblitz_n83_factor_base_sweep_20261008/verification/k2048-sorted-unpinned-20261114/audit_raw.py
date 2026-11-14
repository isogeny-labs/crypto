#!/usr/bin/env python3
"""Audit the sorted K2048 control's retained original formulas and terminals."""

import gzip
import hashlib
import json
from pathlib import Path


HERE = Path(__file__).resolve().parent
EVIDENCE = HERE / "evidence"
LOCK = json.loads((HERE / "LOCK.json").read_text())


def require(condition, message):
    if not condition:
        raise ValueError(message)


def sha(path, compressed=False):
    digest = hashlib.sha256()
    opener = gzip.open if compressed else open
    with opener(path, "rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def load(section, name):
    return json.loads((EVIDENCE / section / name).read_text())


def terminal(section):
    with gzip.open(EVIDENCE / section / "stdout.txt.gz", "rt", errors="replace") as stream:
        transcript = stream.read()
    statuses = [line for line in transcript.splitlines() if line.startswith("s ")]
    conflicts = [line for line in transcript.splitlines()
                 if line.startswith("c conflicts                :")]
    return transcript, statuses, conflicts


def main():
    export = EVIDENCE / "export"
    pinned = EVIDENCE / "pinned-solver"
    unpinned = EVIDENCE / "unpinned-solver"
    for section, expected in (("export", {"pinned.xcnf.gz", "unpinned.xcnf.gz"}),
                              ("pinned-solver", {"stdout.txt.gz"})):
        pointer = EVIDENCE / section / ".artifacts.json"
        if pointer.exists():
            manifest = json.loads(pointer.read_text())
            require(manifest["schema"] == "artifacts.manifest/v1" and
                    set(manifest["files"]) == expected, "artifact pointer identity")
            for name, entry in manifest["files"].items():
                path = EVIDENCE / section / name
                require(path.exists() and path.stat().st_size == entry["bytes"] and
                        sha(path) == entry["sha256"],
                        "artifact pointer or hydrated bytes")
    pinned_export = load("export", "pinned.json")
    unpinned_export = load("export", "unpinned.json")
    known = load("export", "known-model-audit.json")
    pinned_run = load("pinned-solver", "receipt.json")
    replay = load("pinned-solver", "replay.json")
    run = load("unpinned-solver", "receipt.json")
    require(sha(export / "pinned.xcnf.gz", True) ==
            pinned_export["xcnf_sha256"] ==
            "579a3719fd54ae600632db3ef2a5e0bd458ff5f6c6b3d4524a2eed6d3e4e3d08",
            "pinned original formula")
    require(sha(export / "unpinned.xcnf.gz", True) == LOCK["formula_sha256"] and
            sha(export / "unpinned.json") == LOCK["export_receipt_sha256"] and
            sha(export / "known-model-audit.json") == LOCK["known_model_audit_sha256"],
            "unpinned original formula and receipts")
    require(pinned_export["mode"] == "positive-sorted-pinned" and
            unpinned_export["mode"] == "positive-sorted-unpinned" and
            pinned_export["target"] == unpinned_export["target"] == LOCK["target"] and
            pinned_export["positive"] == unpinned_export["positive"] and
            pinned_export["shape"]["positive_x_pins"] == 747 and
            unpinned_export["shape"]["positive_x_pins"] == 0 and
            pinned_export["shape"]["summand_ordering"] ==
            unpinned_export["shape"]["summand_ordering"] == "all_nondecreasing_x",
            "matched sorted export identity")
    require(known["status"] == "PASS_ORIGINAL_XCNF" and
            known["xcnf_sha256"] == LOCK["formula_sha256"] and
            (known["checked_cnf"], known["checked_xor"]) == (1_141_545, 4_980),
            "known model original-formula replay")
    require(sha(pinned / "replay.json") == LOCK["pinned_replay_sha256"] and
            replay["status"] == "PASS_ORIGINAL_XCNF_GROUP_AND_ROW_REPLAY" and
            replay["checked_archive_rows"] == 339_968 and
            replay["columns"] == [1, 3, 4, 0, 2] and
            replay["target"] == LOCK["target"], "pinned independent group replay")
    require(sha(pinned / "stdout.txt.gz", True) ==
            pinned_run["stdout_sha256"] == LOCK["pinned_stdout_sha256"] and
            sha(pinned / "stderr.txt") == pinned_run["stderr_sha256"],
            "pinned solver transcript")
    pinned_text, pinned_statuses, _ = terminal("pinned-solver")
    require(pinned_statuses == ["s SATISFIABLE"] and pinned_run["exit_code"] == 10 and
            any(line.startswith("v ") for line in pinned_text.splitlines()),
            "pinned terminal model")
    require(load("pinned-solver", "command.json") ==
            [LOCK["solver"], *LOCK["solver_flags"], pinned_export["xcnf_path"]],
            "pinned solver command")
    require(load("unpinned-solver", "command.json") ==
            [LOCK["solver"], *LOCK["solver_flags"], LOCK["formula"]],
            "unpinned solver command")
    samples = load("unpinned-solver", "host-preflight.json")
    require(samples == run["host_preflight"] and len(samples) == 3 and
            all(item["load_1m"] <= 24 for item in samples) and
            len({item["swapouts"] for item in samples}) == 1 and
            run["swapouts_after"] == samples[0]["swapouts"], "host gate")
    require(sha(unpinned / "stdout.txt.gz", True) == run["stdout_sha256"] and
            sha(unpinned / "stderr.txt") == run["stderr_sha256"],
            "unpinned solver transcript")
    transcript, statuses, conflicts = terminal("unpinned-solver")
    require(statuses == ["s INDETERMINATE"] and len(conflicts) == 1 and
            int(conflicts[0].split(":", 1)[1]) == 1_000_001 and
            not any(line.startswith("v ") for line in transcript.splitlines()),
            "unpinned terminal and model")
    require(run["status"] == "INDETERMINATE" and run["exit_code"] == 15 and
            run["terminal_conflicts"] == 1_000_001 and not run["timed_out"] and
            run["formula_sha256"] == LOCK["formula_sha256"] and
            run["solver_sha256"] == LOCK["solver_sha256"] and
            run["lock_sha256"] == sha(HERE / "LOCK.json") and
            run["verified_relation"] is None and run["novel_rank"] is None,
            "unpinned terminal receipt")
    files = sorted(path for path in EVIDENCE.rglob("*") if path.is_file() and
                   path.name not in {"audit.json", ".artifacts.json", ".gitignore"})
    hashes = {str(path.relative_to(EVIDENCE)): sha(path) for path in files}
    result = {
        "schema": "n83.k2048-sorted-unpinned-raw-audit/v1",
        "status": "PASS_CUSTODY_AND_TERMINAL_CLASSIFICATION",
        "pinned_model_status": replay["status"],
        "known_model_status": known["status"],
        "unpinned_terminal": run["status"],
        "unpinned_conflicts": run["terminal_conflicts"],
        "file_sha256": hashes,
    }
    output = EVIDENCE / "audit.json"
    if output.exists():
        require(json.loads(output.read_text()) == result, "audit idempotence")
    else:
        output.write_text(json.dumps(result, sort_keys=True, indent=2) + "\n")
    print(json.dumps({key: result[key] for key in result if key != "file_sha256"},
                     sort_keys=True))


if __name__ == "__main__":
    main()
