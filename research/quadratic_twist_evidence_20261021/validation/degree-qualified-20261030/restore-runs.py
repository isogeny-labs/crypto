"""Restore original raw bytes from the archive plus immutable canonical copies."""
from pathlib import Path
import hashlib, lzma, shutil, sys, tarfile

here = Path(__file__).resolve().parent
study = here.parents[1]
out = Path(sys.argv[1]).resolve()
assert not out.exists(), "fresh output required"
out.mkdir(parents=True)
canonical = {}
for variant in ("degree-qualified-20261030", "degree-retained-20261030"):
    for p in (study / "native" / variant).rglob("*.json"):
        canonical.setdefault(hashlib.sha256(p.read_bytes()).hexdigest(), p)
with tarfile.open(here / "runs.tar.xz") as tf:
    tf.extractall(out, filter="data")
entries = []
for line in lzma.decompress((here / "RUNS.sha256.xz").read_bytes()).decode().splitlines():
    digest, name = line.split("  ", 1)
    assert not Path(name).is_absolute() and ".." not in Path(name).parts
    p = out / name
    if not p.exists():
        p.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(canonical[digest], p)
    assert hashlib.sha256(p.read_bytes()).hexdigest() == digest, name
    entries.append((digest, p))
if "--repack" in sys.argv[2:]:
    packed = out / "repacked.tar.xz"
    with tarfile.open(packed, "w:xz", preset=9) as tf:
        for digest, p in entries:
            if digest in canonical:
                continue
            info = tf.gettarinfo(str(p), arcname=p.relative_to(out).as_posix())
            info.uid = info.gid = 0
            info.uname = info.gname = ""
            info.mtime = 0
            with p.open("rb") as f:
                tf.addfile(info, f if info.isreg() else None)
    assert packed.read_bytes() == (here / "runs.tar.xz").read_bytes()
print(f"restored {len(entries)} original hashes; repack requested={ '--repack' in sys.argv[2:] }")
