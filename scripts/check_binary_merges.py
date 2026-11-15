#!/usr/bin/env python3
"""Fail when a merge commit combines two versions of a binary file.

A binary file cannot be merged line by line. When a merge conflict on one is
resolved by keeping both sides (a "union" resolution), the result is the two
versions glued together: still a file, sometimes still readable by a tolerant
reader, but neither version. #2674's merge 617a254f0 did exactly this to
research/cover_transfer_assessment_20261013/cover_transfer_assessment_20261013.pdf
(main's PDF + "\\n" + the branch's PDF), and no other check noticed.

Rule: in every merge commit in BASE..HEAD, a binary file whose parents hold
different versions must come out byte-identical to one of them. A merge
that rebuilt the file from the merged sources says so in its message with one
trailer line per file:

    Binary-Regenerated: path/to/file.pdf

Only files the merge changed relative to every parent are inspected (the
conflict resolutions), found from blob ids alone, so a partial clone
(--filter=blob:none) downloads just those blobs.

    python3 scripts/check_binary_merges.py BASE HEAD
"""
from __future__ import annotations

import argparse
import re
import subprocess
import sys

TRAILER = "Binary-Regenerated"
NULL_SHA = "0" * 40


def git(*args: str) -> str:
    return subprocess.run(["git", *args], check=True, capture_output=True, text=True).stdout


def git_bytes(*args: str) -> bytes:
    return subprocess.run(["git", *args], check=True, capture_output=True).stdout


def merge_commits(base: str, head: str) -> list[str]:
    return git("rev-list", "--merges", "--reverse", f"{base}..{head}").split()


def parents_of(commit: str) -> list[str]:
    return git("rev-list", "--parents", "-n", "1", commit).split()[1:]


def declared_regenerated(commit: str) -> set[str]:
    message = git("log", "-1", "--format=%B", commit)
    return {m.group(1) for m in re.finditer(rf"(?m)^{TRAILER}:\s*(\S.*?)\s*$", message)}


def raw_changes(parent: str, commit: str) -> dict[str, tuple[str, str, str]]:
    """path -> (status, blob in parent, blob in commit) for every path that differs."""
    out = git_bytes("diff-tree", "-r", "--no-renames", "--raw", "-z", parent, commit).split(b"\0")
    changes: dict[str, tuple[str, str, str]] = {}
    for meta, path in zip(out[0::2], out[1::2]):
        if not meta:
            continue
        fields = meta.decode().split()
        changes[path.decode()] = (fields[4][0], fields[2], fields[3])
    return changes


def is_binary(parent: str, commit: str, path: str) -> bool:
    """Git's own verdict (attributes, then content): numstat prints '-' for binary."""
    out = git("diff", "--numstat", "--no-renames", parent, commit, "--", path)
    return any(line.startswith("-\t-\t") for line in out.splitlines())


def blob_bytes(commit: str, path: str) -> bytes | None:
    result = subprocess.run(["git", "cat-file", "blob", f"{commit}:{path}"], capture_output=True)
    return result.stdout if result.returncode == 0 else None


def looks_concatenated(result: bytes, versions: list[bytes]) -> bool:
    for a in versions:
        for b in versions:
            if a is not b and result in (a + b, a + b"\n" + b):
                return True
    return False


def violations(commit: str) -> list[str]:
    parents = parents_of(commit)
    per_parent = [raw_changes(p, commit) for p in parents]
    # Changed relative to every parent: the merge produced new bytes here.
    resolved = set(per_parent[0]).intersection(*per_parent[1:])
    allowed = declared_regenerated(commit)
    found = []
    for path in sorted(resolved):
        statuses = [changes[path] for changes in per_parent]
        if statuses[0][2] == NULL_SHA:  # deleted by the merge
            continue
        holders = [p for p, (status, _, _) in zip(parents, statuses) if status != "A"]
        # Only a real conflict over the file: the parents hold different versions.
        # (Parents that agree leave nothing to combine; a merge that edits such a
        # file is an ordinary change, reviewed like any other.)
        if len({old for status, old, _ in statuses if status != "A"}) < 2 or path in allowed:
            continue
        if not is_binary(parents[0], commit, path):
            continue
        detail = ""
        result = blob_bytes(commit, path)
        versions = [v for v in (blob_bytes(p, path) for p in holders) if v is not None]
        if result is not None and looks_concatenated(result, versions):
            detail = " It is two parent versions concatenated, the signature of a union resolution."
        found.append(
            f"{commit[:12]} {path}: the merge result matches none of its parents' versions "
            f"({', '.join(h[:12] for h in holders)}).{detail} Take one side "
            f"(git checkout <parent> -- {path}) and regenerate the file in a normal commit, "
            f"or, if the merge rebuilt it from the merged sources, add the trailer "
            f"'{TRAILER}: {path}' to the merge commit message."
        )
    return found


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("base", help="commit the range starts after (e.g. the PR base)")
    parser.add_argument("head", help="last commit of the range (e.g. the PR head)")
    args = parser.parse_args(argv)
    merges = merge_commits(args.base, args.head)
    problems = [v for commit in merges for v in violations(commit)]
    for problem in problems:
        print(f"::error::{problem}")
    print(f"checked {len(merges)} merge commit(s) in {args.base[:12]}..{args.head[:12]}: "
          f"{len(problems)} binary file(s) combined by a merge")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
