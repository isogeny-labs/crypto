#!/usr/bin/env python3
"""CT330: preserve original F103/rho/BSGS source in standalone paired Rust test."""
import argparse
import hashlib
from pathlib import Path

import ct328_standalone_source as ct328

MODULE_CLOSER = "\n    }\n"
TEST_MARKER = "fn ct330_paired_original_source_rho_bsgs_q73()"


def construct(raw: bytes, append: str) -> str:
    original = ct328.construct(raw)  # original complete Git blob, fail closed
    if not original.endswith(MODULE_CLOSER):
        raise ValueError("unexpected original ct327 module boundary")
    if append.count(TEST_MARKER) != 1 or append.count("#[test]") != 1:
        raise ValueError("ambiguous/missing CT330 independent appended test")
    if "CT330_PAIRED_JSON " not in append:
        raise ValueError("CT330 record boundary missing")
    direct = ct328.extract_function(raw.decode("utf-8"), "direct_bsgs")
    # Only insert additional test code; original Rust is unchanged byte-for-byte.
    return original[:-len(MODULE_CLOSER)] + "\n" + append + MODULE_CLOSER + "\n" + direct + "\n"


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--source", required=True)
    ap.add_argument("--append", required=True)
    ap.add_argument("--out", required=True)
    args = ap.parse_args()
    raw = Path(args.source).read_bytes()
    appendix = Path(args.append).read_text(encoding="utf-8")
    generated = construct(raw, appendix)
    with Path(args.out).open("x", encoding="utf-8") as fd:
        fd.write(generated)
    print("CT330_ORIGINAL_GIT_BLOB", ct328.EXPECTED_BLOB)
    print("CT330_APPEND_SHA256", hashlib.sha256(appendix.encode()).hexdigest())
    print("CT330_TRANSLATION_SHA256", hashlib.sha256(generated.encode()).hexdigest())
    print("CT330_SCOPE original EC/rho/BSGS + appended paired test only; root/Jacobian/IC not built")


if __name__ == "__main__":
    main()
