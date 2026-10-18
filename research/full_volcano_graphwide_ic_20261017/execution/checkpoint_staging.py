"""Copy metadata, optionally hard-link session payloads from an immutable snapshot.

This is file handling only. Never point it at the live campaign. A linked stage
and its source must remain untouched until the archive is closed and verified.
Delete reproducible caches only after retaining the authoritative campaign and
verified final archive. The option saves one raw-tree copy, not snapshot space.
"""
import os
import shutil
import stat
from pathlib import Path


def stage_checkpoint(source, destination, *, link_sessions=False):
    source, destination = Path(source), Path(destination)
    if destination.exists() or destination.is_symlink():
        raise FileExistsError("fresh stage required")
    if source.is_symlink() or not source.is_dir():
        raise ValueError("checkpoint must be a real directory")
    if not (source / "inventory.json").is_file():
        raise ValueError("native checkpoint inventory required")
    # Native snapshots exclude these files. Their presence indicates an input
    # that could be a live/resumed campaign, not the stopped immutable snapshot.
    for name in ("writer.lock", "STOP"):
        if (source / "campaign" / name).exists():
            raise ValueError(f"not an immutable native snapshot: {name}")
    wal = source / "campaign/ledger.sqlite-wal"
    shm = source / "campaign/ledger.sqlite-shm"
    if wal.exists() and wal.stat().st_size:
        raise ValueError("nonempty SQLite WAL: snapshot is not checkpointed")
    if shm.exists() and not wal.exists():
        raise ValueError("orphaned SQLite shared memory")
    if destination.resolve().is_relative_to(source.resolve()):
        raise ValueError("stage cannot be inside its source")
    for root, directories, files in os.walk(source, followlinks=False):
        for name in directories + files:
            path = Path(root) / name
            mode = path.lstat().st_mode
            if not (stat.S_ISDIR(mode) or stat.S_ISREG(mode)):
                raise ValueError(f"non-regular checkpoint entry: {path}")

    linked_files = linked_bytes = 0

    def copy_file(src, dst):
        nonlocal linked_files, linked_bytes
        relative = Path(src).relative_to(source)
        if link_sessions and relative.parts[:2] == ("campaign", "sessions"):
            # No EXDEV fallback: silently copying could exhaust the working
            # volume. An unsupported hard link must fail the publication.
            os.link(src, dst, follow_symlinks=False)
            linked_files += 1
            linked_bytes += Path(src).stat().st_size
            return str(dst)
        return shutil.copy2(src, dst)

    # Opening the stopped ledger read-only can create an empty WAL and SHM.
    # The completed native summarizer leaves these reproducible reader caches;
    # they are not part of the checkpointed ledger or raw session evidence.
    omitted = [name for name in ("ledger.sqlite-wal", "ledger.sqlite-shm")
               if (source / "campaign" / name).exists()]

    def ignore_reader_cache(directory, names):
        return omitted if Path(directory) == source / "campaign" else []

    shutil.copytree(source, destination, copy_function=copy_file, ignore=ignore_reader_cache)
    return {"mode": "link-sessions" if link_sessions else "copy",
            "linked_files": linked_files, "linked_payload_bytes": linked_bytes,
            "omitted_empty_wal_reader_caches": omitted,
            "scope": "File payload bytes, not measured filesystem allocation."}
