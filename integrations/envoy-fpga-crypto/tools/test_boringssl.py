#!/usr/bin/env python3
"""Fetch a verified BoringSSL pin, build it, and run actual TLS integration tests."""
import argparse
import hashlib
from pathlib import Path
import shutil
import subprocess
import tarfile
import urllib.request

ROOT = Path(__file__).resolve().parents[1]
VERSION = "0.20260929.0"
SHA256 = "04da9ba0664e0a7f028e961c38d604f2cc6dac852a84ca0e51dc1fa051d4c8fe"


def run():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cmake", default=shutil.which("cmake") or "cmake")
    parser.add_argument("--jobs", type=int, default=4)
    args = parser.parse_args()
    cache = ROOT / ".cache"
    cache.mkdir(exist_ok=True)
    archive = cache / f"boringssl-{VERSION}.tar.gz"
    source = cache / f"boringssl-{VERSION}"
    build = cache / "boringssl-build"
    if not archive.exists():
        url = f"https://codeload.github.com/google/boringssl/tar.gz/refs/tags/{VERSION}"
        with urllib.request.urlopen(url, timeout=120) as response:
            data = response.read()
        if hashlib.sha256(data).hexdigest() != SHA256:
            raise SystemExit("BoringSSL archive checksum mismatch")
        archive.write_bytes(data)
    if hashlib.sha256(archive.read_bytes()).hexdigest() != SHA256:
        raise SystemExit("BoringSSL archive checksum mismatch")
    if not source.is_dir():
        with tarfile.open(archive) as bundle:
            bundle.extractall(cache, filter="data")
    subprocess.run([args.cmake, "-S", str(source), "-B", str(build),
                    "-DCMAKE_BUILD_TYPE=Release", "-DBUILD_TESTING=OFF"], check=True)
    subprocess.run([args.cmake, "--build", str(build), "--target", "ssl", "crypto", "-j", str(args.jobs)], check=True)
    subprocess.run(["make", "test-boringssl", f"BORINGSSL_SRC={source}", f"BORINGSSL_BUILD={build}"], cwd=ROOT, check=True)


if __name__ == "__main__":
    run()
