#!/usr/bin/env python3
"""Build the pinned liboqs software plugin without an OpenSSL dependency."""
import argparse
import hashlib
from pathlib import Path
import shutil
import subprocess
import tarfile
import urllib.request

ROOT = Path(__file__).resolve().parents[1]
VERSION = "0.16.0"
SHA256 = "162d5b510518ee5f285f82fa1f16402a885176e818bf1b1a4c3c91c9a2f01eae"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cmake", default=shutil.which("cmake") or "cmake")
    parser.add_argument("--jobs", type=int, default=4)
    parser.add_argument("--cache", type=Path, default=ROOT / ".cache")
    parser.add_argument("--test", action="store_true")
    args = parser.parse_args()
    if not 1 <= args.jobs <= 256:
        parser.error("--jobs must be between 1 and 256")
    cache = args.cache.resolve()
    cache.mkdir(parents=True, exist_ok=True)
    archive = cache / f"liboqs-{VERSION}.tar.gz"
    source, build = cache / f"liboqs-{VERSION}", cache / "liboqs-build"
    if not archive.exists():
        url = f"https://codeload.github.com/open-quantum-safe/liboqs/tar.gz/refs/tags/{VERSION}"
        with urllib.request.urlopen(url, timeout=120) as response:
            data = response.read()
        if hashlib.sha256(data).hexdigest() != SHA256:
            raise SystemExit("liboqs archive checksum mismatch")
        archive.write_bytes(data)
    if hashlib.sha256(archive.read_bytes()).hexdigest() != SHA256:
        raise SystemExit("liboqs archive checksum mismatch")
    if not source.is_dir():
        with tarfile.open(archive) as bundle:
            bundle.extractall(cache, filter="data")
    subprocess.run([args.cmake, "-S", str(source), "-B", str(build),
                    "-DCMAKE_BUILD_TYPE=Release", "-DCMAKE_POSITION_INDEPENDENT_CODE=ON",
                    "-DOQS_BUILD_ONLY_LIB=ON", "-DOQS_USE_OPENSSL=OFF", "-DBUILD_SHARED_LIBS=OFF",
                    "-DOQS_ENABLE_SIG_STFL_XMSS=OFF", "-DOQS_ENABLE_SIG_STFL_LMS=OFF"], check=True)
    subprocess.run([args.cmake, "--build", str(build), "-j", str(args.jobs)], check=True)
    subprocess.run(["make", "test-pq" if args.test else "pq-plugin", f"OQS_BUILD={build}"], cwd=ROOT, check=True)
    print(f"Software plugin: {ROOT / 'build/libfpga_pq_oqs.so'}")


if __name__ == "__main__":
    main()
