"""Atomic publication of an application checkpoint on the shared results volume.

The application owns its state format and when to snapshot it. ``taskq
checkpoint save FILE`` publishes a stable snapshot and a SHA-256 marker;
workers resume only from a matching marker from an earlier attempt.
"""
from __future__ import annotations

import hashlib
import os
import tempfile
from pathlib import Path


def save(source: Path, destination: Path) -> str:
    """Publish one complete snapshot and return its digest."""
    destination.parent.mkdir(parents=True, exist_ok=True)
    fd, tmp = tempfile.mkstemp(prefix=f".{destination.name}.", dir=destination.parent)
    digest = hashlib.sha256()
    try:
        with os.fdopen(fd, "wb") as out, source.open("rb") as inp:
            for chunk in iter(lambda: inp.read(1 << 20), b""):
                out.write(chunk)
                digest.update(chunk)
            out.flush()
            os.fsync(out.fileno())
        os.replace(tmp, destination)
        marker = destination.with_name(destination.name + ".sha256")
        fd, tmp = tempfile.mkstemp(prefix=f".{marker.name}.", dir=marker.parent)
        with os.fdopen(fd, "w") as out:
            out.write(digest.hexdigest() + "\n")
            out.flush()
            os.fsync(out.fileno())
        os.replace(tmp, marker)
        directory = os.open(destination.parent, os.O_RDONLY)
        try:
            os.fsync(directory)
        finally:
            os.close(directory)
        return digest.hexdigest()
    finally:
        if os.path.exists(tmp):
            os.unlink(tmp)


def main(args: list[str]) -> None:
    if len(args) != 2 or args[0] != "save":
        raise SystemExit("usage: taskq checkpoint save FILE")
    target = os.environ.get("TASKQ_CHECKPOINT_FILE")
    if not target:
        raise SystemExit("taskq checkpoint: task has no durable checkpoint configured")
    source = Path(args[1])
    if not source.is_file():
        raise SystemExit(f"taskq checkpoint: missing source file: {source}")
    save(source, Path(target))
