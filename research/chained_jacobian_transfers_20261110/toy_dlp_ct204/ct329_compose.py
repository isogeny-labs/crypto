#!/usr/bin/env python3
"""CT329: compose pinned original F103 Rust with a separately appended audit."""
import argparse
import hashlib
from pathlib import Path
import ct328_standalone_source as ct328


def main():
    ap=argparse.ArgumentParser()
    ap.add_argument("--source",required=True)
    ap.add_argument("--append",required=True)
    ap.add_argument("--out",required=True)
    args=ap.parse_args()
    raw=Path(args.source).read_bytes()
    original=ct328.construct(raw)  # fail-closed original Git blob gate
    direct=ct328.extract_function(raw.decode("utf-8"),"direct_bsgs")
    suffix=Path(args.append).read_text(encoding="utf-8")
    if suffix.count("mod ct329 {")!=1 or suffix.count("ct329_original_bsgs_executed_additions")!=1:
        raise ValueError("CT329 appended audit missing/ambiguous")
    # Source EC code and direct_bsgs remain exact original text. Any changed
    # source fails the imported CT328 SHA check before output is created.
    new=original+"\n"+direct+"\n"+suffix
    Path(args.out).open("x",encoding="utf-8").write(new)
    print("CT329_SOURCE_BLOB",ct328.EXPECTED_BLOB)
    print("CT329_AUDIT_SHA256",hashlib.sha256(suffix.encode()).hexdigest())
    print("CT329_TRANSLATION_SHA256",hashlib.sha256(new.encode()).hexdigest())
    print("CT329_SCOPE pinned F103 source plus separate instrumented BSGS, no root IC/Cantor")

if __name__=="__main__":
    main()
