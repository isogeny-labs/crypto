#!/usr/bin/env python3
"""Inventory tracked research paths and lexical Lean pointers at one Git commit.

This does not build Lean or decide whether a theorem states a research claim.
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
from collections import defaultdict
from pathlib import Path


DECLARATION = re.compile(
    r"^\s*(?:(?:private|protected|noncomputable)\s+)*"
    r"(theorem|lemma)\s+([^\s:(\[{}]+)"
)
ROOTS = ("research", "experiments", "benchmarks")


def git(*args: str, input: bytes | None = None) -> bytes:
    return subprocess.run(
        ["git", *args], input=input, check=True, stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    ).stdout


def tree_entries(commit: str) -> tuple[list[tuple[str, str]], list[dict]]:
    raw = git("ls-tree", "-r", "-z", commit)
    rows: list[tuple[str, str]] = []
    non_blobs: list[dict] = []
    seen: set[str] = set()
    for entry in raw.split(b"\0"):
        if not entry:
            continue
        metadata, path = entry.split(b"\t", 1)
        mode, kind, oid = metadata.decode("ascii").split(" ")
        name = path.decode("utf-8", errors="surrogateescape")
        if name in seen:
            raise ValueError(f"duplicate tracked path: {name!r}")
        seen.add(name)
        if kind == "blob":
            rows.append((name, oid))
        else:
            non_blobs.append({"path": name, "mode": mode, "kind": kind, "oid": oid})
    return rows, non_blobs


def blob_contents(oids: list[str]) -> dict[str, bytes]:
    if not oids:
        return {}
    raw = git("cat-file", "--batch", input=("\n".join(oids) + "\n").encode())
    result: dict[str, bytes] = {}
    pos = 0
    for oid in oids:
        end = raw.index(b"\n", pos)
        actual, kind, size_text = raw[pos:end].decode("ascii").split(" ")
        size = int(size_text)
        if actual != oid or kind != "blob":
            raise ValueError(f"unexpected Git object for {oid}")
        start = end + 1
        result[oid] = raw[start:start + size]
        if len(result[oid]) != size or raw[start + size:start + size + 1] != b"\n":
            raise ValueError(f"incomplete Git blob {oid}")
        pos = start + size + 1
    if pos != len(raw):
        raise ValueError("unexpected trailing cat-file output")
    return result


def inventory(commitish: str) -> dict:
    commit = git("rev-parse", "--verify", f"{commitish}^{{commit}}").decode().strip()
    tree = git("rev-parse", f"{commit}^{{tree}}").decode().strip()
    entries, non_blobs = tree_entries(commit)
    directories: dict[tuple[str, str], list[str]] = defaultdict(list)
    lean_entries: list[tuple[str, str]] = []
    for path, oid in entries:
        parts = path.split("/")
        if len(parts) >= 3 and parts[0] in ROOTS:
            directories[(parts[0], parts[1])].append(path)
        if path.endswith(".lean"):
            lean_entries.append((path, oid))
    blobs = blob_contents(sorted({oid for _, oid in lean_entries}))
    lean_files = []
    for path, oid in lean_entries:
        source = blobs[oid].decode("utf-8", errors="replace")
        declarations = []
        for line_number, line in enumerate(source.splitlines(), 1):
            match = DECLARATION.match(line)
            if match:
                declarations.append({
                    "kind": match.group(1), "name": match.group(2),
                    "line": line_number,
                })
        lean_files.append({"path": path, "blob_oid": oid, "declarations": declarations})
    lean_files.sort(key=lambda row: row["path"])
    by_directory: dict[tuple[str, str], list[dict]] = defaultdict(list)
    for row in lean_files:
        parts = row["path"].split("/")
        if len(parts) >= 3 and parts[0] in ROOTS:
            by_directory[(parts[0], parts[1])].append(row)
    directory_rows = [
        {
            "root": root,
            "directory": f"{root}/{name}",
            "tracked_file_count": len(directories[(root, name)]),
            "lean_paths": [row["path"] for row in by_directory[(root, name)]],
            "lexical_declaration_count": sum(len(row["declarations"]) for row in by_directory[(root, name)]),
        }
        for root, name in sorted(directories)
    ]
    counts_by_root = {
        root: {
            "directories": sum(row["root"] == root for row in directory_rows),
            "with_colocated_lean": sum(row["root"] == root and bool(row["lean_paths"]) for row in directory_rows),
            "without_colocated_lean": sum(row["root"] == root and not row["lean_paths"] for row in directory_rows),
        }
        for root in ROOTS
    }
    return {
        "schema": "formal-coverage-inventory/v1",
        "source_commit": commit,
        "source_tree": tree,
        "method": "complete git ls-tree -r -z; git cat-file --batch for .lean blobs",
        "interpretation": "Path co-location and regex declaration pointers only; no Lean build or claim-fidelity judgment.",
        "roots": ROOTS,
        "counts": {
            "tracked_blobs": len(entries),
            "tracked_non_blob_entries": len(non_blobs),
            "tracked_study_directories": len(directory_rows),
            "tracked_study_directories_with_colocated_lean": sum(bool(row["lean_paths"]) for row in directory_rows),
            "tracked_study_directories_without_colocated_lean": sum(not row["lean_paths"] for row in directory_rows),
            "by_root": counts_by_root,
            "lean_files_total": len(lean_files),
            "lean_files_under_study_roots": sum(row["path"].split("/")[0] in ROOTS for row in lean_files),
            "lexical_theorem_lemma_pointers": sum(len(row["declarations"]) for row in lean_files),
        },
        "study_directories": directory_rows,
        "lean_files": lean_files,
        "non_blob_entries": non_blobs,
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("commit", help="Git commit to freeze, not a moving branch name")
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    result = inventory(args.commit)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2, ensure_ascii=True) + "\n")
    print(json.dumps(result["counts"], sort_keys=True))


if __name__ == "__main__":
    main()
