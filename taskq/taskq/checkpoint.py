"""Atomic publication of an application checkpoint on the shared results volume.

The application owns its state format and when to snapshot it. ``taskq
checkpoint save FILE`` publishes a stable snapshot and a SHA-256 marker;
workers resume only from a matching marker from an earlier attempt.
"""
from __future__ import annotations

import hashlib
import os
import shutil
import tempfile
from pathlib import Path


def save(source: Path, destination: Path) -> str:
    """Publish an immutable generation, then atomically commit its marker.

    The stable destination remains available to callers, but resumption uses
    the generation named by the marker's digest. Updating that stable copy
    can therefore never invalidate a previously committed checkpoint.
    """
    destination.parent.mkdir(parents=True, exist_ok=True)
    marker = destination.with_name(destination.name + ".sha256")
    try:
        previous = marker.read_text().strip()
    except (OSError, UnicodeError):
        previous = ""
    fd, tmp = tempfile.mkstemp(prefix=f".{destination.name}.", dir=destination.parent)
    pending = {tmp}
    digest = hashlib.sha256()
    try:
        with os.fdopen(fd, "wb") as out, source.open("rb") as inp:
            for chunk in iter(lambda: inp.read(1 << 20), b""):
                out.write(chunk)
                digest.update(chunk)
            out.flush()
            os.fsync(out.fileno())
        checksum = digest.hexdigest()
        generation = destination.with_name(f".{destination.name}.{checksum}.snapshot")
        try:
            os.link(tmp, generation)
        except FileExistsError:
            existing = hashlib.sha256()
            with generation.open("rb") as inp:
                for chunk in iter(lambda: inp.read(1 << 20), b""):
                    existing.update(chunk)
            if existing.hexdigest() != checksum:
                raise OSError(f"checkpoint generation is corrupt: {generation}")
        _sync_directory(destination.parent)
        fd, marker_tmp = tempfile.mkstemp(prefix=f".{marker.name}.", dir=marker.parent)
        pending.add(marker_tmp)
        with os.fdopen(fd, "w") as out:
            out.write(checksum + "\n")
            out.flush()
            os.fsync(out.fileno())
        os.replace(marker_tmp, marker)
        _sync_directory(destination.parent)

        # Use a separate inode for the convenient stable copy: an application
        # may still write that path in place, which must not alter a generation.
        fd, alias_tmp = tempfile.mkstemp(prefix=f".{destination.name}.", dir=destination.parent)
        pending.add(alias_tmp)
        with os.fdopen(fd, "wb") as out, generation.open("rb") as inp:
            shutil.copyfileobj(inp, out, 1 << 20)
            out.flush()
            os.fsync(out.fileno())
        os.replace(alias_tmp, destination)
        _sync_directory(destination.parent)
        # Bound checkpoint storage, retaining the preceding committed generation
        # as well as the current one. An already-open reader keeps its inode.
        prefix, suffix = f".{destination.name}.", ".snapshot"
        for old in destination.parent.iterdir():
            if old.name.startswith(prefix) and old.name.endswith(suffix):
                old_digest = old.name[len(prefix):-len(suffix)]
                if (len(old_digest) == 64
                        and all(c in "0123456789abcdef" for c in old_digest)
                        and old_digest not in {checksum, previous}):
                    old.unlink(missing_ok=True)
        return checksum
    finally:
        for path in pending:
            if os.path.exists(path):
                os.unlink(path)


def _sync_directory(path: Path) -> None:
    directory = os.open(path, os.O_RDONLY)
    try:
        os.fsync(directory)
    finally:
        os.close(directory)


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
