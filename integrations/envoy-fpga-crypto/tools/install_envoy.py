#!/usr/bin/env python3
"""Install the source extension into an existing, exact Envoy v1.39.3 checkout."""
import argparse
from pathlib import Path
import shutil

ROOT = Path(__file__).resolve().parents[1]


def install(checkout: Path):
    checkout = checkout.resolve()
    locations = checkout / "bazel/repository_locations.bzl"
    version = checkout / "VERSION.txt"
    if (not version.is_file() or version.read_text().strip() != "1.39.3" or
            not locations.is_file() or 'version = "0.20260929.0"' not in locations.read_text()):
        raise SystemExit("Expected Envoy v1.39.3 with BoringSSL 0.20260929.0; refusing a different ABI")
    destination = checkout / "source/extensions/fpga_crypto"
    destination.mkdir(parents=True, exist_ok=True)
    for name in ("include", "src", "envoy"):
        shutil.copytree(ROOT / name, destination / name, dirs_exist_ok=True)
    for name in ("BUILD", "config.proto"):
        shutil.copy2(ROOT / name, destination / name)
    config = checkout / "source/extensions/extensions_build_config.bzl"
    text = config.read_text()
    for name in ("fpga_crypto", "pq_crypto"):
        entry = f'    "envoy.filters.http.{name}": "//source/extensions/fpga_crypto:config",\n'
        if entry not in text:
            marker = "EXTENSIONS = {\n"
            if text.count(marker) != 1:
                raise SystemExit("Cannot locate the Envoy extension map")
            text = text.replace(marker, marker + entry)
    config.write_text(text)
    print(f"Installed {destination}")
    print("Build in that checkout with:")
    print("  bazel build --config=clang -c opt --jobs=6 //source/exe:envoy-static")
    print("Use the default non-FIPS BoringSSL configuration; disable LTO and do not use --config=openssl/aws-lc.")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("checkout", type=Path)
    install(parser.parse_args().checkout)
