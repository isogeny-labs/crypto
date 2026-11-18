#!/usr/bin/env python3
"""Parse committed src Rust files with rustfmt without rewriting the checkout."""
from __future__ import annotations

import argparse
from pathlib import Path
import shutil
import subprocess
import tempfile


def check(repo: Path, ref: str = "HEAD") -> tuple[int, int, str]:
    formatter = shutil.which("rustfmt")
    if formatter is None:
        raise RuntimeError("rustfmt is required for the Rust syntax gate")
    entries = subprocess.check_output(
        ["git", "ls-tree", "-r", "-z", ref, "--", "src"], cwd=repo
    ).split(b"\0")
    sources = []
    for entry in entries:
        if not entry:
            continue
        metadata, name = entry.split(b"\t", 1)
        relative = name.decode("utf-8")
        if not relative.endswith(".rs"):
            continue
        mode, kind, oid = metadata.decode("ascii").split()
        if mode not in {"100644", "100755"} or kind != "blob":
            raise RuntimeError(f"unsupported committed Rust source mode: {relative}")
        sources.append((relative, oid))
    if not sources:
        raise RuntimeError("no committed src Rust files found")

    with tempfile.TemporaryDirectory(prefix="rust-syntax-gate-") as temporary:
        directory = Path(temporary)
        reader = subprocess.Popen(
            ["git", "cat-file", "--batch"], cwd=repo,
            stdin=subprocess.PIPE, stdout=subprocess.PIPE,
        )
        assert reader.stdin is not None and reader.stdout is not None
        paths = []
        try:
            for relative, oid in sources:
                reader.stdin.write((oid + "\n").encode("ascii"))
                reader.stdin.flush()
                header = reader.stdout.readline().decode("ascii").split()
                if len(header) != 3 or header[:2] != [oid, "blob"]:
                    raise RuntimeError(f"cannot read exact committed blob: {relative}")
                size = int(header[2])
                data = reader.stdout.read(size)
                if len(data) != size or reader.stdout.read(1) != b"\n":
                    raise RuntimeError(f"incomplete committed blob: {relative}")
                path = directory / relative
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes(data)
                paths.append(str(path))
        finally:
            reader.stdin.close()
            reader.stdout.close()
            reader.wait()
        result = subprocess.run(
            [formatter, "--edition", "2021", "--emit", "stdout",
             "--config", "skip_children=true", *paths],
            stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, text=True,
        )
        return result.returncode, len(paths), result.stderr.replace(str(directory) + "/", "")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, default=Path.cwd())
    parser.add_argument("--ref", default="HEAD")
    args = parser.parse_args()
    try:
        code, count, diagnostic = check(args.repo.resolve(), args.ref)
    except (OSError, RuntimeError, subprocess.CalledProcessError) as error:
        print(f"Rust syntax gate failed: {error}")
        return 1
    if diagnostic:
        print(diagnostic, end="" if diagnostic.endswith("\n") else "\n")
    print(f"Rust syntax gate: {count} committed src Rust files, {'PASS' if code == 0 else 'FAIL'}")
    return 0 if code == 0 else 1


if __name__ == "__main__":
    raise SystemExit(main())
