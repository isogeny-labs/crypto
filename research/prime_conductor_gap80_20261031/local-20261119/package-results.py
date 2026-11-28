#!/usr/bin/env python3
"""Deterministic packaging and byte verification only; computes no statistics."""
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile

HERE = Path(__file__).resolve().parent


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def pack(root, output):
    with output.open("wb") as stream:
        tar = subprocess.Popen([
            "tar", "--sort=name", "--mtime=2026-11-19T00:00:00Z",
            "--owner=0", "--group=0", "--numeric-owner", "--format=gnu",
            "--mode=u+rwX,go+rX,go-w", "-cf", "-", "results",
        ], cwd=root, stdout=subprocess.PIPE)
        try:
            subprocess.run(["xz", "-6", "-T1"], stdin=tar.stdout,
                           stdout=stream, check=True)
        finally:
            tar.stdout.close()
        if tar.wait() != 0:
            raise RuntimeError("tar failed")


def main():
    archive = HERE / "local-results.tar.xz"
    pack(HERE, archive)
    files = [{"path": str(p.relative_to(HERE)), "sha256": digest(p),
              "bytes": p.stat().st_size}
             for p in sorted((HERE / "results").rglob("*")) if p.is_file()]
    with tempfile.TemporaryDirectory(prefix="conductor-local-repack-") as name:
        restored = Path(name)
        subprocess.run(["tar", "-xJf", str(archive), "-C", name], check=True)
        actual = [{"path": str(p.relative_to(restored)), "sha256": digest(p),
                   "bytes": p.stat().st_size}
                  for p in sorted((restored / "results").rglob("*")) if p.is_file()]
        assert actual == files, "extracted bytes differ"
        repack = restored / "repacked.tar.xz"
        pack(restored, repack)
        assert digest(repack) == digest(archive), "repack differs"
    sha = digest(archive)
    receipt = {
        "schema": "conductor.local-publication-pending/v1",
        "archive": archive.name, "sha256": sha, "bytes": archive.stat().st_size,
        "desired_s3_uri": f"s3://setec-astronomy-artifacts/store/sha256/{sha[:2]}/{sha}",
        "uploaded": False, "remote_verified": False,
        "publication_blocker": "AWS STS exits255: Unable to locate credentials; see publication/sts.stderr",
        "extraction_hash_check": "pass", "byte_identical_repack": True,
        "files": files,
    }
    (HERE / "publication-pending.json").write_text(json.dumps(receipt, indent=2) + "\n")
    print(json.dumps({k: v for k, v in receipt.items() if k != "files"}, indent=2))


if __name__ == "__main__":
    main()
