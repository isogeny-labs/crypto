#!/usr/bin/env python3
"""Read-only source audit; writes only a new inventory, never repairs files."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import stat
import subprocess


def git(root, *args):
    return subprocess.check_output(["git", *args], cwd=root)


def regular_git_mode(path):
    status = path.lstat()
    if not stat.S_ISREG(status.st_mode):
        raise ValueError(f"not a regular non-symlink file: {path}")
    return "100755" if status.st_mode & stat.S_IXUSR else "100644"


def read_tree(root, source, paths):
    result = {}
    for row in git(root, "ls-tree", "-rz", source, "--", *paths).split(b"\0"):
        if not row:
            continue
        meta, path = row.split(b"\t", 1)
        mode, kind, blob = meta.decode().split()
        if kind != "blob" or mode not in ("100644", "100755"):
            raise ValueError(f"unsupported tree entry: {path!r}")
        result[path.decode()] = (mode, blob)
    if set(result) != set(paths):
        raise ValueError("tree inventory is incomplete or ambiguous")
    return result


def audit(root, source, previous):
    old_bytes = previous.read_bytes()
    old = json.loads(old_bytes)
    if old["schema"] != "exact-source-materialization/v1":
        raise ValueError("expected immutable prior v1 inventory")
    paths = [entry["path"] for entry in old["entries"]]
    if len(set(paths)) != len(paths):
        raise ValueError("duplicate inventory path")
    for path in paths:
        parts = path.split("/")
        if path.startswith("/") or any(part in ("", ".", "..") for part in parts):
            raise ValueError("noncanonical inventory path")
    tree = read_tree(root, source, paths)
    entries = []
    for entry in old["entries"]:
        path = root / entry["path"]
        mode, blob = tree[entry["path"]]
        observed = regular_git_mode(path)
        if observed != mode:
            raise ValueError(f"mode differs: {entry['path']}: {observed} != {mode}")
        # No follow-symlink file open after the lstat admission.
        fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW)
        with os.fdopen(fd, "rb") as stream:
            before = os.fstat(stream.fileno())
            if not stat.S_ISREG(before.st_mode):
                raise ValueError("file replaced with non-regular input")
            if ("100755" if before.st_mode & stat.S_IXUSR else "100644") != mode:
                raise ValueError("mode changed before hashing")
            data = stream.read()
            after = os.fstat(stream.fileno())
        if (before.st_dev, before.st_ino, before.st_mode, before.st_size, before.st_mtime_ns) != (
            after.st_dev, after.st_ino, after.st_mode, after.st_size, after.st_mtime_ns
        ):
            raise ValueError("input changed while hashing")
        actual_blob = hashlib.sha1(b"blob " + str(len(data)).encode() + b"\0" + data).hexdigest()
        sha256 = hashlib.sha256(data).hexdigest()
        if (actual_blob, sha256, len(data)) != (blob, entry["sha256"], entry["bytes"]):
            raise ValueError(f"content differs: {entry['path']}")
        if blob != entry["git_blob"]:
            raise ValueError("prior dependency blob differs from execution source")
        current = path.lstat()
        if (current.st_dev, current.st_ino, current.st_mode, current.st_size, current.st_mtime_ns) != (
            after.st_dev, after.st_ino, after.st_mode, after.st_size, after.st_mtime_ns
        ):
            raise ValueError("path changed after hashing")
        entries.append({**entry, "git_mode": mode})
    # Bind all selected modes/blobs to the same execution tree after scanning.
    if tree != read_tree(root, source, paths):
        raise ValueError("source tree changed")
    result = dict(old)
    result.update(
        schema="exact-source-materialization/v2",
        source_commit=source,
        current_head_dependency_parity_verified=source,
        previous_inventory={"path": str(previous), "sha256": hashlib.sha256(old_bytes).hexdigest()},
        reason="Bind exact Git regular-file modes as well as dependency bytes; mode-only restoration after preflight attempt01; no file repaired by this audit",
        mode_semantics="Git regular-file mode: owner execute bit; not exact POSIX permission equality",
        newly_restored_paths=[],
        entries=entries,
    )
    if len(entries) != old["files"] or sum(e["bytes"] for e in entries) != old["bytes"]:
        raise ValueError("inventory totals differ")
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, required=True)
    parser.add_argument("--source", required=True)
    parser.add_argument("--previous", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    source = git(args.root, "rev-parse", args.source + "^{commit}").decode().strip()
    result = audit(args.root, source, args.previous)
    with args.output.open("x") as stream:
        json.dump(result, stream, indent=2)
        stream.write("\n")
    print(json.dumps({"inventory": str(args.output), "sha256": hashlib.sha256(args.output.read_bytes()).hexdigest(),
        "files": result["files"], "bytes": result["bytes"], "source_commit": source,
        "mode_parity_verified": True, "content_parity_verified": True, "files_modified": 0}))


if __name__ == "__main__":
    main()
