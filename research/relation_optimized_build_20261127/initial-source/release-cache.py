"""Reversible release of exact compiler-cache files; metadata/hashes only."""
from pathlib import Path
import datetime
import hashlib
import json
import os
import tarfile

study = Path(__file__).resolve().parent
base = Path("/dev/shm")
inventory = json.loads((study / "CACHE_FILE_INVENTORY.json").read_text())
validation = json.loads((study / "CACHE_BACKUP_VALIDATION.json").read_text())
guard = json.loads((study / "FROZEN_WORKER_GUARD_BEFORE.json").read_text())
expected = inventory["files"]
archive = Path(validation["backup_path"])

def digest(p):
    h = hashlib.sha256()
    with p.open("rb") as f:
        for data in iter(lambda: f.read(1 << 20), b""):
            h.update(data)
    return h.hexdigest()

assert digest(archive) == validation["backup_sha256"]
assert validation["all_roster_bytes_modes_match"] is True
assert inventory["total_bytes"] - archive.stat().st_size >= 1073741824
selected = {str(base / p) for p in expected}
unknown_other_uids = []
zombies = []
for proc in Path("/proc").iterdir():
    if not proc.name.isdigit():
        continue
    try:
        lines = (proc / "status").read_text().splitlines()
        state = next(x.split(":", 1)[1].strip() for x in lines if x.startswith("State:"))
        if state.startswith("Z"):
            zombies.append(proc.name)
            continue
        if proc.stat().st_uid != os.geteuid():
            unknown_other_uids.append(proc.name)
            continue
        comm = (proc / "comm").read_text().strip()
        assert comm not in ["cargo", "rustc"], (proc.name, comm)
        maps = (proc / "maps").read_text()
        assert not any(p in maps for p in selected), proc.name
        for fd in (proc / "fd").iterdir():
            try:
                assert os.readlink(fd) not in selected, (proc.name, str(fd))
            except FileNotFoundError:
                pass
    except FileNotFoundError:
        pass
for path, sha in guard["files"].items():
    assert digest(Path(path)) == sha, path
for path, metadata in expected.items():
    p = base / path
    assert p.stat().st_uid == os.geteuid() and p.stat().st_nlink == 1
    assert p.stat().st_mode & 0o777 == metadata["mode"]
    assert not p.stat().st_mode & 0o111
    assert digest(p) == metadata["sha256"], path
before = os.statvfs(base).f_bavail * os.statvfs(base).f_frsize
for path in expected:
    (base / path).unlink()
after = os.statvfs(base).f_bavail * os.statvfs(base).f_frsize
reclaimed = after - before
restored = False
if reclaimed < 1073741824:
    # No overwrite of another writer: restoration requires every selected path absent.
    with tarfile.open(archive, "r|xz") as tf:
        for member in tf:
            if member.isdir():
                continue
            assert member.isfile() and member.name in expected
            destination = base / member.name
            assert not destination.exists(), str(destination)
            with destination.open("xb") as f:
                while data := tf.extractfile(member).read(1 << 20):
                    f.write(data)
            destination.chmod(expected[member.name]["mode"])
    for path, metadata in expected.items():
        assert digest(base / path) == metadata["sha256"]
    restored = True
for path, sha in guard["files"].items():
    assert digest(Path(path)) == sha, path
result = {
    "schema": "reversible-cache-release-outcome/v1",
    "at_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
    "files": len(expected), "backup_path": str(archive),
    "backup_sha256": validation["backup_sha256"],
    "actual_filesystem_bytes_reclaimed": reclaimed,
    "protected_worker_hashes_unchanged": len(guard["files"]),
    "qualified_source_or_data_changed": False,
    "other_uid_process_visibility_unknown": unknown_other_uids,
    "zombie_contexts_ignored": zombies,
    "global_no_process_reference_claim": False,
    "restored_due_to_insufficient_reclamation": restored,
    "eligible_for_declared_compile_followup": not restored,
}
(study / "CACHE_RELEASE_OUTCOME.json").write_text(json.dumps(result, indent=2) + "\n")
print(json.dumps(result))
assert not restored, "insufficient actual headroom; cache restored"
