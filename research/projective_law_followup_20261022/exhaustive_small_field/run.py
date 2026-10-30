#!/usr/bin/env python3
"""Compile a byte-preserving test copy of the pinned projective law."""
from __future__ import annotations

import subprocess
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
SOURCE = ROOT / "src/bin/projective_law_gate.rs"
TESTS = HERE / "gate_tests.rs"
GENERATED = ROOT / "src/bin/projective_exhaustive_generated.rs"
LOG = HERE / "cargo-test.log"


def main() -> None:
    if GENERATED.exists():
        raise SystemExit(f"refusing to overwrite {GENERATED}")
    GENERATED.write_bytes(SOURCE.read_bytes() + b"\n" + TESTS.read_bytes())
    command = ["cargo", "test", "--release", "--offline", "--locked",
               "--manifest-path", str(ROOT / "Cargo.toml"), "--bin",
               "projective_exhaustive_generated", "exhaustive_small_field", "--", "--nocapture"]
    try:
        with LOG.open("w") as log:
            outcome = subprocess.run(command, stdout=log, stderr=subprocess.STDOUT)
        if outcome.returncode:
            raise SystemExit(f"cargo test failed with exit {outcome.returncode}; see {LOG}")
        # Cargo emits a spare terminal blank line; normalize only that final
        # separator so the archived text passes the repository whitespace gate.
        LOG.write_text(LOG.read_text().rstrip("\n") + "\n")
    finally:
        GENERATED.unlink()
    print(f"gate passed; raw log: {LOG}")


if __name__ == "__main__":
    main()
