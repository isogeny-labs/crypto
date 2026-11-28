#!/usr/bin/env python3
"""CT328: extract *unchanged source* standalone CT327 rho correctness test.

This never runs the Jacobian/IC benchmark. Only a pre-registered, source-hash
pinned algebraic Rust test is generated; it is not independent signed PCC.
"""
import argparse
import hashlib
import re
from pathlib import Path

EXPECTED_BLOB = "234e14f2aea35b8ef3203065a5e2468aae9eb71f"
FN_NAMES = ("m", "power", "ec_add", "ec_mul", "inverse")


def extract_function(source: str, name: str) -> str:
    hits = list(re.finditer(r"(?m)^fn " + re.escape(name) + r"\(", source))
    if len(hits) != 1:
        raise ValueError(f"expected one top-level function {name}, found {len(hits)}")
    start = hits[0].start()
    opening = source.find("{", start)
    if opening == -1:
        raise ValueError(f"no opening brace for {name}")
    # The five selected mathematical helpers have no literal brace in strings.
    depth = 0
    for pos in range(opening, len(source)):
        if source[pos] == "{":
            depth += 1
        elif source[pos] == "}":
            depth -= 1
            if depth == 0:
                return source[start:pos+1]
    raise ValueError(f"unclosed helper {name}")


def construct(raw: bytes) -> str:
    actual = hashlib.sha1(
        b"blob " + str(len(raw)).encode("ascii") + b"\x00" + raw
    ).hexdigest()
    if actual != EXPECTED_BLOB:
        raise ValueError(f"source Git blob mismatch: got {actual}; expected {EXPECTED_BLOB}")
    source = raw.decode("utf-8")
    decls = []
    for token in (r"const P: i64 = 103;", r"const R: i64 = 109;",
                  r"type Ec = Option<(i64,i64)>;"):
        if source.count(token) != 1:
            raise ValueError(f"missing/ambiguous declaration: {token}")
        decls.append(token)
    opening = "    mod ct327 {"
    stop_marker = "        #[test]\n        fn ct327_same_host_f103_cold_ic_rho_bsgs_triplets()"
    if source.count(opening) != 1 or source.count(stop_marker) != 1:
        raise ValueError("missing or ambiguous original CT327 Rust test boundary")
    start = source.index(opening)
    stop = source.index(stop_marker, start)
    if source.count("        fn ct327_public_source_all_109_and_negative_controls()", start, stop) != 1:
        raise ValueError("the complete original Rust 109-target test was not selected")
    module = source[start:stop]
    # No arithmetic or test-body substitution. Complete the parent module only.
    return ("#![allow(dead_code)]\nuse std::time::Instant;\n"
            + "\n".join(decls) + "\n"
            + "\n\n".join(extract_function(source, name) for name in FN_NAMES)
            + "\n" + module + "\n    }\n")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--source", required=True)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()
    raw = Path(args.source).read_bytes()
    generated = construct(raw)
    output = Path(args.out)
    with output.open("x", encoding="utf-8") as fd:
        fd.write(generated)
    print(f"CT328_ORIGINAL_GIT_BLOB {EXPECTED_BLOB}")
    print(f"CT328_GENERATED_SHA256 {hashlib.sha256(generated.encode()).hexdigest()}")
    print("CT328_SCOPE unchanged F103 EC/rho source + original 109-target native Rust test; no IC/Jacobian")


if __name__ == "__main__":
    main()
