"""Exact, reversible compiler-cache bookkeeping only; never numerical work."""
from pathlib import Path
import hashlib
import json
import os
import stat
import subprocess
import tarfile

s = Path(__file__).resolve().parent
plan = json.loads((s / "PLAN.json").read_text())
archive = Path("/dev/shm/relation-opt-cache-backup-20261127.tar.xz")

def digest(path):
    h = hashlib.sha256()
    with path.open("rb") as f:
        while chunk := f.read(1024 * 1024):
            h.update(chunk)
    return h.hexdigest()

def workers():
    roots = ["relation-root-workers-20261127", "relation-call-instrument-workers-20261127",
             "relation-scan-call-workers-20261127"]
    return {str(p): digest(p) for root in roots for p in sorted((Path("/dev/shm") / root).iterdir())
            if p.is_file() and p.stat().st_mode & 0o111}

assert not (s / "CACHE_RELEASE.json").exists()
assert archive.stat().st_size == 166994936
assert digest(archive) == "8c461673b6a5444793b0d62daa304ca6424f40c7f647a63a3f1742822431d5f0"
inventory = json.loads(subprocess.check_output([
    "git", "show", "HEAD:research/relation_optimized_build_20261127/CACHE_FILE_INVENTORY.json"
]))["files"]
files = {name: Path("/dev/shm") / name for name in plan["cache_files"]}
assert len(files) == 2
for name, path in files.items():
    info = path.lstat()
    assert stat.S_ISREG(info.st_mode) and info.st_nlink == 1
    assert not info.st_mode & 0o111 and info.st_uid == os.getuid()
    assert info.st_size == inventory[name]["bytes"]
    assert stat.S_IMODE(info.st_mode) == inventory[name]["mode"]
    assert digest(path) == inventory[name]["sha256"]

checked = set()
with tarfile.open(archive, "r|xz") as tar:
    for member in tar:
        if member.name in files:
            assert member.isfile() and member.name not in checked
            assert member.size == inventory[member.name]["bytes"]
            h = hashlib.sha256()
            f = tar.extractfile(member)
            while chunk := f.read(1024 * 1024):
                h.update(chunk)
            assert h.hexdigest() == inventory[member.name]["sha256"]
            checked.add(member.name)
assert checked == set(files)

# Scope visibility to live own-user processes; make no global visibility claim.
live_checked = 0
for proc in Path("/proc").iterdir():
    if not proc.name.isdigit():
        continue
    try:
        if proc.stat().st_uid != os.getuid():
            continue
        state = (proc / "stat").read_text().rsplit(") ", 1)[1].split()[0]
        if state == "Z":
            continue
        command = (proc / "comm").read_text().strip()
        assert command not in {"cargo", "rustc", "rust-lld", "ld", "ld.lld"}, command
        for fd in (proc / "fd").iterdir():
            try:
                link = os.readlink(fd)
            except FileNotFoundError:
                continue
            assert link.removesuffix(" (deleted)") not in {str(p) for p in files.values()}
        live_checked += 1
    except (FileNotFoundError, ProcessLookupError):
        continue

protected = workers()
assert protected
with (s / "CACHE_VALIDATED.json").open("x") as f:
    json.dump({"schema": "main99-cache-preunlink/v1", "backup": str(archive),
               "selected_member_bytes_verified": True, "protected_workers": protected,
               "files": {name: inventory[name] for name in files}}, f, indent=2)
    f.write("\n")
subprocess.run(["sha256sum", "-c", str(s / "SOURCE.sha256")], check=True,
               stdout=(s / "raw/cache-source-before.stdout").open("x"))
before = os.statvfs("/dev/shm")
for name, path in files.items():
    assert digest(path) == inventory[name]["sha256"]
    path.unlink()
after = os.statvfs("/dev/shm")
assert workers() == protected
subprocess.run(["sha256sum", "-c", str(s / "SOURCE.sha256")], check=True,
               stdout=(s / "raw/cache-source-after.stdout").open("x"))
reclaimed = (after.f_bavail - before.f_bavail) * after.f_frsize
assert reclaimed >= sum(inventory[name]["bytes"] for name in files)
receipt = {"schema": "main99-two-file-cache-release/v1", "files": {
    name: inventory[name] for name in files}, "backup": str(archive),
    "backup_sha256": digest(archive), "selected_member_bytes_verified": True,
    "reclaimed_filesystem_bytes": reclaimed, "live_own_user_processes_checked": live_checked,
    "protected_workers": protected, "source_and_worker_bytes_unchanged": True,
    "cache_scope": "premerge rebuildable library cache only; backup retained; fresh outputs belong to new source",
    "global_process_visibility_claim": False}
with (s / "CACHE_RELEASE.json").open("x") as f:
    json.dump(receipt, f, indent=2)
    f.write("\n")
print("Two premerge library cache files backed up, verified and released; frozen workers intact.")
