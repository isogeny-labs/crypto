"""Retain/release only the failed build's three rebuildable cache files."""
from pathlib import Path
import hashlib
import json
import os
import stat
import subprocess
import tarfile

s = Path(__file__).resolve().parent
assert (s / "raw/build.exit").read_text().strip() == "143"
assert not (s / "CACHE_V2_RELEASE.json").exists()
for proc in Path("/proc").iterdir():
    if not proc.name.isdigit():
        continue
    try:
        if proc.stat().st_uid == os.getuid():
            state = (proc / "stat").read_text().rsplit(") ", 1)[1].split()[0]
            if state != "Z":
                assert (proc / "comm").read_text().strip() not in {"cargo", "rustc", "ld", "rust-lld", "ld.lld"}
    except (FileNotFoundError, ProcessLookupError):
        continue

def digest(path):
    h = hashlib.sha256()
    with path.open("rb") as f:
        while b := f.read(1024 * 1024):
            h.update(b)
    return h.hexdigest()

root = Path("/dev/shm/relation-root-build-20261127/release/build/crypto/b1af7d8773d35e6c/out")
names = ["crypto_lib-b1af7d8773d35e6c.crypto_lib.1d0e72462248c3b4-cgu.00.rcgu.o",
         "crypto_lib-b1af7d8773d35e6c.d", "libcrypto_lib-b1af7d8773d35e6c.rmeta"]
paths = [root / n for n in names]
inventory = {}
for p in paths:
    info = p.lstat()
    assert stat.S_ISREG(info.st_mode) and info.st_nlink == 1 and not info.st_mode & 0o111
    assert info.st_uid == os.getuid()
    inventory[str(p.relative_to("/dev/shm"))] = {"bytes": info.st_size,
        "mode": stat.S_IMODE(info.st_mode), "sha256": digest(p)}
backup = Path("/dev/shm/relation-main99-failed-root16-cache-20261127.tar.xz")
assert not backup.exists()
with tarfile.open(backup, "w:xz", preset=6) as t:
    for p in paths:
        t.add(p, arcname=str(p.relative_to("/dev/shm")), recursive=False)
with tarfile.open(backup) as t:
    assert {m.name for m in t.getmembers()} == set(inventory)
    for m in t.getmembers():
        assert m.isfile()
        f = t.extractfile(m)
        h = hashlib.sha256()
        while b := f.read(1024 * 1024):
            h.update(b)
        assert h.hexdigest() == inventory[m.name]["sha256"]
protected = json.loads((s / "CACHE_RELEASE.json").read_text())["protected_workers"]
assert all(digest(Path(p)) == h for p, h in protected.items())
subprocess.run(["sha256sum", "-c", str(s / "SOURCE_V2.sha256")], check=True,
               stdout=(s / "raw/cache-v2-source-before.stdout").open("x"))
receipt = {"schema": "failed-root16-cache-v2/v1", "backup": str(backup),
           "backup_sha256": digest(backup), "archive_bytes": backup.stat().st_size,
           "files": inventory, "all_member_bytes_verified": True,
           "protected_workers": protected, "scope": "RAM-local rebuildable cache, not mathematical evidence"}
with (s / "CACHE_V2_BACKUP.json").open("x") as f:
    json.dump(receipt, f, indent=2)
    f.write("\n")
before = os.statvfs("/dev/shm")
for p in paths:
    assert digest(p) == inventory[str(p.relative_to("/dev/shm"))]["sha256"]
    p.unlink()
after = os.statvfs("/dev/shm")
assert all(digest(Path(p)) == h for p, h in protected.items())
subprocess.run(["sha256sum", "-c", str(s / "SOURCE_V2.sha256")], check=True,
               stdout=(s / "raw/cache-v2-source-after.stdout").open("x"))
receipt["reclaimed_filesystem_bytes"] = (after.f_bavail - before.f_bavail) * after.f_frsize
receipt["source_and_workers_unchanged"] = True
with (s / "CACHE_V2_RELEASE.json").open("x") as f:
    json.dump(receipt, f, indent=2)
    f.write("\n")
print("Exactly3 failed-context cache files preserved and released; source/frozen workers unchanged.")
